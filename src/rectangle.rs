use crate::{abs_vector, length_vector, max_f64, max_vector, min_f64, Shape};
use vector2d::Vector2D;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn width(&self) -> f64 {
        self.dimensions.x
    }

    pub fn height(&self) -> f64 {
        self.dimensions.y
    }

    pub fn top_left(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x - self.dimensions.x * 0.5,
            self.center.y + self.dimensions.y * 0.5,
        )
    }

    pub fn top_right(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x + self.dimensions.x * 0.5,
            self.center.y + self.dimensions.y * 0.5,
        )
    }

    pub fn bottom_left(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x - self.dimensions.x * 0.5,
            self.center.y - self.dimensions.y * 0.5,
        )
    }

    pub fn bottom_right(&self) -> Vector2D<f64> {
        Vector2D::new(
            self.center.x + self.dimensions.x * 0.5,
            self.center.y - self.dimensions.y * 0.5,
        )
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.dimensions.x * self.dimensions.y
    }

    fn perimeter(&self) -> f64 {
        (self.dimensions.x + self.dimensions.y) * 2.0
    }

    fn sdf(&self, point: &Vector2D<f64>) -> f64 {
        // translate to center the rectangle at origin
        let translated = *point - self.center;

        let top_right = self.dimensions * 0.5;
        let d = abs_vector(&translated) - top_right;

        length_vector(&max_vector(&d, 0.0)) + min_f64(max_f64(d.x, d.y), 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::rectangle::Rectangle;
    use crate::{get_area, Shape};
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

        assert_eq!(get_area(&rectangle), 400.0);
        //assert_eq!(rectangle.area(), 400.0);
        assert_eq!(rectangle.perimeter(), 80.0);

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
