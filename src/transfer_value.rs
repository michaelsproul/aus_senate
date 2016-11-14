#[derive(Debug)]
pub struct TransferValue {
    numerator: u32,
    denominator: u32,
}

impl TransferValue {
    pub fn new(n: u32, d: u32) -> Self {
        TransferValue {
            numerator: n,
            denominator: d,
        }
    }

    pub fn apply(&self, val: u32) -> u32 {
        (val * self.numerator) / self.denominator
    }
}
