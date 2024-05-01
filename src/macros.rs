#[macro_export]
macro_rules! source {
    ($state: ident) => {
        miette::NamedSource::new(&$state.file, $state.source.clone())
    };
}

#[macro_export]
macro_rules! check {
    ($v: ident: $r: expr => $f: expr) => {
        if semver::VersionReq::parse($r)?.matches(&$v) {
            return Ok($f);
        }
    };

    ($v: ident: =$r: expr => $f: expr) => {
        if semver::Version::parse($r)? == $v {
            return Ok($f);
        }
    };
}
