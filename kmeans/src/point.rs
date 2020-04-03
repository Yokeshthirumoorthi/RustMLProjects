// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use std::ops::{Add, Div, Mul, Sub};

// trait KMeansMath {
//     fn distance(self, other: Self) -> f32;
// }

/// Vector2D strictly defines a (x,y) point type
/// Goal is this kmeans algorithm should run in constant time for n dimensions
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D {
    // 2 column point
    pub point: (f32, f32),
}

impl Vector2D {
    pub fn new(point: (f32, f32)) -> Self {
        Vector2D { point }
    }
    /// squaring a point generates new point
    /// whose dimensions are squares of the given point
    fn square(self) -> Self {
        self * self
    }
    /// sum up all the dimensions of a point
    fn foldsum(self) -> f32 {
        self.point.0 + self.point.1
    }
    /// determine simple euclidian distance
    /// sqrt((x1-x2)^2 + (y1-y2)^2)
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).square().foldsum().sqrt()
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
    assert_eq!(p0.foldsum(), 0.0);
    assert_eq!(p2.foldsum(), 4.0);
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

#[test]
fn vector2d_operator_overloadings() {
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let p2 = Vector2D::new((2.0, 2.0));
    assert_eq!(p0 * p0, p0);
    assert_eq!(p0 * p1, p0);
    assert_eq!(p1 * p2, p2);
    assert_eq!(p2 - p1, p1);
    assert_eq!(p1 - p1, p0);
    assert_eq!(p1 + p1, p2);
    assert_eq!(p1 / 1.0, p1);
    assert_eq!(p0 / 1.0, p0);
}

#[test]
#[should_panic(expected = "Cannot divide by zero-valued `Rational`!")]
fn vector2d_check_div_by_0() {
    let p1 = Vector2D::new((1.0, 1.0));
    p1 / 0.0;
}

// impl KMeansMath for Vector2D {
//     // determine simple euclidian distance
//     // sqrt((x1-x2)^2 + (y1-y2)^2)
//     fn distance(self, rhs: Self) -> f32 {
//         (self - rhs).square().foldsum().sqrt()
//     }
// }

#[test]
fn vector2d_kmeans_math() {
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let p2 = Vector2D::new((2.0, 2.0));
    assert_eq!(p0.distance(p1), 1.4142135);
    assert_eq!(p1.distance(p2), 1.4142135);
    assert_eq!(p2.distance(p2), 0.0);
}
