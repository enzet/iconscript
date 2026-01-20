use kurbo::Point;

// Copy the `are_collinear`` function for testing.
fn are_collinear(p1: Point, p2: Point, p3: Point, epsilon: f64) -> bool {
    let v1x = p2.x - p1.x;
    let v1y = p2.y - p1.y;
    let v2x = p3.x - p1.x;
    let v2y = p3.y - p1.y;

    let cross_product = (v1x * v2y - v1y * v2x).abs();

    let dot1 = v1x * v2x + v1y * v2y;
    let dot2 = (p3.x - p2.x) * v2x + (p3.y - p2.y) * v2y;

    println!("Testing collinearity:");
    println!("  p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3);
    println!("  v1: ({}, {}), v2: ({}, {})", v1x, v1y, v2x, v2y);
    println!("  cross_product: {} (epsilon: {})", cross_product, epsilon);
    println!("  dot1: {}, dot2: {}", dot1, dot2);
    println!(
        "  Result: cross < eps: {}, dot1 > 0: {}, dot2 > 0: {}",
        cross_product < epsilon,
        dot1 > 0.0,
        dot2 > 0.0
    );

    cross_product < epsilon && dot1 > 0.0 && dot2 > 0.0
}

#[test]
fn test_are_collinear() {
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::new(2.0, 2.0);
    let p3 = Point::new(3.0, 3.0);

    let result = are_collinear(p1, p2, p3, 0.5);
    println!("\nFinal result: {}", result);
    assert!(result, "Points should be collinear.");
}
