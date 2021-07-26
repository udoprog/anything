use anyhow::{bail, Result};
use genco::prelude::*;
use serde::{de, Deserialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Base {
    variant: String,
    names: Vec<String>,
    unit: String,
    prefix_bias: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Derived {
    #[serde(deserialize_with = "id_deserializer")]
    id: u32,
    variant: String,
    names: Vec<String>,
    name: String,
    prefix_bias: Option<i32>,
}

impl Derived {
    /// The name of the constant to use when generating stuff.
    fn constant_name(&self) -> &str {
        if let Some((_, last)) = self.name.rsplit_once("::") {
            last
        } else {
            &self.name
        }
    }
}

fn id_deserializer<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = Cow::<'de, str>::deserialize(deserializer)?;
    u32::from_str_radix(&s.as_ref()[2..], 16).map_err(<D::Error as de::Error>::custom)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Unit {
    #[serde(rename = "base")]
    Base(Base),
    #[serde(rename = "derived")]
    Derived(Derived),
}

impl Unit {
    fn names(&self) -> &[String] {
        match self {
            Unit::Base(base) => &base.names,
            Unit::Derived(derived) => &derived.names,
        }
    }

    fn variant(&self) -> &str {
        match self {
            Unit::Base(base) => &base.variant,
            Unit::Derived(derived) => &derived.variant,
        }
    }

    fn prefix_bias(&self) -> Option<i32> {
        match self {
            Unit::Base(base) => base.prefix_bias,
            Unit::Derived(derived) => derived.prefix_bias,
        }
    }

    fn display(&self) -> String {
        match self {
            Unit::Base(base) => base.unit.clone(),
            Unit::Derived(derived) => format!("Unit::Derived(units::{})", derived.name),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Prefix {
    variant: String,
    names: Vec<String>,
    prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct Doc {
    units: Vec<Unit>,
    prefixes: Vec<Prefix>,
}

pub fn ids(doc: &Doc) -> Result<rust::Tokens> {
    let mut seen = HashMap::new();
    let mut derived = Vec::new();

    for unit in &doc.units {
        if let Unit::Derived(d) = unit {
            if let Some(old) = seen.insert(d.id, d) {
                bail!(
                    "tried to register `{}` which has a conflicting id with `{}`",
                    d.variant,
                    old.variant
                )
            }

            derived.push(d);
        }
    }

    let unit_derived = &rust::import("crate::unit", "Derived");
    let units = &rust::import("crate", "units");

    Ok(quote! {
        #(for d in derived.iter().copied() => pub const #(d.constant_name()): u32 = #(d.id);#<push>)

        #("/// Match the given id to the corresponding derived unit")
        pub fn id_to_derived(id: u32) -> Option<#unit_derived> {
            match id {
                #(for d in derived.iter().copied() => #(d.id) => Some(#units::#(&d.name)),#<push>)
                _ => None,
            }
        }
    })
}

pub fn parser(doc: &Doc) -> rust::Tokens {
    let prefixes = doc
        .prefixes
        .iter()
        .flat_map(|p| p.names.iter().cloned().map(move |n| (n, p)))
        .collect::<HashMap<_, _>>();

    let mut suffix_units = HashMap::new();

    for unit in &doc.units {
        for name in unit.names() {
            if let Some(prefix) = prefixes.get(name) {
                suffix_units.insert(prefix.variant.clone(), unit);
            }
        }
    }

    let mut productive_units = Vec::new();

    for unit in &doc.units {
        if !unit.names().iter().all(|name| prefixes.contains_key(name)) {
            productive_units.push(unit);
        }
    }

    let logos = &rust::import("logos", "Logos");
    let unit = &rust::import("crate::unit", "Unit");
    let prefix = &rust::import("crate::prefix", "Prefix");
    let units = &rust::import("crate", "units");

    quote! {
        #(register(unit))
        #(register(prefix))
        #(register(units))

        #[derive(#logos, Debug, Clone, Copy, PartialEq, Eq)]
        enum Combined {
            #(for unit in &productive_units => #(ref t {
                for name in unit.names() {
                    if !prefixes.contains_key(name) {
                        quote_in!(*t => #[token(#(quoted(name)))]#<push>);
                    }
                }

                quote_in!(*t => #(unit.variant()),#<push>);
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
                for name in unit.names() {
                    quote_in!(*t => #[token(#(quoted(name)))]#<push>);
                }

                quote_in!(*t => #(unit.variant()),#<push>);
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
                        Combined::#(unit.variant()) => {
                            #(if let Some(bias) = unit.prefix_bias() => prefix += #bias;)
                            #(unit.display())
                        }
                    )
                    #(for p in &doc.prefixes join(#<push>) =>
                        Combined::#(&p.variant) => {
                            #(if let Some(unit) = suffix_units.get(&p.variant) {
                                if lexer.remainder().is_empty() {
                                    #(if let Some(bias) = unit.prefix_bias() => prefix += #bias;)
                                    return Some(("", prefix, #(unit.display())));
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
                        Units::#(unit.variant()) => {
                            #(if let Some(bias) = unit.prefix_bias() => prefix += #bias;)
                            break #(unit.display());
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
}
