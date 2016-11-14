use util::*;
use std::collections::BTreeMap;

pub struct MultMemo<'a> {
    values: BTreeMap<Frac, Frac>,
    transfer_value: Option<&'a Frac>,
    pub hits: u32,
}

impl <'a> MultMemo<'a> {
    pub fn new(mult: Option<&'a Frac>) -> MultMemo<'a> {
        MultMemo {
            values: BTreeMap::new(),
            transfer_value: mult,
            hits: 0
        }
    }

    fn compute<'b>(&mut self, x: &'b Frac, y: &'b Frac) -> Frac {
        if let Some(cached) = self.values.get(x) {
            self.hits += 1;
            return cached.clone();
        }

        let mut result = x * y;
        //result.normalize();

        self.values.insert(x.clone(), result.clone());

        result
    }

    pub fn mult(&mut self, val: Option<&Frac>) -> Option<Frac> {
        match (val, self.transfer_value) {
            (_, None) => None,
            (Some(x), Some(transfer_value)) => Some(self.compute(x, transfer_value)),
            (None, Some(transfer_value)) => Some(transfer_value.clone())
        }
    }
}
