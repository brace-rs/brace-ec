use std::cmp::Ordering;

use rand::seq::{IndexedRandom, SliceRandom};
use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::population::{IterablePopulation, Population};
use crate::util::iter::Iterable;

use super::Selector;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Lexicase<P: Population + ?Sized>;

impl<P> Selector<P> for Lexicase<P>
where
    P: IterablePopulation<Individual: Individual<Fitness: Iterable<Item: Ord>> + Clone> + ?Sized,
{
    type Output = [P::Individual; 1];
    type Error = LexicaseError;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if population.is_empty() {
            return Err(LexicaseError::Empty);
        }

        let mut candidates = population.iter().collect::<Vec<_>>();
        let mut collected = Vec::with_capacity(candidates.len());

        let num_cases = candidates[0].fitness().iter().count();
        let mut cases = (0..num_cases).collect::<Vec<_>>();

        cases.shuffle(rng);

        for case in cases {
            if candidates.len() == 1 {
                return Ok([candidates[0].clone()]);
            }

            collected.clear();
            collected.push(candidates[0]);

            let mut best = collected[0].fitness().iter().nth(case);

            for candidate in &candidates[1..] {
                let val = candidate.fitness().iter().nth(case);

                match val.cmp(&best) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        collected.push(candidate);
                    }
                    Ordering::Greater => {
                        collected.clear();
                        collected.push(candidate);

                        best = val;
                    }
                }
            }

            std::mem::swap(&mut candidates, &mut collected);
        }

        let candidate = *candidates.choose(rng).ok_or(LexicaseError::Empty)?;

        Ok([candidate.clone()])
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexicaseError {
    #[error("empty population")]
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::core::fitness::summed::Summed;
    use crate::core::individual::scored::Scored;
    use crate::core::operator::selector::Selector;

    use super::Lexicase;

    #[test]
    fn test_select() {
        let mut rng = rand::rng();

        let a = Lexicase
            .select(
                &[
                    Scored::new(0, Summed::new([3])),
                    Scored::new(1, Summed::new([6])),
                    Scored::new(2, Summed::new([9])),
                    Scored::new(3, Summed::new([1])),
                ],
                &mut rng,
            )
            .unwrap();

        assert_eq!(a[0].individual, 2);

        let b = Lexicase
            .select(
                &[
                    Scored::new(0, Summed::new([1, 6])),
                    Scored::new(1, Summed::new([3, 8])),
                    Scored::new(2, Summed::new([7, 0])),
                    Scored::new(3, Summed::new([9, 9])),
                ],
                &mut rng,
            )
            .unwrap();

        assert_eq!(b[0].individual, 3);

        let c = Lexicase
            .select(
                &[
                    Scored::new(0, Summed::new([9])),
                    Scored::new(1, Summed::new([6])),
                    Scored::new(2, Summed::new([9])),
                    Scored::new(3, Summed::new([1])),
                ],
                &mut rng,
            )
            .unwrap();

        assert!(c[0].individual == 0 || c[0].individual == 2);

        let d = Lexicase
            .select(
                &[
                    Scored::new(0, Summed::new([1, 6])),
                    Scored::new(1, Summed::new([3, 8])),
                    Scored::new(2, Summed::new([7, 0])),
                    Scored::new(3, Summed::new([1, 8])),
                ],
                &mut rng,
            )
            .unwrap();

        assert!(d[0].individual == 1 || d[0].individual == 2);
    }
}
