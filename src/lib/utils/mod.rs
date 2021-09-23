//! src/lib/utils/mod.rs

use std::ffi::OsString;
use std::path::PathBuf;

use glob::glob;

/// Go through all the folders
/// Returns a vector with the wanted files path
pub fn find_scripts() -> Vec<PathBuf> {
    let files_wanted = vec![
        "./loyalty/src/**/ExtractBalance.ts",
        "./loyalty/src/**/ExtractAll.ts",
    ];

    files_wanted
        .into_iter()
        .map(|file| glob(file).unwrap().flatten())
        .flatten()
        .collect::<Vec<PathBuf>>()
}

/// Returns provider name from the path
pub fn get_provider_name(path: PathBuf) -> OsString {
    let mut provider_name = path.ancestors();

    OsString::from(provider_name.nth(2).unwrap().file_name().unwrap())
}
