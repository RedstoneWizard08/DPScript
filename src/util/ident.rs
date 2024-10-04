pub trait IsNotIdent {
    fn is_not_ident(&self) -> bool;
}

impl IsNotIdent for char {
    fn is_not_ident(&self) -> bool {
        !self.is_ascii_alphanumeric() && *self != '_'
    }
}
