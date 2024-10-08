mod bits;
mod cursor;
mod ident;
mod span;

pub use bits::*;
pub use cursor::*;
pub use ident::*;
pub use span::*;

use miette::{SourceOffset, SourceSpan};

pub fn fake_span() -> SourceSpan {
    SourceSpan::new(SourceOffset::from_location("", 0, 0), 1)
}
