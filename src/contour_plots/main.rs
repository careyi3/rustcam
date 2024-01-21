mod file_parser;
mod marching_squares;
mod types;

fn main() {
    let data = file_parser::read("./data/test2");
    let (by_height, points) = file_parser::parse(data);

    let strokes = marching_squares::generate(by_height, points);

    println!("G90 G94");
    println!("G17");
    println!("G21");
    println!("G28 G91 Z0");
    println!("G90");
    println!("S5000 M3");
    for (_, stroke) in &strokes {
        println!("G0 F400");
        println!("Z1");
        println!("X{} Y{}", stroke.start.x, stroke.start.y);
        println!("Z-0.4");
        println!("G1 F200");
        println!("X{} Y{}", stroke.end.x, stroke.end.y);
    }
    println!("G0 F400");
    println!("Z2");
    println!("G28 G91 Z0");
    println!("G90");
    println!("G28 G91 X0 Y0");
    println!("G90");
    println!("M5");
    println!("M30");
}