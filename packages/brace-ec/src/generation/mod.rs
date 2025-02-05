use std::ops::AddAssign;

use num_traits::One;

use super::population::Population;

pub trait Generation {
    type Id;
    type Population: Population;

    fn id(&self) -> &Self::Id;

    fn population(&self) -> &Self::Population;

    fn population_mut(&mut self) -> &mut Self::Population;

    fn advance(&mut self);

    fn advance_with(&mut self, population: impl Into<Self::Population>) -> Self::Population {
        self.advance();
        std::mem::replace(self.population_mut(), population.into())
    }

    fn advanced(mut self) -> Self
    where
        Self: Sized,
    {
        self.advance();
        self
    }

    fn advanced_with(mut self, population: impl Into<Self::Population>) -> Self
    where
        Self: Sized,
    {
        self.advance_with(population);
        self
    }
}

impl<T, P> Generation for (T, P)
where
    T: AddAssign + One,
    P: Population,
{
    type Id = T;
    type Population = P;

    fn id(&self) -> &Self::Id {
        &self.0
    }

    fn population(&self) -> &Self::Population {
        &self.1
    }

    fn population_mut(&mut self) -> &mut Self::Population {
        &mut self.1
    }

    fn advance(&mut self) {
        self.0 += T::one();
    }
}

#[cfg(test)]
mod tests {
    use crate::population::Population;

    use super::Generation;

    fn erase<T, G: Generation<Id = T>>(generation: G) -> impl Generation<Id = T> {
        generation
    }

    #[test]
    fn test_generation_tuple() {
        let generation = erase((0, [[0, 0]]));

        assert_eq!(generation.id(), &0);
        assert_eq!(generation.population().len(), 1);
    }
}
