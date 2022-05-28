use std::f64::consts;
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
pub struct Circle {
    center: Vector2D<f64>,
    radius: f64,
}

impl Circle {
    pub fn new(center: Vector2D<f64>, radius: f64) -> Self {
        Circle { center, radius }
    }

    pub fn center(&self) -> Vector2D<f64> {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn area(&self) -> f64 {
        consts::PI * self.radius.powf(2.0)
    }

    pub fn circumference(&self) -> f64 {
        2.0 * consts::PI * self.radius
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}
