pub mod first;
pub mod random;

use rand::Rng;

use crate::core::population::Population;

pub trait Selector: Sized {
    type Population: Population;
    type Output: IntoIterator<Item = <Self::Population as Population>::Individual>;
    type Error;

    fn select<R: Rng>(
        &self,
        population: &Self::Population,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>;
}
