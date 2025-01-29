use brace_ec::core::individual::Individual;
use image::GrayImage;

#[derive(Clone)]
pub struct Image(GrayImage);

impl Image {
    pub fn new(image: GrayImage) -> Self {
        Self(image)
    }
}

impl Individual for Image {
    type Genome = GrayImage;
    type Fitness = [u8; 0];

    fn genome(&self) -> &Self::Genome {
        &self.0
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        &mut self.0
    }

    fn fitness(&self) -> &Self::Fitness {
        &[]
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        &mut []
    }
}
