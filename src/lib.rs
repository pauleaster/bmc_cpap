
use glob::{GlobError, glob};
use std::path::PathBuf;


fn get_data_filenames(data_directory: &str) -> Result<Vec<PathBuf>, GlobError> {

    let mut file_list: Vec<PathBuf> = vec![];
    let expanded_path = shellexpand::tilde(data_directory);
    let pattern : PathBuf = [&expanded_path, "*.[0-9][0-9][0-9]"].iter().collect();
    let mut err: Option<GlobError> = None;

    for entry in glob(pattern.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {file_list.push(path)},
            
            Err(e) => {err = Some(e);
                                break;},
        };
    }
    match err {
        Some(e) => Err(e),
        None => {
            file_list.sort_unstable();
            Ok(file_list)
        }
    }
}