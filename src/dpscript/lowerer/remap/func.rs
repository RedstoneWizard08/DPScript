use super::Remap;
use crate::Function;

impl Remap for Function {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        let mut me = Self {
            args: self.args.clone(),
            attrs: self.attrs.clone(),
            is_compiler: self.is_compiler,
            is_facade: self.is_facade,
            is_inline: self.is_inline,
            is_pub: self.is_pub,
            name: self.name.clone(),
            ret: self.ret.clone(),
            span: self.span,
            locals: None,
            body: self.body.remap_name(orig, new),
        };

        me.get_locals();

        me
    }
}
