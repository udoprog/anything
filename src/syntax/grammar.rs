use std::cmp::Ordering;

use crate::syntax::parser::{Parser, Skip, SyntaxKind};
use rowan::Checkpoint;
use SyntaxKind::*;

/// Parse the root of an expression.
pub fn root(p: &mut Parser<'_>) {
    let mut error = false;

    p.start_node(ROOT);
    let c = p.checkpoint();

    loop {
        let skip = p.count_skip();

        match p.nth(skip, 0) {
            EOF => {
                p.skip(skip);
                break;
            }
            OPEN_BRACE | OPEN_PAREN | WORD | NUMBER => {
                p.skip(skip);

                if !expr(p) {
                    p.error_node_at(c);
                }
            }
            _ => {
                p.skip(skip);
                p.bump();
                error = true;
            }
        }
    }

    if error {
        p.finish_node_at(c, ERROR);
    }

    p.finish_node();
}

fn call_arguments(p: &mut Parser<'_>) -> bool {
    let c = p.checkpoint();

    let skip = loop {
        let skip = p.count_skip();

        match p.nth(skip, 0) {
            CLOSE_PAREN => {
                break skip;
            }
            _ => {
                p.skip(skip);

                if !expr(p) {
                    return false;
                }

                let skip = p.count_skip();

                if !p.eat(skip, &[COMMA]) {
                    break skip;
                }
            }
        }
    };

    p.finish_node_at(c, FN_ARGUMENTS);

    if !p.eat(skip, &[CLOSE_PAREN]) {
        return false;
    }

    true
}

fn value(p: &mut Parser<'_>, skip: Skip) -> Option<Checkpoint> {
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
                p.bump_node(WORD);
                skip = p.count_skip();
                words += 1;
            }

            if words > 1 {
                p.finish_node_at(c, SENTENCE);
            }

            if !p.eat(skip, &[CLOSE_BRACE]) {
                p.bump_until(CLOSE_BRACE);
                return None;
            }

            Some(start)
        }
        WORD => {
            p.skip(skip);
            let start = p.checkpoint();

            let c = p.checkpoint();
            p.bump_node(WORD);

            if let OPEN_PAREN = p.nth(Skip::ZERO, 0) {
                p.finish_node_at(c, FN_NAME);
                p.bump();

                if !call_arguments(p) {
                    return None;
                }

                p.finish_node_at(c, FN_CALL);
                return Some(start);
            }

            let mut skip = p.count_skip();
            let mut is_sentence = false;

            while let WORD | NUMBER = p.nth(skip, 0) {
                p.skip(skip);
                p.bump_node(WORD);
                skip = p.count_skip();
                is_sentence = true;
            }

            if is_sentence {
                p.finish_node_at(c, SENTENCE);
            }

            Some(start)
        }
        NUMBER => {
            p.skip(skip);
            let start = p.checkpoint();

            let c = p.checkpoint();
            p.bump_node(NUMBER);

            let skip = p.count_skip();

            match p.nth(skip, 0) {
                PERCENTAGE => {
                    p.skip(skip);
                    p.bump();
                    p.finish_node_at(c, PERCENTAGE);
                }
                WORD | NUMBER => {
                    if unit(p, skip) {
                        p.finish_node_at(c, WITH_UNIT);
                    } else {
                        p.error_node_at(c);
                    }
                }
                _ => {
                    p.finish_node_at(c, NUMBER);
                }
            }

            Some(start)
        }
        OPEN_PAREN => {
            p.skip(skip);
            let start = p.checkpoint();

            p.bump();

            if !expr(p) {
                return None;
            }

            let skip = p.count_skip();

            if !p.eat(skip, &[CLOSE_PAREN]) {
                return None;
            }

            Some(start)
        }
        _ => None,
    }
}

