pub use num::rational::Ratio;
pub use num::*;

pub type Uint = BigUint;
pub type Frac = Ratio<Uint>;

#[macro_export]
macro_rules! uint {
    ($e:expr) => {
        BigUint::from($e as u64)
    }
}

#[macro_export]
macro_rules! frac {
    ($e:expr) => {
        frac!($e, 1)
    };
    ($e1:expr, $e2:expr) => {
        Ratio::new(BigUint::from($e1 as u64), BigUint::from($e2 as u64))
    };
}
