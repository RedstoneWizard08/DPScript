use super::Remap;
use crate::Block;

impl Remap for Block {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        let mut me = Self {
            is_init: self.is_init,
            is_tick: self.is_tick,
            span: self.span,
            locals: None,
            body: self.body.remap_name(orig, new),
        };

        me.get_locals();

        me
    }
}
