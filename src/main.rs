use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{Files, SimpleFiles};
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::io::Write;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let mut it = std::env::args();
    it.next();
    let query = it.collect::<Vec<_>>().join(" ");

    let db = facts::Db::open()?;

    let mut out = StandardStream::stdout(ColorChoice::Auto);

    let mut files = SimpleFiles::new();
    let id = files.add("<in>", query);

    let config = codespan_reporting::term::Config::default();

    for value in facts::query(files.source(id)?, &db) {
        match value {
            Ok(value) => {
                writeln!(out, "{}", value)?;
            }
            Err(e) => {
                let labels = vec![Label::primary(id, e.range()).with_message(e.to_string())];
                let diagnostic = Diagnostic::error()
                    .with_message(e.to_string())
                    .with_labels(labels);
                term::emit(&mut out, &config, &files, &diagnostic)?;
            }
        }
    }

    Ok(())
}
