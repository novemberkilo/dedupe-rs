use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::data::{Error, FileData};

pub fn lookup_by_size(store: String) -> Result<Vec<Vec<FileData>>, Error> {
    let size_map = &mut HashMap::<u64, Vec<FileData>>::new();
    let store_path = Path::new(&store);
    if !store_path.is_dir() {
        Err(Error::ErrorMessage(format!(
            "Directory {} does not exist.",
            store
        )))
    } else {
        lookup_by_size_prime(store_path, size_map)?;
        size_map.retain(|_, v| v.len() > 1);
        Ok(size_map.values().cloned().collect())
    }
}

pub fn lookup_by_size_prime(
    dir: &Path,
    size_map: &mut HashMap<u64, Vec<FileData>>,
) -> Result<(), Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            lookup_by_size_prime(&path, size_map)?;
        } else {
            let metadata = fs::metadata(path.clone())?;
            let file_size = metadata.len();

            let e = size_map.entry(file_size).or_insert(vec![]);
            e.push(FileData { path });
        }
    }
    Ok(())
}

// one possible implementation of walking a directory only visiting files
//fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
//    if dir.is_dir() {
//        for entry in fs::read_dir(dir)? {
//            let entry = entry?;
//            let path = entry.path();
//            if path.is_dir() {
//                visit_dirs(&path, cb)?;
//            } else {
//                cb(&entry);
//            }
//        }
//    }
//    Ok(())
//}
