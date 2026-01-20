use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::listener::parse_iconscript;
use crate::types::{Icon, PathWithMode};

/// Options for controlling path optimization.
#[derive(Debug, Clone)]
pub struct OptimizationOptions {
    pub enable_rounding: bool,
    pub enable_deduplication: bool,
    pub enable_collinear_simplification: bool,
}

impl Default for OptimizationOptions {
    fn default() -> Self {
        Self {
            enable_rounding: true,
            enable_deduplication: true,
            enable_collinear_simplification: true,
        }
    }
}

/// Create a circle path using SVG arc commands.
pub fn create_circle_path(cx: f64, cy: f64, r: f64) -> Option<String> {
    if !cx.is_finite() || !cy.is_finite() || !r.is_finite() {
        eprintln!("Invalid circle coordinates: ({}, {}, {})", cx, cy, r);
        return None;
    }

    Some(format!(
        "M {} {} A {} {} 0 0 1 {} {} A {} {} 0 0 1 {} {} Z",
        cx - r,
        cy,
        r,
        r,
        cx + r,
        cy,
        r,
        r,
        cx - r,
        cy
    ))
}

/// Create a thick line path as a rectangle.
pub fn create_thick_line_path(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    thickness: f64,
) -> Option<String> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = (dx * dx + dy * dy).sqrt();

    if length == 0.0 {
        return None;
    }

    // Normalize the direction vector.
    let nx = dx / length;
    let ny = dy / length;

    // Perpendicular vector.
    let px = -ny;
    let py = nx;

    // Create a rectangle path.
    let half_thickness = thickness / 2.0;
    let x1a = x1 + px * half_thickness;
    let y1a = y1 + py * half_thickness;
    let x1b = x1 - px * half_thickness;
    let y1b = y1 - py * half_thickness;
    let x2a = x2 + px * half_thickness;
    let y2a = y2 + py * half_thickness;
    let x2b = x2 - px * half_thickness;
    let y2b = y2 - py * half_thickness;

    Some(format!(
        "M {} {} L {} {} L {} {} L {} {} Z",
        x1a, y1a, x2a, y2a, x2b, y2b, x1b, y1b
    ))
}

/// Check if three points are approximately collinear.
fn are_collinear(
    p1: kurbo::Point,
    p2: kurbo::Point,
    p3: kurbo::Point,
    epsilon: f64,
) -> bool {
    // Use cross product to check collinearity. If the cross product of vectors
    // `(p2-p1)` and `(p3-p1)` is near zero, points are collinear.
    let v1x = p2.x - p1.x;
    let v1y = p2.y - p1.y;
    let v2x = p3.x - p1.x;
    let v2y = p3.y - p1.y;

    let cross_product = (v1x * v2y - v1y * v2x).abs();

    // Also check that `p2` is between `p1` and `p3` (not beyond them).
    let dot1 = v1x * v2x + v1y * v2y;
    let dot2 = (p3.x - p2.x) * v2x + (p3.y - p2.y) * v2y;

    cross_product < epsilon && dot1 > 0.0 && dot2 > 0.0
}

/// Remove collinear points from a sequence, keeping only first and last.
fn simplify_collinear_points(
    points: Vec<kurbo::Point>,
    epsilon: f64,
) -> Vec<kurbo::Point> {
    if points.len() < 3 {
        return points;
    }

    let mut result = Vec::new();
    result.push(points[0]);

    let mut i = 0;
    while i < points.len() - 1 {
        let start = points[i];
        let mut end_idx = i + 1;

        // Find the longest sequence of collinear points starting from `i`.
        while end_idx < points.len() - 1 {
            // Check if `points[i]`, `points[end_idx]`, and `points[end_idx+1]`
            // are collinear.
            if are_collinear(
                start,
                points[end_idx],
                points[end_idx + 1],
                epsilon,
            ) {
                end_idx += 1;
            } else {
                break;
            }
        }

        // If we found a sequence of 3+ collinear points, skip the middle ones.
        if end_idx > i + 1 {
            // Skip all middle points, jump to the end of the collinear
            // sequence.
            i = end_idx;
            result.push(points[i]);
        } else {
            // Not collinear, just add the next point.
            result.push(points[i + 1]);
            i += 1;
        }
    }

    result
}

