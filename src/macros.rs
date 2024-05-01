#[macro_export]
macro_rules! source {
    ($state: ident) => {
        miette::NamedSource::new(&$state.file, $state.source.clone())
    };
}
