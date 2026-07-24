use crate::model::{EncodeDecodePhase, Step, VisualState};

pub fn generate_encode_decode_steps(strs: &[String]) -> Vec<Step> {
    let mut steps = Vec::new();

    let input_strs = strs.to_vec();
    let mut encoded_so_far = String::new();
    let mut decoded_so_far = Vec::new();

    // 1. Init step (code_line 3)
    steps.push(Step {
        code_line: 3,
        description: "Initialized empty string res = \"\" for encoding.".to_string(),
        visual: VisualState::EncodeDecode {
            input_strs: input_strs.clone(),
            encoded_so_far: encoded_so_far.clone(),
            decoded_so_far: decoded_so_far.clone(),
            pointer: 0,
            active_str_idx: None,
            phase: EncodeDecodePhase::Init,
        },
    });

    // 2. One step per string being encoded (code_line 5)
    for (idx, s) in input_strs.iter().enumerate() {
        let chunk = format!("{}#{}", s.len(), s);
        encoded_so_far.push_str(&chunk);

        steps.push(Step {
            code_line: 5,
            description: format!(
                "Encoded string \"{}\" with length prefix as \"{}\". Encoded string so far: \"{}\".",
                s, chunk, encoded_so_far
            ),
            visual: VisualState::EncodeDecode {
                input_strs: input_strs.clone(),
                encoded_so_far: encoded_so_far.clone(),
                decoded_so_far: decoded_so_far.clone(),
                pointer: 0,
                active_str_idx: Some(idx),
                phase: EncodeDecodePhase::Encoding,
            },
        });
    }

    // 3. Encoding complete step (code_line 6)
    steps.push(Step {
        code_line: 6,
        description: format!(
            "Encoding complete. Final encoded string: \"{}\".",
            encoded_so_far
        ),
        visual: VisualState::EncodeDecode {
            input_strs: input_strs.clone(),
            encoded_so_far: encoded_so_far.clone(),
            decoded_so_far: decoded_so_far.clone(),
            pointer: 0,
            active_str_idx: None,
            phase: EncodeDecodePhase::EncodingComplete,
        },
    });

    // 4. Decode init step (code_line 10)
    let mut i = 0;
    steps.push(Step {
        code_line: 10,
        description: "Initialized empty result list res = [] and pointer i = 0 for decoding.".to_string(),
        visual: VisualState::EncodeDecode {
            input_strs: input_strs.clone(),
            encoded_so_far: encoded_so_far.clone(),
            decoded_so_far: decoded_so_far.clone(),
            pointer: i,
            active_str_idx: None,
            phase: EncodeDecodePhase::Decoding,
        },
    });

    // 5. For each string being decoded
    while i < encoded_so_far.len() {
        if let Some(hash_rel_pos) = encoded_so_far[i..].find('#') {
            let j = i + hash_rel_pos;

            // 5a. Scanning for '#' step (code_line 13)
            steps.push(Step {
                code_line: 13,
                description: format!(
                    "Scanned from index {} to find delimiter '#' at index {}.",
                    i, j
                ),
                visual: VisualState::EncodeDecode {
                    input_strs: input_strs.clone(),
                    encoded_so_far: encoded_so_far.clone(),
                    decoded_so_far: decoded_so_far.clone(),
                    pointer: i,
                    active_str_idx: Some(decoded_so_far.len()),
                    phase: EncodeDecodePhase::Decoding,
                },
            });

            // 5b. Parsing length step (code_line 15)
            let len_str = &encoded_so_far[i..j];
            let length: usize = len_str.parse().unwrap_or(0);
            steps.push(Step {
                code_line: 15,
                description: format!(
                    "Parsed length {} from substring \"{}\" (indices {}..{}).",
                    length, len_str, i, j
                ),
                visual: VisualState::EncodeDecode {
                    input_strs: input_strs.clone(),
                    encoded_so_far: encoded_so_far.clone(),
                    decoded_so_far: decoded_so_far.clone(),
                    pointer: i,
                    active_str_idx: Some(decoded_so_far.len()),
                    phase: EncodeDecodePhase::Decoding,
                },
            });

            // 5c. Extracting substring step (code_line 16)
            let start = j + 1;
            let end = start + length;
            let extracted = encoded_so_far[start..end].to_string();
            decoded_so_far.push(extracted.clone());

            steps.push(Step {
                code_line: 16,
                description: format!(
                    "Extracted substring \"{}\" of length {} (indices {}..{}) and appended to res.",
                    extracted, length, start, end
                ),
                visual: VisualState::EncodeDecode {
                    input_strs: input_strs.clone(),
                    encoded_so_far: encoded_so_far.clone(),
                    decoded_so_far: decoded_so_far.clone(),
                    pointer: i,
                    active_str_idx: Some(decoded_so_far.len() - 1),
                    phase: EncodeDecodePhase::Decoding,
                },
            });

            // 5d. Advancing pointer step (code_line 17)
            let next_i = end;
            steps.push(Step {
                code_line: 17,
                description: format!("Advanced pointer i from {} to {}.", i, next_i),
                visual: VisualState::EncodeDecode {
                    input_strs: input_strs.clone(),
                    encoded_so_far: encoded_so_far.clone(),
                    decoded_so_far: decoded_so_far.clone(),
                    pointer: next_i,
                    active_str_idx: Some(decoded_so_far.len() - 1),
                    phase: EncodeDecodePhase::Decoding,
                },
            });

            i = next_i;
        } else {
            break;
        }
    }

    // 6. Decode complete step (code_line 18)
    steps.push(Step {
        code_line: 18,
        description: format!(
            "Decoding complete. Restored {} strings: {:?}.",
            decoded_so_far.len(),
            decoded_so_far
        ),
        visual: VisualState::EncodeDecode {
            input_strs: input_strs.clone(),
            encoded_so_far: encoded_so_far.clone(),
            decoded_so_far: decoded_so_far.clone(),
            pointer: i,
            active_str_idx: None,
            phase: EncodeDecodePhase::Complete,
        },
    });

    steps
}
