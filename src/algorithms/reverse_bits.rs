use crate::model::{Step, VisualState};

pub fn generate_reverse_bits_steps(n: u32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut res = 0u32;

    steps.push(Step {
        code_line: 3,
        description: format!("Reversing 32 bits of n = {} ({:032b}).", n, n),
        visual: VisualState::ContainsDuplicate {
            nums: vec![n as i32],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in 0..32 {
        let bit = (n >> i) & 1;
        res |= bit << (31 - i);
    }

    steps.push(Step {
        code_line: 6,
        description: format!("Reversed 32-bit integer = {} ({:032b}).", res, res),
        visual: VisualState::ContainsDuplicate {
            nums: vec![res as i32],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(res as i32),
            has_duplicate: Some(true),
        },
    });

    steps
}

pub fn generate_reverse_bits_binary_string_steps(n: u32) -> Vec<Step> {
    let bits = format!("{n:032b}");
    let original: Vec<i32> = bits.bytes().map(|byte| i32::from(byte - b'0')).collect();
    let reversed_bits: String = bits.chars().rev().collect();
    let reversed: Vec<i32> = reversed_bits
        .bytes()
        .map(|byte| i32::from(byte - b'0'))
        .collect();
    let result = u32::from_str_radix(&reversed_bits, 2)
        .expect("a reversed 32-character binary string always fits in u32");

    vec![
        Step {
            code_line: 3,
            description: format!("Format {n} as the fixed-width binary string {bits}."),
            visual: VisualState::Array1D {
                title: "Reverse Bits: Binary String".into(),
                elements: original,
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: "bits = format(n, '032b')".into(),
                is_success: None,
            },
        },
        Step {
            code_line: 4,
            description: format!("Reverse the 32 characters to obtain {reversed_bits}."),
            visual: VisualState::Array1D {
                title: "Reverse Bits: Binary String".into(),
                elements: reversed.clone(),
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: "reversed_bits = bits[::-1]".into(),
                is_success: None,
            },
        },
        Step {
            code_line: 5,
            description: format!("Convert the reversed binary string back to integer {result}."),
            visual: VisualState::Array1D {
                title: "Reverse Bits: Binary String".into(),
                elements: reversed,
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: format!("return {result}"),
                is_success: Some(true),
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_string_reverse_matches_the_expected_integer() {
        let steps = generate_reverse_bits_binary_string_steps(43_261_596);
        assert!(steps
            .last()
            .expect("trace must not be empty")
            .description
            .contains("integer 964176192"));
    }
}
