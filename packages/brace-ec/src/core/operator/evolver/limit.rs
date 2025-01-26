use crate::core::generation::Generation;

use super::Evolver;

pub struct Limit<G, T>
where
    G: Generation,
    T: Evolver<G>,
{
    evolver: T,
    generation: G::Id,
}

impl<G, T> Limit<G, T>
where
    G: Generation,
    T: Evolver<G>,
{
    pub fn new(evolver: T, generation: G::Id) -> Self {
        Self {
            evolver,
            generation,
        }
    }
}

impl<G, T> Evolver<G> for Limit<G, T>
where
    G: Generation<Id: Ord>,
    T: Evolver<G>,
{
    type Error = T::Error;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if generation.id() >= &self.generation {
            return Ok(generation);
        }

        self.evolver.evolve(generation, rng)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::operator::evolver::Evolver;
    use crate::core::operator::selector::best::Best;
    use crate::core::operator::selector::Selector;

    #[test]
    fn test_evolve() {
        let mut rng = rand::thread_rng();

        let a = Best
            .fill()
            .evolver()
            .limit(10)
            .repeat(20)
            .evolve((0, [1, 2, 3]), &mut rng)
            .unwrap();
        let b = Best
            .fill()
            .evolver()
            .repeat(3)
            .limit(10)
            .repeat(20)
            .evolve((0, [1, 2, 3]), &mut rng)
            .unwrap();

        assert_eq!(a, (10, [3; 3]));
        assert_eq!(b, (12, [3; 3]));
    }
}
