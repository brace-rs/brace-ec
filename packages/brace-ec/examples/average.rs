use brace_ec::core::generation::Generation;
use brace_ec::core::operator::evolver::select::Select;
use brace_ec::core::operator::evolver::Evolver;
use brace_ec::core::operator::mutator::noise::Noise;
use brace_ec::core::operator::mutator::Mutator;
use brace_ec::core::operator::recombinator::average::Average;
use brace_ec::core::operator::selector::best::Best;
use brace_ec::core::operator::selector::tournament::Tournament;
use brace_ec::core::operator::selector::Selector;
use brace_ec::core::population::Population;

pub fn main() {
    let generation = (0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let selector = Tournament::binary()
        .mutate(Noise(1..10).rate(0.5))
        .repeat(2)
        .recombine(Average)
        .mutate(Noise(1..10).rate(0.5));

    print_generation(&generation);

    Select::new(selector)
        .repeat(10)
        .inspect(print_generation)
        .repeat(10)
        .inspect(print_best)
        .evolve(generation, &mut rand::thread_rng())
        .unwrap();
}

fn print_best(generation: &(u64, [i64; 10])) {
    let best = generation.population().select(Best).unwrap();

    println!("Best: {:>4}", best[0]);
}

fn print_generation(generation: &(u64, [i64; 10])) {
    println!(
        "{:>4}: {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4}",
        generation.id(),
        generation.population()[0],
        generation.population()[1],
        generation.population()[2],
        generation.population()[3],
        generation.population()[4],
        generation.population()[5],
        generation.population()[6],
        generation.population()[7],
        generation.population()[8],
        generation.population()[9],
    )
}
