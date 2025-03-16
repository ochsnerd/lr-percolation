use percolation::simulate;

mod percolation;

pub fn main() {
    let l = 300;
    let n_samples = 200;
    let sigma = 10.0;

    let obs = simulate(l, sigma, 0.2, n_samples, 42);

    println!("l = {}, σ = {}", l, sigma);

    println!(
        "Average size: {:?}",
        obs.iter().map(|o| o.average_size).sum::<f64>()
    );

    println!(
        "Average spread: {:?}",
        obs.iter().map(|o| o.size_spread).sum::<f64>()
    );
}
