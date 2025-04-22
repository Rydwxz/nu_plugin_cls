use crate::parse::SArgs;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub fn walk(cwd: PathBuf, args: &SArgs) -> DirList {
    let wd = WalkDir::new(cwd);
    let wd = wd.max_depth(args.recursive as usize);
    let wd = wd.sort_by_file_name();
    let list = wd
        .into_iter()
        .skip(1) // skip cwd
        .filter_map(|e| e.ok())
        .fold(Vec::new(), |mut acc, entry| {
            acc.push(entry);
            acc
        });

    DirList { inner: list }
}

pub struct DirList {
    inner: Vec<DirEntry>,
}
impl DirList {
    pub fn iter(&self) -> DirListIter {
        DirListIter {
            list: self,
            index: 0,
        }
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
