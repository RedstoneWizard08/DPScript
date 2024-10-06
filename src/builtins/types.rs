#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInTypes {
    Identifier,
    NBTPath,
    Selector,
    Component,
    Objective,
    Storage,
    Player,
    Store,
    NBT,
    Bool,
    Float,
    Int,
    String,
    Any,
    Enum,
}

impl BuiltInTypes {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Identifier => "Identifier",
            Self::NBTPath => "NBTPath",
            Self::Selector => "Selector",
            Self::Component => "Component",
            Self::Objective => "Objective",
            Self::Storage => "Storage",
            Self::Player => "Player",
            Self::Store => "Store",
            Self::NBT => "NBT",
            Self::Bool => "bool",
            Self::Float => "float",
            Self::Int => "int",
            Self::String => "str",
            Self::Any => "Any",
            Self::Enum => "Enum",
        }
    }

    pub fn methods(&self) -> Vec<(&'static str, Vec<BuiltInTypes>, Option<BuiltInTypes>)> {
        match self {
            Self::Int => vec![
                ("eq", vec![Self::Int], Some(Self::Bool)),
                ("ltEq", vec![Self::Int], Some(Self::Bool)),
                ("gtEq", vec![Self::Int], Some(Self::Bool)),
                ("lt", vec![Self::Int], Some(Self::Bool)),
                ("gt", vec![Self::Int], Some(Self::Bool)),
                ("equals", vec![Self::Int], Some(Self::Bool)),
                ("lessThanEqual", vec![Self::Int], Some(Self::Bool)),
                ("greaterThanEqual", vec![Self::Int], Some(Self::Bool)),
                ("lessThan", vec![Self::Int], Some(Self::Bool)),
                ("greaterThan", vec![Self::Int], Some(Self::Bool)),
            ],

            Self::Float => vec![
                ("eq", vec![Self::Float], Some(Self::Bool)),
                ("ltEq", vec![Self::Float], Some(Self::Bool)),
                ("gtEq", vec![Self::Float], Some(Self::Bool)),
                ("lt", vec![Self::Float], Some(Self::Bool)),
                ("gt", vec![Self::Float], Some(Self::Bool)),
                ("equals", vec![Self::Float], Some(Self::Bool)),
                ("lessThanEqual", vec![Self::Float], Some(Self::Bool)),
                ("greaterThanEqual", vec![Self::Float], Some(Self::Bool)),
                ("lessThan", vec![Self::Float], Some(Self::Bool)),
                ("greaterThan", vec![Self::Float], Some(Self::Bool)),
            ],

            Self::Store => vec![
                ("set", vec![Self::NBTPath, Self::Any], None),
                ("append", vec![Self::NBTPath, Self::Any], None),
                ("remove", vec![Self::NBTPath], None),
            ],

            Self::Objective => vec![
                ("set", vec![Self::Player, Self::Int], None),
                ("get", vec![Self::Player], Some(Self::Int)),
                ("add", vec![Self::Player, Self::Int], None),
                ("reset", vec![Self::Player], None),
                (
                    "operation",
                    vec![Self::Enum, Self::Player, Self::Int, Self::Player],
                    None,
                ),
            ],

            _ => vec![],
        }
    }
}
