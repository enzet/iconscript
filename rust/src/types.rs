/// Represents a point in 2D space.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// Represents a scope with current drawing state.
#[derive(Debug, Clone)]
pub struct Scope {
    pub uniting: bool,
    pub width: f64,
    pub position: Point,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            uniting: true,
            width: 1.0,
            position: Point::new(0.0, 0.0),
        }
    }

    pub fn deep_copy(&self) -> Self {
        self.clone()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an icon with its name.
#[derive(Debug, Clone)]
pub struct Icon {
    pub name: Option<String>,
}

impl Icon {
    pub fn new() -> Self {
        Self { name: None }
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a path with its mode (union or subtract).
#[derive(Debug, Clone)]
pub struct PathWithMode {
    pub path: String,
    pub mode: bool, // True for union, false for subtract.
}
