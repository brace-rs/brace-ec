use crate::individual::Individual;

use super::Evaluator;

pub struct Function<F> {
    evaluator: F,
}

impl<F> Function<F> {
    pub fn new(evaluator: F) -> Self {
        Self { evaluator }
    }
}

impl<F, I, E> Evaluator<I> for Function<F>
where
    F: Fn(&I) -> Result<I::Fitness, E>,
    I: Individual,
{
    type Error = E;

    fn evaluate<Rng>(&self, individual: &I, _: &mut Rng) -> Result<I::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (self.evaluator)(individual)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::individual::evaluated::Evaluated;
    use crate::operator::evaluator::Evaluator;

    use super::Function;

    fn double(x: &Evaluated<i32, i32>) -> Result<i32, Infallible> {
        Ok(x.individual * 2)
    }

    #[test]
    fn test_evaluate() {
        let mut rng = rand::rng();

        let a = Function::new(|x: &Evaluated<i32, i32>| Ok::<_, Infallible>(x.individual * 2))
            .evaluate(&Evaluated::new(15, 0), &mut rng)
            .unwrap();
        let b = Function::new(double)
            .evaluate(&Evaluated::new(15, 0), &mut rng)
            .unwrap();

        assert_eq!(a, 30);
        assert_eq!(b, 30);
    }
}
