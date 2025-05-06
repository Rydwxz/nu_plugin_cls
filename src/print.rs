use crate::{fs::DirList, parse::SArgs};

pub fn enum_list(list: &DirList, args: &SArgs) {
    for (i, entry) in list.iter().enumerate() {
        println!("{} {}", i, entry.file_name().to_string_lossy())
    }
}

pub fn selected_indexes(sel: Vec<i64>) {
    for idx in sel {
        println!("{}", idx)
    }
}
