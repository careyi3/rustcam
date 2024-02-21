use rustcam;
use rustcam::types::{Arc, Point, ToolPath};

pub fn generate_commands(points: Vec<Point>) -> Vec<String> {
    let mut toolpaths: Vec<ToolPath> = vec![];
    let mut toolpath = ToolPath {
        start: Point {
            x: points.first().unwrap().x,
            y: points.first().unwrap().y,
        },
        end: Point {
            x: points.first().unwrap().x,
            y: points.first().unwrap().y,
        },
        segments: vec![],
    };
    for point in points {
        let start = Point {
            x: point.x,
            y: point.y,
        };
        let end = Point {
            x: point.x,
            y: point.y,
        };
        let arc = Arc {
            start,
            end,
            radius: 1.2,
            direction: true,
        };
        toolpath.segments.push(Box::new(arc));
    }
    toolpaths.push(toolpath);
    return rustcam::generate_gcode(toolpaths);
}
