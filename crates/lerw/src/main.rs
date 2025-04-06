use lerw::Point;
use point2d::Point2D;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod lerw;
mod point2d;

fn main() {
    println!(
        "Hello, {:?}",
        lerw::simple_walk(Point2D::zero(), ChaCha8Rng::seed_from_u64(42))
            .take(10)
            .collect::<Vec<_>>()
    );
}
