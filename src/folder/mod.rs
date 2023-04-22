use std::{
    collections::VecDeque,
    fs::{DirEntry, ReadDir},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Folder {
    path: PathBuf,
}

impl Folder {
    pub fn new(path: PathBuf) -> Self {
        Folder { path }
    }

    /// Creates an iterator of the files in the folder only one level
    pub fn files(&self) -> Files {
        Files::new(&self.path, false)
    }

    /// Creates an deep iterator of the files in the folder and the
    /// files of the folders inside a the childs folders. The order of the 
    /// iteration may not be consisten
    /// For now we don't care about the order of iteration. Good to have: keep 
    /// the order of iteration using a stack of opened folder iterators
    pub fn files_deep(&self) -> Files {
        Files::new(&self.path, true)
    }
}

#[derive(Debug)]
pub struct Files {
    folder_iter: ReadDir,
    queue: VecDeque<PathBuf>,
    deep: bool,
}

impl Files {
    fn new(path: &PathBuf, deep: bool) -> Files {
        Files {
            folder_iter: std::fs::read_dir(path).expect("Can't read the inital Folder"),
            queue: VecDeque::new(),
            deep,
        }
    }
}

impl Iterator for Files {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_item) = self.folder_iter.next() {
            let e = next_item.expect("Can't read the next item");
            if e.path().is_dir() {
                if self.deep {
                    self.queue.push_front(e.path());
                }
                return self.next();
            }
            return Some(e);
        } else {
            if !self.queue.is_empty() {
                self.folder_iter = std::fs::read_dir(
                    self.queue
                        .pop_front()
                        .expect("Failed to get the next folder iterator"),
                )
                .expect("Failed to get the next folder iterator");
                return self.next();
            }
        }
        None
    }
}
