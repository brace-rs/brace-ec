mod args;
mod evolver;
mod individual;
mod renderer;
mod scorer;
mod selector;

use anyhow::{Context, Error};
use brace_ec::core::fitness::FitnessMut;
use brace_ec::core::individual::scored::Scored;
use brace_ec::core::operator::evolver::Evolver;
use brace_ec::core::operator::scorer::Scorer;
use brace_ec_tui::evolver::Terminal;
use clap::Parser;
use image::imageops::FilterType;
use image::GrayImage;
use rand::Rng;

use self::args::Args;
use self::evolver::ImageEvolver;
use self::individual::Image;
use self::renderer::ImageRenderer;
use self::scorer::ImageScorer;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let image = image::open(&args.path)?
        .grayscale()
        .resize(args.width, args.height, FilterType::Nearest)
        .into_luma8();

    let mut population = Vec::with_capacity(args.population);
    let mut rng = rand::thread_rng();

    let scorer = ImageScorer::new(Image::new(image.clone()));

    for _ in 0..args.population {
        let pixels = std::iter::from_fn(|| Some(rng.gen_range(0..255)))
            .take(image.width() as usize * image.height() as usize)
            .collect::<Vec<_>>();

        let image = GrayImage::from_vec(image.width(), image.height(), pixels)
            .context("Invalid image dimensions")?;

        let mut individual = Scored::new(Image::new(image), 0);

        let score = scorer.score(&individual, &mut rng)?;

        individual.set_fitness(score);
        population.push(individual);
    }

    let evolver = Terminal::new(
        ImageEvolver::new(scorer, args.rate, args.parallel),
        ImageRenderer::new(Image::new(image)),
    );

    evolver.evolve((0, population), &mut rng)?;

    Ok(())
}
