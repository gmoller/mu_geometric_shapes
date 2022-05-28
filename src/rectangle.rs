use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    center: Vector2D<f64>,
    dimensions: Vector2D<f64>,
}

impl Rectangle {
    pub fn new(center: Vector2D<f64>, dimensions: Vector2D<f64>) -> Self {
        Rectangle { center, dimensions }
    }

    pub fn center(&self) -> Vector2D<f64> {
        self.center
    }

    pub fn dimensions(&self) -> Vector2D<f64> {
        self.dimensions
    }

    pub fn area(&self) -> f64 {
        self.dimensions.x * self.dimensions.y
    }

    pub fn width(&self) -> f64 {
        self.dimensions.x
    }

    pub fn height(&self) -> f64 {
        self.dimensions.y
    }

    pub fn top_left(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x - self.dimensions.x / 2.0,
            self.center.y + self.dimensions.y / 2.0,
        )
    }

    pub fn top_right(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x + self.dimensions.x / 2.0,
            self.center.y + self.dimensions.y / 2.0,
        )
    }

    pub fn bottom_left(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x - self.dimensions.x / 2.0,
            self.center.y - self.dimensions.y / 2.0,
        )
    }

    pub fn bottom_right(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x + self.dimensions.x / 2.0,
            self.center.y - self.dimensions.y / 2.0,
        )
    }

    /// Signed distance functions are passed the coordinates of a point in space and return the
    /// shortest distance between that point and some surface.
    /// The sign of the return value indicates whether the point is inside that surface (negative)
    /// or outside (positive). A return value of zero indicates the point is exactly on the surface.
    pub fn sdf(&self, point: &Vector2D<f64>) -> f64 {
        // translate to center the rectangle at origin
        let translated = *point - self.center;

        let top_right = self.dimensions / 2.0;
        let d = abs_vector(&translated) - top_right;

        length_vector(&max_vector(&d, 0.0)) + min_float(max_float(d.x, d.y), 0.0)
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

fn min_float(v1: f64, v2: f64) -> f64 {
    v1.min(v2)
}

fn max_float(v1: f64, v2: f64) -> f64 {
    v1.max(v2)
}

#[cfg(test)]
mod tests {
    use crate::rectangle::Rectangle;
    use vector2d::Vector2D;

    #[test]
    fn create_rectangle() {
        let rectangle = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));

        assert_eq!(
            format!("The rectangle is: {rectangle:?}"),
            "The rectangle is: Rectangle { center: Vector2D { x: 10.0, y: 10.0 }, dimensions: Vector2D { x: 20.0, y: 20.0 } }"
        );

        assert_eq!(rectangle.center().x, 10.0);
        assert_eq!(rectangle.center().y, 10.0);
        assert_eq!(rectangle.dimensions().x, 20.0);
        assert_eq!(rectangle.dimensions().y, 20.0);

        assert_eq!(rectangle.area(), 400.0);

        assert_eq!(rectangle.width(), 20.0);
        assert_eq!(rectangle.height(), 20.0);

        assert_eq!(rectangle.top_left().x, 0.0);
        assert_eq!(rectangle.top_left().y, 20.0);

        assert_eq!(rectangle.top_right().x, 20.0);
        assert_eq!(rectangle.top_right().y, 20.0);

        assert_eq!(rectangle.bottom_left().x, 0.0);
        assert_eq!(rectangle.bottom_left().y, 0.0);

        assert_eq!(rectangle.bottom_right().x, 20.0);
        assert_eq!(rectangle.bottom_right().y, 0.0);
    }

    #[test]
    fn rectangle_equality() {
        let rectangle1 = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle2 = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle3 = Rectangle::new(Vector2D::new(11.0, 11.0), Vector2D::new(22.0, 22.0));

        assert_eq!(rectangle1 == rectangle2, true);
        assert_eq!(rectangle1 != rectangle3, true);
    }

    #[test]
    fn rectangle_sdf() {
        let rectangle = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));

        assert_eq!(rectangle.sdf(&Vector2D::new(10.0, 10.0)), -10.0); // center
        assert_eq!(rectangle.sdf(&Vector2D::new(10.0, 8.0)), -8.0);
        assert_eq!(rectangle.sdf(&Vector2D::new(5.0, 5.0)), -5.0);

        assert_eq!(rectangle.sdf(&Vector2D::new(10.0, 0.0)), 0.0);
        assert_eq!(rectangle.sdf(&Vector2D::new(0.0, 10.0)), 0.0);
        assert_eq!(rectangle.sdf(&Vector2D::new(0.0, 0.0)), 0.0);

        assert_eq!(rectangle.sdf(&Vector2D::new(10.0, -10.0)), 10.0);
        assert_eq!(
            rectangle.sdf(&Vector2D::new(-2.0, -2.0)),
            2.8284271247461903
        );
    }
}
