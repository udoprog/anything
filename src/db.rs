use std::fs;

use crate::compound::Compound;
use anyhow::{anyhow, Context, Result};
use num::BigRational;
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, QueryParserError};
use tantivy::schema::{
    Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, Value, STORED,
};
use tantivy::tokenizer::{LowerCaser, NgramTokenizer, TextAnalyzer};
use tantivy::{Document, Index, IndexReader, IndexWriter, ReloadPolicy, TantivyError};
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

/// A match from the database.
pub(crate) enum Match {
    /// A constant was matched.
    Constant(Constant),
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
    field_data: Field,
    field_name: Field,
}

impl Db {
    /// Open the default database.
    pub fn open() -> Result<Self> {
        let mut config = crate::config::open()?;
        let hash = config.hash_assets();

        let mut rebuild = match config.meta.database_hash.as_deref() {
            Some(existing) => existing != hash,
            None => true,
        };

        let force_rebuild = match config.meta.version.as_deref() {
            Some(version) => version != config.this_version,
            _ => false,
        };

        if force_rebuild {
            if config.index_path.is_dir() {
                log::info!("removing index (outdated): {}", config.index_path.display());
                fs::remove_dir_all(&config.index_path)?;
            }
        }

        let index = if config.index_path.is_dir() {
            log::trace!("opening index: {}", config.index_path.display());
            Index::open_in_dir(&config.index_path).ok()
        } else {
            None
        };

        let index = if let Some(index) = index {
            index
        } else {
            rebuild = true;

            fs::create_dir_all(&config.index_path)?;

            let text_field_indexing = TextFieldIndexing::default()
                .set_tokenizer("ngram")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions);
            let text_options = TextOptions::default()
                .set_indexing_options(text_field_indexing)
                .set_stored();

            let mut schema = Schema::builder();
            schema.add_bytes_field("data", STORED);

            schema.add_text_field("name", text_options);
            Index::create_in_dir(&config.index_path, schema.build())?
        };

        let tokenizer = TextAnalyzer::from(NgramTokenizer::new(1, 7, true)).filter(LowerCaser);

        index.tokenizers().register("ngram", tokenizer);

        let schema = index.schema();

        let field_data = schema
            .get_field("data")
            .ok_or_else(|| anyhow!("missing field `data`"))?;

        let field_name = schema
            .get_field("name")
            .ok_or_else(|| anyhow!("missing field `name`"))?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()?;

        let mut db = Self {
            index,
            reader,
            field_data,
            field_name,
        };

        if rebuild {
            log::info!("rebuilding search index at {}", config.index_path.display());

            let mut writer = db.index.writer(50_000_000)?;
            writer.delete_all_documents()?;

            for name in config.assets() {
                if let Some(content) = config.get_asset(name.as_ref()) {
                    db.load_bytes(&mut writer, content.as_ref())
                        .with_context(|| anyhow!("loading: {}", name))?;
                }
            }

            writer.commit()?;
            db.reader.reload()?;

            config.meta.version = Some(config.this_version.to_owned());
            config.meta.database_hash = Some(hash);
            config.write_meta()?;
        }

        Ok(db)
    }

    /// Perform a lookup over the given string.
    pub(crate) fn lookup(&self, query: &str) -> Result<Option<Match>, LookupError> {
        let searcher = self.reader.searcher();

        let query_parser = QueryParser::for_index(&self.index, vec![self.field_name]);
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        for (_score, id) in top_docs {
            let doc = searcher.doc(id)?;

            if let Some(Value::Bytes(data)) = doc.get_first(self.field_data) {
                let c: serde::Constant = match serde_cbor::from_slice(data) {
                    Ok(c) => c,
                    Err(..) => continue,
                };

                let unit = match c.unit.as_deref().map(str::parse::<Compound>).transpose() {
                    Ok(unit) => unit,
                    Err(..) => continue,
                };

                let c = Constant {
                    names: c.names,
                    value: c.value,
                    unit: unit.unwrap_or_default(),
                };

                return Ok(Some(Match::Constant(c)));
            }
        }

        Ok(None)
    }

    /// Load a document from the given bytes.
    pub(crate) fn load_bytes(&mut self, writer: &mut IndexWriter, bytes: &[u8]) -> Result<()> {
        let doc: self::serde::Doc = toml::de::from_slice(bytes)?;

        for c in doc.constants {
            let mut doc = Document::default();
            doc.add_bytes(self.field_data, serde_cbor::to_vec(&c)?);

            for name in &c.names {
                doc.add_text(self.field_name, name.as_ref());
            }

            writer.add_document(doc);
        }

        Ok(())
    }
}

pub(crate) mod serde {
    use num::BigRational;
    use serde::{de, Deserialize, Serialize};
    use std::borrow::Cow;

    use crate::numeric::parse_decimal_big_rational;

    #[derive(Debug, Deserialize)]
    pub struct Doc {
        #[serde(default)]
        pub constants: Vec<RawConstant>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RawConstant {
        pub names: Vec<Box<str>>,
        #[serde(flatten)]
        pub content: serde_cbor::Value,
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
