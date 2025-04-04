use clap::Parser;
use percolation::*;

mod norms;
mod percolation;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Size of the lattice
    #[arg(short, long, default_value_t = 300)]
    lattice_size: usize,

    /// Alpha parameter
    #[arg(short, long, default_value_t = 8.0)]
    alpha: f64,

    /// Beta parameter
    #[arg(short, long, default_value_t = 0.2)]
    beta: f64,

    /// Number of samples
    #[arg(short, long, default_value_t = 200)]
    samples: u64,

    /// Random seed
    #[arg(long, default_value_t = 42)]
    seed: u64,

    /// Norm to use for distance calculations (l1 or linf)
    #[arg(short = 'N', long, value_enum, default_value_t = norms::Norm::L1)]
    norm: norms::Norm,
}

pub fn main() {
    let args = Args::parse();

    let obs = simulate(
        args.norm,
        args.lattice_size,
        args.alpha,
        args.beta,
        args.samples,
        args.seed,
    );

    println!(
        "l = {}, α = {}, β = {}",
        args.lattice_size, args.alpha, args.beta
    );

    println!(
        "Average size: {:?}",
        obs.iter().map(|o| o.average_size).sum::<f64>()
    );

    println!(
        "Average spread: {:?}",
        obs.iter().map(|o| o.size_spread).sum::<f64>()
    );
}
