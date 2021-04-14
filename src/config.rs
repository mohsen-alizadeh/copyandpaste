use dirs;
use std::fs;
use std::path::Path;

pub fn init() {
    let home = dirs::config_dir().unwrap();
    let dir = Path::new(&home).join("copyandpaste");

    if !dir.exists() {
        fs::create_dir(dir).expect("create dir");
    }
}
