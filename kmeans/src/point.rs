// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use std::ops::{Add, Div, Mul, Sub};
use std::fmt;

pub trait KMeansPoint: Sized {
    type Output;
    fn square(self) -> Self;
    fn sum(self) -> Self::Output;
    fn distance(self, other: Self) -> Self::Output;
}

pub type Point = Vector2D;

/// Vector2D strictly defines a (x,y) point type
/// Goal is this kmeans algorithm should run in constant time for n dimensions
#[derive(Copy, Clone, PartialEq)]
pub struct Vector2D {
    // 2 column point
    pub point: (f32, f32),
}

impl Vector2D {
    pub fn new(point: (f32, f32)) -> Self {
        Vector2D { point }
    }
}

#[test]
fn vector2d_works() {
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let p2 = Vector2D::new((2.0, 2.0));
    let p4 = Vector2D::new((4.0, 4.0));
    assert_eq!(p0, Vector2D { point: (0.0, 0.0) });
    assert_eq!(p1, Vector2D { point: (1.0, 1.0) });
    assert_eq!(p0.square(), p0);
    assert_eq!(p1.square(), p1);
    assert_eq!(p2.square(), p4);
    assert_eq!(p0.sum(), 0.0);
    assert_eq!(p2.sum(), 4.0);
}

/// multiply two points and the new point is
/// zip product of each dimentsions
impl Mul for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2D {
            point: (self.point.0 * rhs.point.0, self.point.1 * rhs.point.1),
        }
    }
}

/// subtract two points and the new point is
/// zip diff of each dimentsions
impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            point: (self.point.0 - rhs.point.0, self.point.1 - rhs.point.1),
        }
    }
}

/// add two points and the new point is
/// zip sum of each dimentsions
impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            point: (self.point.0 + rhs.point.0, self.point.1 + rhs.point.1),
        }
    }
}

/// div a point with a number and the new point
/// is all elements divided over the number.
/// It panics if the divisor is 0.
impl Div<f32> for Vector2D {
    type Output = Vector2D;
    fn div(self, n: f32) -> Self::Output {
        if n == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Vector2D {
            point: (self.point.0 / n, self.point.1 / n),
        }
    }
}

impl fmt::Debug for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.point.0)
         .field(&self.point.1)
         .finish()
    }
}

impl KMeansPoint for Vector2D {
    type Output = f32;
    /// squaring a point generates new point
    /// whose dimensions are squares of the given point
    fn square(self) -> Self {
        self * self
    }
    /// sum up all the dimensions of a point
    fn sum(self) -> f32 {
        self.point.0 + self.point.1
    }
    // determine simple euclidian distance
    // sqrt((x1-x2)^2 + (y1-y2)^2)
    fn distance(self, rhs: Self) -> f32 {
        (self - rhs).square().sum().sqrt()
    }
}
