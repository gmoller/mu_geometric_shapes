use crate::circle::Circle;
use crate::hexagon::Hexagon;
use crate::rectangle::{Rectangle, RoundFactors};
use vector2d::Vector2D;

pub mod circle;
pub mod hexagon;
pub mod rectangle;

pub trait Shape {
    fn area(&self) -> f64;

    fn perimeter(&self) -> f64;

    /// Signed distance functions are passed the coordinates of a point in space and return the
    /// shortest distance between that point and some surface.
    /// The sign of the return value indicates whether the point is inside that surface (negative)
    /// or outside (positive). A return value of zero indicates the point is exactly on the surface.
    fn sdf(&self, point: &Vector2D<f64>) -> f64;
}

#[derive(Debug, PartialEq)]
pub enum HexagonOrientation {
    Horizontal, // pointy
    Vertical,   // flat
}

pub struct ShapeFactory;

impl ShapeFactory {
    pub fn new_circle(center: Vector2D<f64>, radius: f64) -> Box<dyn Shape> {
        Box::new(Circle::new(center, radius))
    }

    pub fn new_hexagon_vertical(center: Vector2D<f64>, circumradius: f64) -> Box<dyn Shape> {
        Box::new(Hexagon::new(
            center,
            circumradius,
            HexagonOrientation::Vertical,
        ))
    }

    pub fn new_hexagon_horizontal(center: Vector2D<f64>, circumradius: f64) -> Box<dyn Shape> {
        Box::new(Hexagon::new(
            center,
            circumradius,
            HexagonOrientation::Horizontal,
        ))
    }

    pub fn new_rectangle(center: Vector2D<f64>, dimensions: Vector2D<f64>) -> Box<dyn Shape> {
        Box::new(Rectangle::new(
            center,
            dimensions,
            0.0,
            RoundFactors::new(0.0, 0.0, 0.0, 0.0),
        ))
    }

    pub fn new_rectangle_oriented(
        center: Vector2D<f64>,
        dimensions: Vector2D<f64>,
        rotation_angle_in_degrees: f64,
    ) -> Box<dyn Shape> {
        Box::new(Rectangle::new(
            center,
            dimensions,
            rotation_angle_in_degrees,
            RoundFactors::new(0.0, 0.0, 0.0, 0.0),
        ))
    }

    pub fn new_rectangle_rounded(
        center: Vector2D<f64>,
        dimensions: Vector2D<f64>,
        round_factors: RoundFactors,
    ) -> Box<dyn Shape> {
        Box::new(Rectangle::new(center, dimensions, 0.0, round_factors))
    }

    pub fn new_rectangle_rounded_oriented(
        center: Vector2D<f64>,
        dimensions: Vector2D<f64>,
        rotation_angle_in_degrees: f64,
        round_factors: RoundFactors,
    ) -> Box<dyn Shape> {
        Box::new(Rectangle::new(center, dimensions, rotation_angle_in_degrees, round_factors))
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

fn dot_product(v1: &Vector2D<f64>, v2: &Vector2D<f64>) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

fn rotate_vector_by_degrees(v: &Vector2D<f64>, degrees: f64) -> Vector2D<f64> {
    let radians = degrees.to_radians();

    rotate_vector_by_radians(v, radians)
}

fn rotate_vector_by_radians(v: &Vector2D<f64>, radians: f64) -> Vector2D<f64> {
    let sine = radians.sin();
    let cosine = radians.cos();

    Vector2D::new(v.x * cosine - v.y * sine, v.x * sine + v.y * cosine)
}

fn rotate_vector_by_30_degrees(v: &Vector2D<f64>) -> Vector2D<f64> {
    Vector2D::new(
        v.x * 0.86602540378 - v.y * 0.5,
        v.x * 0.5 + v.y * 0.86602540378,
    )
}

fn min_f64(v1: f64, v2: f64) -> f64 {
    v1.min(v2)
}

fn max_f64(v1: f64, v2: f64) -> f64 {
    v1.max(v2)
}

pub fn get_area<T: Shape>(t: &T) -> f64 {
    t.area()
}

pub fn get_sdf<T: Shape>(t: &T, p: &Vector2D<f64>) -> f64 {
    t.sdf(p)
}
