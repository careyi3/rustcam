use types::ToolPath;

pub mod file_writer;
pub mod types;

pub fn generate_gcode(toolpaths: Vec<ToolPath>) -> Vec<String> {
    let mut commands: Vec<String> = vec![];
    commands.push("G90 G94".to_string());
    commands.push("G17".to_string());
    commands.push("G21".to_string());
    commands.push("G28 G91 Z0".to_string());
    commands.push("G90".to_string());
    commands.push("S5000 M3".to_string());
    for path in toolpaths {
        let start = path.start;
        commands.push("G0 F400".to_string());
        commands.push("Z1".to_string());
        commands.push(format!("X{} Y{}", start.x, start.y));
        commands.push("Z-0.4".to_string());
        commands.push("G1 F200".to_string());
        for segment in path.segments {
            let g_code = segment.generate_gcode();
            commands.push(g_code);
        }
    }
    commands.push("G0 F400".to_string());
    commands.push("Z2".to_string());
    commands.push("G28 G91 Z0".to_string());
    commands.push("G90".to_string());
    commands.push("G28 G91 X0 Y0".to_string());
    commands.push("G90".to_string());
    commands.push("M5".to_string());
    commands.push("M30".to_string());
    return commands;
}
