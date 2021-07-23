use crate::unit::Unit;

/// Helper to parse collection of units from a string.
pub struct UnitParser<'a> {
    source: &'a str,
}

impl<'a> UnitParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Parse the next unit and base.
    pub fn next(&mut self) -> Result<Option<(i32, Unit)>, &'a str> {
        if self.source.is_empty() {
            return Ok(None);
        }

        match crate::generated::unit::parse(self.source) {
            Some((remainder, prefix, unit)) => {
                self.source = remainder;
                Ok(Some((prefix, unit)))
            }
            None => Err(self.source),
        }
    }
}
