use std::convert::Infallible;

use brace_ec::individual::scored::Scored;
use brace_ec::individual::Individual;
use brace_ec::operator::scorer::Scorer;

use crate::individual::Image;

#[derive(Clone)]
pub struct ImageScorer {
    image: Image,
}

impl ImageScorer {
    pub fn new(image: Image) -> Self {
        Self { image }
    }
}

impl Scorer<Scored<Image, u64>> for ImageScorer {
    type Error = Infallible;

    fn score<Rng>(&self, input: &Scored<Image, u64>, _: &mut Rng) -> Result<u64, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let score = input
            .genome()
            .iter()
            .zip(self.image.genome().iter())
            .map(|(&lhs, &rhs)| (lhs.abs_diff(rhs) as u64).pow(2))
            .sum();

        Ok(score)
    }
}
