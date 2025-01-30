use crate::core::fitness::Fitness;

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
    S: Fitness,
{
    type Genome = T::Genome;
    type Fitness = S;

    fn genome(&self) -> &Self::Genome {
        self.individual.genome()
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self.individual.genome_mut()
    }

    fn fitness(&self) -> &Self::Fitness {
        &self.score
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        &mut self.score
    }
}

impl<T, S> From<T> for Scored<T, S>
where
    S: Fitness,
{
    fn from(individual: T) -> Self {
        Self::new(individual, S::nil())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::individual::Individual;

    use super::Scored;

    #[test]
    fn test_individual() {
        let mut a = Scored::<_, i32>::from([1, 0]);
        let mut b = [1, 0].scored();
        let mut c = [1, 0].scored::<u8>();

        assert_eq!(a.genome(), [1, 0]);
        assert_eq!(b.genome(), [1, 0]);
        assert_eq!(c.genome(), [1, 0]);

        assert_eq!(*a.fitness(), 0);
        assert_eq!(*b.fitness(), 0);
        assert_eq!(*c.fitness(), 0);

        a.score = 10;
        b.score = 10;
        c.score = 10;

        assert_eq!(a.genome(), [1, 0]);
        assert_eq!(b.genome(), [1, 0]);
        assert_eq!(c.genome(), [1, 0]);

        assert_eq!(*a.fitness(), 10);
        assert_eq!(*b.fitness(), 10);
        assert_eq!(*c.fitness(), 10);
    }
}
