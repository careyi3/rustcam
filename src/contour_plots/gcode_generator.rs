use crate::types::Path;
use rustcam;
use rustcam::types::{Point, ToolPath};

pub fn generate_commands(paths: Vec<Path>) -> Vec<String> {
    let mut toolpaths: Vec<ToolPath> = vec![];
    for path in paths {
        let mut toolpath = ToolPath {
            start: Point {
                x: path.start.x,
                y: path.start.y,
            },
            end: Point {
                x: path.end.x,
                y: path.end.y,
            },
            points: vec![],
        };
        for stroke in path.strokes {
            toolpath.points.push(Point {
                x: stroke.start.x,
                y: stroke.start.y,
            });
            toolpath.points.push(Point {
                x: stroke.end.x,
                y: stroke.end.y,
            });
        }
        toolpaths.push(toolpath);
    }
    return rustcam::generate_gcode(toolpaths);
}
