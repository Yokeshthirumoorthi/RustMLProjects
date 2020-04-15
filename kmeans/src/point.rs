// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use std::ops::{Add, Div, Mul, Sub};

pub trait KMeansPoint: Sized {
    type Output;
    fn square(self) -> Self;
    fn sum(self) -> Self::Output;
    fn distance(self, other: Self) -> Self::Output;
}

pub type Point = Vector;

/// Vector type defines on point in dataset.
#[derive(Clone, PartialEq, Debug)]
pub struct Vector {
    pub point: Vec<f32>,
}

impl Vector {
    pub fn new(point: Vec<f32>) -> Self {
        Vector {point}
    }
}

/// multiply two points and the new point is
/// zip product of each dimentsions
impl Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector {
            point: (self.point.iter()).zip(rhs.point.iter()).map(|(a,b)| a*b).collect(),
        }
    }
}

/// subtract two points and the new point is
/// zip diff of each dimentsions
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            point: (self.point.iter()).zip(rhs.point.iter()).map(|(a,b)| a-b).collect(),
        }
    }
}

/// add two points and the new point is
/// zip sum of each dimentsions
impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            point: (self.point.iter()).zip(rhs.point.iter()).map(|(a,b)| a+b).collect(),
        }
    }
}

/// div a point with a number and the new point
/// is all elements divided over the number.
/// It panics if the divisor is 0.
impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, n: f32) -> Self::Output {
        if n == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Vector {
            point: self.point.iter().map(|a| a/n).collect(),
        }
    }
}

impl KMeansPoint for Vector {
    type Output = f32;
    /// squaring a point generates new point
    /// whose dimensions are squares of the given point
    fn square(self) -> Self {
        self.clone() * self
    }
    /// sum up all the dimensions of a point
    fn sum(self) -> f32 {
        self.point.iter().sum()
    }
    // determine simple euclidian distance
    // sqrt((x1-x2)^2 + (y1-y2)^2)
    fn distance(self, rhs: Self) -> f32 {
        (self - rhs).square().sum().sqrt()
    }
}