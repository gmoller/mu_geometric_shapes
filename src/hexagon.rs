use crate::{
    abs_vector, dot_product, max_f64, rotate_vector_by_30_degrees, HexagonOrientation, Shape,
};
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
pub struct Hexagon {
    center: Vector2D<f64>,
    circumradius: f64, // (size)
    orientation: HexagonOrientation,
}

impl Hexagon {
    pub fn new(center: Vector2D<f64>, circumradius: f64, orientation: HexagonOrientation) -> Self {
        Hexagon {
            center,
            circumradius,
            orientation,
        }
    }

    pub fn center(&self) -> Vector2D<f64> {
        self.center
    }

    pub fn circumradius(&self) -> f64 {
        // R (size)
        self.circumradius
    }

    pub fn inradius(&self) -> f64 {
        // r
        (1.7320508 / 2.0) * self.circumradius()
    }

    pub fn maximal_diameter(&self) -> f64 {
        // D (height)
        self.circumradius() * 2.0
    }

    pub fn minimal_diameter(&self) -> f64 {
        // d (width)
        self.inradius() * 2.0
    }

    pub fn apothem(&self) -> f64 {
        // a
        self.inradius()
    }

    pub fn side_length(&self) -> f64 {
        // t
        self.circumradius()
    }
}

impl Shape for Hexagon {
    fn area(&self) -> f64 {
        2.0 * self.inradius().powf(2.0) * 1.7320508
    }

    fn perimeter(&self) -> f64 {
        6.0 * self.circumradius()
    }

    fn sdf(&self, point: &Vector2D<f64>) -> f64 {
        // translate to center the circle at origin
        let translated = *point - self.center;

        let translated = match self.orientation {
            HexagonOrientation::Horizontal => translated,
            HexagonOrientation::Vertical => rotate_vector_by_30_degrees(&translated),
        };

        let s = Vector2D::new(1.0, 1.7320508) * 0.5;
        let p = abs_vector(&translated);

        max_f64(dot_product(&p, &s), p.x) - self.inradius()
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_area, get_sdf, Hexagon, HexagonOrientation, Shape};
    use vector2d::Vector2D;

    #[test]
    fn create_hexagon() {
        let hexagon = Hexagon::new(
            Vector2D::new(10.0, 10.0),
            10.0,
            HexagonOrientation::Vertical,
        );

        assert_eq!(
            format!("The hexagon is: {hexagon:?}"),
            "The hexagon is: Hexagon { center: Vector2D { x: 10.0, y: 10.0 }, circumradius: 10.0, orientation: Vertical }"
        );

        assert_eq!(hexagon.center().x, 10.0);
        assert_eq!(hexagon.center().y, 10.0);
        assert_eq!(hexagon.circumradius(), 10.0);
        assert_eq!(hexagon.inradius(), 8.660254);
        assert_eq!(hexagon.maximal_diameter(), 20.0);
        assert_eq!(hexagon.minimal_diameter(), 17.320508);
        assert_eq!(hexagon.apothem(), 8.660254);
        assert_eq!(hexagon.side_length(), 10.0);

        assert_eq!(get_area(&hexagon), 259.8076177293368);
        //assert_eq!(hexagon.area(), 259.8076177293368);
        assert_eq!(hexagon.perimeter(), 60.0);
    }

    #[test]
    fn hexagon_equality() {
        let hexagon1 = Hexagon::new(
            Vector2D::new(10.0, 10.0),
            10.0,
            HexagonOrientation::Vertical,
        );
        let hexagon2 = Hexagon::new(
            Vector2D::new(10.0, 10.0),
            10.0,
            HexagonOrientation::Vertical,
        );
        let hexagon3 = Hexagon::new(
            Vector2D::new(11.0, 11.0),
            11.0,
            HexagonOrientation::Vertical,
        );

        assert_eq!(hexagon1 == hexagon2, true);
        assert_eq!(hexagon1 != hexagon3, true);
    }

    #[test]
    fn hexagon_sdf_horizontal() {
        let hexagon = Hexagon::new(
            Vector2D::new(10.0, 10.0),
            10.0,
            HexagonOrientation::Horizontal,
        );

        assert_eq!(get_sdf(&hexagon, &Vector2D::new(10.0, 10.0)), -8.660254); // center
        assert_eq!(hexagon.sdf(&Vector2D::new(5.0, 5.0)), -1.830127);

        assert_eq!(hexagon.sdf(&Vector2D::new(10.0, 0.0)), 0.0);
        assert_eq!(
            hexagon.sdf(&Vector2D::new(1.339, 10.0)),
            0.0007459999999994693
        );

        assert_eq!(hexagon.sdf(&Vector2D::new(0.0, 0.0)), 5.0);
        assert_eq!(
            hexagon.sdf(&Vector2D::new(-10.0, -10.0)),
            18.660254000000002
        );
    }

    #[test]
    fn hexagon_sdf_vertical() {
        let hexagon = Hexagon::new(
            Vector2D::new(10.0, 10.0),
            10.0,
            HexagonOrientation::Vertical,
        );

        assert_eq!(get_sdf(&hexagon, &Vector2D::new(10.0, 10.0)), -8.660254); // center
        assert_eq!(hexagon.sdf(&Vector2D::new(5.0, 5.0)), -1.830127006956321);

        assert_eq!(hexagon.sdf(&Vector2D::new(10.0, 0.0)), 1.3397459671873602);
        assert_eq!(
            hexagon.sdf(&Vector2D::new(0.0, 10.0)),
            3.7799999574872345e-8
        );

        assert_eq!(hexagon.sdf(&Vector2D::new(0.0, 0.0)), 4.999999986087358);
        assert_eq!(
            hexagon.sdf(&Vector2D::new(-10.0, -10.0)),
            18.660253972174715
        );
    }
}