/// Remove duplicate consecutive path elements that have the same position.
pub fn deduplicate_path_elements(path: &kurbo::BezPath) -> kurbo::BezPath {
    use kurbo::{BezPath, PathEl, Point};

    let mut result = BezPath::new();
    let mut last_point: Option<Point> = None;
    let mut subpath_start: Option<Point> = None;

    // Use epsilon of 0.01 to merge points within 0.01 pixels. This catches
    // floating point precision issues from boolean operations.
    const EPSILON: f64 = 0.01;

    // Use stricter epsilon for detecting truly degenerate curves. We only want
    // to convert curves to lines when control points are VERY close to
    // endpoint.  0.0015 catches differences up to ~0.001 while preserving
    // legitimate small curves (0.003+).
    const DEGENERATE_EPSILON: f64 = 0.0015;

    fn points_equal(p1: Point, p2: Point) -> bool {
        (p1.x - p2.x).abs() < EPSILON && (p1.y - p2.y).abs() < EPSILON
    }

    fn points_very_close(p1: Point, p2: Point) -> bool {
        (p1.x - p2.x).abs() < DEGENERATE_EPSILON
            && (p1.y - p2.y).abs() < DEGENERATE_EPSILON
    }

    for el in path.iter() {
        match el {
            PathEl::MoveTo(p) => {
                result.push(PathEl::MoveTo(p));
                last_point = Some(p);
                subpath_start = Some(p);
            }
            PathEl::LineTo(p) => {
                // Skip if this point is essentially the same as the last point.
                if last_point.map_or(true, |last| !points_equal(last, p)) {
                    // Also skip if this line returns to the subpath start (will
                    // be handled by `ClosePath`).
                    if subpath_start
                        .map_or(false, |start| points_equal(p, start))
                    {
                        // Line returns to start, don't add it - `ClosePath`
                        // will handle it.
                        continue;
                    }
                    result.push(PathEl::LineTo(p));
                    last_point = Some(p);
                }
            }
            PathEl::QuadTo(p1, p2) => {
                let start = last_point.unwrap_or(p1);

                // Skip degenerate curve where start and end are nearly
                // identical.
                if points_equal(start, p2) {
                    // Curve goes nowhere, skip it entirely.
                    continue;
                }

                // Only skip if this curve returns to the subpath start AND the
                // control point is also very close to start/end (meaning it's
                // essentially a straight line back).
                if subpath_start.map_or(false, |substart| {
                    points_equal(p2, substart)
                        && (points_equal(p1, start) || points_equal(p1, p2))
                }) {
                    continue;
                }

                // Skip if the end point is the same as last point.
                if last_point.map_or(true, |last| !points_equal(last, p2)) {
                    // Check if control point is VERY close to the endpoint
                    // (degenerate curve). Use strict epsilon to avoid
                    // converting small curve segments to lines.
                    if points_very_close(p1, p2) {
                        // Control point is at the endpoint, convert to
                        // `LineTo`.
                        result.push(PathEl::LineTo(p2));
                        last_point = Some(p2);
                    } else {
                        // Simplify control point if it's very close to start.
                        let simplified_p1 =
                            if points_equal(p1, start) { start } else { p1 };

                        result.push(PathEl::QuadTo(simplified_p1, p2));
                        last_point = Some(p2);
                    }
                }
            }
            PathEl::CurveTo(p1, p2, p3) => {
                let start = last_point.unwrap_or(p1);

                // Skip degenerate curve where start and end are nearly
                // identical.
                if points_equal(start, p3) {
                    // Curve goes nowhere, skip it entirely.
                    continue;
                }

                // Only skip if this curve returns to the subpath start AND both
                // control points are very close to start/end (meaning it's
                // essentially a straight line back).
                if subpath_start.map_or(false, |substart| {
                    points_equal(p3, substart)
                        && points_equal(p1, start)
                        && points_equal(p2, p3)
                }) {
                    continue;
                }

                // Skip if the end point is the same as last point.
                if last_point.map_or(true, |last| !points_equal(last, p3)) {
                    // Check if both control points are VERY close to the
                    // endpoint (degenerate curve).  Use strict epsilon to avoid
                    // converting small curve segments to lines.
                    let p1_near_end = points_very_close(p1, p3);
                    let p2_near_end = points_very_close(p2, p3);

                    if p1_near_end && p2_near_end {
                        // Both control points are at the endpoint, convert to
                        // LineTo.
                        result.push(PathEl::LineTo(p3));
                        last_point = Some(p3);
                    } else {
                        // Simplify control points if they're very close to
                        // start or end.
                        let simplified_p1 =
                            if points_equal(p1, start) { start } else { p1 };
                        let simplified_p2 =
                            if points_equal(p2, p3) { p3 } else { p2 };

                        result.push(PathEl::CurveTo(
                            simplified_p1,
                            simplified_p2,
                            p3,
                        ));
                        last_point = Some(p3);
                    }
                }
            }
            PathEl::ClosePath => {
                result.push(PathEl::ClosePath);
                last_point = subpath_start;
                subpath_start = None;
            }
        }
    }

    result
}

