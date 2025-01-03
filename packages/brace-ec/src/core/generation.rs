use super::population::Population;

pub trait Generation {
    type Id;
    type Population: Population;

    fn id(&self) -> &Self::Id;

    fn population(&self) -> &Self::Population;

    fn population_mut(&mut self) -> &mut Self::Population;
}

impl<T, P> Generation for (T, P)
where
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
}

#[cfg(test)]
mod tests {
    use crate::core::population::Population;

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
