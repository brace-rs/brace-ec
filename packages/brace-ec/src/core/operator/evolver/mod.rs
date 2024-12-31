pub mod select;

use crate::core::generation::Generation;

pub trait Evolver {
    type Generation: Generation;
    type Error;

    fn evolve(&self, generation: Self::Generation) -> Result<Self::Generation, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::Evolver;

    struct Increment;

    impl Evolver for Increment {
        type Generation = (u8, [u8; 2]);
        type Error = Infallible;

        fn evolve(
            &self,
            mut generation: Self::Generation,
        ) -> Result<Self::Generation, Self::Error> {
            generation.0 += 1;
            generation.1[0] += 1;
            generation.1[1] += 1;

            Ok(generation)
        }
    }

    #[test]
    fn test_evolver() {
        let generation = Increment.evolve((0, [0, 1])).unwrap();

        assert_eq!(generation, (1, [1, 2]));
    }
}
