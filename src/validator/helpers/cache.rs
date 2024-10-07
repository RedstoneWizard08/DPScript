use crate::{Block, CheckerContext, Conditional, Function, Loop, Module, Node, Result};

impl Module {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        self.get_imported_objects(cx)?;

        Ok(())
    }
}

impl Function {
    pub fn cache(&mut self) -> Result<()> {
        self.get_locals();

        Ok(())
    }
}

impl Block {
    pub fn cache(&mut self) -> Result<()> {
        self.get_locals();

        Ok(())
    }
}

impl Conditional {
    pub fn cache(&mut self) -> Result<()> {
        self.get_locals();

        Ok(())
    }
}

impl Loop {
    pub fn cache(&mut self) -> Result<()> {
        self.get_locals();

        Ok(())
    }
}

impl Node {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        match self {
            Self::Module(module) => module.cache(cx)?,
            _ => {}
        }

        Ok(())
    }
}
