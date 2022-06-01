use crate::circle::Circle;
use crate::rectangle::Rectangle;
use crate::rectangle_oriented::RectangleOriented;
use crate::rectangle_rounded::{RectangleRounded, RoundFactors};
use vector2d::Vector2D;

pub mod circle;
pub mod rectangle;
pub mod rectangle_oriented;
pub mod rectangle_rounded;

pub trait Shape {
    fn area(&self) -> f64;

    fn perimeter(&self) -> f64;

    /// Signed distance functions are passed the coordinates of a point in space and return the
    /// shortest distance between that point and some surface.
    /// The sign of the return value indicates whether the point is inside that surface (negative)
    /// or outside (positive). A return value of zero indicates the point is exactly on the surface.
    fn sdf(&self, point: &Vector2D<f64>) -> f64;
}

pub struct ShapeFactory;

impl ShapeFactory {
    pub fn new_circle(center: Vector2D<f64>, radius: f64) -> Box<dyn Shape> {
        Box::new(Circle::new(center, radius))
    }

    pub fn new_rectangle(center: Vector2D<f64>, dimensions: Vector2D<f64>) -> Box<dyn Shape> {
        Box::new(Rectangle::new(center, dimensions))
    }

    pub fn new_rectangle_oriented(
        center: Vector2D<f64>,
        dimensions: Vector2D<f64>,
        rotation_angle_in_degrees: f64,
    ) -> Box<dyn Shape> {
        let rectangle = Rectangle::new(center, dimensions);

        Box::new(RectangleOriented::new(rectangle, rotation_angle_in_degrees))
    }

    pub fn new_rectangle_rounded(
        center: Vector2D<f64>,
        dimensions: Vector2D<f64>,
        round_factors: RoundFactors,
    ) -> Box<dyn Shape> {
        let rectangle = Rectangle::new(center, dimensions);

        Box::new(RectangleRounded::new(rectangle, round_factors))
    }
}

fn abs_vector(v: &Vector2D<f64>) -> Vector2D<f64> {
    Vector2D::new(v.x.abs(), v.y.abs())
}

fn length_vector(v: &Vector2D<f64>) -> f64 {
    v.length()
}

fn max_vector(v: &Vector2D<f64>, scalar: f64) -> Vector2D<f64> {
    Vector2D::new(v.x.max(scalar), v.y.max(scalar))
}

fn rotate_vector_by_degrees(v: &Vector2D<f64>, degrees: f64) -> Vector2D<f64> {
    let radians = degrees.to_radians();

    rotate_vector_by_radians(&v, radians)
}

fn rotate_vector_by_radians(v: &Vector2D<f64>, radians: f64) -> Vector2D<f64> {
    let sine = radians.sin();
    let cosine = radians.cos();

    Vector2D::new(v.x * cosine - v.y * sine, v.x * sine + v.y * cosine)
}

fn min_float(v1: f64, v2: f64) -> f64 {
    v1.min(v2)
}

fn max_float(v1: f64, v2: f64) -> f64 {
    v1.max(v2)
}

pub fn get_area<T: Shape>(t: &T) -> f64 {
    t.area()
}

pub fn get_sdf<T: Shape>(t: &T, p: &Vector2D<f64>) -> f64 {
    t.sdf(p)
}
