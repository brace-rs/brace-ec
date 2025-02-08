use std::convert::Infallible;

use brace_ec::individual::evaluated::Evaluated;
use brace_ec::individual::Individual;
use brace_ec::operator::evaluator::Evaluator;

use crate::individual::Image;

#[derive(Clone)]
pub struct ImageEvaluator {
    image: Image,
}

impl ImageEvaluator {
    pub fn new(image: Image) -> Self {
        Self { image }
    }
}

impl Evaluator<Evaluated<Image, u64>> for ImageEvaluator {
    type Error = Infallible;

    fn evaluate<Rng>(
        &self,
        individual: &Evaluated<Image, u64>,
        _: &mut Rng,
    ) -> Result<u64, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let fitness = individual
            .genome()
            .iter()
            .zip(self.image.genome().iter())
            .map(|(&lhs, &rhs)| (lhs.abs_diff(rhs) as u64).pow(2))
            .sum();

        Ok(fitness)
    }
}
