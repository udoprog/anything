use std::cmp::Ordering;

use crate::syntax::parser::{Parser, Skip, Syntax};
use syntree::{Id, TreeError};

use Syntax::*;

/// Parse the root of an expression.
pub fn root(p: &mut Parser<'_>) -> Result<(), TreeError> {
    let mut error = false;
    let mut skip = p.count_skip();

    let c = p.checkpoint();

    loop {
        match p.nth(skip, 0) {
            EOF => {
                p.skip(skip);
                break;
            }
            OPEN_BRACE | OPEN_PAREN | WORD | NUMBER => {
                if let Some(s) = operation(p, skip)? {
                    skip = s;
                } else {
                    p.error_node_at(c)?;
                }
            }
            _ => {
                p.skip(skip);
                p.bump();
                error = true;
                skip = p.count_skip();
            }
        }
    }

    if error {
        p.close_at(c, ERROR)?;
    }

    Ok(())
}

fn call_arguments(p: &mut Parser<'_>) -> Result<bool, TreeError> {
    let c = p.checkpoint();

    let skip = loop {
        let skip = p.count_skip();

        match p.nth(skip, 0) {
            CLOSE_PAREN => {
                break skip;
            }
            _ => {
                let skip = if let Some(s) = operation(p, skip)? {
                    s
                } else {
                    return Ok(false);
                };

                if !p.eat(skip, &[COMMA]) {
                    break skip;
                }
            }
        }
    };

    p.close_at(c, FN_ARGUMENTS)?;
    Ok(p.eat(skip, &[CLOSE_PAREN]))
}

fn value(p: &mut Parser<'_>, skip: Skip) -> Result<Option<Id>, TreeError> {
    match p.nth(skip, 0) {
        // Escape sequence.
        OPEN_BRACE => {
            p.skip(skip);
            let start = p.checkpoint();

            p.bump();

            let c = p.checkpoint();
            let mut skip = p.count_skip();
            let mut words = 0usize;

            while let WORD = p.nth(skip, 0) {
                p.skip(skip);
                p.bump_node(WORD)?;
                skip = p.count_skip();
                words += 1;
            }

            if words > 1 {
                p.close_at(c, SENTENCE)?;
            }

            if !p.eat(skip, &[CLOSE_BRACE]) {
                p.bump_until(CLOSE_BRACE);
                return Ok(None);
            }

            Ok(Some(start))
        }
        WORD => {
            p.skip(skip);
            let start = p.checkpoint();

            let c = p.checkpoint();
            p.bump_node(WORD)?;

            if let OPEN_PAREN = p.nth(Skip::ZERO, 0) {
                p.close_at(c, FN_NAME)?;
                p.bump();

                if !call_arguments(p)? {
                    return Ok(None);
                }

                p.close_at(c, FN_CALL)?;
                return Ok(Some(start));
            }

            let mut skip = p.count_skip();
            let mut is_sentence = false;

            while let WORD | NUMBER = p.nth(skip, 0) {
                p.skip(skip);
                p.bump_node(WORD)?;
                skip = p.count_skip();
                is_sentence = true;
            }

            if is_sentence {
                p.close_at(c, SENTENCE)?;
            }

            Ok(Some(start))
        }
        NUMBER => {
            p.skip(skip);
            let c = p.checkpoint();
            p.bump();

            let skip = p.count_skip();

            let kind = match p.nth(skip, 0) {
                PERCENTAGE => {
                    p.skip(skip);
                    p.bump();
                    PERCENTAGE
                }
                _ => {
                    if unit(p, skip)?.is_some() {
                        WITH_UNIT
                    } else {
                        NUMBER
                    }
                }
            };

            p.close_at(c, kind)?;
            Ok(Some(c))
        }
        OPEN_PAREN => {
            p.skip(skip);
            let c = p.checkpoint();
            p.bump();

            let skip = match operation(p, skip)? {
                Some(skip) => skip,
                None => return Ok(None),
            };

            if !p.eat(skip, &[CLOSE_PAREN]) {
                return Ok(None);
            }

            Ok(Some(c))
        }
        _ => Ok(None),
    }
}

/// Parse an operation. An operation is [value]s separated by one or more
/// operators of the same priority.
pub fn operation(p: &mut Parser<'_>, mut skip: Skip) -> Result<Option<Skip>, TreeError> {
    let open = p.checkpoint();

    let mut stack = Vec::<(Id, i32, bool)>::new();
    let mut first = true;

    loop {
        let is_unit = stack.last().map(|e| e.2).unwrap_or_default();

        let cur = match operand(p, skip, is_unit)? {
            Some(c) => c,
            None => return Ok(None),
        };

        let (priority, operator, extra, cur_skip) = match op(p) {
            Some(out) => out,
            None => break,
        };

        if std::mem::take(&mut first) {
            stack.push((open, priority, extra));
        }

        while let Some(prev) = stack.last_mut() {
            match priority.cmp(&prev.1) {
                Ordering::Less => {
                    p.close_at(prev.0, OPERATION)?;
                    *prev = (prev.0, priority, extra);
                    continue;
                }
                Ordering::Greater => {
                    stack.push((cur, priority, extra));
                    break;
                }
                Ordering::Equal => {
                    break;
                }
            }
        }

        // Defer the skip as long as possible so it's not included in the
        // OPERATION span.
        p.skip(cur_skip);
        p.bump_node(operator)?;
        skip = p.count_skip();
    }

    while let Some((last, _, _)) = stack.pop() {
        p.close_at(last, OPERATION)?;
    }

    return Ok(Some(skip));

    fn operand(p: &mut Parser<'_>, skip: Skip, is_unit: bool) -> Result<Option<Id>, TreeError> {
        let c = if is_unit {
            p.skip(skip);
            unit(p, Skip::ZERO)?
        } else {
            value(p, skip)?
        };

        Ok(c)
    }

    /// Get the binding power of an operator.
    fn op(p: &mut Parser<'_>) -> Option<(i32, Syntax, bool, Skip)> {
        let skip = p.count_skip();

        let (prio, kind, is_unit) = match p.nth(skip, 0) {
            TO => (1, OP_CAST, true),
            PLUS => (2, OP_ADD, false),
            DASH => (2, OP_SUB, false),
            STAR => (3, OP_MUL, false),
            SLASH => (3, OP_DIV, false),
            CARET | STARSTAR => (10, OP_POWER, false),
            _ => return None,
        };

        Some((prio, kind, is_unit, skip))
    }
}

/// Parse a unit.
pub fn unit(p: &mut Parser<'_>, mut skip: Skip) -> Result<Option<Id>, TreeError> {
    let mut c = None;

    'outer: loop {
        // lead
        let kind = match p.nth(skip, 0) {
            NUMBER => NUMBER,
            WORD => WORD,
            _ => break,
        };

        p.skip(skip);

        if c.is_none() {
            c = Some(p.checkpoint());
        }

        p.bump_node(kind)?;

        // Trailing no-skip symbols.
        skip = loop {
            let kind = match p.nth(Skip::ZERO, 0) {
                WORD | TO => WORD,
                NUMBER => NUMBER,
                STAR => OP_MUL,
                SLASH => OP_DIV,
                CARET | STARSTAR => OP_POWER,
                WHITESPACE => break Skip::ONE,
                _ => break 'outer,
            };

            p.bump_node(kind)?;
        };
    }

    if let Some(c) = c {
        p.close_at(c, UNIT)?;
    }

    Ok(c)
}
