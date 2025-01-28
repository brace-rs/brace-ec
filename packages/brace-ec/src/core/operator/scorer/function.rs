use crate::core::individual::Individual;

use super::Scorer;

pub struct Function<F> {
    scorer: F,
}

impl<F> Function<F> {
    pub fn new(scorer: F) -> Self {
        Self { scorer }
    }
}

impl<F, I, S, E> Scorer<I> for Function<F>
where
    F: Fn(&I) -> Result<S, E>,
    I: Individual,
    S: Ord,
{
    type Score = S;
    type Error = E;

    fn score<Rng>(&self, individual: &I, _: &mut Rng) -> Result<Self::Score, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
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
        let mut rng = rand::rng();

        let individual = [10, 20];

        let a = Function::new(|[a, b]: &[i32; 2]| Ok::<_, Infallible>(a + b));
        let b = Function::new(sum);

        assert_eq!(a.score(&individual, &mut rng).unwrap(), 30);
        assert_eq!(b.score(&individual, &mut rng).unwrap(), 30);
    }
}
