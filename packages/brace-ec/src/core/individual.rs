use rand::thread_rng;

use super::operator::mutator::Mutator;

pub trait Individual {
    type Genome: ?Sized;

    fn genome(&self) -> &Self::Genome;

    fn genome_mut(&mut self) -> &mut Self::Genome;

    fn mutate<M>(self, mutator: M) -> Result<Self, M::Error>
    where
        M: Mutator<Individual = Self>,
        Self: Sized,
    {
        mutator.mutate(self, &mut thread_rng())
    }
}

impl<T, const N: usize> Individual for [T; N] {
    type Genome = [T];

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }
}

impl<T> Individual for Vec<T> {
    type Genome = [T];

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Individual;

    fn erase<G: ?Sized, I: Individual<Genome = G>>(individual: I) -> impl Individual<Genome = G> {
        individual
    }

    #[test]
    fn test_individual_array() {
        let individual = erase([0, 0]);

        assert_eq!(individual.genome(), [0, 0]);
    }

    #[test]
    fn test_individual_vec() {
        let individual = erase(vec![0, 0]);

        assert_eq!(individual.genome(), [0, 0]);
    }
}
