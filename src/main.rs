mod folder;
use folder::Folder;

fn main() {
    let folder = Folder::new(std::env::current_dir().unwrap());
    for file in folder.files() {
        println!("{}", file.path().to_str().unwrap());
    }
}


