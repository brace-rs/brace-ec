use std::convert::Infallible;
use std::ops::AddAssign;

use num_traits::Zero;

use crate::core::fitness::Fitness;
use crate::core::operator::scorer::Scorer;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Hiff<T: Fitness>;

impl<T> Hiff<T>
where
    T: Fitness<Value: AddAssign<usize>>,
{
    fn hiff(bits: &[bool], score: &mut T::Value) -> bool {
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
    T: Fitness<Genome: AsRef<[bool]>, Value: AddAssign<usize> + Zero>,
{
    type Score = T::Value;
    type Error = Infallible;

    fn score<Rng>(&self, individual: &T, _: &mut Rng) -> Result<Self::Score, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut score = T::Value::zero();

        Self::hiff(individual.genome().as_ref(), &mut score);

        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::individual::Individual;
    use crate::core::operator::scorer::Scorer;

    use super::Hiff;

    #[test]
    fn test_score() {
        let mut rng = rand::thread_rng();

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
}
