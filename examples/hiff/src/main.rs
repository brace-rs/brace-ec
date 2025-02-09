pub mod args;
pub mod renderer;

use std::io::Write;

use anyhow::Error;
use brace_ec::fitness::summed::Summed;
use brace_ec::generation::Generation;
use brace_ec::individual::evaluated::Evaluated;
use brace_ec::individual::Individual;
use brace_ec::operator::evaluator::hiff::Hiff;
use brace_ec::operator::evolver::Evolver;
use brace_ec::operator::generator::random::Random;
use brace_ec::operator::generator::Generator;
use brace_ec::operator::mutator::invert::Invert;
use brace_ec::operator::mutator::Mutator;
use brace_ec::operator::recombinator::point::TwoPointCrossover;
use brace_ec::operator::selector::best::Best;
use brace_ec::operator::selector::lexicase::Lexicase;
use brace_ec::operator::selector::tournament::Tournament;
use brace_ec::operator::selector::Selector;
use brace_ec::operator::weighted::Weighted;
use brace_ec::operator::IntoParallelOperator;
use brace_ec::population::Population;
use brace_ec_tui::evolver::Terminal;
use clap::Parser;

use self::args::Args;
use self::renderer::HiffRenderer;

type Ind = Evaluated<Vec<bool>, Summed<Vec<usize>>>;
type Pop = Vec<Ind>;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut rng = rand::rng();

    let population: Pop = Random::bernoulli(0.5)
        .populate(args.bits)
        .evaluate(Hiff)
        .populate(args.population)
        .generate(&mut rng)?;

    let selector = Weighted::selector(Best, 1)
        .with_selector(Lexicase, 5)
        .with_selector(Tournament::binary(), args.population as u64 - 1)
        .twice() // ICE: .repeat(2).take::<2>()
        .reproduce(TwoPointCrossover)
        .mutate(Invert.each_reciprocal_rate())
        .evaluate(Hiff)
        .fill()
        .parallel_if(args.parallel);

    if args.visualize {
        Terminal::new(
            selector.evolver().limit(args.generations as u64),
            HiffRenderer,
        )
        .evolve((0, population), &mut rng)?;
    } else {
        let generation = (0, population);

        print_best(&generation);

        selector
            .evolver()
            .inspect(print_best)
            .repeat(args.generations)
            .evolve(generation, &mut rng)?;
    }

    Ok(())
}

fn print_best(generation: &(u64, Pop)) {
    let [best] = generation.population().select(Best).unwrap();

    let mut stdout = std::io::stdout().lock();

    writeln!(
        stdout,
        "Generation = {}, Fitness = {}, Genome =",
        generation.id(),
        best.fitness().total()
    )
    .ok();

    for &bit in best.genome() {
        write!(stdout, " {}", bit as u8).ok();
    }

    writeln!(stdout).ok();
}
