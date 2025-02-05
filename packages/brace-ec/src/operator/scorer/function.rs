use crate::individual::Individual;

use super::Scorer;

pub struct Function<F> {
    scorer: F,
}

impl<F> Function<F> {
    pub fn new(scorer: F) -> Self {
        Self { scorer }
    }
}

impl<F, I, E> Scorer<I> for Function<F>
where
    F: Fn(&I) -> Result<I::Fitness, E>,
    I: Individual,
{
    type Error = E;

    fn score<Rng>(&self, individual: &I, _: &mut Rng) -> Result<I::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (self.scorer)(individual)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::individual::scored::Scored;
    use crate::operator::scorer::Scorer;

    use super::Function;

    fn double(x: &Scored<i32, i32>) -> Result<i32, Infallible> {
        Ok(x.individual * 2)
    }

    #[test]
    fn test_score() {
        let mut rng = rand::rng();

        let a = Function::new(|x: &Scored<i32, i32>| Ok::<_, Infallible>(x.individual * 2))
            .score(&Scored::new(15, 0), &mut rng)
            .unwrap();
        let b = Function::new(double)
            .score(&Scored::new(15, 0), &mut rng)
            .unwrap();

        assert_eq!(a, 30);
        assert_eq!(b, 30);
    }
}
