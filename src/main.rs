use anyhow::{anyhow, bail, Result};
use facts::db;
use facts::parser;
use facts::parser::{SyntaxKind, SyntaxNode};
use facts::Numeric;
use facts::Unit;
use rowan::NodeOrToken;
use std::collections::VecDeque;
use SyntaxKind::*;

#[allow(unused)]
fn print(node: SyntaxNode) {
    let mut queue = VecDeque::new();
    queue.push_back((0, NodeOrToken::Node(node)));

    while let Some((indent, n)) = queue.pop_front() {
        print!("{:indent$}", "", indent = indent);

        match n {
            NodeOrToken::Node(node) => {
                println!("- {:?}", node.kind());
                let v = node.children_with_tokens().collect::<Vec<_>>();

                for child in v.into_iter().rev() {
                    queue.push_front((indent + 2, child));
                }
            }
            NodeOrToken::Token(token) => {
                println!(
                    "- {:?} {:?} ({:?})",
                    token.text(),
                    token.kind(),
                    token.text_range()
                );
            }
        }
    }
}

fn add(a: Numeric, b: Numeric) -> Result<Numeric> {
    if a.unit == b.unit || !b.unit.is_empty() {
        bail!("cannot add together the units `{} + {}`", a.unit, b.unit)
    }

    Ok(Numeric::new(a.value + b.value, a.unit))
}

fn sub(a: Numeric, b: Numeric) -> Result<Numeric> {
    if a.unit == b.unit || !b.unit.is_empty() {
        bail!("cannot subtract the units `{} - {}`", a.unit, b.unit)
    }

    Ok(Numeric::new(a.value - b.value, a.unit))
}

fn div(a: Numeric, b: Numeric) -> Result<Numeric> {
    let unit = match a.unit.clone().div(b.unit.clone()) {
        Some(unit) => unit,
        None => {
            bail!("cannot divide the units `{} / {}`", a.unit, b.unit);
        }
    };

    Ok(Numeric::new(a.value / b.value, unit))
}

fn mul(a: Numeric, b: Numeric) -> Result<Numeric> {
    let unit = a.unit.mul(b.unit);
    Ok(Numeric::new(a.value * b.value, unit))
}

/// Evaluate the syntax node.
fn eval(source: &str, node: SyntaxNode, db: &db::Db) -> Result<Numeric> {
    match node.kind() {
        OPERATION => {
            let mut it = node.children();
            let mut base = eval(source, it.next().unwrap(), db)?;

            while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
                let op = op
                    .first_token()
                    .map(|t| t.kind())
                    .ok_or_else(|| anyhow!("missing op"))?;
                let rhs = eval(source, rhs, db)?;

                let op = match op {
                    PLUS => add,
                    DASH => sub,
                    SLASH => div,
                    STAR => mul,
                    kind => {
                        bail!("unsuported op: {:?}", kind);
                    }
                };

                base = op(base, rhs)?;
            }

            Ok(base)
        }
        NUMBER => {
            let s = &source[node.text_range()];
            let int = str::parse::<f64>(s)?;
            Ok(Numeric::new(int, Unit::empty()))
        }
        NUMBER_WITH_UNIT => {
            let mut it = node.children();

            let number = it.next().unwrap();
            let number = &source[number.text_range()];
            let number = str::parse::<f64>(number)?;

            let unit = it.next().unwrap();
            let unit = &source[unit.text_range()];
            let unit = str::parse::<Unit>(unit)?;

            Ok(Numeric::new(number, unit))
        }
        SENTENCE => {
            let s = &source[node.text_range()];

            let m = match db.lookup(s) {
                Some(m) => m,
                None => bail!("found nothing matching `{}`", s),
            };

            match m {
                db::Match::Constant(c) => Ok(Numeric::new(c.value, c.unit.clone())),
            }
        }
        PERCENTAGE => {
            let number = node.first_token().expect("number of percentage");
            let number = &source[number.text_range()];
            let number = str::parse::<f64>(number)?;
            Ok(Numeric::new(number / 100.0, Unit::empty()))
        }
        kind => {
            bail!("unsupported expression: {:?}", kind)
        }
    }
}

fn main() -> Result<()> {
    let mut it = std::env::args();
    it.next();
    let query = it.collect::<Vec<_>>().join(" ");

    let db = db::open()?;

    let parser = parser::Parser::new(&query);
    let node = parser.parse();

    // print(node.clone());

    for expr in node.children() {
        let value = eval(&query, expr, &db)?;
        println!("{}", value);
    }

    Ok(())
}