/// Simplify path by removing collinear points in sequences of `LineTo`
/// commands.  Also replaces curves with `LineTo` if the curve endpoint is
/// collinear with preceding lines.
pub fn simplify_path_collinear(path: &kurbo::BezPath) -> kurbo::BezPath {
    use kurbo::{BezPath, PathEl, Point};

    const EPSILON: f64 = 0.1; // Tolerance for collinearity detection.

    let mut result = BezPath::new();
    let mut current_line_sequence: Vec<Point> = Vec::new();
    let mut current_position: Option<Point> = None;
    let mut line_sequence_start: Option<Point> = None;

    // Helper to flush accumulated line sequence.
    let flush_line_sequence =
        |result: &mut BezPath, sequence: &mut Vec<Point>, start: Point| {
            if sequence.is_empty() {
                return;
            }

            // Build full sequence: start point + all `LineTo` endpoints.
            let mut full_sequence = vec![start];
            full_sequence.extend(sequence.iter());

            // Simplify collinear points.
            let simplified = simplify_collinear_points(full_sequence, EPSILON);

            // Add all simplified points except the first (it's already in the
            // path).
            for point in simplified.iter().skip(1) {
                result.push(PathEl::LineTo(*point));
            }

            sequence.clear();
        };

    for el in path.iter() {
        match el {
            PathEl::MoveTo(p) => {
                // Flush any accumulated line sequence.
                if let Some(start) = line_sequence_start {
                    flush_line_sequence(
                        &mut result,
                        &mut current_line_sequence,
                        start,
                    );
                    line_sequence_start = None;
                }

                result.push(PathEl::MoveTo(p));
                current_position = Some(p);
            }
            PathEl::LineTo(p) => {
                // Record the start of the line sequence if this is the first
                // `LineTo`.
                if line_sequence_start.is_none() {
                    line_sequence_start = current_position;
                }

                // Add to current line sequence.
                current_line_sequence.push(p);
                current_position = Some(p);
            }
            PathEl::QuadTo(p1, p2) => {
                // Flush any accumulated line sequence first.
                if let Some(start) = line_sequence_start {
                    flush_line_sequence(
                        &mut result,
                        &mut current_line_sequence,
                        start,
                    );
                    line_sequence_start = None;
                }

                // Always output curves as-is - don't try to simplify them.
                result.push(PathEl::QuadTo(p1, p2));
                current_position = Some(p2);
            }
            PathEl::CurveTo(p1, p2, p3) => {
                // Flush any accumulated line sequence first.
                if let Some(start) = line_sequence_start {
                    flush_line_sequence(
                        &mut result,
                        &mut current_line_sequence,
                        start,
                    );
                    line_sequence_start = None;
                }

                // Always output curves as-is - don't try to simplify them.
                result.push(PathEl::CurveTo(p1, p2, p3));
                current_position = Some(p3);
            }
            PathEl::ClosePath => {
                // Flush line sequence before closing.
                if let Some(start) = line_sequence_start {
                    flush_line_sequence(
                        &mut result,
                        &mut current_line_sequence,
                        start,
                    );
                    line_sequence_start = None;
                }

                result.push(PathEl::ClosePath);
                current_position = None;
            }
        }
    }

    // Flush any remaining line sequence.
    if let Some(start) = line_sequence_start {
        flush_line_sequence(&mut result, &mut current_line_sequence, start);
    }

    result
}

