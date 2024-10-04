use miette::{NamedSource, SourceOffset, SourceSpan};

#[derive(Clone)]
pub struct Cursor {
    src: String,
    inner: Vec<char>,
    pos: usize,
    source: NamedSource<String>,
}

impl Cursor {
    pub fn new(file: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let s = data.as_ref().to_string();

        Self {
            src: s.clone(),
            inner: s.chars().collect(),
            pos: 0,
            source: NamedSource::new(file, s),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() <= self.pos
    }

    pub fn has_next(&self) -> bool {
        !self.is_empty()
    }

    pub fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.inner.get(self.pos - 1).cloned()
    }

    pub fn peek(&self) -> Option<char> {
        self.inner.get(self.pos).cloned()
    }

    pub fn peek_ahead(&self, num: usize) -> Option<char> {
        self.inner.get(self.pos + num).cloned()
    }

    pub fn peek_str(&self, start: usize, num: usize) -> Option<String> {
        let mut parts = Vec::new();

        for i in 0..num {
            if let Some(ch) = self.peek_ahead(start + i) {
                parts.push(ch);
            } else {
                return None;
            }
        }

        Some(parts.iter().collect())
    }

    pub fn skip(&mut self, num: usize) {
        self.pos += num;
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn source(&self) -> NamedSource<String> {
        self.source.clone()
    }

    fn find_line(&self) -> usize {
        let mut lines = 0;

        for item in &self.inner[0..self.pos] {
            if *item == '\n' {
                lines += 1;
            }
        }

        lines
    }

    fn find_char(&self) -> usize {
        let line = self.find_line();
        let mut lines = 0;
        let mut chars = 0;

        for item in &self.inner[0..self.pos] {
            if *item == '\n' {
                lines += 1;
            } else {
                if line == lines {
                    chars += 1;
                }
            }
        }

        chars
    }

    pub fn span(&self, length: usize) -> SourceSpan {
        SourceSpan::new(
            SourceOffset::from_location(&self.src, self.find_line() + 1, self.find_char()),
            length,
        )
    }
}

pub trait IsNotIdent {
    fn is_not_ident(&self) -> bool;
}

impl IsNotIdent for char {
    fn is_not_ident(&self) -> bool {
        !self.is_ascii_alphanumeric() && *self != '_'
    }
}
