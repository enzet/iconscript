use anyhow::Result;
use kurbo::{Affine, BezPath, Circle, Ellipse, PathEl, Point, Rect, RoundedRect, Shape, Vec2};
use std::fs;
use std::path::Path;

use crate::generator::{icon_to_svg, OptimizationOptions};
use crate::types::{Icon, PathWithMode};

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Black,
    White,
    Red,
    None,
    Other,
}

struct StyleInfo {
    fill: Color,
    stroke_color: Color,
    stroke_width: f64,
}

fn parse_color(s: &str) -> Color {
    match s.trim() {
        "#000000" | "#000" | "black" => Color::Black,
        "#ffffff" | "#fff" | "white" => Color::White,
        "#ff0000" | "#f00" | "red" => Color::Red,
        "none" | "" => Color::None,
        _ => Color::Other,
    }
}

fn parse_length(s: &str) -> f64 {
    s.trim_end_matches("px")
        .trim_end_matches("pt")
        .trim_end_matches("em")
        .trim()
        .parse::<f64>()
        .unwrap_or(1.0)
}

fn parse_style(style: &str) -> StyleInfo {
    let mut fill = Color::Other;
    let mut stroke_color = Color::None;
    let mut stroke_width = 1.0f64;

    for part in style.split(';') {
        let mut kv = part.splitn(2, ':');
        let key = match kv.next() {
            Some(k) => k.trim(),
            None => continue,
        };
        let value = match kv.next() {
            Some(v) => v.trim(),
            None => continue,
        };
        match key {
            "fill" => fill = parse_color(value),
            "stroke" => stroke_color = parse_color(value),
            "stroke-width" => stroke_width = parse_length(value),
            _ => {}
        }
    }

    StyleInfo {
        fill,
        stroke_color,
        stroke_width,
    }
}

