use rand::Rng;

use crate::core::population::Population;

pub trait Recombinator {
    type Parents: Population;
    type Output: IntoIterator<Item = <Self::Parents as Population>::Individual>;
    type Error;

    fn recombine<R: Rng>(
        &self,
        parents: Self::Parents,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::Rng;

    use crate::core::population::Population;

    use super::Recombinator;

    struct Swap;

    impl Recombinator for Swap {
        type Parents = [[i32; 2]; 2];
        type Output = [[i32; 2]; 2];
        type Error = Infallible;

        fn recombine<R: Rng>(
            &self,
            parents: Self::Parents,
            _: &mut R,
        ) -> Result<Self::Output, Self::Error> {
            Ok([parents[1], parents[0]])
        }
    }

    #[test]
    fn test_recombinator() {
        let individuals = [[0, 0], [1, 1]].recombine(Swap).unwrap();

        assert_eq!(individuals[0], [1, 1]);
        assert_eq!(individuals[1], [0, 0]);
    }
}