#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

    pub fn build_stroke(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Stroke {
        let start = Point {
            x: start_x,
            y: start_y,
        };
        let end = Point { x: end_x, y: end_y };

        return Stroke { start, end };
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Path {
    pub start: Point,
    pub end: Point,
    pub strokes: Vec<Stroke>,
}

impl Path {
    pub fn build_path_from_stroke(stroke: Stroke) -> Path {
        let strokes: Vec<Stroke> = vec![stroke];

        return Path {
            strokes,
            start: stroke.start,
            end: stroke.end,
        };
    }

    pub fn build_path_from_strokes(strokes: Vec<Stroke>) -> Path {
        let strokes_clone = strokes.clone();
        let start = strokes.first().unwrap();
        let end = strokes.last().unwrap();
        return Path {
            strokes: strokes_clone,
            start: start.start,
            end: end.end,
        };
    }
}
