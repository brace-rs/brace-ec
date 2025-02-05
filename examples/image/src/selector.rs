use std::convert::Infallible;

use brace_ec::individual::evaluated::Evaluated;
use brace_ec::individual::Individual;
use brace_ec::operator::evaluate::{Evaluate, EvaluateError};
use brace_ec::operator::selector::and::{And, AndError};
use brace_ec::operator::selector::first::{First, FirstError};
use brace_ec::operator::selector::windows::{ArrayWindows, ParArrayWindows, WindowsError};
use brace_ec::operator::selector::Selector;
use thiserror::Error;

use crate::evaluator::ImageEvaluator;
use crate::individual::Image;

#[allow(clippy::type_complexity)]
pub enum ImageSelector {
    Serial(
        And<
            First<[Evaluated<Image, u64>]>,
            ArrayWindows<
                2,
                Evaluate<ImageWindowsSelector, ImageEvaluator>,
                [Evaluated<Image, u64>],
            >,
        >,
    ),
    Parallel(
        And<
            First<[Evaluated<Image, u64>]>,
            ParArrayWindows<
                2,
                Evaluate<ImageWindowsSelector, ImageEvaluator>,
                [Evaluated<Image, u64>],
            >,
        >,
    ),
}

impl ImageSelector {
    pub fn new(evaluator: ImageEvaluator, rate: f64, parallel: bool) -> Self {
        match parallel {
            false => Self::Serial(
                First.and(
                    ImageWindowsSelector::new(rate)
                        .evaluate(evaluator)
                        .array_windows(),
                ),
            ),
            true => Self::Parallel(
                First.and(
                    ImageWindowsSelector::new(rate)
                        .evaluate(evaluator)
                        .par_array_windows(),
                ),
            ),
        }
    }
}

impl Selector<Vec<Evaluated<Image, u64>>> for ImageSelector {
    type Output = Vec<Evaluated<Image, u64>>;
    type Error = ImageSelectorError;

    fn select<Rng>(
        &self,
        population: &Vec<Evaluated<Image, u64>>,
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
    AndError<FirstError, WindowsError<EvaluateError<Infallible, Infallible>>>,
);

pub struct ImageWindowsSelector {
    rate: f64,
}

impl ImageWindowsSelector {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl Selector<[Evaluated<Image, u64>; 2]> for ImageWindowsSelector {
    type Output = [Evaluated<Image, u64>; 1];
    type Error = Infallible;

    fn select<Rng>(
        &self,
        [a, b]: &[Evaluated<Image, u64>; 2],
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
