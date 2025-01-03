use std::marker::PhantomData;

use crate::core::individual::Individual;

use super::Scorer;

pub struct Function<F, I> {
    scorer: F,
    marker: PhantomData<fn() -> I>,
}

impl<F, I> Function<F, I> {
    pub fn new(scorer: F) -> Self {
        Self {
            scorer,
            marker: PhantomData,
        }
    }
}

impl<F, I, S, E> Scorer for Function<F, I>
where
    F: Fn(&I) -> Result<S, E>,
    I: Individual,
    S: Ord,
{
    type Individual = I;
    type Score = S;
    type Error = E;

    fn score(&self, individual: &Self::Individual) -> Result<Self::Score, Self::Error> {
        (self.scorer)(individual)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::core::operator::scorer::Scorer;

    use super::Function;

    fn sum([a, b]: &[i32; 2]) -> Result<i32, Infallible> {
        Ok(a + b)
    }

    #[test]
    fn test_score() {
        let individual = [10, 20];

        let a = Function::new(|[a, b]: &[i32; 2]| Ok::<_, Infallible>(a + b));
        let b = Function::new(sum);

        assert_eq!(a.score(&individual).unwrap(), 30);
        assert_eq!(b.score(&individual).unwrap(), 30);
    }
}
