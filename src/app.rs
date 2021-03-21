use std::collections::HashMap;
use std::fs;
use std::path::Path;

extern crate sha1;

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

pub fn lookup_by_hash(
    duplicates_by_size: &Vec<Vec<FileData>>
) -> Result<Vec<Vec<FileData>>, Error> {
    let mut duplicates: Vec<Vec<FileData>> = vec![];

    for possible_duplicates in duplicates_by_size.iter() {
        let sha_map = &mut HashMap::<String, Vec<FileData>>::new();

        for candidate in possible_duplicates.iter() {
            let buffer: Vec<u8> = fs::read(candidate.path.clone())?;
            let sha1 = sha1::Sha1::from(buffer).digest().to_string();
            let e = sha_map.entry(sha1).or_insert(vec![]);
            e.push(candidate.clone());
        }
        sha_map.retain(|_sha1, candidates| candidates.len() > 1);
        for val in sha_map.values() {
            duplicates.push(val.clone())
        }
    }
    Ok(duplicates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_lookup_by_size_prime() {
        let size_map = &mut HashMap::<u64, Vec<FileData>>::new();
        let store = "fixtures/mangled";
        let store_path = Path::new(&store);

        lookup_by_size_prime(store_path, size_map).unwrap();
        assert!(size_map.contains_key(&7));
        let (_k, res) = size_map.get_key_value(&7).unwrap();
        let mut files: Vec<_> = res.iter().map(|v| v.path.to_string_lossy()).collect();
        files.sort();
        assert!(files == vec!["fixtures/mangled/hello", "fixtures/mangled/world"])
    }

    #[test]
    fn test_lookup_by_size() {
        let store = "fixtures/mangled".to_string();
        let results = lookup_by_size(store).unwrap();
        let hello_world = vec![
            FileData {
                path: Path::new("fixtures/mangled/hello").to_path_buf(),
            },
            FileData {
                path: Path::new("fixtures/mangled/world").to_path_buf(),
            },
        ];
        let world_hello = vec![
            FileData {
                path: Path::new("fixtures/mangled/world").to_path_buf(),
            },
            FileData {
                path: Path::new("fixtures/mangled/hello").to_path_buf(),
            },
        ];
        assert!(results.contains(&hello_world) || results.contains(&world_hello))
    }

    #[test]
    fn test_lookup_by_hash() {
        let store = "fixtures/mangled".to_string();
        let results = lookup_by_size(store).unwrap();
        let better_results = lookup_by_hash(&results).unwrap();
        assert!(better_results.len() == 3)
    }
}
