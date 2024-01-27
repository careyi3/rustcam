use crate::types::Stroke;
use log::info;
use std::collections::HashMap;

const STROKE_OFFSETS: &'static [[[i32; 4]; 2]; 16] = &[
    [[-1; 4], [-1; 4]],           // 0
    [[0, 1, 1, 2], [-1; 4]],      // 1
    [[1, 2, 2, 1], [-1; 4]],      // 2
    [[0, 1, 2, 1], [-1; 4]],      // 3
    [[1, 0, 2, 1], [-1; 4]],      // 4
    [[1, 2, 2, 1], [0, 1, 1, 0]], // 5
    [[1, 2, 1, 0], [-1; 4]],      // 6
    [[0, 1, 1, 0], [-1; 4]],      // 7
    [[0, 1, 1, 0], [-1; 4]],      // 8
    [[1, 2, 1, 0], [-1; 4]],      // 9
    [[0, 1, 1, 2], [1, 0, 2, 1]], // 10
    [[1, 0, 2, 1], [-1; 4]],      // 11
    [[0, 1, 2, 1], [-1; 4]],      // 12
    [[1, 2, 2, 1], [-1; 4]],      // 13
    [[0, 1, 1, 2], [-1; 4]],      // 14
    [[-1; 4], [-1; 4]],           // 15
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
    let mut visited: HashMap<String, i32> = Default::default();

    for n in (min..max - 1).step_by(step).rev() {
        let mut strokes: HashMap<String, Stroke> = Default::default();
        let offset = n + step as i32;
        let heights_ref: Vec<&i32> = by_height
            .keys()
            .filter(|x| *x > &n && *x < &offset)
            .collect();
        let mut heights: Vec<i32> = vec![];
        for h in heights_ref {
            heights.push(*h);
        }
        heights.sort();
        let mut coord_list: Vec<String> = vec![];
        for height in &heights {
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

                let s = get_strokes(case, x * 2, y * 2);
                for stroke in s {
                    let key = Stroke::key(&stroke);
                    if !strokes.contains_key(&key) {
                        if !visited.contains_key(&key) {
                            strokes.insert(key.clone(), stroke);
                            visited.insert(key.clone(), 1);
                        } else {
                            let val = visited.get_mut(&key).unwrap();
                            *val += 1;
                        }
                    }
                }
            }
        }
        stroke_hash.insert(n, strokes.into_values().collect());
        info!("{}/{}", n, max - 1);
    }

    return stroke_hash;
}

fn get_point_value(height: i32, x: i32, y: i32, points: &HashMap<String, i32>) -> usize {
    let key = format!("{}:{}", x, y);
    if points.contains_key(&key) {
        let val = points[&key];
        return if val > height { 1 } else { 0 };
    }

    return 0;
}

fn get_strokes(case: usize, x: i32, y: i32) -> Vec<Stroke> {
    let mut data: Vec<Stroke> = vec![];
    for stroke_offset in STROKE_OFFSETS[case] {
        if stroke_offset[0] == -1 {
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
