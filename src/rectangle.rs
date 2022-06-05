use crate::{
    abs_vector, length_vector, max_f64, max_vector, min_f64, rotate_vector_by_degrees, Shape,
};
use std::f64::consts::PI;
use vector2d::Vector2D;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    center: Vector2D<f64>,
    dimensions: Vector2D<f64>,
    rotation_angle_in_degrees: f64,
    round_factors: RoundFactors,
}

impl Rectangle {
    pub fn new(
        center: Vector2D<f64>,
        dimensions: Vector2D<f64>,
        rotation_angle_in_degrees: f64,
        round_factors: RoundFactors,
    ) -> Self {
        Rectangle {
            center,
            dimensions,
            rotation_angle_in_degrees,
            round_factors,
        }
    }

    pub fn center(&self) -> Vector2D<f64> {
        self.center
    }

    pub fn dimensions(&self) -> Vector2D<f64> {
        self.dimensions
    }

    pub fn rotation_angle_in_degrees(&self) -> f64 {
        self.rotation_angle_in_degrees
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
        self.dimensions.x
    }

    pub fn height(&self) -> f64 {
        self.dimensions.y
    }

    pub fn top_left(&self) -> Vector2D<f64> {
        //todo
        let v = Vector2D::new(
            self.center.x - self.dimensions.x * 0.5,
            self.center.y + self.dimensions.y * 0.5,
        );

        self.rotate_point(v)
    }

    pub fn top_right(&self) -> Vector2D<f64> {
        //todo
        let v = Vector2D::new(
            self.center.x + self.dimensions.x * 0.5,
            self.center.y + self.dimensions.y * 0.5,
        );

        self.rotate_point(v)
    }

    pub fn bottom_left(&self) -> Vector2D<f64> {
        //todo
        let v = Vector2D::new(
            self.center.x - self.dimensions.x * 0.5,
            self.center.y - self.dimensions.y * 0.5,
        );

        self.rotate_point(v)
    }

    pub fn bottom_right(&self) -> Vector2D<f64> {
        //todo
        let v = Vector2D::new(
            self.center.x + self.dimensions.x * 0.5,
            self.center.y - self.dimensions.y * 0.5,
        );

        self.rotate_point(v)
    }

    fn rotate_point(&self, v: Vector2D<f64>) -> Vector2D<f64> {
        if self.rotation_angle_in_degrees == 0.0 {
            v
        } else {
            let v = v - self.center;
            rotate_vector_by_degrees(&v, self.rotation_angle_in_degrees);

            v + self.center
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        let r_squared1 = self.round_factors.top_left.powf(2.0);
        let r_squared2 = self.round_factors.top_right.powf(2.0);
        let r_squared3 = self.round_factors.bottom_left.powf(2.0);
        let r_squared4 = self.round_factors.bottom_right.powf(2.0);
        let r_squared = (r_squared1 + r_squared2 + r_squared3 + r_squared4) / 4.0;

        self.dimensions.x * self.dimensions.y - 4.0 * r_squared + PI * r_squared
    }

    fn perimeter(&self) -> f64 {
        let r1 = self.round_factors.top_left;
        let r2 = self.round_factors.top_right;
        let r3 = self.round_factors.bottom_left;
        let r4 = self.round_factors.bottom_right;
        let r = (r1 + r2 + r3 + r4) / 4.0;

        ((self.dimensions.x + self.dimensions.y) * 2.0) - 8.0 * r + 2.0 * PI * r
    }

    fn sdf(&self, point: &Vector2D<f64>) -> f64 {
        // translate to center the rectangle at origin
        let mut translated = *point - self.center;

        if self.rotation_angle_in_degrees != 0.0 {
            translated = rotate_vector_by_degrees(&translated, self.rotation_angle_in_degrees);
        }

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
        let d = abs_vector(&translated) - top_right + Vector2D::new(r, r);

        length_vector(&max_vector(&d, 0.0)) + min_f64(max_f64(d.x, d.y), 0.0) - r
    }
}

#[cfg(test)]
mod tests {
    use crate::rectangle::Rectangle;
    use crate::{get_area, get_sdf_grid, RoundFactors, Shape};
    use vector2d::Vector2D;

    #[test]
    fn create_rectangle() {
        let rectangle = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            0.0,
            Default::default(),
        );

