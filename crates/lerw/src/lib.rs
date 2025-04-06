pub use lerw::Point;
pub use point2d::Point2D;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod lerw;
mod point2d;

pub fn simulate(len: usize, seed: u64) -> Vec<Point2D> {
    lerw::simple_walk(Point2D::zero(), ChaCha8Rng::seed_from_u64(seed))
        .take(len)
        .collect()
}
