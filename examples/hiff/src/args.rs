use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The population size.
    #[arg(long, default_value_t = 100)]
    pub population: usize,

    /// The number of bits in a bitstring.
    #[arg(long, default_value_t = 128)]
    pub bits: usize,

    /// The number of generations.
    #[arg(long, default_value_t = 100)]
    pub generations: usize,

    /// Visualize the best individual.
    #[arg(long)]
    pub visualize: bool,

    /// Enable parallel selection.
    #[arg(long)]
    pub parallel: bool,
}
