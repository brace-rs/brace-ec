use std::convert::Infallible;
use std::iter::Sum;
use std::marker::PhantomData;

use num_traits::{One, Zero};

use crate::core::individual::Individual;
use crate::core::operator::scorer::Scorer;
use crate::util::iter::Iterable;

pub struct Count<T, U>
where
    T: Individual<Genome: Iterable>,
{
    value: <T::Genome as Iterable>::Item,
    marker: PhantomData<fn() -> U>,
}

impl<T> Count<T, ()>
where
    T: Individual<Genome: Iterable>,
{
    pub fn new<U>(value: <T::Genome as Iterable>::Item) -> Count<T, U> {
        Count {
            value,
            marker: PhantomData,
        }
    }
}

impl<T, U> Scorer<T> for Count<T, U>
where
    T: Individual<Genome: Iterable<Item: PartialEq>, Fitness: Sum<U>>,
    U: Zero + One,
{
    type Error = Infallible;

    fn score<Rng>(&self, individual: &T, _: &mut Rng) -> Result<T::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let fitness = individual
            .genome()
            .iter()
            .map(|item| match item == &self.value {
                true => U::one(),
                false => U::zero(),
            })
            .sum();

        Ok(fitness)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fitness::summed::Summed;
    use crate::core::individual::Individual;
    use crate::core::operator::scorer::Scorer;

    use super::Count;

    #[test]
    fn test_score() {
        let mut rng = rand::rng();

        let a = [false, false, true, false, true, true, true, true].scored::<usize>();
        let b = [false, false, false, false, true, false, false, true].scored::<u64>();
        let c = [false, false, false, false, false, false, false, false].scored::<u32>();
        let d = [true, true, true, true, true, true, true, true].scored::<i16>();
        let e = [true, false, true, false, true, false, true, false].scored::<u8>();

        assert_eq!(Count::new::<usize>(true).score(&a, &mut rng).unwrap(), 5);
        assert_eq!(Count::new::<u64>(false).score(&b, &mut rng).unwrap(), 6);
        assert_eq!(Count::new::<u32>(true).score(&c, &mut rng).unwrap(), 0);
        assert_eq!(Count::new::<i16>(true).score(&d, &mut rng).unwrap(), 8);
        assert_eq!(Count::new::<u8>(false).score(&e, &mut rng).unwrap(), 4);
    }

    #[test]
    fn test_score_summed() {
        let mut rng = rand::rng();

        let a = [false, false, true, false, true, true, true, true];
        let a = Count::new(true)
            .score(&a.scored::<Summed<Vec<usize>>>(), &mut rng)
            .unwrap();

        assert_eq!(a.total(), &5);
        assert_eq!(a.value(), &[0, 0, 1, 0, 1, 1, 1, 1]);

        let b = [false, false, false, false, true, false, false, true];
        let b = Count::new(false)
            .score(&b.scored::<Summed<Vec<u16>>>(), &mut rng)
            .unwrap();

        assert_eq!(b.total(), &6);
        assert_eq!(b.value(), &[1, 1, 1, 1, 0, 1, 1, 0]);

        let c = [false, false, false, false, false, false, false, false];
        let c = Count::new(true)
            .score(&c.scored::<Summed<Vec<u32>>>(), &mut rng)
            .unwrap();

        assert_eq!(c.total(), &0);
        assert_eq!(c.value(), &[0, 0, 0, 0, 0, 0, 0, 0]);

        let d = [true, true, true, true, true, true, true, true];
        let d = Count::new(true)
            .score(&d.scored::<Summed<Vec<u64>>>(), &mut rng)
            .unwrap();

        assert_eq!(d.total(), &8);
        assert_eq!(d.value(), &[1, 1, 1, 1, 1, 1, 1, 1]);

        let e = [true, false, true, false, true, false, true, false];
        let e = Count::new::<u64>(false)
            .score(&e.scored::<Summed<Vec<_>>>(), &mut rng)
            .unwrap();

        assert_eq!(e.total(), &4);
        assert_eq!(e.value(), &[0, 1, 0, 1, 0, 1, 0, 1]);
    }
}
