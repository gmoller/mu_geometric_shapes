# mu_geometric_shapes
- Circle
- Rectangle (regular, rounded, oriented)
- Hexagon

Usage Example:
```
let shape = ShapeFactory::new_circle(
    Vector2D::new(canvas.width as f64 / 2.0, canvas.height as f64 / 2.0),
    radius,
);

let area = shape.area();
let sdf = shape.sdf(&p);
```
