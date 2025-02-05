mod args;
mod evolver;
mod generator;
mod individual;
mod renderer;
mod scorer;
mod selector;

use anyhow::Error;
use brace_ec::operator::evolver::Evolver;
use brace_ec::operator::generator::Generator;
use brace_ec_tui::evolver::Terminal;
use clap::Parser;
use image::imageops::FilterType;

use self::args::Args;
use self::evolver::ImageEvolver;
use self::generator::ImageGenerator;
use self::individual::Image;
use self::renderer::ImageRenderer;
use self::scorer::ImageScorer;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let image = image::open(&args.path)?
        .grayscale()
        .resize(args.width, args.height, FilterType::Nearest)
        .into_luma8();

    let mut rng = rand::rng();

    let scorer = ImageScorer::new(Image::new(image.clone()));
    let generator = ImageGenerator::new(image.width(), image.height())
        .score(scorer.clone())
        .populate::<Vec<_>>(args.population);

    let population = generator.generate(&mut rng)?;

    let evolver = Terminal::new(
        ImageEvolver::new(scorer, args.rate, args.parallel),
        ImageRenderer::new(Image::new(image)),
    );

    evolver.evolve((0, population), &mut rng)?;

    Ok(())
}
