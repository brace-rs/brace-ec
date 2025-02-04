use std::convert::Infallible;
use std::ops::AddAssign;

use crate::core::fitness::Fitness;
use crate::core::individual::Individual;
use crate::core::operator::scorer::Scorer;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Hiff<T: Individual>;

impl<T> Hiff<T>
where
    T: Individual<Fitness: AddAssign<usize>>,
{
    fn hiff(bits: &[bool], score: &mut T::Fitness) -> bool {
        let len = bits.len();

        if len < 2 {
            *score += len;

            true
        } else {
            let half = len / 2;
            let same_lhs = Self::hiff(&bits[..half], score);
            let same_rhs = Self::hiff(&bits[half..], score);
            let same = same_lhs && same_rhs && bits[0] == bits[half];

            *score += same.then_some(len).unwrap_or_default();
            same
        }
    }
}

impl<T> Scorer<T> for Hiff<T>
where
    T: Individual<Genome: AsRef<[bool]>, Fitness: AddAssign<usize>>,
{
    type Error = Infallible;

    fn score<Rng>(&self, individual: &T, _: &mut Rng) -> Result<T::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut score = T::Fitness::nil();

        Self::hiff(individual.genome().as_ref(), &mut score);

        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fitness::summed::Summed;
    use crate::core::individual::Individual;
    use crate::core::operator::scorer::Scorer;

    use super::Hiff;

    #[test]
    fn test_score() {
        let mut rng = rand::rng();

        let a = [false, false, true, false, true, true, true, true].scored::<usize>();
        let b = [false, false, false, false, true, false, false, true].scored::<usize>();
        let c = [false, false, false, false, false, false, false, false].scored::<usize>();
        let d = [true, true, true, true, true, true, true, true].scored::<usize>();
        let e = [true, false, true, false, true, false, true, false].scored::<usize>();

        assert_eq!(Hiff.score(&a, &mut rng).unwrap(), 18);
        assert_eq!(Hiff.score(&b, &mut rng).unwrap(), 16);
        assert_eq!(Hiff.score(&c, &mut rng).unwrap(), 32);
        assert_eq!(Hiff.score(&d, &mut rng).unwrap(), 32);
        assert_eq!(Hiff.score(&e, &mut rng).unwrap(), 8);
    }

    #[test]
    fn test_score_summed() {
        let mut rng = rand::rng();

        let a = [false, false, true, false, true, true, true, true];
        let a = Hiff
            .score(&a.scored::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(a.total(), &18);
        assert_eq!(a.value(), &[1, 1, 2, 1, 1, 0, 0, 1, 1, 2, 1, 1, 2, 4, 0]);

        let b = [false, false, false, false, true, false, false, true];
        let b = Hiff
            .score(&b.scored::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(b.total(), &16);
        assert_eq!(b.value(), &[1, 1, 2, 1, 1, 2, 4, 1, 1, 0, 1, 1, 0, 0, 0]);

        let c = [false, false, false, false, false, false, false, false];
        let c = Hiff
            .score(&c.scored::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(c.total(), &32);
        assert_eq!(c.value(), &[1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8]);

        let d = [true, true, true, true, true, true, true, true];
        let d = Hiff
            .score(&d.scored::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(d.total(), &32);
        assert_eq!(d.value(), &[1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8]);

        let e = [true, false, true, false, true, false, true, false];
        let e = Hiff
            .score(&e.scored::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(e.total(), &8);
        assert_eq!(e.value(), &[1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]);
    }
}
