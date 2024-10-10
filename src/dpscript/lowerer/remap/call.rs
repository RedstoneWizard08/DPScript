use super::Remap;
use crate::Call;

impl Remap for Call {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        Call {
            function: self.function.remap_name(orig, new),
            args: self.args.remap_name(orig, new),
            span: self.span,
            parent: self.parent.clone().map(|v| v.remap_name(orig, new)),
        }
    }
}
