use std::{env, fs, path::Path};

fn main() {
    let path = match env::args().next() {
        Some(p) => p,
        None => panic!("Cannot make folder without at least a name for it"),
    };

    let path = Path::new(path.as_str());

    if let true = path.is_dir() {
        let path = path.to_str().expect("Path provided must be valid UTF-8");
        panic!("{} already exists", path)
    };

    match fs::create_dir(path) {
        Ok(_) => (),
        Err(e) => panic!("Error creating new folder: {}", e),
    };

    match env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => panic!("Cannot change directory to new path due to {}", e),
    }
}
