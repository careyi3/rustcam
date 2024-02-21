use env_logger::{Builder, Env, Target};
use log::info;
use rustcam;
use rustcam::file_writer;
use rustcam::types::Point;
use std::env;
use std::time::Instant;

mod gcode_generator;

fn main() {
    init_logging();
    let now = Instant::now();
    {
        let args: Vec<String> = env::args().collect();

        let (_, output) = parse_args(args);

        let points: Vec<Point> = vec![Point { x: 0.0, y: 0.0 }];

        let commands = gcode_generator::generate_commands(points);

        file_writer::write_file(output + ".nc", commands);
    }
    let elapsed = now.elapsed();
    info!("Runtime: {:.2?}", elapsed);
}

fn parse_args(args: Vec<String>) -> (String, String) {
    let mut input = "./data/spotify_codes/test".to_string();
    let mut output_prefix = "./data/spotify_codes/test".to_string();

    if args.get(1) != None {
        input = args[1].clone();
        output_prefix = args[1].clone();
    }

    return (input, output_prefix);
}

fn init_logging() {
    let mut builder = Builder::from_env(Env::default().default_filter_or("info"));
    builder.target(Target::Stdout);
    builder.init();
}
