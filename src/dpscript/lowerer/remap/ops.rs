use super::Remap;
use crate::Operation;

impl Remap for Operation {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        let mut me = self.clone();
        me.lhs = Box::new(self.lhs.remap_name(orig, new));
        me.rhs = Box::new(self.rhs.remap_name(orig, new));
        me
    }
}
