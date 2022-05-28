pub mod circle;

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

        assert_eq!(circle.area(), 314.1592653589793);

        assert_eq!(circle.center().x, 10.0);
        assert_eq!(circle.center().y, 10.0);

        assert_eq!(circle.circumference(), 62.83185307179586);

        assert_eq!(circle.diameter(), 20.0);

        assert_eq!(circle.radius(), 10.0);
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
        let circle = Circle::new(Vector2D::new(10.0, 0.0), 10.0);

        assert_eq!(circle.sdf(Vector2D::new(0.0, 0.0)), 0.0);
        assert_eq!(circle.sdf(Vector2D::new(10.0, 0.0)), -10.0);
        assert_eq!(circle.sdf(Vector2D::new(0.0, 10.0)), 4.142135623730951);
        assert_eq!(circle.sdf(Vector2D::new(-10.0, 20.0)), 18.284271247461902);
    }

    // #[test]
    // fn circle_sdf() {
    //     let circle = Circle::new(Vector2D::new(0.0, 0.0), 10.0);
    //
    //     assert_eq!(circle.sdf(Vector2D::new(0.0, 0.0)), -10.0);
    //     assert_eq!(circle.sdf(Vector2D::new(10.0, 0.0)), 0.0);
    //     assert_eq!(circle.sdf(Vector2D::new(-20.0, 20.0)), 18.284271247461902);
    // }
}
