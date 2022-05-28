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
    pub fn sdf(&self, point: &Vector2D<f64>) -> f64 {
        // translate to center the circle at origin
        let translated = *point - self.center;

        translated.length() - self.radius
    }
}

#[cfg(test)]
mod tests {
    use crate::circle::Circle;
    use vector2d::Vector2D;

    #[test]
    fn create_circle() {
        let circle = Circle::new(Vector2D::new(10.0, 10.0), 10.0);

        assert_eq!(
            format!("The circle is: {circle:?}"),
            "The circle is: Circle { center: Vector2D { x: 10.0, y: 10.0 }, radius: 10.0 }"
        );

        assert_eq!(circle.center().x, 10.0);
        assert_eq!(circle.center().y, 10.0);
        assert_eq!(circle.radius(), 10.0);

        assert_eq!(circle.area(), 314.1592653589793);

        assert_eq!(circle.circumference(), 62.83185307179586);

        assert_eq!(circle.diameter(), 20.0);
    }

    #[test]
    fn circle_equality() {
        let circle1 = Circle::new(Vector2D::new(10.0, 10.0), 10.0);
        let circle2 = Circle::new(Vector2D::new(10.0, 10.0), 10.0);
        let circle3 = Circle::new(Vector2D::new(11.0, 11.0), 11.0);

        assert_eq!(circle1 == circle2, true);
        assert_eq!(circle1 != circle3, true);
    }

    #[test]
    fn circle_sdf() {
        let circle = Circle::new(Vector2D::new(10.0, 10.0), 10.0);

        assert_eq!(circle.sdf(&Vector2D::new(10.0, 10.0)), -10.0); // center
        assert_eq!(circle.sdf(&Vector2D::new(5.0, 5.0)), -2.9289321881345245);

        assert_eq!(circle.sdf(&Vector2D::new(10.0, 0.0)), 0.0);
        assert_eq!(circle.sdf(&Vector2D::new(0.0, 10.0)), 0.0);

        assert_eq!(circle.sdf(&Vector2D::new(0.0, 0.0)), 4.142135623730951);
        assert_eq!(circle.sdf(&Vector2D::new(-10.0, -10.0)), 18.284271247461902);
    }
}
