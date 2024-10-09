use crate::{Block, CheckerContext, Conditional, Function, Loop, Module, Node, Result, Subroutine};

impl Module {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        self.imported_objects = None;
        self.get_imported_objects(cx)?;

        for node in &mut self.body {
            node.cache(cx)?;
        }

        Ok(())
    }
}

impl Function {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        self.locals = None;
        self.get_locals();

        for node in &mut self.body {
            node.cache(cx)?;
        }

        Ok(())
    }
}

impl Block {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        self.locals = None;
        self.get_locals();

        for node in &mut self.body {
            node.cache(cx)?;
        }

        Ok(())
    }
}

impl Conditional {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        self.if_locals = None;
        self.get_if_locals();
        self.else_locals = None;
        self.get_else_locals();

        for node in &mut self.body {
            node.cache(cx)?;
        }

        Ok(())
    }
}

impl Loop {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        self.locals = None;
        self.get_locals();

        for node in &mut self.body {
            node.cache(cx)?;
        }

        Ok(())
    }
}

impl Subroutine {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        self.locals = None;
        self.get_locals();

        for node in &mut self.body {
            node.cache(cx)?;
        }

        Ok(())
    }
}

impl Node {
    pub fn cache(&mut self, cx: &CheckerContext) -> Result<()> {
        match self {
            Self::Module(it) => it.cache(cx)?,
            Self::Function(it) => it.cache(cx)?,
            Self::Block(it) => it.cache(cx)?,
            Self::Conditional(it) => it.cache(cx)?,
            Self::Loop(it) => it.cache(cx)?,
            Self::Subroutine(it) => it.cache(cx)?,
            _ => {}
        }

        Ok(())
    }
}