/// Helper to parse a hierarchy of expressions which can have differen
/// priorities. These are called operations and are separated by operators.
fn operations<T>(
    p: &mut Parser<'_>,
    operand: fn(p: &mut Parser<'_>, Skip, T) -> Option<(Checkpoint, Skip)>,
    op: fn(p: &mut Parser<'_>, Skip) -> Option<(i32, SyntaxKind, usize, T)>,
) -> bool
where
    T: Copy + Default,
{
    let start = p.checkpoint();

    let mut stack: Vec<(Checkpoint, i32, T)> = vec![];
    let mut first = true;

    loop {
        let skip = p.count_skip();

        let extra = stack.last().map(|e| e.2).unwrap_or_default();

        let (c, skip) = match operand(p, skip, extra) {
            Some((c, skip)) => (c, skip),
            None => return false,
        };

        let (priority, operator, steps, extra) = match op(p, skip) {
            Some(n) => n,
            None => break,
        };

        if std::mem::take(&mut first) {
            stack.push((start, priority, extra));
        }

        while let Some(prev) = stack.last_mut() {
            match priority.cmp(&prev.1) {
                Ordering::Less => {
                    p.finish_node_at(prev.0, OPERATION);
                    *prev = (prev.0, priority, extra);
                    continue;
                }
                Ordering::Greater => {
                    stack.push((c, priority, extra));
                    break;
                }
                Ordering::Equal => {
                    break;
                }
            }
        }

        // Defer the skip as long as possible so it's not included in the
        // OPERATION span.
        p.skip(skip);

        let c = p.checkpoint();

        for _ in 0..steps {
            p.bump();
        }

        p.finish_node_at(c, operator);
    }

    while let Some((last, _, _)) = stack.pop() {
        p.finish_node_at(last, OPERATION);
    }

    true
}

/// Parse an expression.
pub fn expr(p: &mut Parser<'_>) -> bool {
    return operations(p, operand, op);

    fn operand(p: &mut Parser<'_>, skip: Skip, is_unit: bool) -> Option<(Checkpoint, Skip)> {
        let c = if is_unit {
            p.skip(skip);
            let c = p.checkpoint();

            if !unit(p, Skip::ZERO) {
                return None;
            }

            c
        } else {
            value(p, skip)?
        };

        Some((c, p.count_skip()))
    }

    /// Get the binding power of an operator.
    fn op(p: &mut Parser<'_>, skip: Skip) -> Option<(i32, SyntaxKind, usize, bool)> {
        let out = match p.nth(skip, 0) {
            TO => (1, OP_CAST, 1, true),
            PLUS => (2, OP_ADD, 1, false),
            DASH => (2, OP_SUB, 1, false),
            STAR => (3, OP_MUL, 1, false),
            SLASH => (3, OP_DIV, 1, false),
            CARET | STARSTAR => (10, OP_POWER, 1, false),
            _ => return None,
        };

        Some(out)
    }
}

/// Parse a unit.
pub fn unit(p: &mut Parser<'_>, mut skip: Skip) -> bool {
    let mut start = None;

    loop {
        // lead
        match p.nth(skip, 0) {
            NUMBER => {
                p.skip(skip);
                start.get_or_insert_with(|| p.checkpoint());
                p.bump_node(NUMBER);
            }
            WORD => {
                p.skip(skip);
                start.get_or_insert_with(|| p.checkpoint());
                p.bump_node(WORD);
            }
            TO if skip == Skip::ZERO => {
                p.skip(skip);
                start.get_or_insert_with(|| p.checkpoint());
                p.bump_node(WORD);
            }
            _ => break,
        }

        // trailing
        loop {
            let what = match p.nth(Skip::ZERO, 0) {
                WORD | TO => WORD,
                NUMBER => NUMBER,
                STAR => OP_MUL,
                SLASH => OP_DIV,
                CARET | STARSTAR => OP_POWER,
                _ => break,
            };

            p.bump_node(what);
        }

        skip = p.count_skip();
    }

    if let Some(c) = start {
        p.finish_node_at(c, UNIT);
        true
    } else {
        false
    }
}
