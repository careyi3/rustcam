use std::{collections::HashMap, fs};

pub fn read(path: &str) -> Vec<Vec<i32>> {
    let contents: String =
        fs::read_to_string(path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    let mut data: Vec<Vec<i32>> = vec![];

    for line in lines.iter() {
        let values: Vec<i32> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        data.push(values);
    }

    return data;
}

pub fn parse(data: Vec<Vec<i32>>) -> (HashMap<i32, Vec<String>>, HashMap<String, i32>) {
    let mut by_height: HashMap<i32, Vec<String>> = Default::default();
    let mut points: HashMap<String, i32> = Default::default();

    for (y, line) in data.iter().enumerate() {
        for (x, val) in (0i32..).zip(line) {
            if by_height.contains_key(&val) {
                by_height.get_mut(&val).unwrap().push(format!("{x}:{y}"));
            } else {
                let mut vec: Vec<String> = Vec::new();
                vec.push(format!("{x}:{y}"));
                by_height.insert(*val, vec);
            }
            points.insert(format!("{x}:{y}"), *val);
        }
    }

    return (by_height, points);
}
