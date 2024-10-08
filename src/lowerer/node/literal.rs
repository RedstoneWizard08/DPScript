use uuid::Uuid;

use super::Lowerable;
use crate::{
    AddDataOperation, CheckerContext, CopyDataOperation, IRDataOperation, IRDefinition, IRLiteral,
    IRNode, Literal, LoweringContext, Node, ParserError, Result, VariableAlias,
};

impl Literal {
    pub fn get_value(&self, cx: &CheckerContext, nodes: &mut Vec<IRNode>) -> Result<IRNode> {
        let var = format!("__tmp_var_{}", Uuid::new_v4().to_string().replace("-", "_"));

        nodes.push(IRNode::Definition(IRDefinition::VariableAlias(
            VariableAlias {
                name: var.clone(),
                store: "dpscript:core/vars".into(),
                path: var.clone(),
            },
        )));

        let temp = format!("__tmp_var_{}", Uuid::new_v4().to_string().replace("-", "_"));

        nodes.push(IRNode::Definition(IRDefinition::VariableAlias(
            VariableAlias {
                name: temp.clone(),
                store: "dpscript:core/vars".into(),
                path: temp.clone(),
            },
        )));

        Ok(match self.clone() {
            Self::Array((vals, _)) => {
                nodes.push(IRNode::DataOperation(IRDataOperation::Set(
                    AddDataOperation {
                        var: var.clone(),
                        value: Box::new(IRNode::Literal(IRLiteral::String("[]".into()))),
                    },
                )));

                for item in vals {
                    let add = match item {
                        Node::Literal(it) => {
                            vec![IRNode::DataOperation(IRDataOperation::Append(
                                AddDataOperation {
                                    var: var.clone(),
                                    value: Box::new(it.get_value(cx, nodes)?),
                                },
                            ))]
                        }

                        Node::Ident((id, _)) => vec![
                            IRNode::DataOperation(IRDataOperation::Copy(CopyDataOperation {
                                source: id,
                                target: temp.clone(),
                            })),
                            IRNode::DataOperation(IRDataOperation::Append(AddDataOperation {
                                var: var.clone(),
                                value: Box::new(IRNode::Reference(temp.clone())),
                            })),
                        ],

                        _ => {
                            return Err(ParserError {
                                src: cx.get_source(),
                                at: item.span(),
                                err: "This node does not provide a real value!".into(),
                            }
                            .into())
                        }
                    };

                    nodes.extend(add);

                    nodes.push(IRNode::DataOperation(IRDataOperation::Append(
                        AddDataOperation {
                            var: var.clone(),
                            value: Box::new(IRNode::Reference(var.clone())),
                        },
                    )));
                }

                IRNode::Reference(var)
            }

            Literal::Bool((b, _)) => {
                IRNode::Literal(IRLiteral::String((if b { "1b" } else { "0b" }).into()))
            }

            Literal::Component((c, _)) => {
                IRNode::Literal(IRLiteral::String(format!("{{ \"text\": \"{}\" }}", c)))
            }

            Literal::Entity((e, _)) => {
                IRNode::Literal(IRLiteral::String(format!("{{ \"selector\": \"{}\" }}", e)))
            }

            Literal::EnumValue(_, (v, _)) => IRNode::Literal(IRLiteral::String(v.to_lowercase())),

            Literal::Float((v, _)) => IRNode::Literal(IRLiteral::String(format!("{}f", v))),

            Literal::Identifier((id, _)) => IRNode::Reference(id),

            Literal::Int((i, _)) => IRNode::Literal(IRLiteral::String(i.to_string())),

            Literal::Nbt((v, _)) => IRNode::Literal(IRLiteral::String(serde_json::to_string(&v)?)),

            Literal::Path((p, _)) => IRNode::Literal(IRLiteral::String(p)),

            Literal::Selector((s, _)) => {
                IRNode::Literal(IRLiteral::String(format!("{{ \"selector\": \"{}\" }}", s)))
            }

            Literal::Store((s, _)) => IRNode::Literal(IRLiteral::String(s)),

            Literal::String((s, _)) => IRNode::Literal(IRLiteral::String(format!("\"{}\"", s))),
        })
    }
}

impl Lowerable for Literal {
    fn lower(&self, _cx: &mut CheckerContext, _lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        Ok(Vec::new())
    }
}
