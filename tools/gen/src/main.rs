use anyhow::Result;
use genco::prelude::*;
use genco::{fmt, quote_in};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct Unit {
    variant: String,
    names: Vec<String>,
    unit: String,
    prefix_bias: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Prefix {
    variant: String,
    names: Vec<String>,
    prefix: String,
}

#[derive(Debug, Deserialize)]
struct Doc {
    units: Vec<Unit>,
    prefixes: Vec<Prefix>,
}

fn write(doc: Doc) -> rust::Tokens {
    let prefixes = doc
        .prefixes
        .iter()
        .flat_map(|p| p.names.iter().cloned().map(move |n| (n, p)))
        .collect::<HashMap<_, _>>();

    let mut suffix_units = HashMap::new();

    for unit in &doc.units {
        for name in &unit.names {
            if let Some(prefix) = prefixes.get(name) {
                suffix_units.insert(prefix.variant.clone(), unit);
            }
        }
    }

    let mut productive_units = Vec::new();

    for unit in &doc.units {
        if !unit.names.iter().all(|name| prefixes.contains_key(name)) {
            productive_units.push(unit);
        }
    }

    let mut t = rust::Tokens::new();

    let logos = &rust::import("logos", "Logos");
    let unit = &rust::import("crate::unit", "Unit");
    let prefix = &rust::import("crate::prefix", "Prefix");
    let units = &rust::import("crate", "units");

    quote_in! {
        t =>
        #(register(unit))
        #(register(prefix))
        #(register(units))

        #[derive(#logos, Debug, Clone, Copy, PartialEq, Eq)]
        enum Combined {
            #(for unit in &productive_units => #(ref t {
                for name in &unit.names {
                    if !prefixes.contains_key(name) {
                        quote_in!(*t => #[token(#(quoted(name)))]#<push>);
                    }
                }

                quote_in!(*t => #(&unit.variant),#<push>);
            }))
            #("/// Prefixes")
            #(for unit in &doc.prefixes => #(ref t {
                for name in &unit.names {
                    quote_in!(*t => #[token(#(quoted(name)))]#<push>);
                }

                quote_in!(*t => #(&unit.variant),#<push>);
            }))
            #[token("-")]
            Separator,
            #[error]
            Error,
        }

        #[derive(#logos, Debug, Clone, Copy, PartialEq, Eq)]
        enum Units {
            #(for unit in &doc.units => #(ref t {
                for name in &unit.names {
                    quote_in!(*t => #[token(#(quoted(name)))]#<push>);
                }

                quote_in!(*t => #(&unit.variant),#<push>);
            }))
            #[token("-")]
            Separator,
            #[error]
            Error,
        }

        #("/// Generated unit parsing function")
        pub fn parse(s: &str) -> Option<(&str, i32, Unit)> {
            let mut lexer = Combined::lexer(s);
            let mut prefix = 0;

            loop {
                let token = lexer.next()?;

                let unit = match token {
                    #(for unit in &productive_units join(#<push>) =>
                        Combined::#(&unit.variant) => {
                            #(if let Some(bias) = unit.prefix_bias => prefix += #bias;)
                            #(&unit.unit)
                        }
                    )
                    #(for p in &doc.prefixes join(#<push>) =>
                        Combined::#(&p.variant) => {
                            #(if let Some(unit) = suffix_units.get(&p.variant) {
                                if lexer.remainder().is_empty() {
                                    #(if let Some(bias) = unit.prefix_bias => prefix += #bias;)
                                    return Some(("", prefix, #(&unit.unit)));
                                }#<line>
                            })
                            prefix += #(&p.prefix);
                            break;
                        }
                    )
                    Combined::Separator => {
                        continue;
                    }
                    Combined::Error => {
                        return None;
                    }
                };

                return Some((lexer.remainder(), prefix, unit));
            };

            let mut lexer = Units::lexer(lexer.remainder());

            let unit = loop {
                let token = lexer.next()?;

                match token {
                    #(for unit in &doc.units join(#<push>) =>
                        Units::#(&unit.variant) => {
                            #(if let Some(bias) = unit.prefix_bias => prefix += #bias;)
                            break #(&unit.unit);
                        }
                    )
                    Units::Separator => {
                        continue;
                    }
                    Units::Error => {
                        return None;
                    }
                }
            };

            Some((lexer.remainder(), prefix, unit))
        }
    }

    t
}

fn main() -> Result<()> {
    let out = Path::new("src").join("generated").join("unit.rs");
    let units_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("data.toml");
    let units = fs::read(units_path)?;
    let doc: Doc = toml::from_slice(&units)?;

    let t = write(doc);

    let out = fs::File::create(out)?;

    let mut w = fmt::IoWriter::new(out);
    let fmt = fmt::Config::from_lang::<Rust>().with_indentation(fmt::Indentation::Space(4));
    let config = rust::Config::default();

    t.format_file(&mut w.as_formatter(&fmt), &config)?;
    Ok(())
}
