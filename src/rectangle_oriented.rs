use crate::{
    abs_vector, length_vector, max_float, max_vector, min_float, rotate_vector_by_degrees,
    Rectangle, Shape,
};
use vector2d::Vector2D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectangleOriented {
    rectangle: Rectangle,
    rotation_angle_in_degrees: f64,
}

impl RectangleOriented {
    pub fn new(rectangle: Rectangle, rotation_angle_in_degrees: f64) -> Self {
        RectangleOriented {
            rectangle,
            rotation_angle_in_degrees,
        }
    }

    pub fn center(&self) -> Vector2D<f64> {
        self.rectangle.center()
    }

    pub fn dimensions(&self) -> Vector2D<f64> {
        self.rectangle.dimensions()
    }

    pub fn width(&self) -> f64 {
        self.rectangle.width()
    }

    pub fn height(&self) -> f64 {
        self.rectangle.height()
    }
}

impl Shape for RectangleOriented {
    fn area(&self) -> f64 {
        self.rectangle.area()
    }

    fn perimeter(&self) -> f64 {
        self.rectangle.perimeter()
    }

    fn sdf(&self, point: &Vector2D<f64>) -> f64 {
        // translate to center the rectangle at origin
        let translated = *point - self.center();

        let rotated = rotate_vector_by_degrees(&translated, self.rotation_angle_in_degrees);

        let top_right = self.dimensions() * 0.5;
        let d = abs_vector(&rotated) - top_right;

        length_vector(&max_vector(&d, 0.0)) + min_float(max_float(d.x, d.y), 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::rectangle_oriented::RectangleOriented;
    use crate::{get_area, Rectangle, Shape};
    use vector2d::Vector2D;

    #[test]
    fn create_rectangle_oriented() {
        let rectangle = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_oriented = RectangleOriented::new(rectangle, 45.0);

        assert_eq!(
            format!("The rectangle_oriented is: {rectangle_oriented:?}"),
            "The rectangle_oriented is: RectangleOriented { rectangle: Rectangle { center: Vector2D { x: 10.0, y: 10.0 }, dimensions: Vector2D { x: 20.0, y: 20.0 } }, rotation_angle_in_degrees: 45.0 }"
        );

        assert_eq!(rectangle_oriented.center().x, 10.0);
        assert_eq!(rectangle_oriented.center().y, 10.0);
        assert_eq!(rectangle_oriented.dimensions().x, 20.0);
        assert_eq!(rectangle_oriented.dimensions().y, 20.0);

        assert_eq!(get_area(&rectangle_oriented), 400.0);
        assert_eq!(rectangle_oriented.area(), 400.0);
        assert_eq!(rectangle_oriented.perimeter(), 80.0);

        assert_eq!(rectangle_oriented.width(), 20.0);
        assert_eq!(rectangle_oriented.height(), 20.0);

        // assert_eq!(rectangle_oriented.top_left().x, 0.0);
        // assert_eq!(rectangle_oriented.top_left().y, 20.0);

        // assert_eq!(rectangle_oriented.top_right().x, 20.0);
        // assert_eq!(rectangle_oriented.top_right().y, 20.0);

        // assert_eq!(rectangle_oriented.bottom_left().x, 0.0);
        // assert_eq!(rectangle_oriented.bottom_left().y, 0.0);

        // assert_eq!(rectangle_oriented.bottom_right().x, 20.0);
        // assert_eq!(rectangle_oriented.bottom_right().y, 0.0);
    }

    #[test]
    fn rectangle_oriented_equality() {
        let rectangle1 = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_oriented1 = RectangleOriented::new(rectangle1, 45.0);
        let rectangle2 = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_oriented2 = RectangleOriented::new(rectangle2, 45.0);
        let rectangle3 = Rectangle::new(Vector2D::new(11.0, 11.0), Vector2D::new(22.0, 22.0));
        let rectangle_oriented3 = RectangleOriented::new(rectangle3, 90.0);

        assert_eq!(rectangle_oriented1 == rectangle_oriented2, true);
        assert_eq!(rectangle_oriented1 != rectangle_oriented3, true);
    }

    #[test]
    fn rectangle_oriented_sdf() {
        let rectangle = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_oriented = RectangleOriented::new(rectangle, 45.0);

        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(10.0, 10.0)), -10.0); // center
        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(10.0, 8.0)), -8.585786437626904);
        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(5.0, 5.0)), -2.9289321881345245);

        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(10.0, 0.0)), -2.9289321881345245);
        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(0.0, 10.0)), -2.9289321881345245);
        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(0.0, 0.0)), 4.142135623730951);

        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(10.0, -10.0)), 5.857864376269049);
        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(-2.0, -2.0)),
            6.970562748477143
        );
    }
}
