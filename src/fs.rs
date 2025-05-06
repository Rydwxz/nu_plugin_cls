use crate::parse::SArgs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

enum Dent {
    Dir(DirEntry, Box<Vec<Dent>>),
    Link(Box<Dent>),
    File(DirEntry),
}
pub struct DirList {
    inner: Vec<Dent>,
}
impl DirList {
    pub fn new(cwd: PathBuf, args: &SArgs) -> Self {
        let (dirs, files) = match std::fs::read_dir(cwd) {
            Err(_) => (vec![], vec![]),
            Ok(rdir) => rdir
                .into_iter()
                .fold((vec![], vec![]), |acc, entry| entry.map(|e| {
                    if e.
                })),
        };
        // .into_iter() .skip(1) // skip cwd
        // .filter_map(|e| e.ok())
        // .fold(Vec::new(), |mut acc, entry| {
        //     acc.push(entry);
        //     acc
        // });

        DirList { inner: list }
    }
    pub fn iter(&self) -> DirListIter {
        DirListIter {
            list: self,
            index: 0,
        }
    }
    pub fn nth(&self, i: i64) -> Path {
        self.inner[i as usize].path()
    }
}

pub struct DirListIter<'a> {
    list: &'a DirList,
    index: usize,
}
impl<'a> Iterator for DirListIter<'a> {
    type Item = &'a DirEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.inner.len() {
            let res = &self.list.inner[self.index];
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}
