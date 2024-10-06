#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInEnums {
    ScoreboardOps,
}

impl BuiltInEnums {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ScoreboardOps => "ScoreboardOps",
        }
    }

    /// Returns a list of variants: (variant name, minecraft command translation)
    pub fn variants(&self) -> Vec<(&'static str, &'static str)> {
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
            ],
        }
    }
}
