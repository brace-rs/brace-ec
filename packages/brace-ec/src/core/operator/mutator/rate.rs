use crate::core::individual::Individual;

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

impl<T, M> Mutator<T> for Rate<M>
where
    M: Mutator<T>,
    T: Individual,
{
    type Error = M::Error;

    fn mutate<Rng>(&self, individual: T, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if rng.random_bool(self.rate) {
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
