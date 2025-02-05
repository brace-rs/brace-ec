use std::convert::Infallible;
use std::ops::AddAssign;

use crate::fitness::Fitness;
use crate::individual::Individual;
use crate::operator::evaluator::Evaluator;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Hiff<T: Individual>;

impl<T> Hiff<T>
where
    T: Individual<Fitness: AddAssign<usize>>,
{
    fn hiff(bits: &[bool], fitness: &mut T::Fitness) -> bool {
        let len = bits.len();

        if len < 2 {
            *fitness += len;

            true
        } else {
            let half = len / 2;
            let same_lhs = Self::hiff(&bits[..half], fitness);
            let same_rhs = Self::hiff(&bits[half..], fitness);
            let same = same_lhs && same_rhs && bits[0] == bits[half];

            *fitness += same.then_some(len).unwrap_or_default();
            same
        }
    }
}

impl<T> Evaluator<T> for Hiff<T>
where
    T: Individual<Genome: AsRef<[bool]>, Fitness: AddAssign<usize>>,
{
    type Error = Infallible;

    fn evaluate<Rng>(&self, individual: &T, _: &mut Rng) -> Result<T::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut fitness = T::Fitness::nil();

        Self::hiff(individual.genome().as_ref(), &mut fitness);

        Ok(fitness)
    }
}

#[cfg(test)]
mod tests {
    use crate::fitness::summed::Summed;
    use crate::individual::Individual;
    use crate::operator::evaluator::Evaluator;

    use super::Hiff;

    #[test]
    fn test_evaluate() {
        let mut rng = rand::rng();

        let a = [false, false, true, false, true, true, true, true].evaluated::<usize>();
        let b = [false, false, false, false, true, false, false, true].evaluated::<usize>();
        let c = [false, false, false, false, false, false, false, false].evaluated::<usize>();
        let d = [true, true, true, true, true, true, true, true].evaluated::<usize>();
        let e = [true, false, true, false, true, false, true, false].evaluated::<usize>();

        assert_eq!(Hiff.evaluate(&a, &mut rng).unwrap(), 18);
        assert_eq!(Hiff.evaluate(&b, &mut rng).unwrap(), 16);
        assert_eq!(Hiff.evaluate(&c, &mut rng).unwrap(), 32);
        assert_eq!(Hiff.evaluate(&d, &mut rng).unwrap(), 32);
        assert_eq!(Hiff.evaluate(&e, &mut rng).unwrap(), 8);
    }

    #[test]
    fn test_evaluate_summed() {
        let mut rng = rand::rng();

        let a = [false, false, true, false, true, true, true, true];
        let a = Hiff
            .evaluate(&a.evaluated::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(a.total(), &18);
        assert_eq!(a.value(), &[1, 1, 2, 1, 1, 0, 0, 1, 1, 2, 1, 1, 2, 4, 0]);

        let b = [false, false, false, false, true, false, false, true];
        let b = Hiff
            .evaluate(&b.evaluated::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(b.total(), &16);
        assert_eq!(b.value(), &[1, 1, 2, 1, 1, 2, 4, 1, 1, 0, 1, 1, 0, 0, 0]);

        let c = [false, false, false, false, false, false, false, false];
        let c = Hiff
            .evaluate(&c.evaluated::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(c.total(), &32);
        assert_eq!(c.value(), &[1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8]);

        let d = [true, true, true, true, true, true, true, true];
        let d = Hiff
            .evaluate(&d.evaluated::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(d.total(), &32);
        assert_eq!(d.value(), &[1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8]);

        let e = [true, false, true, false, true, false, true, false];
        let e = Hiff
            .evaluate(&e.evaluated::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(e.total(), &8);
        assert_eq!(e.value(), &[1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]);
    }
}
