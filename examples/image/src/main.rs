mod args;
mod evaluator;
mod evolver;
mod generator;
mod individual;
mod renderer;
mod selector;

use anyhow::Error;
use brace_ec::operator::evolver::Evolver;
use brace_ec::operator::generator::Generator;
use brace_ec_tui::evolver::Terminal;
use clap::Parser;
use image::imageops::FilterType;

use self::args::Args;
use self::evaluator::ImageEvaluator;
use self::evolver::ImageEvolver;
use self::generator::ImageGenerator;
use self::individual::Image;
use self::renderer::ImageRenderer;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let image = image::open(&args.path)?
        .grayscale()
        .resize(args.width, args.height, FilterType::Nearest)
        .into_luma8();

    let mut rng = rand::rng();

    let evaluator = ImageEvaluator::new(Image::new(image.clone()));
    let generator = ImageGenerator::new(image.width(), image.height())
        .evaluate(evaluator.clone())
        .populate::<Vec<_>>(args.population);

    let population = generator.generate(&mut rng)?;

    let evolver = Terminal::new(
        ImageEvolver::new(evaluator, args.rate, args.parallel),
        ImageRenderer::new(Image::new(image)),
    );

    evolver.evolve((0, population), &mut rng)?;

    Ok(())
}
