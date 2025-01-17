use brace_ec::core::fitness::Fitness;
use brace_ec::core::generation::Generation;
use brace_ec::core::individual::scored::Scored;
use brace_ec::core::operator::evolver::Evolver;
use brace_ec::core::operator::selector::Selector;

use crate::individual::Image;
use crate::scorer::ImageScorer;
use crate::selector::{ImageSelector, ImageSelectorError};

pub struct ImageEvolver {
    selector: ImageSelector,
}

impl ImageEvolver {
    pub fn new(scorer: ImageScorer, rate: f64) -> Self {
        Self {
            selector: ImageSelector::new(scorer, rate),
        }
    }
}

impl Evolver<(u64, Vec<Scored<Image, u64>>)> for ImageEvolver {
    type Error = ImageSelectorError;

    fn evolve<Rng>(
        &self,
        mut generation: (u64, Vec<Scored<Image, u64>>),
        rng: &mut Rng,
    ) -> Result<(u64, Vec<Scored<Image, u64>>), Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut population = self.selector.select(generation.population(), rng)?;

        population.sort_by_key(|individual| individual.fitness());

        generation.0 += 1;
        generation.1 = population;

        Ok(generation)
    }
}
