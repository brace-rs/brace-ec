use std::convert::Infallible;

use brace_ec::core::individual::scored::Scored;
use brace_ec::core::individual::Individual;
use brace_ec::core::operator::scorer::Scorer;

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
    type Score = u64;
    type Error = Infallible;

    fn score<Rng>(
        &self,
        input: &Scored<Image, u64>,
        _: &mut Rng,
    ) -> Result<Self::Score, Self::Error>
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
