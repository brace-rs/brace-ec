use std::convert::Infallible;

use brace_ec::individual::evaluated::Evaluated;
use brace_ec::operator::generator::random::Random;
use brace_ec::operator::generator::Generator;
use image::GrayImage;

use crate::individual::Image;

pub struct ImageGenerator {
    width: u32,
    height: u32,
}

impl ImageGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Generator<Evaluated<Image, u64>> for ImageGenerator {
    type Error = Infallible;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<Evaluated<Image, u64>, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let pixels = Random::from(0..=255)
            .populate(self.width as usize * self.height as usize)
            .generate(rng)
            .expect("infallible");

        let image = GrayImage::from_vec(self.width, self.height, pixels).expect("valid");

        Ok(Evaluated::new(Image::new(image), 0))
    }
}
