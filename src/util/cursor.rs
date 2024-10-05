use miette::{NamedSource, SourceOffset, SourceSpan};

use crate::{ParserError, ParserResult};

use super::bits::HasBits;

#[derive(Debug, Clone)]
pub struct Cursor<T: HasBits + Clone, M = ()> {
    src: T,
    inner: Vec<T::Bit>,
    pos: usize,
    meta: M,
}

impl<T: HasBits + Clone, M: Default> Cursor<T, M> {
    pub fn new(data: T) -> Self {
        Self {
            src: data.clone(),
            inner: data.get_bits(),
            pos: 0,
            meta: M::default(),
        }
    }
}

impl<T: HasBits + Clone> Cursor<T, NamedSource<String>> {
    pub fn new_from_src(file: impl AsRef<str>, code: impl AsRef<str>, data: T) -> Self {
        Self {
            src: data.clone(),
            inner: data.get_bits(),
            pos: 0,
            meta: NamedSource::new(file, code.as_ref().into()),
        }
    }
}

impl<T: HasBits + Clone, M> Cursor<T, M> {
    pub fn is_empty(&self) -> bool {
        self.inner.len() <= self.pos
    }

    pub fn has_next(&self) -> bool {
        !self.is_empty()
    }

    pub fn next(&mut self) -> Option<T::Bit> {
        self.pos += 1;
        self.inner.get(self.pos - 1).cloned()
    }

    pub fn peek(&self) -> Option<T::Bit> {
        self.inner.get(self.pos).cloned()
    }

    pub fn peek_ahead(&self, num: usize) -> Option<T::Bit> {
        self.inner.get(self.pos + num).cloned()
    }

    pub fn skip(&mut self, num: usize) {
        self.pos += num;
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl<T: HasBits + Clone + FromIterator<T::Bit>, M> Cursor<T, M> {
    pub fn peek_many(&self, start: usize, num: usize) -> Option<T> {
        let mut parts = Vec::new();

        for i in 0..num {
            if let Some(bit) = self.peek_ahead(start + i) {
                parts.push(bit);
            } else {
                return None;
            }
        }

        Some(parts.iter().cloned().collect())
    }
}

impl<T: HasBits + Clone> Cursor<T, NamedSource<String>> {
    pub fn source(&self) -> NamedSource<String> {
        self.meta.clone()
    }

    pub fn next_or_die(&mut self, span: SourceSpan) -> ParserResult<T::Bit> {
        self.pos += 1;

        match self.inner.get(self.pos - 1).cloned() {
            Some(v) => Ok(v),
            None => Err(ParserError {
                src: self.source(),
                at: span,
                err: "Unexpected end of file!".into(),
            }),
        }
    }
}

impl Cursor<String, NamedSource<String>> {
    pub fn new_from_code(file: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let s = data.as_ref().to_string();

        Self {
            src: s.clone(),
            inner: s.chars().collect(),
            pos: 0,
            meta: NamedSource::new(file, s),
        }
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
