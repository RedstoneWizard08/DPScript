use std::collections::HashMap;

use serde_json::Value;

use super::{Lowerable, Valued};
use crate::{
    CheckerContext, IRArgumentOperation, IRLiteral, IRNode, IRSetArgument, Literal,
    LoweringContext, Node, Operation, Result,
};

impl Valued for Operation {
    fn get_value(
        &mut self,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
        nodes: &mut Vec<IRNode>,
    ) -> Result<IRNode> {
        'auto: {
            if let Node::Literal(lhs) = &*self.lhs {
                if let Node::Literal(rhs) = &*self.rhs {
                    let mut lhs = match lhs {
                        Literal::Nbt((nbt, _)) => serde_json::from_str::<HashMap<String, Value>>(
                            &serde_json::to_string(&nbt)?,
                        )?,
                        Literal::Component((comp, _)) => HashMap::from_iter(vec![(
                            "text".to_string(),
                            Value::String(comp.clone()),
                        )]),
                        Literal::Selector((sel, _)) => HashMap::from_iter(vec![(
                            "selector".to_string(),
                            Value::String(sel.clone()),
                        )]),
                        Literal::Entity((ent, _)) => HashMap::from_iter(vec![(
                            "selector".to_string(),
                            Value::String(ent.clone()),
                        )]),
                        _ => break 'auto,
                    };

                    let rhs = match rhs {
                        Literal::Nbt((nbt, _)) => serde_json::from_str::<HashMap<String, Value>>(
                            &serde_json::to_string(&nbt)?,
                        )?,
                        Literal::Component((comp, _)) => HashMap::from_iter(vec![(
                            "text".to_string(),
                            Value::String(comp.clone()),
                        )]),
                        Literal::Selector((sel, _)) => HashMap::from_iter(vec![(
                            "selector".to_string(),
                            Value::String(sel.clone()),
                        )]),
                        Literal::Entity((ent, _)) => HashMap::from_iter(vec![(
                            "selector".to_string(),
                            Value::String(ent.clone()),
                        )]),
                        _ => break 'auto,
                    };

                    lhs.extend(rhs);

                    return Ok(IRNode::Literal(IRLiteral::String(serde_json::to_string(
                        &lhs,
                    )?)));
                }
            }
        }

        let lhs = IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
            index: 0,
            value: Box::new(self.lhs.get_value(cx, lcx, nodes)?),
        }));

        let rhs = IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
            index: 0,
            value: Box::new(self.rhs.get_value(cx, lcx, nodes)?),
        }));

        nodes.push(lhs);
        nodes.push(rhs);

        // TODO: Get the IR function name and call it

        Ok(IRNode::Reference("__RETURN_VAL__".into()))
    }
}

impl Lowerable for Operation {
    fn lower(
        &mut self,
        _cx: &mut CheckerContext,
        _lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        Ok(Vec::new())
    }
}
