mod arg;
mod block;
mod call;
mod cmd;
mod concat;
mod condition;
mod data;
mod def;
mod exec;
mod func;
mod literal;
mod node;
mod tag;

pub trait SerializeNode {
    fn serialize_node(&self) -> String;
}

pub trait Indented<T> {
    fn indented(self, num: usize) -> Vec<T>;
}

impl<T: Iterator<Item = String>> Indented<String> for T {
    fn indented(self, num: usize) -> Vec<String> {
        let indent = (0..num)
            .map(|_| "    ".to_string())
            .collect::<Vec<_>>()
            .join("");

        self.collect::<Vec<_>>()
            .join("\n")
            .split("\n")
            .map(|v| format!("{}{}", indent.clone(), v))
            .collect()
    }
}
