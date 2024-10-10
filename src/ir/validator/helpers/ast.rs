use crate::{IRAst, IRCheckerContext};

impl IRAst {
    pub fn create_checker_context(&mut self) -> IRCheckerContext {
        let ast = self.index().clone();
        let funcs = ast.funcs.unwrap();
        let vars = ast.vars.unwrap();
        let tags = ast.tags.unwrap();

        IRCheckerContext {
            funcs,
            vars,
            tags,

            // State
            cur_fn: None,
            cur_block: None,
        }
    }
}
