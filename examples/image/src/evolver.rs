use brace_ec::generation::Generation;
use brace_ec::individual::evaluated::Evaluated;
use brace_ec::individual::Individual;
use brace_ec::operator::evolver::Evolver;
use brace_ec::operator::selector::Selector;

use crate::evaluator::ImageEvaluator;
use crate::individual::Image;
use crate::selector::{ImageSelector, ImageSelectorError};

pub struct ImageEvolver {
    selector: ImageSelector,
}

impl ImageEvolver {
    pub fn new(evaluator: ImageEvaluator, rate: f64, parallel: bool) -> Self {
        Self {
            selector: ImageSelector::new(evaluator, rate, parallel),
        }
    }
}

impl Evolver<(u64, Vec<Evaluated<Image, u64>>)> for ImageEvolver {
    type Error = ImageSelectorError;

    fn evolve<Rng>(
        &self,
        mut generation: (u64, Vec<Evaluated<Image, u64>>),
        rng: &mut Rng,
    ) -> Result<(u64, Vec<Evaluated<Image, u64>>), Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut population = self.selector.select(generation.population(), rng)?;

        population.sort_by_key(|individual| *individual.fitness());

        generation.0 += 1;
        generation.1 = population;

        Ok(generation)
    }
}
