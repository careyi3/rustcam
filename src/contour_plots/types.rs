pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Stroke {
    pub start: Point,
    pub end: Point,
}

impl Stroke {
    pub fn key(args: &Stroke) -> String {
        return format!(
            "{}:{}:{}:{}",
            args.start.x, args.start.y, args.end.x, args.end.y
        );
    }
}

pub struct Path {
    pub start: Point,
    pub end: Point,
    pub strokes: Vec<Stroke>,
}

pub fn build_stroke(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Stroke {
    let start = Point {
        x: start_x,
        y: start_y,
    };
    let end = Point { x: end_x, y: end_y };

    return Stroke { start, end };
}
