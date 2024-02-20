#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ToolPath {
    pub start: Point,
    pub end: Point,
    pub points: Vec<Point>,
}
