use std::{
    collections::VecDeque,
    fs::{DirEntry, ReadDir},
    path::PathBuf,
};

pub struct Folder {
    path: PathBuf,
}

impl Folder {
    pub fn new(path: PathBuf) -> Self {
        Folder { path }
    }

    pub fn files(self) -> Files {
        Files {
            folder_iter: std::fs::read_dir(self.path).expect("Can't read the inital Folder"),
            queue: VecDeque::new(),
            deep: false,
        }
    }

    pub fn files_deep(self) -> Files {
        Files {
            folder_iter: std::fs::read_dir(self.path).expect("Can't read the inital Folder"),
            queue: VecDeque::new(),
            deep: true,
        }
    }
}

pub struct Files {
    folder_iter: ReadDir,
    queue: VecDeque<PathBuf>,
    deep: bool,
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
