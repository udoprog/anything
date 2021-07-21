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
            OPEN_PAREN | WORD | NUMBER => {
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

            while let WORD = p.nth(skip, 0) {
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

/// Parse an expression.
pub fn expr(p: &mut Parser<'_>) -> bool {
    let start = p.checkpoint();

    let mut stack: Vec<(Checkpoint, i32, bool)> = vec![];
    let mut first = true;

    loop {
        let skip = p.count_skip();
        p.skip(skip);

        let last = p.checkpoint();

        if stack.last().map(|e| e.2).unwrap_or_default() {
            if !unit(p) {
                return false;
            }
        } else {
            if !value(p) {
                return false;
            }
        }

        let skip = p.count_skip();

        let (next, operator, unit) = match op(p.nth(skip, 0)) {
            Some(n) => n,
            None => break,
        };

        if std::mem::take(&mut first) {
            stack.push((start, next, unit));
        }

        while let Some((pop_last, pop_current, pop_unit)) = stack.pop() {
            match (pop_current - next).signum() {
                -1 => {
                    stack.push((pop_last, pop_current, pop_unit));
                    stack.push((last, next, unit));
                    break;
                }
                1 => {
                    p.finish_node_at(pop_last, OPERATION);
                    stack.push((pop_last, next, unit));
                }
                _ => {
                    stack.push((pop_last, pop_current, pop_unit));
                    break;
                }
            }
        }

        p.skip(skip);
        p.bump_node(operator);
    }

    while let Some((last, _, _)) = stack.pop() {
        p.finish_node_at(last, OPERATION);
    }

    return true;

    /// Get the binding power of an operator.
    fn op(kind: SyntaxKind) -> Option<(i32, SyntaxKind, bool)> {
        let out = match kind {
            TO => (1, OP_CAST, true),
            PLUS => (2, OP_ADD, false),
            DASH => (2, OP_SUB, false),
            STAR => (3, OP_MUL, false),
            SLASH => (3, OP_DIV, false),
            CARET => (10, OP_POWER, false),
            _ => return None,
        };

        Some(out)
    }
}

fn unit_component(p: &mut Parser<'_>) -> bool {
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

/// Parse a unit.
pub fn unit(p: &mut Parser<'_>) -> bool {
    let start = p.checkpoint();

    let mut stack = vec![];
    let mut first = true;

    let mut last = p.checkpoint();

    if !unit_component(p) {
        return false;
    }

    loop {
        let (next, operator, steps) = match op(p.nth(Skip::ZERO, 0)) {
            Some(n) => n,
            None => break,
        };

        if std::mem::take(&mut first) {
            stack.push((start, next));
        }

        while let Some((pop_last, pop_current)) = stack.pop() {
            match (pop_current - next).signum() {
                -1 => {
                    stack.push((pop_last, pop_current));
                    stack.push((last, next));
                    break;
                }
                1 => {
                    p.finish_node_at(pop_last, OPERATION);
                    stack.push((pop_last, next));
                }
                _ => {
                    stack.push((pop_last, pop_current));
                    break;
                }
            }
        }

        let c = p.checkpoint();

        for _ in 0..steps {
            p.bump();
        }

        p.finish_node_at(c, operator);

        last = p.checkpoint();

        if !unit_component(p) {
            return false;
        }
    }

    while let Some((last, _)) = stack.pop() {
        p.finish_node_at(last, OPERATION);
    }

    return true;

    /// Get the binding power of an operator.
    fn op(kind: SyntaxKind) -> Option<(i32, SyntaxKind, usize)> {
        let out = match kind {
            STAR => (3, OP_MUL, 1),
            SLASH => (3, OP_DIV, 1),
            WORD => (3, OP_IMPLICIT_MUL, 0),
            CARET => (10, OP_POWER, 1),
            _ => return None,
        };

        Some(out)
    }
}
