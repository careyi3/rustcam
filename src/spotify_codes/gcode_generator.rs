use rustcam;
use rustcam::types::{Point, ToolPath};

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
        points: vec![],
    };
    for point in points {
        toolpath.points.push(Point {
            x: point.x,
            y: point.y,
        });
    }
    toolpaths.push(toolpath);
    return rustcam::generate_gcode(toolpaths);
}
