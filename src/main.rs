use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{Files, SimpleFiles};
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::io::Write;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let mut it = std::env::args();
    it.next();

    let mut help = false;
    let mut describe = false;
    let mut query = Vec::new();

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--" => break,
            "--help" => {
                help = true;
            }
            "--describe" => {
                describe = true;
            }
            _ => {
                query.push(arg);
            }
        }
    }

    let mut out = StandardStream::stdout(ColorChoice::Auto);

    if help {
        writeln!(out, "facts [--help] [--describe] [--] <query>")?;
        writeln!(out)?;
        writeln!(out, " --help     - Show this help.")?;
        writeln!(
            out,
            " --describe - Describe the contents of the query (constants used)."
        )?;
        return Ok(());
    }

    query.extend(it);

    let query = query.join(" ");

    let db = facts::Db::open()?;

    let mut files = SimpleFiles::new();
    let id = files.add("<in>", query);

    let config = codespan_reporting::term::Config::default();

    let options = facts::Options::default();
    let mut descriptions = Vec::new();

    let options = if describe {
        options.describe()
    } else {
        options
    };

    for value in facts::query(files.source(id)?, &db, options, &mut descriptions) {
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

    if !descriptions.is_empty() {
        writeln!(out, "# Description of constants used (--describe):")?;

        for description in descriptions {
            match description {
                facts::Description::Constant(query, c) => {
                    write!(out, "{:?} => {}", query, c.description)?;

                    if let Some(s) = c.source.and_then(|id| db.get_source(id)) {
                        if let Some(url) = &s.url {
                            write!(out, " ({}) <{}>", s.description, url)?;
                        } else {
                            write!(out, "({})", s.description)?;
                        }
                    }

                    writeln!(out)?;
                }
            }
        }
    }

    Ok(())
}
