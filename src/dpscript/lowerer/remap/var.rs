use super::Remap;
use crate::Variable;

impl Remap for Variable {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        let mut me = self.clone();
        me.value = self
            .value
            .clone()
            .map(|v| Box::new(v.remap_name(orig, new)));
        me
    }
}