/// Round a number to a maximum of 4 decimal places, removing trailing zeros.
fn round_coordinate(value: f64) -> f64 {
    // Round to 4 decimal places.
    let rounded = (value * 10000.0).round() / 10000.0;

    // Check if it's very close to an integer.
    if (rounded - rounded.round()).abs() < 0.0001 {
        rounded.round()
    } else {
        rounded
    }
}

/// Round all coordinates in a path to maximum 4 decimal places.
pub fn round_path_coordinates(path: &kurbo::BezPath) -> kurbo::BezPath {
    use kurbo::{BezPath, PathEl, Point};

    let mut result = BezPath::new();

    for el in path.iter() {
        let rounded_el = match el {
            PathEl::MoveTo(p) => PathEl::MoveTo(Point::new(
                round_coordinate(p.x),
                round_coordinate(p.y),
            )),
            PathEl::LineTo(p) => PathEl::LineTo(Point::new(
                round_coordinate(p.x),
                round_coordinate(p.y),
            )),
            PathEl::QuadTo(p1, p2) => PathEl::QuadTo(
                Point::new(round_coordinate(p1.x), round_coordinate(p1.y)),
                Point::new(round_coordinate(p2.x), round_coordinate(p2.y)),
            ),
            PathEl::CurveTo(p1, p2, p3) => PathEl::CurveTo(
                Point::new(round_coordinate(p1.x), round_coordinate(p1.y)),
                Point::new(round_coordinate(p2.x), round_coordinate(p2.y)),
                Point::new(round_coordinate(p3.x), round_coordinate(p3.y)),
            ),
            PathEl::ClosePath => PathEl::ClosePath,
        };
        result.push(rounded_el);
    }

    result
}

/// Combine paths using linesweeper boolean operations.
pub fn combine_paths(
    paths: &[PathWithMode],
    options: &OptimizationOptions,
) -> Option<String> {
    use kurbo::BezPath;
    use linesweeper::{binary_op, BinaryOp, FillRule};

    if paths.is_empty() {
        eprintln!("No paths to combine.");
        return None;
    }

    // Deduplicate identical paths with the same mode. `Union(A, A) = A`, so we
    // only need to include each unique path once per mode.
    use std::collections::HashSet;
    let mut seen_union: HashSet<String> = HashSet::new();
    let mut seen_difference: HashSet<String> = HashSet::new();
    let mut deduplicated_paths: Vec<PathWithMode> = Vec::new();
    let mut skipped_count = 0;

    for path_with_mode in paths {
        let seen = if path_with_mode.mode {
            &mut seen_union
        } else {
            &mut seen_difference
        };

        if seen.contains(&path_with_mode.path) {
            // Skip duplicate path with same mode.
            skipped_count += 1;
            continue;
        }

        seen.insert(path_with_mode.path.clone());
        deduplicated_paths.push(path_with_mode.clone());
    }

    if skipped_count > 0 {
        eprintln!("Skipped {} duplicate path(s)", skipped_count);
    }

    if deduplicated_paths.is_empty() {
        eprintln!("No paths to combine after deduplication.");
        return None;
    }

    // If only one path, return it directly.
    if deduplicated_paths.len() == 1 {
        return Some(deduplicated_paths[0].path.clone());
    }

    // Parse the first path as the starting result.
    let mut result = match BezPath::from_svg(&deduplicated_paths[0].path) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to parse first path: {:?}", e);
            return None;
        }
    };

    // Apply boolean operations for each subsequent path.
    for (i, path_with_mode) in deduplicated_paths[1..].iter().enumerate() {
        let path = match BezPath::from_svg(&path_with_mode.path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to parse path {}: {:?}", i + 1, e);
                continue;
            }
        };

        let op = if path_with_mode.mode {
            BinaryOp::Union
        } else {
            BinaryOp::Difference
        };

        // Perform boolean operation and convert result back to `BezPath`.
        let contours = match binary_op(&result, &path, FillRule::NonZero, op) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Boolean operation {} failed for path {}: {:?}",
                    if path_with_mode.mode {
                        "union"
                    } else {
                        "difference"
                    },
                    i + 1,
                    e
                );
                continue; // Skip this operation on error.
            }
        };

        // Convert contours back to a single `BezPath` by concatenating all
        // contour paths.
        result = BezPath::new();
        for group in contours.grouped() {
            for contour_idx in group {
                let contour_path = &contours[contour_idx].path;
                result.extend(contour_path.iter());
            }
        }
    }

    // Apply optimizations based on options.
    let mut optimized = result;

    if options.enable_rounding {
        // Round coordinates to maximum 4 decimal places.
        optimized = round_path_coordinates(&optimized);
    }

    if options.enable_deduplication {
        // Remove duplicate consecutive anchors with identical positions.
        optimized = deduplicate_path_elements(&optimized);
    }

    if options.enable_collinear_simplification {
        // Remove collinear points.
        optimized = simplify_path_collinear(&optimized);
    }

    Some(optimized.to_svg())
}

