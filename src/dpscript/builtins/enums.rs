use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInEnums {
    ScoreboardOps,
}

impl BuiltInEnums {
    pub fn all() -> Vec<Self> {
        vec![Self::ScoreboardOps]
    }

    pub fn names() -> Vec<&'static str> {
        Self::all().iter().map(Self::name).collect()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ScoreboardOps => "ScoreboardOps",
        }
    }

    /// Returns a list of variants: (variant name, minecraft command translation)
    pub fn variants(&self) -> HashMap<&'static str, &'static str> {
        match self {
            Self::ScoreboardOps => vec![
                ("Equals", "="),
                ("Add", "+="),
                ("Sub", "-="),
                ("Mul", "*="),
                ("Div", "/="),
                ("Mod", "%="),
                ("Swap", "><"),
                ("Min", "<"),
                ("Max", ">"),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn variant_names(&self) -> Vec<&'static str> {
        self.variants().keys().cloned().collect()
    }
}
