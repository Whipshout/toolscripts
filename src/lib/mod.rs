//! src/lib/mod.rs

use std::io::Error;
use std::time::Instant;

use crate::files::{create_report_file, read_file};
use crate::utils::find_scripts;

pub mod files;
pub mod utils;

/// Main function to run
/// Glue all pieces
pub fn run() -> Result<(), Error> {
    let start = Instant::now();

    let files_path = find_scripts();
    let report = read_file(files_path)?;

    let duration = start.elapsed();

    create_report_file(report.join("\n"), duration)?;

    Ok(())
}
