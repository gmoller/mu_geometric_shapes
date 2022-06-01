use crate::{abs_vector, length_vector, max_float, max_vector, min_float, Rectangle, Shape};
use std::f64::consts::PI;
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
pub struct RoundFactors {
    top_left: f64,
    top_right: f64,
    bottom_left: f64,
    bottom_right: f64,
}

impl RoundFactors {
    pub fn new(top_left: f64, top_right: f64, bottom_left: f64, bottom_right: f64) -> Self {
        RoundFactors {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RectangleRounded {
    rectangle: Rectangle,
    round_factors: RoundFactors,
}

impl RectangleRounded {
    pub fn new(rectangle: Rectangle, round_factors: RoundFactors) -> Self {
        RectangleRounded {
            rectangle,
            round_factors,
        }
    }

    pub fn center(&self) -> Vector2D<f64> {
        self.rectangle.center()
    }

    pub fn dimensions(&self) -> Vector2D<f64> {
        self.rectangle.dimensions()
    }

    pub fn round_factors(&self) -> RoundFactors {
        RoundFactors {
            top_left: self.round_factors.top_left,
            top_right: self.round_factors.top_right,
            bottom_left: self.round_factors.bottom_left,
            bottom_right: self.round_factors.bottom_right,
        }
    }

    pub fn width(&self) -> f64 {
        self.rectangle.width()
    }

    pub fn height(&self) -> f64 {
        self.rectangle.height()
    }
}

impl Shape for RectangleRounded {
    fn area(&self) -> f64 {
        let r_squared1 = self.round_factors.top_left.powf(2.0);
        let r_squared2 = self.round_factors.top_right.powf(2.0);
        let r_squared3 = self.round_factors.bottom_left.powf(2.0);
        let r_squared4 = self.round_factors.bottom_right.powf(2.0);
        let r_squared = (r_squared1 + r_squared2 + r_squared3 + r_squared4) / 4.0;

        self.rectangle.area() - 4.0 * r_squared + PI * r_squared
    }

    fn perimeter(&self) -> f64 {
        let r1 = self.round_factors.top_left;
        let r2 = self.round_factors.top_right;
        let r3 = self.round_factors.bottom_left;
        let r4 = self.round_factors.bottom_right;
        let r = (r1 + r2 + r3 + r4) / 4.0;

        self.rectangle.perimeter() - 8.0 * r + 2.0 * PI * r
    }

    fn sdf(&self, point: &Vector2D<f64>) -> f64 {
        // translate to center the rectangle at origin
        let translated = *point - self.center();

        let mut r = 0.0;
        if translated.x >= 0.0 && translated.y >= 0.0 {
            r = self.round_factors.top_right
        } else if translated.x >= 0.0 && translated.y < 0.0 {
            r = self.round_factors.bottom_right
        } else if translated.x < 0.0 && translated.y >= 0.0 {
            r = self.round_factors.top_left
        } else if translated.x < 0.0 && translated.y < 0.0 {
            r = self.round_factors.bottom_left
        }

        let top_right = self.dimensions() * 0.5;
        let q = abs_vector(&translated) - top_right + Vector2D::new(r, r);

        length_vector(&max_vector(&q, 0.0)) + min_float(max_float(q.x, q.y), 0.0) - r
    }
}

#[cfg(test)]
mod tests {
    use crate::rectangle::Rectangle;
    use crate::rectangle_rounded::{RectangleRounded, RoundFactors};
    use crate::{get_area, Shape};
    use vector2d::Vector2D;

    #[test]
    fn create_rectangle_rounded() {
        let rectangle = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_rounded =
            RectangleRounded::new(rectangle, RoundFactors::new(2.0, 2.0, 2.0, 2.0));

        assert_eq!(
            format!("The rectangle_rounded is: {rectangle_rounded:?}"),
            "The rectangle_rounded is: RectangleRounded { rectangle: Rectangle { center: Vector2D { x: 10.0, y: 10.0 }, dimensions: Vector2D { x: 20.0, y: 20.0 } }, round_factors: RoundFactors { top_left: 2.0, top_right: 2.0, bottom_left: 2.0, bottom_right: 2.0 } }"
        );

        assert_eq!(rectangle_rounded.center().x, 10.0);
        assert_eq!(rectangle_rounded.center().y, 10.0);
        assert_eq!(rectangle_rounded.dimensions().x, 20.0);
        assert_eq!(rectangle_rounded.dimensions().y, 20.0);

        assert_eq!(get_area(&rectangle_rounded), 396.5663706143592);
        //assert_eq!(rectangle_rounded.area(), 396.5663706143592);
        assert_eq!(rectangle_rounded.perimeter(), 76.56637061435917);

        assert_eq!(rectangle_rounded.width(), 20.0);
        assert_eq!(rectangle_rounded.height(), 20.0);
    }

    #[test]
    fn rectangle_rounded_equality() {
        let rectangle1 = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_rounded1 =
            RectangleRounded::new(rectangle1, RoundFactors::new(2.0, 2.0, 2.0, 2.0));
        let rectangle2 = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_rounded2 =
            RectangleRounded::new(rectangle2, RoundFactors::new(2.0, 2.0, 2.0, 2.0));
        let rectangle3 = Rectangle::new(Vector2D::new(11.0, 11.0), Vector2D::new(22.0, 22.0));
        let rectangle_rounded3 =
            RectangleRounded::new(rectangle3, RoundFactors::new(2.0, 2.0, 2.0, 2.0));

        assert_eq!(rectangle_rounded1 == rectangle_rounded2, true);
        assert_eq!(rectangle_rounded1 != rectangle_rounded3, true);
    }

    #[test]
    fn rectangle_rounded_sdf() {
        let rectangle = Rectangle::new(Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
        let rectangle_rounded =
            RectangleRounded::new(rectangle, RoundFactors::new(2.0, 2.0, 2.0, 2.0));

        assert_eq!(rectangle_rounded.sdf(&Vector2D::new(10.0, 10.0)), -10.0); // center
        assert_eq!(rectangle_rounded.sdf(&Vector2D::new(10.0, 8.0)), -8.0);
        assert_eq!(rectangle_rounded.sdf(&Vector2D::new(5.0, 5.0)), -5.0); // ???

        assert_eq!(rectangle_rounded.sdf(&Vector2D::new(10.0, 0.0)), 0.0);
        assert_eq!(rectangle_rounded.sdf(&Vector2D::new(0.0, 10.0)), 0.0);
        assert_eq!(
            rectangle_rounded.sdf(&Vector2D::new(0.0, 0.0)),
            0.8284271247461903
        );

        assert_eq!(rectangle_rounded.sdf(&Vector2D::new(10.0, -10.0)), 10.0);
        assert_eq!(
            rectangle_rounded.sdf(&Vector2D::new(-2.0, -2.0)),
            3.6568542494923806
        );
    }
}
