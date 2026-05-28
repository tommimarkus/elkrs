#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    pub fn left(&self) -> f64 {
        self.origin.x
    }

    pub fn right(&self) -> f64 {
        self.origin.x + self.size.width
    }

    pub fn top(&self) -> f64 {
        self.origin.y
    }

    pub fn bottom(&self) -> f64 {
        self.origin.y + self.size.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Insets {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Insets {
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn all(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_rectangles_intersect() {
        let left = Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let right = Rect::new(Point::new(5.0, 5.0), Size::new(10.0, 10.0));

        assert!(left.intersects(&right));
    }

    #[test]
    fn edge_touching_rectangles_do_not_intersect() {
        let left = Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let right = Rect::new(Point::new(10.0, 0.0), Size::new(10.0, 10.0));

        assert!(!left.intersects(&right));
    }

    #[test]
    fn insets_all_sets_each_side() {
        let insets = Insets::all(3.0);

        assert_eq!(insets.top, 3.0);
        assert_eq!(insets.right, 3.0);
        assert_eq!(insets.bottom, 3.0);
        assert_eq!(insets.left, 3.0);
    }
}