/// Generate SVG from icon.
pub fn icon_to_svg(
    icon: &Icon,
    sketch_mode: bool,
    paths: &[PathWithMode],
    options: &OptimizationOptions,
) -> String {
    let svg_content = if sketch_mode {
        // In sketch mode, output raw path elements without combining.
        let mut path_elements = String::new();
        for path_with_mode in paths {
            let class = if path_with_mode.mode {
                "sketch-path-union"
            } else {
                "sketch-path-subtract"
            };
            path_elements.push_str(&format!(
                r#"<path d="{}" class="{}" />"#,
                path_with_mode.path, class
            ));
        }
        path_elements
    } else {
        // In final mode, combine all paths into a single SVG path.
        if let Some(combined) = combine_paths(paths, options) {
            format!(r#"<path d="{}" fill="black" stroke="none" />"#, combined)
        } else {
            eprintln!(
                "No combined path constructed for icon `{}`.",
                icon.name.as_deref().unwrap_or("unnamed")
            );
            String::new()
        }
    };

    format!(
        concat!(
            r#"<?xml version="1.0" encoding="utf-8" ?>"#,
            r#"<svg baseProfile="tiny" height="16px" version="1.2" width="16px" "#,
            r#"viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" "#,
            r#"xmlns:ev="http://www.w3.org/2001/xml-events" "#,
            r#"xmlns:xlink="http://www.w3.org/1999/xlink"><defs />{}</svg>"#
        ),
        svg_content
    )
}

/// Generate all icons and save them to files.
pub fn generate_icons(
    content: &str,
    output_dir: &Path,
    sketch_mode: bool,
    options: &OptimizationOptions,
) -> Result<usize> {
    // Parse the iconscript file.
    let icons = parse_iconscript(content, sketch_mode)?;

    // Ensure output directory exists.
    fs::create_dir_all(output_dir)?;

    let mut icon_count = 0;

    for (i, (icon, paths)) in icons.iter().enumerate() {
        let svg = icon_to_svg(icon, sketch_mode, paths, options);

        if !svg.is_empty() {
            let filename = if let Some(ref name) = icon.name {
                if name != "temp" {
                    format!("{}.svg", name)
                } else {
                    format!("icon_{}.svg", i)
                }
            } else {
                format!("icon_{}.svg", i)
            };

            let filepath = output_dir.join(&filename);
            fs::write(&filepath, svg)?;
            println!("Generated: {}", filename);
            icon_count += 1;
        }
    }

    Ok(icon_count)
}
