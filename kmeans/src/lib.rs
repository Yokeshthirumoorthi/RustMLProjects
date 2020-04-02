// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use std::ops::{Add, Mul, Sub, Div};

trait KMeansMath {
    fn distance(self, other: Self) -> f32;
}

/// Vector2D strictly defines a (x,y) point type
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D {
    // 2 column point
    point: (f32, f32),
}

impl Vector2D {
    pub fn new(point: (f32, f32)) -> Self {
        Vector2D { point }
    }
    fn square(self) -> Self {
        self * self
    }
    fn foldsum(self) -> f32 {
        self.point.0 + self.point.1
    }
}

#[test]
fn vector2D_works() {
    let p0 = Vector2D::new((0.0,0.0));
    let p1 = Vector2D::new((1.0,1.0));
    let p2 = Vector2D::new((2.0,2.0));
    assert_eq!(p0, Vector2D { point: (0.0, 0.0) });
    assert_eq!(p1, Vector2D { point: (1.0, 1.0) });
    assert_eq!(p0.square(), p0);
    assert_eq!(p2.foldsum(), 4.0);
}

impl Mul for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2D {
            point: (self.point.0 * rhs.point.0, self.point.1 * rhs.point.1),
        }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            point: (self.point.0 - rhs.point.0, self.point.1 - rhs.point.1),
        }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            point: (self.point.0 + rhs.point.0, self.point.1 + rhs.point.1),
        }
    }
}

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
fn vector2D_operator_overloadings() {
    let p0 = Vector2D::new((0.0,0.0));
    let p1 = Vector2D::new((1.0,1.0));
    let p2 = Vector2D::new((2.0,2.0));
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
#[should_panic("Cannot divide by zero-valued `Rational`!")]
fn vector2D_check_div_by_0() {
    Vector2D::new((0.0,0.0)) / 0.0;
}

impl KMeansMath for Vector2D {
    fn distance(self, rhs: Self) -> f32 {
        // determine simple euclidian distance
        // sqrt((x1-x2)^2 + (y1-y2)^2)
        (self - rhs).square().foldsum().sqrt()
    }
}


#[test]
fn vector2D_kmeans_math() {
    let p0 = Vector2D::new((0.0,0.0));
    let p1 = Vector2D::new((1.0,1.0));
    let p2 = Vector2D::new((2.0,2.0));
    assert_eq!(p0.distance(p1), 1.4142135);
    assert_eq!(p1.distance(p2), 1.4142135);
    assert_eq!(p2.distance(p2), 0.0);
}