use crate::compound::Compound;
use crate::rational::Rational;
use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, QueryParserError};
use tantivy::schema::{
    Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, Value, STORED,
};
use tantivy::tokenizer::{LowerCaser, NgramTokenizer, TextAnalyzer};
use tantivy::{Document, Index, IndexReader, IndexWriter, ReloadPolicy, TantivyError};
use thiserror::Error;

const SOURCES_BIN_GZ: &str = "sources.bin.gz";

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    /// The identifier of a source.
    #[serde(default)]
    pub source: Option<u64>,
    /// Tokens that can be queried for.
    #[serde(default)]
    pub tokens: Vec<Box<str>>,
    /// The description of a constant.
    pub description: Box<str>,
    /// The value of a constant.
    pub value: Rational,
    /// The unit of a constant.
    pub unit: Compound,
}

/// A single source.
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    /// The unique identifier of a source.
    pub id: u64,
    /// The description of a source.
    pub description: Box<str>,
    /// The URL that a source came from.
    pub url: Option<Box<str>>,
}

#[derive(Debug, Default)]
struct Sources {
    /// Vector of sources.
    sources: Vec<Source>,
    /// Map of identifiers to source index.
    map: HashMap<u64, usize>,
}

impl<'de> Deserialize<'de> for Sources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawSources {
            /// Vector of sources.
            sources: Vec<Source>,
        }

        let sources = RawSources::deserialize(deserializer)?;

        let mut map = HashMap::new();

        for (index, source) in sources.sources.iter().enumerate() {
            map.insert(source.id, index);
        }

        Ok(Self {
            sources: sources.sources,
            map,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Doc {
    #[serde(default)]
    pub constants: Vec<PartialConstant>,
    #[serde(default)]
    pub sources: Vec<Source>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialConstant {
    pub tokens: Vec<Box<str>>,
    #[serde(flatten)]
    pub content: serde_cbor::Value,
}

/// The database of facts.
pub struct Db {
    sources: Sources,
    index: Index,
    reader: IndexReader,
    field_data: Field,
    field_name: Field,
}

impl Db {
    /// Get information on the given source by Id.
    pub fn get_source(&self, id: u64) -> Option<&Source> {
        let index = self.sources.map.get(&id).copied()?;
        self.sources.sources.get(index)
    }

    /// Only open the database in-memory.
    pub fn in_memory() -> Result<Self> {
        Self::open_inner(true)
    }

    /// Open the default database.
    pub fn open() -> Result<Self> {
        Self::open_inner(false)
    }

    fn open_inner(in_memory: bool) -> Result<Self> {
        let mut config = crate::config::open()?;

        let hash = config.hash_assets();

        let mut rebuild = match config.meta.database_hash.as_deref() {
            Some(existing) if !in_memory => existing != hash,
            _ => true,
        };

        let index = if in_memory {
            let schema = build_schema();
            Index::create_in_ram(schema)
        } else {
            let (index_rebuild, index) = open_index(&config)?;
            rebuild = rebuild || index_rebuild;
            index
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

        let sources = match config.get_asset(SOURCES_BIN_GZ) {
            Some(bytes) => load_bytes(bytes.data.as_ref())?,
            None => Default::default(),
        };

        let mut db = Self {
            sources,
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
                if name == SOURCES_BIN_GZ {
                    continue;
                }

                if let Some(content) = config.get_asset(name.as_ref()) {
                    db.load_bytes(&mut writer, content.data.as_ref())
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
        let top_docs = searcher.search(&query, &TopDocs::with_limit(1))?;

        for (_score, id) in top_docs {
            let doc = searcher.doc(id)?;

            if let Some(Value::Bytes(data)) = doc.get_first(self.field_data) {
                let c: Constant = match serde_cbor::from_slice(data) {
                    Ok(c) => c,
                    Err(..) => continue,
                };

                return Ok(Some(Match::Constant(c)));
            }
        }

        Ok(None)
    }

    /// Load a document from the given bytes.
    pub(crate) fn load_bytes(&mut self, writer: &mut IndexWriter, bytes: &[u8]) -> Result<()> {
        let doc: Doc = load_bytes(bytes)?;

        for c in doc.constants {
            let mut doc = Document::default();
            doc.add_bytes(self.field_data, serde_cbor::to_vec(&c)?);

            for token in &c.tokens {
                doc.add_text(self.field_name, token.as_ref());
            }

            writer.add_document(doc);
        }

        Ok(())
    }
}

fn open_index(config: &crate::config::Config) -> Result<(bool, Index)> {
    let force_rebuild = match config.meta.version.as_deref() {
        Some(version) => version != config.this_version,
        _ => true,
    };

    if !force_rebuild {
        if let Some(index) = Index::open_in_dir(&config.index_path).ok() {
            log::trace!("opened index: {}", config.index_path.display());
            return Ok((false, index));
        }
    }

    if config.index_path.is_dir() {
        log::info!("removing index: {}", config.index_path.display());
        fs::remove_dir_all(&config.index_path)?;
    }

    fs::create_dir_all(&config.index_path)?;
    let schema = build_schema();
    Ok((true, Index::create_in_dir(&config.index_path, schema)?))
}

fn build_schema() -> Schema {
    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("ngram")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();

    let mut schema = Schema::builder();
    schema.add_bytes_field("data", STORED);

    schema.add_text_field("name", text_options);
    schema.build()
}

fn load_bytes<T>(bytes: &[u8]) -> Result<T, serde_cbor::Error>
where
    for<'de> T: Deserialize<'de>,
{
    let bytes = GzDecoder::new(Cursor::new(bytes));
    serde_cbor::from_reader(bytes)
}
