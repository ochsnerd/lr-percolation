use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "David Ochsner", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!(
        "Hello {} (from open-weightlifting)!",
        args.name.unwrap_or("world".to_string())
    );
}
