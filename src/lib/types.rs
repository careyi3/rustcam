#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub trait Segment {
    fn generate_gcode(&self) -> String;
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Segment for Line {
    fn generate_gcode(&self) -> String {
        return format!("X{} Y{}", self.end.x, self.end.y);
    }
}

pub struct Arc {
    pub start: Point,
    pub end: Point,
    pub radius: f32,
    pub direction: bool,
}

impl Segment for Arc {
    fn generate_gcode(&self) -> String {
        if self.direction {
            return format!(
                "G2 X{} Y{} I{} J0 F200",
                self.end.x, self.end.y, self.radius
            );
        }
        return format!(
            "G3 X{} Y{} I{} J0 F200",
            self.end.x, self.end.y, self.radius
        );
    }
}

pub struct ToolPath {
    pub start: Point,
    pub end: Point,
    pub segments: Vec<Box<dyn Segment>>,
}
