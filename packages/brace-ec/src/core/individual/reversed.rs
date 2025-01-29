use std::cmp::Reverse;

use bytemuck::TransparentWrapper;

use crate::core::fitness::{Fitness, FitnessMut};

use super::Individual;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reversed<T> {
    pub individual: T,
}

impl<T> Reversed<T> {
    pub fn new(individual: T) -> Self {
        Self { individual }
    }
}

impl<T> Individual for Reversed<T>
where
    T: Individual,
{
    type Genome = T::Genome;

    fn genome(&self) -> &Self::Genome {
        self.individual.genome()
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self.individual.genome_mut()
    }
}

impl<T> Fitness for Reversed<T>
where
    T: Fitness,
{
    type Fitness = Reverse<T::Fitness>;

    fn fitness(&self) -> &Self::Fitness {
        Reverse::wrap_ref(self.individual.fitness())
    }
}

impl<T> FitnessMut for Reversed<T>
where
    T: FitnessMut,
{
    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        Reverse::wrap_mut(self.individual.fitness_mut())
    }
}

impl<T> From<T> for Reversed<T> {
    fn from(individual: T) -> Self {
        Self::new(individual)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fitness::Fitness;
    use crate::core::individual::scored::Scored;
    use crate::core::individual::Individual;

    use super::Reversed;

    #[test]
    fn test_individual() {
        assert!(0.fitness() < 100.fitness());
        assert!(0.reversed().fitness() > 100.reversed().fitness());

        let a = Reversed::new(Scored::new([1, 2, 3], 3));
        let b = Reversed::new(Scored::new([4, 5, 6], 6));

        assert!(a.fitness() > b.fitness());
    }
}
