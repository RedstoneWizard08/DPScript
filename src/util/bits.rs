pub trait HasBits {
    type Bit: Clone;

    fn get_bits(&self) -> Vec<Self::Bit>;
}

impl HasBits for String {
    type Bit = char;

    fn get_bits(&self) -> Vec<Self::Bit> {
        self.chars().collect()
    }
}

impl<T: Clone> HasBits for Vec<T> {
    type Bit = T;

    fn get_bits(&self) -> Vec<Self::Bit> {
        self.clone()
    }
}
