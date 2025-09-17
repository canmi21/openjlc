/* src/d_code_normalization.rs */

// This module contains the implementation for D-code normalization.
pub fn process_d_codes(gerber_data: String) -> String {
    let input_lines: Vec<&str> = gerber_data.split('\n').collect();
    let mut processed_lines = Vec::with_capacity(input_lines.len());

    for current_line in input_lines {
        // Skip lines that are already definitions or use G54D codes.
        if current_line.contains("%ADD") || current_line.contains("G54D") {
            processed_lines.push(current_line.to_string());
            continue;
        }

        let mut updated_line = String::with_capacity(current_line.len() + 10);
        let mut last_pos = 0;
        let mut search_pos = 0;

        // Manually search for D-codes like D10*, D123*, etc.
        while let Some(d_pos) = current_line[search_pos..].find('D') {
            let absolute_d_pos = search_pos + d_pos;
            let suffix = &current_line[absolute_d_pos + 1..];
            let num_len = suffix.chars().take_while(|c| c.is_ascii_digit()).count();

            // Check if the number has 2-4 digits and is followed by a '*'.
            if (2..=4).contains(&num_len)
                && suffix.len() > num_len
                && suffix.as_bytes()[num_len] == b'*'
            {
                let end_of_match = absolute_d_pos + 1 + num_len + 1;
                // Append content before the match.
                updated_line.push_str(&current_line[last_pos..absolute_d_pos]);
                // Prepend the required G54 prefix.
                updated_line.push_str("G54");
                // Append the original D-code.
                updated_line.push_str(&current_line[absolute_d_pos..end_of_match]);
                last_pos = end_of_match;
                search_pos = end_of_match;
            } else {
                search_pos = absolute_d_pos + 1;
            }
        }

        // Append any remaining part of the line after the last match.
        updated_line.push_str(&current_line[last_pos..]);
        processed_lines.push(updated_line);
    }

    processed_lines.join("\n")
}
