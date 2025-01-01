use rand::Rng;

use super::Mutator;

pub struct Rate<M> {
    mutator: M,
    rate: f64,
}

impl<M> Rate<M> {
    pub fn new(mutator: M, rate: f64) -> Self {
        Self { mutator, rate }
    }
}

impl<M> Mutator for Rate<M>
where
    M: Mutator,
{
    type Individual = M::Individual;
    type Error = M::Error;

    fn mutate<R>(
        &self,
        individual: Self::Individual,
        rng: &mut R,
    ) -> Result<Self::Individual, Self::Error>
    where
        R: Rng + ?Sized,
    {
        if rng.gen_bool(self.rate) {
            self.mutator.mutate(individual, rng)
        } else {
            Ok(individual)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::individual::Individual;
    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::mutator::Mutator;

    #[test]
    fn test_mutate() {
        let a = 1.mutate(Add(1).rate(1.0)).unwrap();
        let b = 1.mutate(Add(1).rate(0.0)).unwrap();

        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }
}
