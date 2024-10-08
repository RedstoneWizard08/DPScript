use super::Remap;
use crate::Loop;

impl Remap for Loop {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        let mut me = Self {
            array: self.array.remap_name(orig, new),
            var_name: self.var_name.clone(),
            span: self.span,
            locals: None,
            body: self.body.remap_name(orig, new),
        };

        me.get_locals();

        me
    }
}
