use super::Remap;
use crate::Module;

impl Remap for Module {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        let mut me = self.clone();
        me.body = self.body.remap_name(orig, new);
        me
    }
}
