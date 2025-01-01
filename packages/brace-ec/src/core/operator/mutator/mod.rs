use rand::Rng;

use crate::core::individual::Individual;

pub trait Mutator: Sized {
    type Individual: Individual;
    type Error;

    fn mutate<R>(
        &self,
        individual: Self::Individual,
        rng: &mut R,
    ) -> Result<Self::Individual, Self::Error>
    where
        R: Rng + ?Sized;
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::Rng;

    use crate::core::individual::Individual;

    use super::Mutator;

    struct Swap;

    impl Mutator for Swap {
        type Individual = [u32; 2];
        type Error = Infallible;

        fn mutate<R>(
            &self,
            individual: Self::Individual,
            _: &mut R,
        ) -> Result<Self::Individual, Self::Error>
        where
            R: Rng + ?Sized,
        {
            Ok([individual[1], individual[0]])
        }
    }

    #[test]
    fn test_mutator() {
        let individual = [0, 1].mutate(Swap).unwrap();

        assert_eq!(individual, [1, 0]);
    }
}
