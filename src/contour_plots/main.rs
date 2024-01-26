use env_logger::{Builder, Env, Target};
use log::info;
use std::env;
use std::time::Instant;

mod file_parser;
mod file_writer;
mod marching_squares;
mod path_generator;
mod types;

fn main() {
    init_logging();
    let now = Instant::now();
    {
        let args: Vec<String> = env::args().collect();

        let (step, min, input, output_prefix) = parse_args(args);

        info!("Reading input data...");
        let data = file_parser::read(&input);
        let (by_height, points) = file_parser::parse(data);

        info!("Processing contours...");
        let strokes_hash = marching_squares::generate(by_height, points, step, min);
        info!("Num strokes: {}", strokes_hash.values().flatten().count());

        info!("Stitching Paths...");
        let paths = path_generator::generate_paths(strokes_hash);
        info!("Num paths: {}", paths.as_slice().len());

        let mut commands: Vec<String> = vec![];
        commands.push("G90 G94".to_string());
        commands.push("G17".to_string());
        commands.push("G21".to_string());
        commands.push("G28 G91 Z0".to_string());
        commands.push("G90".to_string());
        commands.push("S5000 M3".to_string());
        for path in paths {
            let start = path.start;
            commands.push("G0 F400".to_string());
            commands.push("Z1".to_string());
            commands.push(format!("X{} Y{}", start.x, start.y));
            commands.push("Z-0.4".to_string());
            commands.push("G1 F200".to_string());
            for stroke in path.strokes {
                commands.push(format!("X{} Y{}", stroke.end.x, stroke.end.y));
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

        file_writer::write_file(output_prefix + ".nc", commands);
    }
    let elapsed = now.elapsed();
    info!("Runtime: {:.2?}", elapsed);
}

fn parse_args(args: Vec<String>) -> (usize, i32, String, String) {
    let mut step = 1;
    let mut min = 1;
    let mut input = "test".to_string();
    let mut output_prefix = "test".to_string();

    if args.get(1) != None {
        let step_arg = args[1].parse::<usize>();
        if step_arg.is_ok() {
            step = step_arg.unwrap();
        }
    }

    if args.get(2) != None {
        let min_arg = args[2].parse::<i32>();
        if min_arg.is_ok() {
            min = min_arg.unwrap();
        }
    }

    if args.get(3) != None {
        input = args[3].clone();
        output_prefix = args[3].clone();
    }

    return (step, min, input, output_prefix);
}

fn init_logging() {
    let mut builder = Builder::from_env(Env::default().default_filter_or("info"));
    builder.target(Target::Stdout);
    builder.init();
}
