use crate::compound::Compound;
use anyhow::{anyhow, Context, Result};
use hashbrown::{HashMap, HashSet};
use num::BigRational;
use std::fmt;
use std::sync::Arc;

const MATH: &[u8] = include_bytes!("../db/math.toml");
const PHYSICS: &[u8] = include_bytes!("../db/physics.toml");
const DISTANCES: &[u8] = include_bytes!("../db/distances.toml");

const SOURCES: [(&str, &[u8]); 3] = [
    ("math", MATH),
    ("physics", PHYSICS),
    ("distances", DISTANCES),
];

const SEED: u64 = 0x681da70f3e1e3494;

/// A match from the database.
pub(crate) enum Match<'a> {
    /// A constant was matched.
    Constant(&'a DbConstant),
}

/// The hash of the constant.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct Hash(u64);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#08x}", self.0)
    }
}

pub(crate) struct Hasher(());

impl Hasher {
    pub(crate) fn hash(&self, s: &str) -> Hash {
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
pub(crate) struct DbConstant {
    names: HashSet<Box<str>>,
    pub(crate) value: BigRational,
    pub(crate) unit: Compound,
}

impl DbConstant {
    /// If the given constant matches.
    fn matches(&self, s: &str) -> bool {
        self.names.contains(s)
    }
}

/// The database of facts.
pub struct Db {
    hasher: Hasher,
    constants: HashMap<Hash, Vec<Arc<DbConstant>>>,
}

impl Db {
    /// Open the default database.
    pub fn open() -> Result<Self> {
        let hasher = Hasher(());

        let mut db = Self {
            hasher,
            constants: HashMap::new(),
        };

        for (name, source) in SOURCES.iter().copied() {
            db.load_bytes(source)
                .with_context(|| anyhow!("loading: {}", name))?;
        }

        Ok(db)
    }

    /// Perform a lookup over the given string.
    pub(crate) fn lookup<'a>(&'a self, s: &str) -> Option<Match<'a>> {
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

    /// Load a document from the given bytes.
    pub(crate) fn load_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let doc: self::serde::Doc = toml::de::from_slice(bytes)?;

        for c in doc.constants {
            let constant = Arc::new(DbConstant {
                names: c.names.iter().cloned().collect(),
                value: c.value,
                unit: c
                    .unit
                    .as_deref()
                    .map(str::parse::<Compound>)
                    .transpose()?
                    .unwrap_or_default(),
            });

            for name in &constant.names {
                let hash = self.hasher.hash(name);
                self.constants
                    .entry(hash)
                    .or_default()
                    .push(constant.clone());
            }
        }

        Ok(())
    }
}

pub(crate) mod serde {
    use num::BigRational;
    use serde::{de, Deserialize};
    use std::borrow::Cow;

    use crate::numeric::parse_decimal_big_rational;

    #[derive(Debug, Deserialize)]
    pub struct Doc {
        #[serde(default)]
        pub constants: Vec<Constant>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Constant {
        pub names: Vec<Box<str>>,
        #[serde(deserialize_with = "des_value")]
        pub value: BigRational,
        #[serde(default)]
        pub unit: Option<Box<str>>,
    }

    fn des_value<'de, D>(d: D) -> Result<BigRational, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        match parse_decimal_big_rational(Cow::<str>::deserialize(d)?.as_ref()) {
            Ok(ratio) => Ok(ratio),
            Err(e) => Err(<D::Error as de::Error>::custom(e)),
        }
    }
}
