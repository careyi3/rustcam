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

```rust
pub struct Point {
    pub x: i32,
    pub y: i32,
}
```

#### ToolPath

```rust
pub struct ToolPath {
    pub start: Point,
    pub end: Point,
    pub points: Vec<Point>,
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
