use kurbo::Point;

fn are_collinear(p1: Point, p2: Point, p3: Point, epsilon: f64) -> bool {
    let v1x = p2.x - p1.x;
    let v1y = p2.y - p1.y;
    let v2x = p3.x - p1.x;
    let v2y = p3.y - p1.y;

    let cross_product = (v1x * v2y - v1y * v2x).abs();
    let dot1 = v1x * v2x + v1y * v2y;
    let dot2 = (p3.x - p2.x) * v2x + (p3.y - p2.y) * v2y;

    cross_product < epsilon && dot1 > 0.0 && dot2 > 0.0
}

fn simplify_collinear_points(points: Vec<Point>, epsilon: f64) -> Vec<Point> {
    println!(
        "\nSimplifying {} points with epsilon {}",
        points.len(),
        epsilon
    );
    for (i, p) in points.iter().enumerate() {
        println!("  [{}]: {:?}", i, p);
    }

    if points.len() < 3 {
        println!("Not enough points, returning as-is.");
        return points;
    }

    let mut result = Vec::new();
    result.push(points[0]);

    let mut i = 0;
    while i < points.len() - 1 {
        let start = points[i];
        let mut end_idx = i + 1;

        println!("\nChecking from index {}", i);

        while end_idx < points.len() - 1 {
            let is_col = are_collinear(
                start,
                points[end_idx],
                points[end_idx + 1],
                epsilon,
            );
            println!(
                "  are_collinear({:?}, {:?}, {:?}) = {}",
                start,
                points[end_idx],
                points[end_idx + 1],
                is_col
            );

            if is_col {
                end_idx += 1;
            } else {
                break;
            }
        }

        if end_idx > i + 1 {
            println!(
                "  Found collinear sequence from {} to {}, keeping only {}.",
                i, end_idx, end_idx
            );
            i = end_idx;
            result.push(points[i]);
        } else {
            println!("  No collinear sequence, adding point at {}", i + 1);
            result.push(points[i + 1]);
            i += 1;
        }
    }

    println!("\nResult: {} points", result.len());
    for (i, p) in result.iter().enumerate() {
        println!("  [{}]: {:?}", i, p);
    }

    result
}

#[test]
fn test_simplify() {
    let points = vec![
        Point::new(1.0, 1.0),
        Point::new(2.0, 2.0),
        Point::new(3.0, 3.0),
    ];

    let simplified = simplify_collinear_points(points, 0.5);
    assert_eq!(simplified.len(), 2, "Should simplify to 2 points");
    assert_eq!(simplified[0], Point::new(1.0, 1.0));
    assert_eq!(simplified[1], Point::new(3.0, 3.0));
}
