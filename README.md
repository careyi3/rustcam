# RustCAM

Welcome! This is the RustCAM library and app. This app and library are intended to be a set of tools for programmatically generating G-CODE for CNC machines using algorithms written in Rust. Niche, I know... Enjoy!

## Contour Plots

This is a binary that gets built from this code for generating G-CODE for abstract contour plots from raster data. It is based on an original Ruby version you can find [here](https://github.com/careyi3/contour_plots).

## Spotify Codes

***Under Construction: currently does nothing.***

This is a binary that gets built from this code for generating G-CODE for Spotify Code key chains generated from SVGs.

## Library

This is the main library code of RustCAM. Currently it uses some hard coded defaults to generate G-CODE for a simple series of linear toolpaths.

### Types

#### Point

A point on the X-Y plane. All points in this system are assumed to be at the same Z height.

```rust
pub struct Point {
    pub x: i32,
    pub y: i32,
}
```

#### Segment

A segment wraps the behaviour that differing kinds of movemenets within a toolpath generate different g-code.

```rust
pub trait Segment {
    fn generate_gcode(&self) -> String;
}
```

#### Line

A line defines a movement linearly between two points in the system.

```rust
pub struct Line {
    pub start: Point,
    pub end: Point,
}
```

#### Arc

An arc defines a curve moving between a start and end point of a certain radius and drawn in a certain direction (clockwise or anticlockwise).

```rust
pub struct Arc {
    pub start: Point,
    pub end: Point,
    pub radius: f32,
    pub direction: bool,
}
```

#### ToolPath

A toolpath wraps a series of connected segments together into a logical grouping in which a travel move can be performed to the starting point and then cutting moves can be performed one after another for each segment in order.

```rust
pub struct ToolPath {
    pub start: Point,
    pub end: Point,
    pub segments: Vec<Box<dyn Segment>>,
}
```

### Methods

#### generate_gcode

```rust
rustcam::generate_gcode(toolpaths: Vec<ToolPath>) -> Vec<String>
```

#### write_file

```rust
file_writer::write_file(path: String, lines: Vec<String>)
```
