use crate::types::Path;
use rustcam;
use rustcam::types::{Line, Point, ToolPath};

pub fn generate_commands(paths: Vec<Path>) -> Vec<String> {
    let mut toolpaths: Vec<ToolPath> = vec![];
    for path in paths {
        let mut toolpath = ToolPath {
            start: Point {
                x: path.start.x as f32,
                y: path.start.y as f32,
            },
            end: Point {
                x: path.end.x as f32,
                y: path.end.y as f32,
            },
            segments: vec![],
        };
        for stroke in path.strokes {
            let start = Point {
                x: stroke.start.x as f32,
                y: stroke.start.y as f32,
            };
            let end = Point {
                x: stroke.end.x as f32,
                y: stroke.end.y as f32,
            };
            let line = Line { start, end };
            toolpath.segments.push(Box::new(line));
        }
        toolpaths.push(toolpath);
    }
    return rustcam::generate_gcode(toolpaths);
}
