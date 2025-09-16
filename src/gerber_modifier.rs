/* src/gerber_modifier.rs */

use md5::{Digest, Md5};
use rand::Rng;
use regex::Regex;
use std::io;

/// Convert D-code format for KiCad files.
/// This version uses simpler and more robust logic,
/// thus avoiding regex panic.
pub fn convert_kicad_aperture_format(content: String) -> String {
    let lines: Vec<&str> = content.split('\n').collect();
    let mut result_lines = Vec::new();
    let aperture_regex = Regex::new(r"(D\d{2,4}\*)").unwrap();

    for line in lines {
        if line.contains("%ADD") || line.contains("G54D") {
            result_lines.push(line.to_string());
        } else {
            // Only perform replacement on safe lines
            let modified_line = aperture_regex.replace_all(line, "G54$1");
            result_lines.push(modified_line.to_string());
        }
    }

    result_lines.join("\n")
}

/// Add a hash aperture to the Gerber file as a file fingerprint.
pub fn add_hash_aperture_to_gerber(
    content: String,
    is_imported_pcb_doc: bool,
) -> Result<String, io::Error> {
    if content.len() > 30_000_000 {
        return Ok(content);
    }

    let lines: Vec<&str> = content.split('\n').collect();
    let aperture_regex = Regex::new(r"^%ADD(\d{2,4})\D.*").unwrap();

    let mut aperture_definitions = Vec::new();
    let mut aperture_numbers = Vec::new();

    // Step 1a: Scan and collect existing aperture definitions
    for line in lines.iter().take(200) {
        if let Some(caps) = aperture_regex.captures(line) {
            if let Ok(num) = caps[1].parse::<u32>() {
                aperture_definitions.push(line.to_string());
                aperture_numbers.push(num);
            }
        }
    }

    // If no apertures are found, or too few, skip processing
    if aperture_definitions.len() < 5 {
        return Ok(content);
    }

    // Step 1b: Smartly choose injection point and template
    let mut rng = rand::rng();
    let selection_index = rng.random_range(5..aperture_definitions.len());

    let selected_aperture_template = aperture_definitions[selection_index].clone();
    let target_number = aperture_numbers[selection_index]; // This is the aperture number to be “freed up”

    // Step 2: Globally renumber to “make room” for the injection point
    let renumber_regex = Regex::new(r"(?m)^(%ADD|G54D)(\d{2,4})").unwrap();
    let renumbered_content = renumber_regex
        .replace_all(&content, |caps: &regex::Captures| {
            let prefix = &caps[1];
            let number: u32 = caps[2].parse().unwrap_or(0);

            if number >= target_number {
                format!("{}{}", prefix, number + 1)
            } else {
                caps[0].to_string()
            }
        })
        .to_string();

    // Step 3: Generate the “magic number” size
    let hash_content = if is_imported_pcb_doc {
        format!("494d{}", renumbered_content)
    } else {
        renumbered_content.clone()
    };

    let mut hasher = Md5::new();
    hasher.update(hash_content.as_bytes());
    let hash_result = hasher.finalize();
    let hash_hex = format!("{:x}", hash_result);

    let last_two_hex = &hash_hex[hash_hex.len() - 2..];
    let hash_number = u32::from_str_radix(last_two_hex, 16).unwrap_or(0) % 100;
    let hash_suffix = format!("{:02}", hash_number);

    let base_size: f64 = rng.random_range(0.0..1.0);
    let size_with_hash = format!("{:.2}{}", base_size, hash_suffix);
    let final_size = if size_with_hash.parse::<f64>().unwrap_or(0.0) == 0.0 {
        "0.0100".to_string()
    } else {
        size_with_hash
    };

    // Step 4a: Create the new hash aperture definition line
    let size_regex = Regex::new(r",([\d.]+)").unwrap();
    let hash_aperture = if let Some(cap) = size_regex.captures(&selected_aperture_template) {
        let original_size_part = cap.get(0).unwrap().as_str();
        let new_size_part = format!(",{}", final_size);
        selected_aperture_template.replace(original_size_part, &new_size_part)
    } else {
        // If no size is found in the template (unlikely), create a default one
        format!("%ADD{}C,{}*%", target_number, final_size)
    };
    // Ensure it uses the target number we freed up
    let final_hash_aperture = Regex::new(&format!("ADD{}", aperture_numbers[selection_index]))
        .unwrap()
        .replace(&hash_aperture, &format!("ADD{}", target_number));

    // Step 4b: Smartly insert the new line into the renumbered content
    let insertion_anchor = format!("%ADD{}", target_number - 1);
    let mut result_lines: Vec<String> = Vec::new();
    let mut inserted = false;

    for line in renumbered_content.split('\n') {
        result_lines.push(line.to_string());
        if line.starts_with(&insertion_anchor) {
            result_lines.push(final_hash_aperture.to_string());
            inserted = true;
        }
    }

    if !inserted {
        let mut final_lines: Vec<String> = Vec::new();
        let mut mo_found = false;
        for line in renumbered_content.split('\n') {
            if !mo_found && line.starts_with("%MO") {
                mo_found = true;
            } else if mo_found && !inserted && (line.starts_with("%LP") || line.starts_with('G')) {
                final_lines.push(final_hash_aperture.to_string());
                inserted = true;
            }
            final_lines.push(line.to_string());
        }
        return Ok(final_lines.join("\n"));
    }

    Ok(result_lines.join("\n"))
}
