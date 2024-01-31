use crate::types::{Path, Stroke};
use log::info;
use std::collections::HashMap;

pub fn generate_paths(stroke_hash: HashMap<i32, Vec<Stroke>>) -> Vec<Path> {
    let mut paths: Vec<Vec<Path>> = Default::default();

    let mut heights: Vec<&i32> = stroke_hash.keys().collect();
    heights.sort();

    let path_group_count = stroke_hash.keys().count();
    for h in heights {
        let strokes = &stroke_hash[h];
        let mut inner_paths: Vec<Path> = Default::default();
        for stroke in strokes {
            let path = Path::build_path_from_stroke(*stroke);
            inner_paths.push(path);
        }
        inner_paths.sort();
        paths.push(inner_paths);
    }

    let mut path_num = 1;
    let mut final_paths: Vec<Path> = vec![];
    for mut inner_paths in paths {
        let mut paths_found = 0;
        let mut paths_count: usize = inner_paths.as_slice().len();
        if paths_count > 0 {
            let mut current_path = inner_paths.pop().unwrap();
            while paths_count > 0 {
                let mut new_inner_paths: Vec<Path> = vec![];
                let mut matched = false;
                for path in inner_paths {
                    if !matched {
                        if current_path.end == path.start {
                            let strokes = current_path.strokes;
                            let to_append = path.strokes;
                            current_path = update_current_path(strokes, to_append);
                            matched = true;
                            continue;
                        }
                        if current_path.start == path.end {
                            let strokes = path.strokes;
                            let to_append = current_path.strokes;
                            current_path = update_current_path(strokes, to_append);
                            matched = true;
                            continue;
                        }
                        if current_path.start == path.start {
                            let strokes = reverse_strokes(path.strokes);
                            let to_append = current_path.strokes;
                            current_path = update_current_path(strokes, to_append);
                            matched = true;
                            continue;
                        }
                        if current_path.end == path.end {
                            let strokes = current_path.strokes;
                            let to_append = reverse_strokes(path.strokes);
                            current_path = update_current_path(strokes, to_append);
                            matched = true;
                            continue;
                        }
                    }
                    new_inner_paths.push(path);
                }
                if !matched {
                    paths_found += 1;
                    final_paths.push(current_path.clone());
                    if new_inner_paths.as_slice().len() > 0 {
                        current_path = new_inner_paths.pop().unwrap();
                    }
                }
                inner_paths = new_inner_paths;
                paths_count = inner_paths.as_slice().len();
            }
            paths_found += 1;
            final_paths.push(current_path);
        }

        info!("{}/{} - {}", path_num, path_group_count, paths_found);
        path_num += 1;
    }

    return final_paths;
}

fn update_current_path(mut strokes: Vec<Stroke>, mut to_append: Vec<Stroke>) -> Path {
    strokes.append(&mut to_append);
    let new_path = Path::build_path_from_strokes(strokes);
    return new_path;
}

fn reverse_strokes(strokes: Vec<Stroke>) -> Vec<Stroke> {
    let mut reversed: Vec<Stroke> = vec![];

    for stroke in strokes {
        let new_stroke =
            Stroke::build_stroke(stroke.end.x, stroke.end.y, stroke.start.x, stroke.start.y);
        reversed.insert(0, new_stroke)
    }

    return reversed;
}
