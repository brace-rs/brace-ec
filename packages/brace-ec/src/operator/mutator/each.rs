use std::marker::PhantomData;

use crate::individual::Individual;
use crate::operator::mutator::Mutator;
use crate::util::iter::{Iterable, IterableMut};

pub struct Each<M, I> {
    mutator: M,
    marker: PhantomData<fn() -> I>,
}

impl<M, I> Each<M, I> {
    pub fn new(mutator: M) -> Self {
        Self {
            mutator,
            marker: PhantomData,
        }
    }
}

impl<M, I> Mutator<I> for Each<M, I>
where
    I: Individual<Genome: IterableMut<Item: Individual + Clone>>,
    M: Mutator<<I::Genome as Iterable>::Item>,
{
    type Error = M::Error;

    fn mutate<Rng>(&self, mut individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        for item in individual.genome_mut().iter_mut() {
            *item = self.mutator.mutate(item.clone(), rng)?;
        }

        Ok(individual)
    }
}

#[cfg(test)]
mod tests {
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;

    #[test]
    fn test_mutate() {
        let mut rng = rand::rng();

        let a = Add(1).rate(1.0).each().mutate([1, 2, 3], &mut rng).unwrap();
        let b = Add(1).rate(0.0).each().mutate([1, 2, 3], &mut rng).unwrap();
        let c = Add(1)
            .rate(1.0)
            .each()
            .rate(0.0)
            .mutate([1, 2, 3], &mut rng)
            .unwrap();
        let d = Add(1)
            .rate(1.0)
            .each()
            .rate(1.0)
            .each()
            .rate(1.0)
            .mutate([[1, 2], [2, 3], [3, 4]], &mut rng)
            .unwrap();

        assert_eq!(a, [2, 3, 4]);
        assert_eq!(b, [1, 2, 3]);
        assert_eq!(c, [1, 2, 3]);
        assert_eq!(d, [[2, 3], [3, 4], [4, 5]]);
    }
}
