use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The path to the image.
    pub path: PathBuf,

    /// The population size.
    #[arg(long, default_value_t = 1000)]
    pub population: usize,

    /// The maximum width.
    #[arg(long, default_value_t = 30)]
    pub width: u32,

    /// The maximum height.
    #[arg(long, default_value_t = 30)]
    pub height: u32,

    /// The mutation rate.
    #[arg(long, default_value_t = 0.1)]
    pub rate: f64,
}
