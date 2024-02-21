use rustcam;
use rustcam::types::{Arc, Line, Point, ToolPath};

pub fn generate_commands(values: Vec<(String, String, String)>) -> Vec<String> {
    let mut toolpaths: Vec<ToolPath> = vec![];

    let circle_point = Point { x: 40.0, y: 50.0 };
    let arc = Arc {
        start: circle_point,
        end: circle_point,
        radius: 20.0,
        direction: true,
    };
    let arc_toolpath = ToolPath {
        start: circle_point,
        end: circle_point,
        segments: vec![Box::new(arc)],
    };
    toolpaths.push(arc_toolpath);

    for idx in 1..values.len() - 1 {
        let height = values[idx].0.parse::<f32>().unwrap();
        let x = values[idx].1.parse::<f32>().unwrap();
        let y = values[idx].2.parse::<f32>().unwrap();
        let start = Point { x, y };
        let end = Point { x, y: y + height };
        let line = Line { start, end };
        let mut toolpath = ToolPath {
            start,
            end,
            segments: vec![],
        };
        toolpath.segments.push(Box::new(line));
        toolpaths.push(toolpath);
    }
    return rustcam::generate_gcode(toolpaths);
}
