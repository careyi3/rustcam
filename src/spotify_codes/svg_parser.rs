use svg::{self, parser::Event};

pub fn parse(path: String) -> Vec<(String, String, String)> {
    let mut values: Vec<(String, String, String)> = vec![];
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(path, _, attributes) => {
                if path == "rect" {
                    let vals = (
                        (*attributes["height"]).to_owned(),
                        (*attributes["x"]).to_owned(),
                        (*attributes["y"]).to_owned(),
                    );
                    values.push(vals);
                }
            }
            _ => {}
        }
    }
    return values;
}
