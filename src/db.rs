use crate::compound::Compound;
use anyhow::{anyhow, Context, Result};
use num::BigRational;
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, QueryParserError};
use tantivy::schema::{Field, Schema, Value, STORED, TEXT};
use tantivy::{Document, Index, IndexReader, ReloadPolicy, TantivyError};
use thiserror::Error;

/// Error that can happen during lookup.
#[derive(Debug, Error)]
pub enum LookupError {
    #[error("search error: {error}")]
    TantivyError {
        #[source]
        #[from]
        error: TantivyError,
    },
    #[error("bad query: {error}")]
    QueryParserError {
        #[source]
        #[from]
        error: QueryParserError,
    },
}

const MATH: &[u8] = include_bytes!("../db/math.toml");
const PHYSICS: &[u8] = include_bytes!("../db/physics.toml");
const DISTANCES: &[u8] = include_bytes!("../db/distances.toml");

const SOURCES: [(&str, &[u8]); 3] = [
    ("math", MATH),
    ("physics", PHYSICS),
    ("distances", DISTANCES),
];

/// A match from the database.
pub(crate) enum Match<'a> {
    /// A constant was matched.
    Constant(&'a Constant),
}

/// A single constant.
#[derive(Debug)]
pub(crate) struct Constant {
    pub(crate) names: Vec<Box<str>>,
    pub(crate) value: BigRational,
    pub(crate) unit: Compound,
}

/// The database of facts.
pub struct Db {
    index: Index,
    reader: IndexReader,
    field_id: Field,
    field_name: Field,
    constants: Vec<Constant>,
}

impl Db {
    /// Open the default database.
    pub fn open() -> Result<Self> {
        let mut schema = Schema::builder();

        let field_id = schema.add_u64_field("id", STORED);
        let field_name = schema.add_text_field("name", TEXT);

        let index = Index::create_in_ram(schema.build());

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()?;

        let mut db = Self {
            index,
            reader,
            field_id,
            field_name,
            constants: Vec::new(),
        };

        for (name, source) in SOURCES.iter().copied() {
            db.load_bytes(source)
                .with_context(|| anyhow!("loading: {}", name))?;
        }

        Ok(db)
    }

    /// Perform a lookup over the given string.
    pub(crate) fn lookup<'a>(&'a self, query: &str) -> Result<Option<Match<'a>>, LookupError> {
        let searcher = self.reader.searcher();

        dbg!(query);

        let query_parser = QueryParser::for_index(&self.index, vec![self.field_name]);
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        for (_score, id) in top_docs {
            let doc = searcher.doc(id)?;

            let value = doc.get_first(self.field_id);

            dbg!(value);

            if let Some(Value::U64(id)) = value {
                if let Some(c) = self.constants.get(*id as usize) {
                    return Ok(Some(Match::Constant(c)));
                }
            }
        }

        Ok(None)
    }

    /// Load a document from the given bytes.
    pub(crate) fn load_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let doc: self::serde::Doc = toml::de::from_slice(bytes)?;

        let mut writer = self.index.writer(50_000_000)?;

        for c in doc.constants {
            let id = self.constants.len() as u64;

            let mut doc = Document::default();
            doc.add_u64(self.field_id, id);

            for name in &c.names {
                doc.add_text(self.field_name, name.as_ref());
            }

            let constant = Constant {
                names: c.names,
                value: c.value,
                unit: c
                    .unit
                    .as_deref()
                    .map(str::parse::<Compound>)
                    .transpose()?
                    .unwrap_or_default(),
            };

            self.constants.push(constant);
            writer.add_document(doc);
        }

        writer.commit()?;
        self.reader.reload()?;
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
