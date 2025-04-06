use std::ops::{Add, AddAssign};

use crate::lerw::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Add for Point2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point for Point2D {
    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    fn directions() -> Vec<Self> {
        vec![
            Self { x: 1, y: 0 },
            Self { x: -1, y: 0 },
            Self { x: 0, y: 1 },
            Self { x: 0, y: -1 },
        ]
    }
}
