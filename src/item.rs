use skim::prelude::*;
use std::fs::File;
use std::io::Read as _;
use yaml_rust::YamlLoader;

struct Item {
    title: String,
    solution: String,
}

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.title)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.solution)
    }
}

impl Item {
    fn new(title: String, solution: String) -> Item {
        Item {
            title: title,
            solution: solution,
        }
    }
}

pub fn load_from_yaml(filepath: &str, tx: SkimItemSender) {
    let mut file = File::open(filepath).expect("open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("read file");
    let yaml = YamlLoader::load_from_str(&content).unwrap();

    let records = &yaml[0];

    let mut i = 0;

    while !records[i].is_badvalue() {
        let record = &records[i];
        let title = record["title"].as_str().unwrap().to_string();
        let solution = record["solution"].as_str().unwrap().to_string();

        let _ = tx.send(Arc::new(Item::new(title, solution)));

        i += 1;
    }
}
