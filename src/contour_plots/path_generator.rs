use crate::types::{Path, Stroke};
use std::collections::HashMap;

pub fn generate_paths(stroke_hash: HashMap<i32, Vec<Stroke>>) -> Vec<Path> {
    let mut paths: Vec<Vec<Path>> = Default::default();

    for (_, strokes) in stroke_hash {
        let mut inner_paths: Vec<Path> = Default::default();
        for stroke in strokes {
            let path = Path::build_path_from_stroke(stroke);
            inner_paths.push(path);
        }
        paths.push(inner_paths);
    }

    let mut final_paths: Vec<Path> = vec![];
    for mut inner_paths in paths {
        let mut unmatched_count: usize = 0;
        let mut paths_count: usize = inner_paths.as_slice().len();
        while unmatched_count < paths_count && paths_count > 0 {
            let current_path = inner_paths.pop().unwrap();
            let mut paths: Vec<Path> = vec![];

            let mut matched = false;
            for path in inner_paths {
                if !matched {
                    if path.end == current_path.start {
                        paths = insert_new_path(paths, path.strokes, current_path.strokes.clone());
                        matched = true;
                        continue;
                    }
                    if path.start == current_path.end {
                        paths = insert_new_path(paths, current_path.strokes.clone(), path.strokes);
                        matched = true;
                        continue;
                    }
                    if path.start == current_path.start {
                        paths = insert_new_path(
                            paths,
                            reverse_strokes(path.strokes),
                            current_path.strokes.clone(),
                        );
                        matched = true;
                        continue;
                    }
                    if path.end == current_path.end {
                        paths = insert_new_path(
                            paths,
                            path.strokes,
                            reverse_strokes(current_path.strokes.clone()),
                        );
                        matched = true;
                        continue;
                    }
                }
                paths.push(path);
            }
            if matched {
                unmatched_count = 0;
            } else {
                paths.push(current_path);
                unmatched_count += 1;
            }
            inner_paths = paths;
            paths_count = inner_paths.as_slice().len();
        }
        final_paths.append(&mut inner_paths);
    }

    return final_paths;
}

fn insert_new_path(
    mut paths: Vec<Path>,
    mut strokes: Vec<Stroke>,
    mut strokes_to_append: Vec<Stroke>,
) -> Vec<Path> {
    strokes.append(&mut strokes_to_append);
    let new_path = Path::build_path_from_strokes(strokes);
    paths.insert(0, new_path);
    return paths;
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
