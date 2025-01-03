pub mod function;

use crate::core::individual::Individual;

pub trait Scorer {
    type Individual: Individual;
    type Score: Ord;
    type Error;

    fn score(&self, individual: &Self::Individual) -> Result<Self::Score, Self::Error>;
}
