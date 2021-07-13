use crate::parser::Parser;
use crate::parser::Skip;
use crate::parser::SyntaxKind;
use rowan::Checkpoint;
use SyntaxKind::*;

/// Parse the root of an expression.
pub fn root(p: &mut Parser<'_>) {
    p.start_node(ROOT);

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

                if !expr(p, c, None) {
                    p.error_node_at(c);
                }
            }
            _ => {
                p.skip(skip);
                p.bump();
            }
        }
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

fn value(p: &mut Parser<'_>) -> bool {
    let skip = p.count_skip();

    match p.nth(skip, 0) {
        WORD => {
            p.skip(skip);

            let c = p.checkpoint();
            p.bump();

            let mut skip = p.count_skip();

            while let WORD = p.nth(skip, 0) {
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
            p.bump();

            let c = p.checkpoint();

            if !expr(p, c, None) {
                return false;
            }

            let skip = p.count_skip();
            p.eat(skip, &[CLOSE_PAREN])
        }
        _ => false,
    }
}

fn expr(p: &mut Parser<'_>, check: Checkpoint, mut level: Option<u32>) -> bool {
    let mut last = p.checkpoint();

    if !value(p) {
        return false;
    }

    loop {
        let skip = p.count_skip();

        match op(p.nth(skip, 0)) {
            None => {
                if level.is_some() {
                    p.finish_node_at(check, OPERATION);
                }

                break;
            }
            Some(n) => {
                p.skip(skip);
                p.bump_node(OPERATOR);

                match level {
                    None => {
                        level = Some(n);
                    }
                    Some(c) => {
                        if c < n {
                            if !expr(p, last, Some(n)) {
                                return false;
                            }

                            continue;
                        } else if c > n {
                            p.finish_node_at(check, OPERATION);
                            break;
                        }
                    }
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
