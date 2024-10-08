use miette::SourceSpan;

use super::Spanned;

pub trait HasBits {
    type Bit: Clone;

    fn get_bits(&self) -> Vec<Self::Bit>;
}

impl HasBits for String {
    type Bit = char;

    fn get_bits(&self) -> Vec<Self::Bit> {
        self.chars().collect()
    }
}

impl<T: Clone> HasBits for Vec<T> {
    type Bit = T;

    fn get_bits(&self) -> Vec<Self::Bit> {
        self.clone()
    }
}

pub trait HasSpan {
    fn get_span(&self) -> SourceSpan;
}

impl<T> HasSpan for Spanned<T> {
    fn get_span(&self) -> SourceSpan {
        self.1
    }
}
