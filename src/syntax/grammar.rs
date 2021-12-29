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

fn value(p: &mut Parser<'_>) -> bool {
    let skip = p.count_skip();

    match p.nth(skip, 0) {
        // Escape sequence.
        OPEN_BRACE => {
            p.skip(skip);
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
                return false;
            }

            true
        }
        WORD => {
            p.skip(skip);

            let c = p.checkpoint();
            p.bump_node(WORD);

            if let OPEN_PAREN = p.nth(Skip::ZERO, 0) {
                p.finish_node_at(c, FN_NAME);
                p.bump();

                if !call_arguments(p) {
                    return false;
                }

                p.finish_node_at(c, FN_CALL);
                return true;
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

            true
        }
        NUMBER => {
            p.skip(skip);

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
                    p.skip(skip);

                    if unit(p) {
                        p.finish_node_at(c, WITH_UNIT);
                    } else {
                        p.error_node_at(c);
                    }
                }
                _ => {
                    p.finish_node_at(c, NUMBER);
                }
            }

            true
        }
        OPEN_PAREN => {
            p.skip(skip);
            p.bump();

            if !expr(p) {
                return false;
            }

            let skip = p.count_skip();

            if !p.eat(skip, &[CLOSE_PAREN]) {
                return false;
            }

            true
        }
        _ => false,
    }
}

/// Helper to parse a hierarchy of expressions which can have differen
/// priorities. These are called operations and are separated by operators.
fn operations<T>(
    p: &mut Parser<'_>,
    operand: fn(p: &mut Parser<'_>, T) -> bool,
    op: fn(p: &mut Parser<'_>) -> Option<(i32, SyntaxKind, usize, T)>,
) -> bool
where
    T: Copy + Default,
{
    let start = p.checkpoint();

    let mut stack: Vec<(Checkpoint, i32, T)> = vec![];
    let mut first = true;

    loop {
        let skip = p.count_skip();
        p.skip(skip);

        let last = p.checkpoint();

        let extra = stack.last().map(|e| e.2).unwrap_or_default();

        if !operand(p, extra) {
            return false;
        }

        let (next, operator, steps, extra) = match op(p) {
            Some(n) => n,
            None => break,
        };

        if std::mem::take(&mut first) {
            stack.push((start, next, extra));
        }

        while let Some((pop_last, pop_current, pop_extra)) = stack.pop() {
            match (pop_current - next).signum() {
                -1 => {
                    stack.push((pop_last, pop_current, pop_extra));
                    stack.push((last, next, extra));
                    break;
                }
                1 => {
                    p.finish_node_at(pop_last, OPERATION);
                    stack.push((pop_last, next, extra));
                }
                _ => {
                    stack.push((pop_last, pop_current, pop_extra));
                    break;
                }
            }
        }

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

    fn operand(p: &mut Parser<'_>, is_unit: bool) -> bool {
        if is_unit {
            let skip = p.count_skip();
            p.skip(skip);
            unit(p)
        } else {
            value(p)
        }
    }

    /// Get the binding power of an operator.
    fn op(p: &mut Parser<'_>) -> Option<(i32, SyntaxKind, usize, bool)> {
        let skip = p.count_skip();

        let out = match p.nth(skip, 0) {
            TO => (1, OP_CAST, 1, true),
            PLUS => (2, OP_ADD, 1, false),
            DASH => (2, OP_SUB, 1, false),
            STAR => (3, OP_MUL, 1, false),
            SLASH => (3, OP_DIV, 1, false),
            CARET | STARSTAR => (10, OP_POWER, 1, false),
            _ => return None,
        };

        p.skip(skip);
        Some(out)
    }
}

/// Parse a unit.
pub fn unit(p: &mut Parser<'_>) -> bool {
    return operations(p, operand, op);

    fn operand(p: &mut Parser<'_>, (): ()) -> bool {
        match p.nth(Skip::ZERO, 0) {
            NUMBER => {
                p.bump_node(NUMBER);
                true
            }
            WORD | TO => {
                p.bump_node(WORD);
                true
            }
            _ => false,
        }
    }

    /// Get the binding power of an operator.
    fn op(p: &mut Parser<'_>) -> Option<(i32, SyntaxKind, usize, ())> {
        let out = match p.nth(Skip::ZERO, 0) {
            STAR => (3, OP_MUL, 1, ()),
            SLASH => (3, OP_DIV, 1, ()),
            WORD => (3, OP_IMPLICIT_MUL, 0, ()),
            CARET | STARSTAR => (10, OP_POWER, 1, ()),
            _ => return None,
        };

        Some(out)
    }
}
