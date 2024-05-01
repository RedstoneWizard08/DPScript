#[derive(Debug, Clone, Default)]
pub struct LineBuilder {
    pub lines: Vec<String>,
}

impl LineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_str(s: impl AsRef<str>) -> Self {
        Self {
            lines: vec![s.as_ref().into()],
        }
    }

    pub fn push(&mut self, line: impl AsRef<str>) {
        self.lines.push(line.as_ref().to_string());
    }

    pub fn build(self) -> String {
        self.lines.join("\n")
    }
}
