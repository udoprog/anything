use crate::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedUnit {
    pub prefix: i32,
    pub name: Unit,
}

impl ParsedUnit {
    pub fn new(prefix: i32, name: Unit) -> Self {
        Self { prefix, name }
    }
}

/// Helper to parse collection of units from a string.
pub struct UnitParser<'a> {
    source: &'a str,
}

impl<'a> UnitParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Parse the next unit and base.
    pub fn next(&mut self) -> Result<Option<ParsedUnit>, &'a str> {
        if self.source.is_empty() {
            return Ok(None);
        }

        match crate::generated::unit::parse(self.source) {
            Some((remainder, prefix, unit)) => {
                self.source = remainder;
                Ok(Some(ParsedUnit::new(prefix, unit)))
            }
            None => Err(self.source),
        }
    }
}
