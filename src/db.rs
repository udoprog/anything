use crate::unit::Unit;
use anyhow::Result;
use hashbrown::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

const PHYSICS: &[u8] = include_bytes!("../db/physics.toml");
const SEED: u64 = 0x681da70f3e1e3494;

/// A matched thing from the database.
pub enum Match<'a> {
    /// A constant was matched.
    Constant(&'a Constant),
}

/// Open the database.
pub fn open() -> Result<Db> {
    let hasher = Hasher(());

    let doc: self::serde::Doc = toml::de::from_slice(PHYSICS)?;

    let mut constants: HashMap<Hash, Vec<Arc<Constant>>> = HashMap::new();

    for c in doc.constants {
        let constant = Arc::new(Constant {
            names: c.names.iter().cloned().collect(),
            value: c.value,
            unit: c
                .unit
                .as_deref()
                .map(str::parse::<Unit>)
                .transpose()?
                .unwrap_or_default(),
        });

        for name in &constant.names {
            let hash = hasher.hash(name);
            constants.entry(hash).or_default().push(constant.clone());
        }
    }

    Ok(Db { hasher, constants })
}

/// The hash of the constant.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Hash(u64);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#08x}", self.0)
    }
}

pub struct Hasher(());

impl Hasher {
    pub fn hash(&self, s: &str) -> Hash {
        use std::hash::{Hash as _, Hasher};

        let mut hash = twox_hash::xxh3::Hash64::with_seed(SEED);

        for p in s.split(char::is_whitespace) {
            let p = p.trim();

            if p.is_empty() {
                continue;
            }

            eudex::Hash::new(p).hash(&mut hash);
        }

        Hash(hash.finish())
    }
}

/// A single constant.
#[derive(Debug)]
pub struct Constant {
    names: HashSet<Box<str>>,
    pub value: f64,
    pub unit: Unit,
}

impl Constant {
    /// If the given constant matches.
    fn matches(&self, s: &str) -> bool {
        self.names.contains(s)
    }
}

/// The database of facts.
pub struct Db {
    hasher: Hasher,
    constants: HashMap<Hash, Vec<Arc<Constant>>>,
}

impl Db {
    /// Construct a hash.
    pub fn hash(&self, s: &str) -> Hash {
        self.hasher.hash(s)
    }

    pub fn lookup<'a>(&'a self, s: &str) -> Option<Match<'a>> {
        let hash = self.hasher.hash(s);

        if let Some(matches) = self.constants.get(&hash) {
            for m in matches {
                if m.matches(s) {
                    return Some(Match::Constant(m));
                }
            }
        }

        None
    }
}

pub(crate) mod serde {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Doc {
        pub constants: Vec<Constant>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Constant {
        pub names: Vec<Box<str>>,
        pub value: f64,
        #[serde(default)]
        pub unit: Option<Box<str>>,
    }
}
