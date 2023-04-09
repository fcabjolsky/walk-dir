use std::path::PathBuf;

fn main() {
    walk(std::env::current_dir().unwrap());
}

fn walk(path: PathBuf) {
    for f in std::fs::read_dir(path).unwrap() {
        let f = f.unwrap();
        println!("{}", f.path().to_str().unwrap());
        if f.path().is_dir() {
            walk(f.path());
        }
    }

}

