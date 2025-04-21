use crate::fs::DirList;

pub fn enum_list(list: DirList) {
    for (i, entry) in list.iter().enumerate() {
        println!("{} {}", i, entry.file_name().to_string_lossy())
    }
}
