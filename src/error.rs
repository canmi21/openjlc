/* src/error.rs */

use crate::cli::get_input_file_path;
use crate::reporter::generate_report;

pub fn report_error() {
    if let Some(file_path) = get_input_file_path() {
        generate_report(file_path);
    }
}
