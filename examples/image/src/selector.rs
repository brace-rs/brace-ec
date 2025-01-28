use std::convert::Infallible;

use brace_ec::core::individual::scored::Scored;
use brace_ec::core::individual::Individual;
use brace_ec::core::operator::score::{Score, ScoreError};
use brace_ec::core::operator::selector::and::{And, AndError};
use brace_ec::core::operator::selector::first::{First, FirstError};
use brace_ec::core::operator::selector::windows::{ArrayWindows, ParArrayWindows, WindowsError};
use brace_ec::core::operator::selector::Selector;
use thiserror::Error;

use crate::individual::Image;
use crate::scorer::ImageScorer;

#[allow(clippy::type_complexity)]
pub enum ImageSelector {
    Serial(
        And<
            First<[Scored<Image, u64>]>,
            ArrayWindows<2, Score<ImageWindowsSelector, ImageScorer>, [Scored<Image, u64>]>,
        >,
    ),
    Parallel(
        And<
            First<[Scored<Image, u64>]>,
            ParArrayWindows<2, Score<ImageWindowsSelector, ImageScorer>, [Scored<Image, u64>]>,
        >,
    ),
}

impl ImageSelector {
    pub fn new(scorer: ImageScorer, rate: f64, parallel: bool) -> Self {
        match parallel {
            false => Self::Serial(
                First.and(
                    ImageWindowsSelector::new(rate)
                        .score(scorer)
                        .array_windows(),
                ),
            ),
            true => Self::Parallel(
                First.and(
                    ImageWindowsSelector::new(rate)
                        .score(scorer)
                        .par_array_windows(),
                ),
            ),
        }
    }
}

impl Selector<Vec<Scored<Image, u64>>> for ImageSelector {
    type Output = Vec<Scored<Image, u64>>;
    type Error = ImageSelectorError;

    fn select<Rng>(
        &self,
        population: &Vec<Scored<Image, u64>>,
        rng: &mut Rng,
    ) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self {
            Self::Serial(serial) => serial
                .select(population.as_slice(), rng)
                .map_err(ImageSelectorError),
            Self::Parallel(parallel) => parallel
                .select(population.as_slice(), rng)
                .map_err(ImageSelectorError),
        }
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ImageSelectorError(
    AndError<FirstError, WindowsError<ScoreError<Infallible, Infallible>>>,
);

pub struct ImageWindowsSelector {
    rate: f64,
}

impl ImageWindowsSelector {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl Selector<[Scored<Image, u64>; 2]> for ImageWindowsSelector {
    type Output = [Scored<Image, u64>; 1];
    type Error = Infallible;

    fn select<Rng>(
        &self,
        [a, b]: &[Scored<Image, u64>; 2],
        rng: &mut Rng,
    ) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut image = b.clone();

        for (lhs, &rhs) in image.genome_mut().iter_mut().zip(a.genome().iter()) {
            if rng.random_bool(self.rate) {
                *lhs = rhs;
            }
        }

        Ok([image])
    }
}
