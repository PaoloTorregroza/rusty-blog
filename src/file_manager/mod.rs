use std::fs;

pub fn file_exists(path: &String) -> bool {
    let metadata = fs::metadata(path);
    metadata.is_ok_and(|el| el.is_file())
}
