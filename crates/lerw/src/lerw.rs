use std::ops::AddAssign;

use rand::{Rng, seq::IndexedRandom};

pub trait Point: Sized + Copy + AddAssign + Eq {
    fn zero() -> Self;
    fn directions() -> Vec<Self>;
}

pub fn simple_walk<P: Point, R: Rng>(start: P, rng: R) -> SimpleWalk<P, R> {
    SimpleWalk::new(start, rng)
}

pub struct SimpleWalk<P: Point, R: Rng> {
    pos: P,
    rng: R,
    directions: Vec<P>,
}

impl<P: Point, R: Rng> SimpleWalk<P, R> {
    pub fn new(pos: P, rng: R) -> Self {
        SimpleWalk {
            pos,
            rng,
            directions: P::directions(),
        }
    }
}

impl<P: Point, R: Rng> Iterator for SimpleWalk<P, R> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pos;
        self.pos += *self.directions.choose(&mut self.rng).unwrap();
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point2D;
    use rand::RngCore;

    // A mock RNG that always returns the same value
    struct MockRng {
        value: u64,
    }

    impl MockRng {
        fn new(value: u64) -> Self {
            Self { value }
        }
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.value as u8;
            }
        }
    }

    #[test]
    fn test_walk_in_one_direction() {
        let start = Point2D::zero();
        let mock_rng = MockRng::new(0);

        let walk = simple_walk(start, mock_rng).take(100).collect::<Vec<_>>();

        assert_eq!(walk[0], start);

        let expected_direction = Point2D::directions()[0];

        assert_eq!(walk[1], start + expected_direction);
        assert_eq!(walk[2], start + expected_direction + expected_direction);
    }
}
