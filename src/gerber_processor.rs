/* src/gerber_processor.rs */

use crate::d_code_normalization;
use crate::fingerprint_helpers;
use std::io;

pub fn normalize_kicad_d_codes(gerber_data: String) -> String {
    d_code_normalization::process_d_codes(gerber_data)
}

pub fn inject_fingerprint_into_gerber(
    file_contents: String,
    is_foreign_board_file: bool,
) -> Result<String, io::Error> {
    let content_lines: Vec<&str> = file_contents.split('\n').collect();
    let (existing_definitions, existing_aperture_ids) =
        fingerprint_helpers::scan_for_aperture_definitions(&content_lines);
    if existing_definitions.len() < 6 {
        return Ok(file_contents);
    }
    let (template_definition_line, injection_aperture_id, original_template_id) =
        fingerprint_helpers::select_injection_template(
            &existing_definitions,
            &existing_aperture_ids,
        );
    let content_with_shifted_ids =
        fingerprint_helpers::renumber_apertures(&file_contents, injection_aperture_id);
    let final_dimension_str = fingerprint_helpers::generate_hashed_dimension(
        &content_with_shifted_ids,
        is_foreign_board_file,
    );
    let final_fingerprint_line = fingerprint_helpers::create_fingerprint_aperture_line(
        &template_definition_line,
        injection_aperture_id,
        original_template_id,
        &final_dimension_str,
    );
    let final_content = fingerprint_helpers::insert_new_aperture_line(
        &content_with_shifted_ids,
        &final_fingerprint_line,
        injection_aperture_id,
    );
    Ok(final_content)
}
