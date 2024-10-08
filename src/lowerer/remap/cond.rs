use super::Remap;
use crate::Conditional;

impl Remap for Conditional {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        let mut me = Self {
            condition: Box::new(self.condition.remap_name(orig, new)),
            span: self.span,
            if_locals: None,
            else_locals: None,
            body: self.body.remap_name(orig, new),
            else_body: self.else_body.remap_name(orig, new),
        };

        me.get_if_locals();
        me.get_else_locals();

        me
    }
}
