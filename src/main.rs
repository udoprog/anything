use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{Files, SimpleFiles};
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use num::One;
use rational::DisplaySpec;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "facts", about = "Calculate facts about the world.")]
struct Opts {
    /// Describe the looked up components in the expression.
    #[structopt(long)]
    describe: bool,
    /// Show the exact fractional result.
    #[structopt(long)]
    exact: bool,
    /// Dump syntax tree.
    #[structopt(long)]
    syntax: bool,
    /// The query to run.
    query: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    pretty_env_logger::init();

    let mut out = StandardStream::stdout(ColorChoice::Auto);

    let query = opts.query.join(" ");

    let db = facts::Db::open()?;

    let mut files = SimpleFiles::new();
    let id = files.add("<in>", query);

    let config = codespan_reporting::term::Config::default();

    let options = facts::Options::default();
    let mut descriptions = Vec::new();

    let options = if opts.describe {
        options.describe()
    } else {
        options
    };

    let node = facts::parse(files.source(id)?);

    if opts.syntax {
        node.emit(&mut out)?;
    }

    for value in facts::query(node, &db, options, &mut descriptions) {
        match value {
            Ok(value) => {
                if opts.exact {
                    if !value.value.denom().is_one() {
                        write!(out, "{}/{}", value.value.numer(), value.value.denom())?;
                    } else {
                        write!(out, "{}", value.value.numer())?;
                    }
                } else {
                    let mut spec = DisplaySpec::default();

                    spec.limit = 12;
                    spec.exponent_limit = 12;
                    spec.show_continuation = true;

                    write!(out, "{}", value.value.display(&spec))?;
                }

                if value.unit.has_numerator() {
                    write!(out, " ")?;
                }

                let disp = value.unit.display(!value.value.is_one());
                writeln!(out, "{}", disp)?;
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
