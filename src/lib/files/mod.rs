//! src/lib/files/mod.rs

use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::utils::get_provider_name;

/// Read the file in the path given
/// Check if contains our patterns
/// Returns a vector with name and path if true
pub fn read_file(paths: Vec<PathBuf>) -> Result<Vec<String>, Error> {
    println!("****************** OPEN SEARCH ******************\n");

    let mut report_text: Vec<String> = vec![];

    for path in paths {
        let file_path = path.display().to_string();
        println!("Processing file: {:?}", file_path);

        let contents = fs::read_to_string(&file_path)?;

        if contents.contains("expiringbalance") && !contents.contains("currentbalance") {
            let provider_name = get_provider_name(path);
            report_text.push(format!(
                "Possible error in provider: {:?}\nFile: {}\n",
                provider_name, file_path
            ));
        }
    }

    println!("\n****************** CLOSE SEARCH ******************");

    Ok(report_text)
}

/// Create report.txt file
/// With providers and operation duration
pub fn create_report_file(content: String, duration: Duration) -> Result<(), Error> {
    let duration_report = format!(
        "\n\n********************************************\nSearch done in: {:?}\n********************************************",
        duration
    );

    let path = Path::new("report.txt");
    let report = content + duration_report.as_str();

    let mut file = File::create(&path)?;

    file.write_all(report.as_bytes())?;

    println!("\n****************** Report created ******************");

    Ok(())
}
