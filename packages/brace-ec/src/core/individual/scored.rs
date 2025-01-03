use crate::core::fitness::{Fitness, FitnessMut};

use super::Individual;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Scored<T, S> {
    pub individual: T,
    pub score: S,
}

impl<T, S> Scored<T, S> {
    pub fn new(individual: T, score: S) -> Self {
        Self { individual, score }
    }
}

impl<T, S> Individual for Scored<T, S>
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

impl<T, S> Fitness for Scored<T, S>
where
    T: Individual,
    S: Ord + Clone,
{
    type Value = S;

    fn fitness(&self) -> Self::Value {
        self.score.clone()
    }
}

impl<T, S> FitnessMut for Scored<T, S>
where
    T: Individual,
    S: Ord + Clone,
{
    fn set_fitness(&mut self, fitness: Self::Value) {
        self.score = fitness;
    }
}

impl<T, S> From<T> for Scored<T, S>
where
    S: Default,
{
    fn from(individual: T) -> Self {
        Self::new(individual, S::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fitness::Fitness;
    use crate::core::individual::Individual;

    use super::Scored;

    #[test]
    fn test_individual() {
        let mut individual = Scored::<_, i32>::from([1, 0]);

        assert_eq!(individual.genome(), [1, 0]);
        assert_eq!(individual.fitness(), 0);

        individual.score = 10;

        assert_eq!(individual.genome(), [1, 0]);
        assert_eq!(individual.fitness(), 10);
    }
}
