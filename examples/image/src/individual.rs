use brace_ec::fitness::nil::Nil;
use brace_ec::individual::Individual;
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
    type Fitness = Nil;

    fn genome(&self) -> &Self::Genome {
        &self.0
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        &mut self.0
    }

    fn fitness(&self) -> &Self::Fitness {
        Nil::r#ref()
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        Nil::r#mut()
    }
}
