use crate::parse::SArgs;
use std::cmp::Ordering;
use std::fs::DirEntry;
use std::path::PathBuf;

pub struct DirList {
    inner: Vec<Dent>,
}
impl DirList {
    pub fn new(cwd: PathBuf, args: &SArgs) -> Result<Self, std::io::Error> {
        Ok(DirList {
            inner: DirList::get_dirlist(cwd, args, args.recursive())?,
        })
    }
    pub fn iter(&self) -> DirListIter {
        DirListIter {
            list: self,
            index: 0,
        }
    }
    pub fn nth(&self, i: i64) -> &str {
        self.inner[i as usize].pthstr()
    }
    fn get_dirlist(cwd: PathBuf, args: &SArgs, depth: i64) -> Result<Vec<Dent>, std::io::Error> {
        let mut list: Vec<Dent> = std::fs::read_dir(cwd)?
            .into_iter()
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    if let Ok(md) = entry.metadata() {
                        let ft = md.file_type();
                        if ft.is_dir() {
                            Some(Dent::Dir {
                                pthstr: entry.path().display().to_string(),
                                namestr: entry2name(&entry),
                                contents: if depth > 0 {
                                    if let Ok(dl) =
                                        DirList::get_dirlist(entry.path(), args, depth - 1)
                                    {
                                        Some(Box::new(dl))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                },
                            })
                        } else if ft.is_file() {
                            Some(Dent::File {
                                pthstr: entry.path().display().to_string(),
                                namestr: entry2name(&entry),
                            })
                        } else if ft.is_symlink() {
                            Some(Dent::Link {
                                pthstr: entry.path().display().to_string(),
                                namestr: entry2name(&entry),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        list.sort_by(|a, b| a.cmp(b));

        Ok(list)
    }
}

pub enum Dent {
    Dir {
        pthstr: String,
        namestr: String,
        contents: Option<Box<Vec<Dent>>>,
        // modified: String,
        // acessed: String,
    },
    Link {
        pthstr: String,
        namestr: String,
    },
    File {
        pthstr: String,
        namestr: String,
        // ft: String,
        // modified: String,
        // acessed: String,
    },
}
impl Dent {
    pub fn pthstr(&self) -> &str {
        match self {
            Dent::Dir { pthstr, .. } => &pthstr,
            Dent::File { pthstr, .. } => &pthstr,
            Dent::Link { pthstr, .. } => &pthstr,
        }
    }
    pub fn namestr(&self) -> &str {
        match self {
            Dent::Dir { namestr, .. } => &namestr,
            Dent::File { namestr, .. } => &namestr,
            Dent::Link { namestr, .. } => &namestr,
        }
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.namestr().cmp(other.namestr())
    }
}

pub struct DirListIter<'a> {
    list: &'a DirList,
    index: usize,
}
impl<'a> Iterator for DirListIter<'a> {
    type Item = &'a Dent;
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

fn entry2name(input: &DirEntry) -> String {
    match input.file_name().to_os_string().into_string() {
        Ok(s) => s,
        Err(_) => "<<invalid filename>>".to_string(),
    }
}
