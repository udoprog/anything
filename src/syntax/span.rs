/// The span as output from a lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub(crate) fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Get the start of the span.
    pub(crate) fn start(&self) -> usize {
        self.start as usize
    }

    /// Get the end of the span.
    pub(crate) fn end(&self) -> usize {
        self.end as usize
    }
}
