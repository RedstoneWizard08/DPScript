use miette::SourceSpan;

pub type Spanned<T> = (T, SourceSpan);

pub trait AddSpan {
    fn add(&self, other: SourceSpan) -> SourceSpan;
}

impl AddSpan for SourceSpan {
    fn add(&self, other: SourceSpan) -> SourceSpan {
        (
            self.offset(),
            (other.offset() - self.offset()) + other.len(),
        )
            .into()
    }
}