fn parse_transform(s: &str) -> Affine {
    let s = s.trim();

    if let Some(inner) = s.strip_prefix("matrix(").and_then(|s| s.strip_suffix(')')) {
        let vals: Vec<f64> = inner
            .split(|c: char| c == ',' || c.is_ascii_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        if vals.len() == 6 {
            return Affine::new([vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]]);
        }
    } else if let Some(inner) = s.strip_prefix("translate(").and_then(|s| s.strip_suffix(')')) {
        let vals: Vec<f64> = inner
            .split(|c: char| c == ',' || c.is_ascii_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        if !vals.is_empty() {
            let tx = vals[0];
            let ty = vals.get(1).copied().unwrap_or(0.0);
            return Affine::translate((tx, ty));
        }
    } else if let Some(inner) = s.strip_prefix("scale(").and_then(|s| s.strip_suffix(')')) {
        let vals: Vec<f64> = inner
            .split(|c: char| c == ',' || c.is_ascii_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap_or(1.0))
            .collect();
        if !vals.is_empty() {
            let sx = vals[0];
            let sy = vals.get(1).copied().unwrap_or(sx);
            return Affine::new([sx, 0.0, 0.0, sy, 0.0, 0.0]);
        }
    } else if let Some(inner) = s.strip_prefix("rotate(").and_then(|s| s.strip_suffix(')')) {
        let vals: Vec<f64> = inner
            .split(|c: char| c == ',' || c.is_ascii_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        if !vals.is_empty() {
            let angle = vals[0].to_radians();
            let (sin, cos) = angle.sin_cos();
            if vals.len() >= 3 {
                // rotate(angle, cx, cy) = translate(cx,cy) * rotate(angle) * translate(-cx,-cy)
                let cx = vals[1]; let cy = vals[2];
                return Affine::translate((cx, cy))
                    * Affine::new([cos, sin, -sin, cos, 0.0, 0.0])
                    * Affine::translate((-cx, -cy));
            }
            return Affine::new([cos, sin, -sin, cos, 0.0, 0.0]);
        }
    }

    Affine::IDENTITY
}

fn parse_attr_f64(node: &roxmltree::Node, attr: &str, default: f64) -> f64 {
    node.attribute(attr)
        .and_then(|s| s.trim().parse::<f64>().ok())
        .unwrap_or(default)
}


/// Right-hand unit normal of a direction vector (positive = right side of travel).
fn right_unit_normal(dx: f64, dy: f64) -> (f64, f64) {
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-9 { return (0.0, 0.0); }
    (dy / len, -dx / len)
}

/// Append a circular arc as cubic bezier segments from `center + by*n1` to
/// `center + by*n2`, sweeping the shorter angular path. Each segment spans at
/// most π/2 so the cubic approximation error is < 0.03 % of the radius.
fn add_arc_curves(path: &mut BezPath, center: Point, n1: (f64, f64), n2: (f64, f64), by: f64) {
    use std::f64::consts::PI;
    let a1 = n1.1.atan2(n1.0);
    let a2 = n2.1.atan2(n2.0);
    let mut da = a2 - a1;
    while da > PI  { da -= 2.0 * PI; }
    while da < -PI { da += 2.0 * PI; }
    let n = ((da.abs() / (PI / 2.0)).ceil() as usize).max(1);
    let step = da / n as f64;
    for i in 0..n {
        let a_s = a1 + step * i as f64;
        let a_e = a1 + step * (i + 1) as f64;
        // k = (4/3)*tan(Δθ/4); tangent direction at angle a is (-sin a, cos a).
        let k = (4.0 / 3.0) * (step / 4.0).tan();
        let (cs, ss) = (a_s.cos(), a_s.sin());
        let (ce, se) = (a_e.cos(), a_e.sin());
        let p1 = Point::new(center.x + by * (cs + k * (-ss)), center.y + by * (ss + k * cs));
        let p2 = Point::new(center.x + by * (ce - k * (-se)), center.y + by * (se - k * ce));
        let p3 = Point::new(center.x + by * ce,               center.y + by * se);
        path.curve_to(p1, p2, p3);
    }
}

/// Evaluate a cubic bezier at parameter t.
fn cubic_eval(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> Point {
    let u = 1.0 - t;
    let u2 = u * u; let t2 = t * t;
    Point::new(
        u2*u*p0.x + 3.0*u2*t*p1.x + 3.0*u*t2*p2.x + t2*t*p3.x,
        u2*u*p0.y + 3.0*u2*t*p1.y + 3.0*u*t2*p2.y + t2*t*p3.y,
    )
}

/// Right-hand unit normal of the tangent of a cubic bezier at parameter t.
fn cubic_normal_at(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let dx = 3.0*(u*u*(p1.x-p0.x) + 2.0*u*t*(p2.x-p1.x) + t*t*(p3.x-p2.x));
    let dy = 3.0*(u*u*(p1.y-p0.y) + 2.0*u*t*(p2.y-p1.y) + t*t*(p3.y-p2.y));
    right_unit_normal(dx, dy)
}

/// De Casteljau split of a cubic bezier at t = 0.5.
fn cubic_split(
    p0: Point, p1: Point, p2: Point, p3: Point,
) -> ((Point, Point, Point, Point), (Point, Point, Point, Point)) {
    let m = |a: Point, b: Point| Point::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
    let m01 = m(p0, p1); let m12 = m(p1, p2); let m23 = m(p2, p3);
    let m012 = m(m01, m12); let m123 = m(m12, m23);
    let mid  = m(m012, m123);
    ((p0, m01, m012, mid), (mid, m123, m23, p3))
}

/// Curvature-based control-arm scale factor for a bezier offset.
///
/// For the offset at distance `d` to the right of a curve whose signed
/// curvature at the start is κ, the correct arm length scales by `1 + d·κ`:
/// - κ > 0 (CW arc in SVG): right offset expands → scale > 1
/// - κ < 0 (CCW arc):       right offset contracts → scale < 1
/// - κ = 0 (straight):      scale = 1 (no change needed)
///
/// κ at the START of cubic (pa, pb, pc, …) is (2/3)·cross(pb−pa, pa−2pb+pc) / |pb−pa|³.
/// Clamped to 0.01 to prevent degenerate reversed control arms.
fn control_arm_scale(pa: Point, pb: Point, pc: Point, d: f64) -> f64 {
    let dx = pb.x - pa.x;
    let dy = pb.y - pa.y;
    let len_sq = dx * dx + dy * dy;
    if len_sq < 1e-18 { return 1.0; }
    let len3 = len_sq * len_sq.sqrt();
    let cx = pa.x - 2.0 * pb.x + pc.x;
    let cy = pa.y - 2.0 * pb.y + pc.y;
    let kappa = (2.0 / 3.0) * (dx * cy - dy * cx) / len3;
    (1.0 + d * kappa).max(0.01)
}

/// Tiller-Hanson parallel-curve offset of a cubic bezier, with curvature-
/// corrected control arm lengths so inset arms are shorter and outset arms
/// are longer in proportion to the local radius of curvature.
/// `d > 0` → right side, `d < 0` → left side.
fn offset_cubic(
    p0: Point, p1: Point, p2: Point, p3: Point, d: f64,
) -> (Point, Point, Point, Point) {
    let (tx0, ty0) = {
        let dx = p1.x - p0.x; let dy = p1.y - p0.y;
        if dx * dx + dy * dy > 1e-18 { (dx, dy) }
        else {
            let dx = p2.x - p0.x; let dy = p2.y - p0.y;
            if dx * dx + dy * dy > 1e-18 { (dx, dy) }
            else { (p3.x - p0.x, p3.y - p0.y) }
        }
    };
    let (tx1, ty1) = {
        let dx = p3.x - p2.x; let dy = p3.y - p2.y;
        if dx * dx + dy * dy > 1e-18 { (dx, dy) }
        else {
            let dx = p3.x - p1.x; let dy = p3.y - p1.y;
            if dx * dx + dy * dy > 1e-18 { (dx, dy) }
            else { (p3.x - p0.x, p3.y - p0.y) }
        }
    };
    let (nx0, ny0) = right_unit_normal(tx0, ty0);
    let (nx1, ny1) = right_unit_normal(tx1, ty1);
    let p0o = Point::new(p0.x + d * nx0, p0.y + d * ny0);
    let p3o = Point::new(p3.x + d * nx1, p3.y + d * ny1);
    // Scale control arms: s0 uses κ at start (forward), s1 uses κ at end.
    // For the end we pass -d because control_arm_scale computes κ along the
    // direction pa→pb; for the end arm that direction is reversed (p3→p2),
    // so the curvature sign flips, which -d corrects.
    let s0 = control_arm_scale(p0, p1, p2, d);
    let s1 = control_arm_scale(p3, p2, p1, -d);
    let p1o = Point::new(p0o.x + (p1.x - p0.x) * s0, p0o.y + (p1.y - p0.y) * s0);
    let p2o = Point::new(p3o.x + (p2.x - p3.x) * s1, p3o.y + (p2.y - p3.y) * s1);
    (p0o, p1o, p2o, p3o)
}

/// Append the offset of a single cubic bezier segment to `path`, subdividing
/// adaptively with two independent guards:
///
/// **Angular-span guard** — subdivide whenever the start-to-end tangent turn
/// exceeds ≈45° (dot < cos 45° ≈ 0.707).  This is the primary trigger for
/// near-circular arcs: the point-error at t=0.5 is nearly zero for such arcs
/// (the midpoint happens to lie on the true offset arc), so a purely
/// error-based check misses them.  A 90° arc always splits into two ≤45°
/// segments; each further half-arc passes the guard.
///
/// **Point-error guard** — check the signed distance between the true offset
/// (original curve + d·normal) and the Tiller-Hanson bezier at t = ¼, ½, ¾.
/// The maximum error of a cubic arc approximation occurs near the quartiles,
/// not at the midpoint, so checking all three catches what t=0.5 alone misses.
///
/// After a de Casteljau split both halves share the exact offset endpoint, so
/// no extra join geometry is required.  Depth is capped at 8.
fn push_offset_cubic(
    path: &mut BezPath,
    p0: Point, p1: Point, p2: Point, p3: Point,
    d: f64,
    tol: f64,
    depth: u32,
) {
    let (op0, op1, op2, op3) = offset_cubic(p0, p1, p2, p3, d);

    if depth < 8 {
        // Normalized start tangent.
        let (t0x, t0y) = {
            let (dx, dy) = {
                let dx = p1.x-p0.x; let dy = p1.y-p0.y;
                if dx*dx+dy*dy > 1e-18 { (dx,dy) }
                else { let dx = p2.x-p0.x; let dy = p2.y-p0.y;
                       if dx*dx+dy*dy > 1e-18 { (dx,dy) } else { (p3.x-p0.x,p3.y-p0.y) } }
            };
            let len = (dx*dx+dy*dy).sqrt().max(1e-9);
            (dx/len, dy/len)
        };
        // Normalized end tangent.
        let (t1x, t1y) = {
            let (dx, dy) = {
                let dx = p3.x-p2.x; let dy = p3.y-p2.y;
                if dx*dx+dy*dy > 1e-18 { (dx,dy) }
                else { let dx = p3.x-p1.x; let dy = p3.y-p1.y;
                       if dx*dx+dy*dy > 1e-18 { (dx,dy) } else { (p3.x-p0.x,p3.y-p0.y) } }
            };
            let len = (dx*dx+dy*dy).sqrt().max(1e-9);
            (dx/len, dy/len)
        };
        // cos of the tangent-turn angle; < cos(45°) means arc > 45° → subdivide.
        let dot_tt = t0x*t1x + t0y*t1y;

        // Maximum point error at t = 0.25, 0.5, 0.75.
        let mut max_err_sq = 0.0f64;
        for &t in &[0.25_f64, 0.5, 0.75] {
            let pm       = cubic_eval(p0, p1, p2, p3, t);
            let (nx, ny) = cubic_normal_at(p0, p1, p2, p3, t);
            let true_pt  = Point::new(pm.x + d * nx, pm.y + d * ny);
            let approx   = cubic_eval(op0, op1, op2, op3, t);
            max_err_sq   = max_err_sq.max(
                (true_pt.x - approx.x).powi(2) + (true_pt.y - approx.y).powi(2),
            );
        }

        // cos(45°) ≈ 0.7071; using 0.7 gives a tiny margin so that an exactly
        // 45° half-arc (dot ≈ 0.7071) is never re-subdivided.
        if max_err_sq > tol * tol || dot_tt < 0.7 {
            let (l, r) = cubic_split(p0, p1, p2, p3);
            push_offset_cubic(path, l.0, l.1, l.2, l.3, d, tol, depth + 1);
            push_offset_cubic(path, r.0, r.1, r.2, r.3, d, tol, depth + 1);
            return;
        }
    }

    path.curve_to(op1, op2, op3);
}

/// A segment parsed from a BezPath.  QuadTo is promoted to Cubic at parse time.
enum Seg {
    Line(Point, Point),
    Cubic(Point, Point, Point, Point),
}

impl Seg {
    fn start(&self) -> Point {
        match self { Seg::Line(p, _) | Seg::Cubic(p, _, _, _) => *p }
    }
    fn end(&self) -> Point {
        match self { Seg::Line(_, p) => *p, Seg::Cubic(_, _, _, p) => *p }
    }
    fn start_tangent(&self) -> (f64, f64) {
        match self {
            Seg::Line(p0, p1) => (p1.x - p0.x, p1.y - p0.y),
            Seg::Cubic(p0, p1, p2, p3) => {
                let dx = p1.x - p0.x; let dy = p1.y - p0.y;
                if dx * dx + dy * dy > 1e-18 { return (dx, dy); }
                let dx = p2.x - p0.x; let dy = p2.y - p0.y;
                if dx * dx + dy * dy > 1e-18 { return (dx, dy); }
                (p3.x - p0.x, p3.y - p0.y)
            }
        }
    }
    fn end_tangent(&self) -> (f64, f64) {
        match self {
            Seg::Line(p0, p1) => (p1.x - p0.x, p1.y - p0.y),
            Seg::Cubic(p0, p1, p2, p3) => {
                let dx = p3.x - p2.x; let dy = p3.y - p2.y;
                if dx * dx + dy * dy > 1e-18 { return (dx, dy); }
                let dx = p3.x - p1.x; let dy = p3.y - p1.y;
                if dx * dx + dy * dy > 1e-18 { return (dx, dy); }
                (p3.x - p0.x, p3.y - p0.y)
            }
        }
    }
    fn offset_start(&self, d: f64) -> Point {
        let (tx, ty) = self.start_tangent();
        let (nx, ny) = right_unit_normal(tx, ty);
        let p = self.start();
        Point::new(p.x + d * nx, p.y + d * ny)
    }
    fn offset_end(&self, d: f64) -> Point {
        let (tx, ty) = self.end_tangent();
        let (nx, ny) = right_unit_normal(tx, ty);
        let p = self.end();
        Point::new(p.x + d * nx, p.y + d * ny)
    }
    /// Append the offset version of this segment to `path` (no MoveTo; caller
    /// is responsible for ensuring the current point is `offset_start(d)`).
    fn push_offset(&self, path: &mut BezPath, d: f64) {
        match self {
            Seg::Line(p0, p1) => {
                let (tx, ty) = (p1.x - p0.x, p1.y - p0.y);
                let (nx, ny) = right_unit_normal(tx, ty);
                path.line_to(Point::new(p1.x + d * nx, p1.y + d * ny));
            }
            Seg::Cubic(p0, p1, p2, p3) => {
                push_offset_cubic(path, *p0, *p1, *p2, *p3, d, 0.1, 0);
            }
        }
    }

    /// Append this segment traversed in reverse at right-offset `d`.
    /// Equivalent to the right-side offset of the segment with endpoints
    /// swapped, which equals the left-side of the original traversed end→start.
    /// Caller must be at `self.offset_end(-d)` before calling.
    fn push_offset_reversed(&self, path: &mut BezPath, d: f64) {
        match self {
            Seg::Line(p0, p1) => {
                let (tx, ty) = (p0.x - p1.x, p0.y - p1.y);
                let (nx, ny) = right_unit_normal(tx, ty);
                path.line_to(Point::new(p0.x + d * nx, p0.y + d * ny));
            }
            Seg::Cubic(p0, p1, p2, p3) => {
                push_offset_cubic(path, *p3, *p2, *p1, *p0, d, 0.1, 0);
            }
        }
    }
}

/// Parse a BezPath into sub-paths of `Seg` (QuadTo → Cubic).
fn bezpath_to_segs(bezpath: &BezPath) -> Vec<(Vec<Seg>, bool)> {
    let mut result = Vec::new();
    let mut segs: Vec<Seg> = Vec::new();
    let mut cur = Point::ZERO;
    let mut start = Point::ZERO;

    for el in bezpath.iter() {
        match el {
            PathEl::MoveTo(p) => {
                if !segs.is_empty() {
                    result.push((std::mem::take(&mut segs), false));
                }
                cur = p; start = p;
            }
            PathEl::LineTo(p) => { segs.push(Seg::Line(cur, p)); cur = p; }
            PathEl::QuadTo(p1, p2) => {
                let cp1 = Point::new(cur.x + 2.0/3.0*(p1.x-cur.x), cur.y + 2.0/3.0*(p1.y-cur.y));
                let cp2 = Point::new(p2.x + 2.0/3.0*(p1.x-p2.x), p2.y + 2.0/3.0*(p1.y-p2.y));
                segs.push(Seg::Cubic(cur, cp1, cp2, p2));
                cur = p2;
            }
            PathEl::CurveTo(p1, p2, p3) => {
                segs.push(Seg::Cubic(cur, p1, p2, p3));
                cur = p3;
            }
            PathEl::ClosePath => {
                let dx = cur.x - start.x; let dy = cur.y - start.y;
                if (dx*dx + dy*dy).sqrt() > 1e-6 {
                    segs.push(Seg::Line(cur, start));
                }
                result.push((std::mem::take(&mut segs), true));
                cur = start;
            }
        }
    }
    if !segs.is_empty() {
        result.push((segs, false));
    }
    result
}

/// Insert the join geometry between two consecutive offset segments.
///
/// Convex corner (gap on the offset side): cubic bezier arc centered at the
/// original vertex.  Concave corner (overlap): single straight line.
///
/// n1/n2 for the arc are derived directly from the actual offset positions so
/// this function works correctly for both forward and reverse traversal.
fn add_offset_join(
    path: &mut BezPath,
    vertex: Point,
    end_tangent: (f64, f64),
    start_tangent: (f64, f64),
    end_off: Point,
    start_off: Point,
    d: f64,
) {
    let dx = start_off.x - end_off.x;
    let dy = start_off.y - end_off.y;
    if (dx * dx + dy * dy).sqrt() < 1e-6 { return; }
    let (etx, ety) = end_tangent;
    let (stx, sty) = start_tangent;
    let cross = etx * sty - ety * stx;
    // Convex gap: cross*d < 0 (right turn on right side, or left turn on left side).
    if cross * d < 0.0 {
        let dx1 = end_off.x - vertex.x;
        let dy1 = end_off.y - vertex.y;
        let r1 = (dx1 * dx1 + dy1 * dy1).sqrt();
        let dx2 = start_off.x - vertex.x;
        let dy2 = start_off.y - vertex.y;
        let r2 = (dx2 * dx2 + dy2 * dy2).sqrt();
        if r1 > 1e-9 && r2 > 1e-9 {
            add_arc_curves(path, vertex, (dx1/r1, dy1/r1), (dx2/r2, dy2/r2), d.abs());
        }
    } else {
        path.line_to(start_off);
    }
}

/// Compute the Tiller-Hanson offset of a closed sub-path.
/// `d` encodes side (+) and magnitude; caller must pass the correct sign for
/// the winding direction so that positive `d` is always outward.
fn build_offset_closed(segs: &[Seg], d: f64) -> Option<BezPath> {
    if segs.is_empty() { return None; }
    let n = segs.len();
    let mut path = BezPath::new();
    path.move_to(segs[0].offset_start(d));
    for i in 0..n {
        segs[i].push_offset(&mut path, d);
        let next = (i + 1) % n;
        add_offset_join(
            &mut path,
            segs[i].end(),
            segs[i].end_tangent(),
            segs[next].start_tangent(),
            segs[i].offset_end(d),
            segs[next].offset_start(d),
            d,
        );
    }
    path.close_path();
    Some(path)
}


/// Stroke outline for an open sub-path using the full bezier offset pipeline:
/// right-side offset forward, butt cap, left-side offset in reverse, close.
fn build_stroke_open(segs: &[Seg], d: f64) -> Option<BezPath> {
    if segs.is_empty() { return None; }
    let n = segs.len();
    let mut path = BezPath::new();

    // Right side: forward offset at +d.
    path.move_to(segs[0].offset_start(d));
    for i in 0..n {
        segs[i].push_offset(&mut path, d);
        if i + 1 < n {
            add_offset_join(
                &mut path, segs[i].end(),
                segs[i].end_tangent(), segs[i+1].start_tangent(),
                segs[i].offset_end(d), segs[i+1].offset_start(d),
                d,
            );
        }
    }

    // End butt cap (straight line to left-side end).
    path.line_to(segs[n-1].offset_end(-d));

    // Left side: reverse offset at -d (traverse segments in reverse order).
    // push_offset_reversed(d) moves from offset_end(-d) to offset_start(-d).
    for i in (0..n).rev() {
        segs[i].push_offset_reversed(&mut path, d);
        if i > 0 {
            let (etx, ety) = segs[i].start_tangent();
            let (stx, sty) = segs[i-1].end_tangent();
            add_offset_join(
                &mut path, segs[i].start(),
                (-etx, -ety), (-stx, -sty),
                segs[i].offset_start(-d), segs[i-1].offset_end(-d),
                -d,
            );
        }
    }

    // Start butt cap: close_path draws straight line back to the MoveTo point.
    path.close_path();
    Some(path)
}


/// Return a set of BezPaths representing the stroke region of `bezpath`.
///
/// Both closed and open sub-paths use proper parallel-curve offsetting
/// (Tiller-Hanson for segments, cubic arc approximation for convex joins).
/// No polyline sampling — the output contains only `LineTo` for straight
/// input edges and `CurveTo` for curved edges and join arcs.
/// Returns `(path, is_inner)` pairs. `is_inner = true` marks the inner
/// boundary of a closed-path stroke ring; callers must subtract it when
/// there is no fill covering the interior.
fn stroke_approx(bezpath: &BezPath, stroke_width: f64) -> Vec<(BezPath, bool)> {
    let d = stroke_width / 2.0;
    let mut result = Vec::new();

    for (segs, closed) in bezpath_to_segs(bezpath) {
        if closed {
            let area: f64 = segs.iter().map(|s| {
                let p0 = s.start(); let p1 = s.end();
                p0.x * p1.y - p1.x * p0.y
            }).sum::<f64>() / 2.0;
            let signed_d = if area >= 0.0 { d } else { -d };
            if let Some(p) = build_offset_closed(&segs, signed_d) {
                result.push((p, false));
            }
            if let Some(p) = build_offset_closed(&segs, -signed_d) {
                result.push((p, true));
            }
            for seg in &segs {
                result.push((Circle::new(seg.end(), d).to_path(0.1), false));
            }
        } else {
            if let Some(p) = build_stroke_open(&segs, d) {
                result.push((p, false));
            }
            // Round caps at endpoints and round joins at interior joints.
            result.push((Circle::new(segs[0].start(), d).to_path(0.1), false));
            for seg in &segs {
                result.push((Circle::new(seg.end(), d).to_path(0.1), false));
            }
        }
    }

    result
}

/// Produce `(BezPath, is_union)` entries for a single SVG element, respecting
/// fill, stroke color, and stroke width.
fn element_path_entries(
    node: &roxmltree::Node,
    style: &StyleInfo,
    combined_transform: Affine,
) -> Vec<(BezPath, bool)> {
    let has_fill = matches!(style.fill, Color::Black | Color::White);
    let fill_is_union = style.fill == Color::Black;
    let has_stroke = style.stroke_width > 0.0
        && (style.stroke_color == Color::Black || style.stroke_color == Color::White);
    let stroke_is_union = style.stroke_color == Color::Black;

    let mut results: Vec<(BezPath, bool)> = Vec::new();

    match node.tag_name().name() {
        "rect" => {
            let x = parse_attr_f64(node, "x", 0.0);
            let y = parse_attr_f64(node, "y", 0.0);
            let w = parse_attr_f64(node, "width", 0.0);
            let h = parse_attr_f64(node, "height", 0.0);
            if w <= 0.0 || h <= 0.0 {
                return results;
            }
            let rx = parse_attr_f64(node, "rx", 0.0);
            let ry = parse_attr_f64(node, "ry", rx);
            let r = rx.min(ry);

            let make_rect_path = |pad: f64| -> BezPath {
                let x0 = x - pad;
                let y0 = y - pad;
                let x1 = x + w + pad;
                let y1 = y + h + pad;
                let r_exp = (r + pad).max(0.0).min((x1 - x0) / 2.0).min((y1 - y0) / 2.0);
                if r_exp > 0.0 {
                    RoundedRect::new(x0, y0, x1, y1, r_exp).to_path(0.1)
                } else {
                    Rect::new(x0, y0, x1, y1).to_path(0.1)
                }
            };

            // When fill and stroke are the same color, emit only the
            // stroke-expanded shape (which fully contains the fill area).
            if has_fill && has_stroke && fill_is_union == stroke_is_union {
                let mut p = make_rect_path(style.stroke_width / 2.0);
                p.apply_affine(combined_transform);
                results.push((p, fill_is_union));
            } else {
                if has_fill {
                    let mut p = make_rect_path(0.0);
                    p.apply_affine(combined_transform);
                    results.push((p, fill_is_union));
                }
                if has_stroke {
                    let d = style.stroke_width / 2.0;
                    let mut outer = make_rect_path(d);
                    outer.apply_affine(combined_transform);
                    results.push((outer, stroke_is_union));
                    if !has_fill && w > style.stroke_width && h > style.stroke_width {
                        let mut inner = make_rect_path(-d);
                        inner.apply_affine(combined_transform);
                        results.push((inner, !stroke_is_union));
                    }
                }
            }
        }

        "circle" => {
            let cx = parse_attr_f64(node, "cx", 0.0);
            let cy = parse_attr_f64(node, "cy", 0.0);
            let r = parse_attr_f64(node, "r", 0.0);
            if r <= 0.0 {
                return results;
            }
            let make_circle = |pad: f64| -> BezPath {
                Circle::new((cx, cy), r + pad).to_path(0.1)
            };

            if has_fill && has_stroke && fill_is_union == stroke_is_union {
                let mut p = make_circle(style.stroke_width / 2.0);
                p.apply_affine(combined_transform);
                results.push((p, fill_is_union));
            } else {
                if has_fill {
                    let mut p = make_circle(0.0);
                    p.apply_affine(combined_transform);
                    results.push((p, fill_is_union));
                }
                if has_stroke {
                    let d = style.stroke_width / 2.0;
                    let mut outer = make_circle(d);
                    outer.apply_affine(combined_transform);
                    results.push((outer, stroke_is_union));
                    if !has_fill && r > d {
                        let mut inner = make_circle(-d);
                        inner.apply_affine(combined_transform);
                        results.push((inner, !stroke_is_union));
                    }
                }
            }
        }

        "ellipse" => {
            let cx = parse_attr_f64(node, "cx", 0.0);
            let cy = parse_attr_f64(node, "cy", 0.0);
            let rx = parse_attr_f64(node, "rx", 0.0);
            let ry = parse_attr_f64(node, "ry", 0.0);
            if rx <= 0.0 || ry <= 0.0 {
                return results;
            }
            let make_ellipse = |pad: f64| -> BezPath {
                Ellipse::new((cx, cy), Vec2::new(rx + pad, ry + pad), 0.0).to_path(0.1)
            };

            if has_fill && has_stroke && fill_is_union == stroke_is_union {
                let mut p = make_ellipse(style.stroke_width / 2.0);
                p.apply_affine(combined_transform);
                results.push((p, fill_is_union));
            } else {
                if has_fill {
                    let mut p = make_ellipse(0.0);
                    p.apply_affine(combined_transform);
                    results.push((p, fill_is_union));
                }
                if has_stroke {
                    let d = style.stroke_width / 2.0;
                    let mut outer = make_ellipse(d);
                    outer.apply_affine(combined_transform);
                    results.push((outer, stroke_is_union));
                    if !has_fill && rx > d && ry > d {
                        let mut inner = make_ellipse(-d);
                        inner.apply_affine(combined_transform);
                        results.push((inner, !stroke_is_union));
                    }
                }
            }
        }

        "path" => {
            let d = match node.attribute("d") {
                Some(d) => d,
                None => return results,
            };
            let local_path = match BezPath::from_svg(d) {
                Ok(p) => p,
                Err(_) => return results,
            };

            let mut global_path = local_path;
            global_path.apply_affine(combined_transform);

            if has_fill {
                results.push((global_path.clone(), fill_is_union));
            }

            if has_stroke {
                for (thick, is_inner) in stroke_approx(&global_path, style.stroke_width) {
                    if is_inner && has_fill {
                        continue;
                    }
                    let mode = if is_inner { !stroke_is_union } else { stroke_is_union };
                    results.push((thick, mode));
                }
            }
        }

        _ => {}
    }

    results
}

/// Compute the bounding box of a path element in canvas space (for the red
/// size-indicator element). Returns None if the element is unrecognised.
fn element_bbox(node: &roxmltree::Node, transform: Affine) -> Option<Rect> {
    // Use Black fill so element_path_entries actually generates a path for us.
    let style = StyleInfo {
        fill: Color::Black,
        stroke_color: Color::None,
        stroke_width: 0.0,
    };
    let entries = element_path_entries(node, &style, transform);
    let mut bbox: Option<Rect> = None;
    for (p, _) in &entries {
        let b = p.bounding_box();
        if b.x0.is_finite() {
            bbox = Some(match bbox {
                None => b,
                Some(existing) => existing.union(b),
            });
        }
    }
    bbox
}

fn union_rect(a: Option<Rect>, b: Rect) -> Option<Rect> {
    if !b.x0.is_finite() || !b.y0.is_finite() || !b.x1.is_finite() || !b.y1.is_finite() {
        return a;
    }
    Some(match a {
        None => b,
        Some(existing) => existing.union(b),
    })
}

fn is_icon_id(id: &str) -> bool {
    // Match IDs like "vending_candy_v0": ends with _v followed by digits.
    if let Some(pos) = id.rfind("_v") {
        let suffix = &id[pos + 2..];
        !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}

fn format_f64(v: f64) -> String {
    if (v - v.round()).abs() < 0.001 {
        format!("{}", v.round() as i64)
    } else {
        format!("{:.3}", v)
    }
}

fn process_icon_group(
    node: roxmltree::Node,
    id: &str,
) -> Option<(Icon, Vec<PathWithMode>, String)> {
    let group_transform = node
        .attribute("transform")
        .map(parse_transform)
        .unwrap_or(Affine::IDENTITY);

    let mut bez_paths: Vec<(BezPath, bool)> = Vec::new();
    let mut red_bbox: Option<Rect> = None;
    let mut all_bbox: Option<Rect> = None;

    for child in node.children() {
        if !child.is_element() {
            continue;
        }

        let style_str = child.attribute("style").unwrap_or("");
        let style = parse_style(style_str);
        let elem_transform = child
            .attribute("transform")
            .map(parse_transform)
            .unwrap_or(Affine::IDENTITY);
        let combined = group_transform * elem_transform;

        if style.fill == Color::Red {
            // Use the red element's bounding box to determine icon size.
            if let Some(bbox) = element_bbox(&child, combined) {
                red_bbox = union_rect(red_bbox, bbox);
            }
            continue;
        }

        let entries = element_path_entries(&child, &style, combined);
        for (ref p, _) in &entries {
            all_bbox = union_rect(all_bbox, p.bounding_box());
        }
        bez_paths.extend(entries);
    }

    // Prefer the red element bbox if it is large enough to be a boundary
    // rectangle (at least 4×4 px). Small red markers are ignored.
    let icon_bbox = red_bbox
        .filter(|r| r.width() > 4.0 && r.height() > 4.0)
        .or(all_bbox)?;

    // Translate everything so the icon origin is at (0, 0).
    let translate = Affine::translate((-icon_bbox.x0, -icon_bbox.y0));

    let paths: Vec<PathWithMode> = bez_paths
        .into_iter()
        .map(|(mut p, is_union)| {
            p.apply_affine(translate);
            PathWithMode {
                path: p.to_svg(),
                mode: is_union,
            }
        })
        .collect();

    let view_box = format!(
        "0 0 {} {}",
        format_f64(icon_bbox.width()),
        format_f64(icon_bbox.height())
    );

    Some((Icon { name: Some(id.to_string()) }, paths, view_box))
}

/// Parse an SVG file and return icons ready for generation.
pub fn parse_sketch(content: &str) -> Result<Vec<(Icon, Vec<PathWithMode>, String)>> {
    let doc = roxmltree::Document::parse(content)
        .map_err(|e| anyhow::anyhow!("Failed to parse SVG: {}", e))?;

    let mut results = Vec::new();

    for node in doc.descendants() {
        if !node.is_element() || node.tag_name().name() != "g" {
            continue;
        }
        let id = match node.attribute("id") {
            Some(id) if is_icon_id(id) => id,
            _ => continue,
        };

        if let Some(processed) = process_icon_group(node, id) {
            results.push(processed);
        }
    }

    Ok(results)
}

/// Generate icons from an SVG file and save them to the output directory.
pub fn generate_icons_from_svg(
    content: &str,
    output_dir: &Path,
    options: &OptimizationOptions,
) -> Result<usize> {
    let icons = parse_sketch(content)?;

    fs::create_dir_all(output_dir)?;

    let mut count = 0;

    for (icon, paths, view_box) in &icons {
        let svg = icon_to_svg(icon, false, paths, options, view_box);

        if !svg.is_empty() {
            let filename = match &icon.name {
                Some(name) => format!("{}.svg", name),
                None => format!("icon_{}.svg", count),
            };
            let filepath = output_dir.join(&filename);
            fs::write(&filepath, svg)?;
            println!("Generated: {}", filename);
            count += 1;
        }
    }

    Ok(count)
}
