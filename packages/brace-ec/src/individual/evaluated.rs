use crate::fitness::Fitness;
use crate::population::Population;
use crate::util::iter::TryFromIterator;

use super::Individual;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Evaluated<T, S> {
    pub individual: T,
    pub fitness: S,
}

impl<T, S> Evaluated<T, S> {
    pub fn new(individual: T, fitness: S) -> Self {
        Self {
            individual,
            fitness,
        }
    }
}

impl<T, S> Individual for Evaluated<T, S>
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
        &self.fitness
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        &mut self.fitness
    }
}

impl<T, S> Population for Evaluated<T, S>
where
    T: Individual<Genome: Population>,
{
    type Individual = <T::Genome as Population>::Individual;

    fn len(&self) -> usize {
        self.individual.genome().len()
    }
}

impl<U, T, S> TryFromIterator<U> for Evaluated<T, S>
where
    T: TryFromIterator<U>,
    S: Fitness,
{
    type Error = T::Error;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = U>,
    {
        T::try_from_iter(iter).map(Self::from)
    }
}

impl<T, S> From<T> for Evaluated<T, S>
where
    S: Fitness,
{
    fn from(individual: T) -> Self {
        Self::new(individual, S::nil())
    }
}

#[cfg(test)]
mod tests {
    use crate::individual::Individual;

    use super::Evaluated;

    #[test]
    fn test_individual() {
        let mut a = Evaluated::<_, i32>::from([1, 0]);
        let mut b = [1, 0].evaluated();
        let mut c = [1, 0].evaluated::<u8>();

        assert_eq!(a.genome(), [1, 0]);
        assert_eq!(b.genome(), [1, 0]);
        assert_eq!(c.genome(), [1, 0]);

        assert_eq!(*a.fitness(), 0);
        assert_eq!(*b.fitness(), 0);
        assert_eq!(*c.fitness(), 0);

        a.fitness = 10;
        b.fitness = 10;
        c.fitness = 10;

        assert_eq!(a.genome(), [1, 0]);
        assert_eq!(b.genome(), [1, 0]);
        assert_eq!(c.genome(), [1, 0]);

        assert_eq!(*a.fitness(), 10);
        assert_eq!(*b.fitness(), 10);
        assert_eq!(*c.fitness(), 10);
    }
}
