#![feature(test)]
#![allow(dead_code)]

#[cfg(test)]
extern crate test;
extern crate rand;
extern crate num_bigint;
extern crate num_integer;


mod table;
mod trial_division;
mod aks;
mod miller_rabin;
mod solovay_strassen;

pub use self::table::table_query_u16;
pub use self::aks::aks_primality_test_usize;
pub use self::trial_division::trial_division_u64;
pub use self::trial_division::trial_division_u128;
pub use self::trial_division::trial_division_biguint;
pub use self::miller_rabin::miller_rabin_primality_test_u64;
pub use self::miller_rabin::miller_rabin_primality_test_biguint;
pub use self::solovay_strassen::solovay_strassen_primality_test_u64;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Primality {
    ZeroOrOne,
    Prime,
    Composite,
    ProbablyPrime,
}

impl Into<bool> for Primality {
    fn into(self) -> bool {
        match self {
            Primality::Prime
            | Primality::ProbablyPrime => true,
            _ => false,
        }
    }
}

impl From<bool> for Primality {
    fn from(v: bool) -> Primality {
        match v {
            true => Primality::Prime,
            false => Primality::Composite,
        }
    }
}