        assert_eq!(
            format!("The rectangle is: {rectangle:?}"),
            "The rectangle is: Rectangle { center: Vector2D { x: 10.0, y: 10.0 }, dimensions: Vector2D { x: 20.0, y: 20.0 }, rotation_angle_in_degrees: 0.0, round_factors: RoundFactors { top_left: 0.0, top_right: 0.0, bottom_left: 0.0, bottom_right: 0.0 } }"
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
    fn create_rectangle_oriented() {
        let rectangle_oriented = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            45.0,
            Default::default(),
        );

        assert_eq!(
            format!("The rectangle_oriented is: {rectangle_oriented:?}"),
            "The rectangle_oriented is: Rectangle { center: Vector2D { x: 10.0, y: 10.0 }, dimensions: Vector2D { x: 20.0, y: 20.0 }, rotation_angle_in_degrees: 45.0, round_factors: RoundFactors { top_left: 0.0, top_right: 0.0, bottom_left: 0.0, bottom_right: 0.0 } }"
        );

        assert_eq!(rectangle_oriented.center().x, 10.0);
        assert_eq!(rectangle_oriented.center().y, 10.0);
        assert_eq!(rectangle_oriented.dimensions().x, 20.0);
        assert_eq!(rectangle_oriented.dimensions().y, 20.0);

        assert_eq!(get_area(&rectangle_oriented), 400.0);
        //assert_eq!(rectangle_oriented.area(), 400.0);
        assert_eq!(rectangle_oriented.perimeter(), 80.0);

        assert_eq!(rectangle_oriented.width(), 20.0);
        assert_eq!(rectangle_oriented.height(), 20.0);

        assert_eq!(rectangle_oriented.top_left().x, 0.0);
        assert_eq!(rectangle_oriented.top_left().y, 20.0);

        assert_eq!(rectangle_oriented.top_right().x, 20.0);
        assert_eq!(rectangle_oriented.top_right().y, 20.0);

        assert_eq!(rectangle_oriented.bottom_left().x, 0.0);
        assert_eq!(rectangle_oriented.bottom_left().y, 0.0);

        assert_eq!(rectangle_oriented.bottom_right().x, 20.0);
        assert_eq!(rectangle_oriented.bottom_right().y, 0.0);
    }

    #[test]
    fn create_rectangle_rounded() {
        let rectangle_rounded = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            0.0,
            RoundFactors::new(2.0, 2.0, 2.0, 2.0),
        );

        assert_eq!(
            format!("The rectangle_rounded is: {rectangle_rounded:?}"),
            "The rectangle_rounded is: Rectangle { center: Vector2D { x: 10.0, y: 10.0 }, dimensions: Vector2D { x: 20.0, y: 20.0 }, rotation_angle_in_degrees: 0.0, round_factors: RoundFactors { top_left: 2.0, top_right: 2.0, bottom_left: 2.0, bottom_right: 2.0 } }"
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
    fn rectangle_equality() {
        let rectangle1 = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            0.0,
            Default::default(),
        );
        let rectangle2 = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            0.0,
            Default::default(),
        );
        let rectangle3 = Rectangle::new(
            Vector2D::new(11.0, 11.0),
            Vector2D::new(22.0, 22.0),
            0.0,
            Default::default(),
        );

        assert_eq!(rectangle1 == rectangle2, true);
        assert_eq!(rectangle1 != rectangle3, true);
    }

    #[test]
    fn rectangle_sdf() {
        let rectangle = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            0.0,
            Default::default(),
        );

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

    #[test]
    fn rectangle_oriented_sdf() {
        let rectangle_oriented = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            45.0,
            Default::default(),
        );

        assert_eq!(rectangle_oriented.sdf(&Vector2D::new(10.0, 10.0)), -10.0); // center
        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(10.0, 8.0)),
            -8.585786437626904
        );
        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(5.0, 5.0)),
            -2.9289321881345245
        );

        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(10.0, 0.0)),
            -2.9289321881345245
        );
        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(0.0, 10.0)),
            -2.9289321881345245
        );
        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(0.0, 0.0)),
            4.142135623730951
        );

        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(10.0, -10.0)),
            5.857864376269049
        );
        assert_eq!(
            rectangle_oriented.sdf(&Vector2D::new(-2.0, -2.0)),
            6.970562748477143
        );
    }

    #[test]
    fn rectangle_rounded_sdf() {
        let rectangle_rounded = Rectangle::new(
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
            0.0,
            RoundFactors::new(2.0, 2.0, 2.0, 2.0),
        );

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

    #[test]
    fn rectangle_sdf_grid() {
        let rectangle = Rectangle::new(
            Vector2D::new(1.0, 1.0),
            Vector2D::new(1.0, 1.0),
            0.0,
            Default::default(),
        );

        let sdf_grid = get_sdf_grid(&rectangle, 3, 3);

        assert_eq!(sdf_grid.get_value(0, 0), 0.7071067811865476);
        assert_eq!(sdf_grid.get_value(1, 0), 0.5);
        assert_eq!(sdf_grid.get_value(2, 0), 0.7071067811865476);

        assert_eq!(sdf_grid.get_value(0, 1), 0.5);
        assert_eq!(sdf_grid.get_value(1, 1), -0.5);
        assert_eq!(sdf_grid.get_value(2, 1), 0.5);

        assert_eq!(sdf_grid.get_value(0, 2), 0.7071067811865476);
        assert_eq!(sdf_grid.get_value(1, 2), 0.5);
        assert_eq!(sdf_grid.get_value(2, 2), 0.7071067811865476);
    }
}
