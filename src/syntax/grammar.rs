use crate::syntax::parser::{Parser, Skip, SyntaxKind};
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

                let c = p.checkpoint();

                if !expr(p, None) {
                    p.error_node_at(c);
                }

                p.finish_node_at(c, OPERATION);
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

fn unit_component(c: &mut Parser<'_>, mut word: bool) {
    loop {
        match c.nth(Skip::ZERO, 0) {
            UNIT_WORD => {
                word = true;
                c.bump();
            }
            UNIT_ESCAPED_WORD => {
                c.bump();
            }
            STAR if word => {
                word = false;
                c.bump();
            }
            CARET if word => {
                word = false;

                if !c.eat(Skip::ZERO, &[CARET, UNIT_NUMBER]) {
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }
}

pub(crate) fn unit(p: &mut Parser<'_>) -> bool {
    p.set_mode(true, true);
    let skip = p.count_skip();

    let unit = match p.nth(skip, 0) {
        UNIT_WORD | UNIT_ESCAPED_WORD => {
            p.skip(skip);
            p.set_mode(true, false);

            let c = p.checkpoint();
            unit_component(p, false);

            if p.eat(Skip::ZERO, &[SLASH]) {
                unit_component(p, true);
            }

            p.finish_node_at(c, UNIT);
            true
        }
        _ => false,
    };

    p.set_mode(false, true);
    unit
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

                let c = p.checkpoint();

                if !expr(p, None) {
                    return false;
                }

                p.finish_node_at(c, OPERATION);

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
            p.bump();

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

            while let (WORD, _) | (AS | TO, WORD) = (p.nth(skip, 0), p.nth(skip, 1)) {
                p.skip(skip);
                p.bump();
                skip = p.count_skip();
            }

            p.finish_node_at(c, SENTENCE);
            true
        }
        NUMBER => {
            p.skip(skip);

            let c = p.checkpoint();
            p.bump();

            match p.nth(Skip::ZERO, 0) {
                PERCENTAGE => {
                    p.bump();
                    p.finish_node_at(c, PERCENTAGE);
                }
                _ => {
                    p.finish_node_at(c, NUMBER);

                    if unit(p) {
                        p.finish_node_at(c, NUMBER_WITH_UNIT);
                    }
                }
            }

            true
        }
        OPEN_PAREN => {
            p.skip(skip);

            let c = p.checkpoint();
            p.bump();

            if !expr(p, None) {
                return false;
            }

            if !p.eat(skip, &[CLOSE_PAREN]) {
                return false;
            }

            p.finish_node_at(c, OPERATION);
            true
        }
        _ => false,
    }
}

fn expr(p: &mut Parser<'_>, level: Option<u32>) -> bool {
    if !expr_nested(p, level) {
        return false;
    }

    loop {
        let skip = p.count_skip();

        if !matches!(p.nth(skip, 0), AS | TO) {
            break;
        }

        p.skip(skip);

        p.start_node(OPERATOR);
        p.bump();
        p.finish_node();

        if !unit(p) {
            return false;
        }
    }

    true
}

fn expr_nested(p: &mut Parser<'_>, mut level: Option<u32>) -> bool {
    let mut last = p.checkpoint();

    if !value(p) {
        return false;
    }

    loop {
        let skip = p.count_skip();

        match op(p.nth(skip, 0)) {
            None => {
                break;
            }
            Some(n) => {
                p.skip(skip);
                p.bump_node(OPERATOR);

                match level {
                    None => {
                        level = Some(n);
                    }
                    Some(c) if c < n => {
                        if !expr(p, Some(n)) {
                            return false;
                        }

                        p.finish_node_at(last, OPERATION);
                        continue;
                    }
                    Some(c) if c > n => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        last = p.checkpoint();

        if !value(p) {
            return false;
        }
    }

    true
}

/// Get the binding power of an operator.
fn op(kind: SyntaxKind) -> Option<u32> {
    match kind {
        PLUS | DASH => Some(1),
        STAR | SLASH => Some(2),
        CARET => Some(3),
        _ => None,
    }
}
