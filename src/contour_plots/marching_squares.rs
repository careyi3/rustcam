use crate::types::Stroke;
use std::collections::HashMap;

const STROKE_OFFSETS: &'static [[[f32; 4]; 2]; 16] = &[
    [[-1.0; 4], [-1.0; 4]],                       // 0
    [[0.0, 0.5, 0.5, 1.0], [-1.0; 4]],            // 1
    [[0.5, 1.0, 1.0, 0.5], [-1.0; 4]],            // 2
    [[0.0, 0.5, 1.0, 0.5], [-1.0; 4]],            // 3
    [[0.5, 0.0, 1.0, 0.5], [-1.0; 4]],            // 4
    [[0.5, 1.0, 1.0, 0.5], [0.0, 0.5, 0.5, 0.0]], // 5
    [[0.5, 1.0, 0.5, 0.0], [-1.0; 4]],            // 6
    [[0.0, 0.5, 0.5, 0.0], [-1.0; 4]],            // 7
    [[0.0, 0.5, 0.5, 0.0], [-1.0; 4]],            // 8
    [[0.5, 1.0, 0.5, 0.0], [-1.0; 4]],            // 9
    [[0.0, 0.5, 0.5, 1.0], [0.5, 0.0, 1.0, 0.5]], // 10
    [[0.5, 0.0, 1.0, 0.5], [-1.0; 4]],            // 11
    [[0.0, 0.5, 1.0, 0.5], [-1.0; 4]],            // 12
    [[0.5, 1.0, 1.0, 0.5], [-1.0; 4]],            // 13
    [[0.0, 0.5, 0.5, 1.0], [-1.0; 4]],            // 14
    [[-1.0; 4], [-1.0; 4]],                       // 15
];

pub fn generate(
    by_height: HashMap<i32, Vec<String>>,
    points: HashMap<String, i32>,
    step: usize,
    min: i32,
) -> HashMap<i32, Vec<Stroke>> {
    let mut stroke_hash: HashMap<i32, Vec<Stroke>> = Default::default();
    let keys: Vec<&i32> = by_height.keys().collect();
    let max = **keys.iter().max().unwrap();

    for n in (min + 1..max + 1).step_by(step) {
        // TODO: Deal with remainders from step_by
        let mut strokes: HashMap<String, Stroke> = Default::default();
        if by_height.contains_key(&n) {
            let heights: Vec<&i32> = by_height.keys().filter(|x| *x >= &n).collect();
            let mut coord_list: Vec<String> = vec![];
            for height in heights {
                coord_list.append(&mut by_height[height].clone());
            }
            for coord in coord_list {
                let v: Vec<i32> = coord.split(":").map(|val| val.parse().unwrap()).collect();
                let (x, y) = (v[0], v[1]);
                let coords = [
                    (x, y),
                    (x + 1, y),
                    (x - 1, y),
                    (x, y + 1),
                    (x, y - 1),
                    (x + 1, y + 1),
                    (x - 1, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y + 1),
                ];

                for coord in coords {
                    let (x, y) = (coord.0, coord.1);
                    let case: usize = [
                        get_point_value(n, x, y, &points),
                        get_point_value(n, x + 1, y, &points),
                        get_point_value(n, x + 1, y + 1, &points),
                        get_point_value(n, x, y + 1, &points),
                    ]
                    .into_iter()
                    .fold(0, |acc, digit| (acc << 1) + digit);

                    let s = get_strokes(case, x as f32, y as f32);
                    for stroke in s {
                        let key = Stroke::key(&stroke);
                        if !strokes.contains_key(&key) {
                            strokes.insert(key, stroke);
                        }
                    }
                }
            }
        }
        stroke_hash.insert(n, strokes.into_values().collect());
    }

    return stroke_hash;
}

fn get_point_value(height: i32, x: i32, y: i32, points: &HashMap<String, i32>) -> usize {
    let key = format!("{}:{}", x, y);
    if points.contains_key(&key) {
        let val = points[&key];
        return if val >= height { 1 } else { 0 };
    }

    return 0;
}

fn get_strokes(case: usize, x: f32, y: f32) -> Vec<Stroke> {
    let mut data: Vec<Stroke> = vec![];
    for stroke_offset in STROKE_OFFSETS[case] {
        if stroke_offset[0] == -1.0 {
            break;
        }
        data.push(Stroke::build_stroke(
            x + stroke_offset[0],
            y + stroke_offset[1],
            x + stroke_offset[2],
            y + stroke_offset[3],
        ))
    }
    return data;
}
