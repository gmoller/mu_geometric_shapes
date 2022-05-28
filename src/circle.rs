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

    /// Signed distance functions are passed the coordinates of a point in space and return the
    /// shortest distance between that point and some surface.
    /// The sign of the return value indicates whether the point is inside that surface (negative)
    /// or outside (positive). A return value of zero indicates the point is exactly on the surface.
    pub fn sdf(&self, point: Vector2D<f64>) -> f64 {
        // translate to center the circle at origin
        let translated = point - self.center;

        translated.length() - self.radius
    }
}
