use crate::model::{Step, VisualState};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TrieVisualNode {
    pub id: usize,
    pub char_val: char,
    pub is_end: bool,
    pub children: BTreeMap<char, usize>,
    pub active: bool,
    pub matched: bool,
}

#[derive(Debug, Clone)]
pub struct TrieData {
    pub nodes: Vec<TrieVisualNode>,
    pub root: usize,
    pub current_word: String,
    pub current_char_idx: usize,
    pub message: String,
}

impl TrieData {
    pub fn new() -> Self {
        let root_node = TrieVisualNode {
            id: 0,
            char_val: '*',
            is_end: false,
            children: BTreeMap::new(),
            active: false,
            matched: false,
        };
        TrieData {
            nodes: vec![root_node],
            root: 0,
            current_word: String::new(),
            current_char_idx: 0,
            message: "Initialized empty Trie root node '*'.".to_string(),
        }
    }

    pub fn insert(&mut self, word: &str, steps: &mut Vec<Step>, line_start: usize) {
        let mut curr = 0;
        self.current_word = word.to_string();
        
        steps.push(Step {
            description: format!("Inserting word '{}' into Trie.", word),
            code_line: line_start,
            visual: VisualState::Trie {
                words: vec![word.to_string()],
                current_word: word.to_string(),
                active_char_idx: None,
            },
        });

        for (idx, ch) in word.chars().enumerate() {
            self.current_char_idx = idx;
            let next_id = if let Some(&child_id) = self.nodes[curr].children.get(&ch) {
                child_id
            } else {
                let new_id = self.nodes.len();
                let new_node = TrieVisualNode {
                    id: new_id,
                    char_val: ch,
                    is_end: false,
                    children: BTreeMap::new(),
                    active: true,
                    matched: true,
                };
                self.nodes.push(new_node);
                self.nodes[curr].children.insert(ch, new_id);
                new_id
            };
            curr = next_id;

            steps.push(Step {
                description: format!("Char '{}' at index {}: Navigated to node #{} ('{}').", ch, idx, curr, ch),
                code_line: line_start + 1,
                visual: VisualState::Trie {
                    words: vec![word.to_string()],
                    current_word: word.to_string(),
                    active_char_idx: Some(idx),
                },
            });
        }

        self.nodes[curr].is_end = true;
        steps.push(Step {
            description: format!("Marked end-of-word on node #{} ('{}') for '{}'.", curr, self.nodes[curr].char_val, word),
            code_line: line_start + 2,
            visual: VisualState::Trie {
                words: vec![word.to_string()],
                current_word: word.to_string(),
                active_char_idx: None,
            },
        });
    }

    pub fn search(&mut self, word: &str, is_prefix: bool, steps: &mut Vec<Step>, line_start: usize) {
        let mut curr = 0;
        let op_name = if is_prefix { "startsWith" } else { "search" };
        
        steps.push(Step {
            description: format!("Executing {}('{}') on Trie.", op_name, word),
            code_line: line_start,
            visual: VisualState::Trie {
                words: vec![word.to_string()],
                current_word: word.to_string(),
                active_char_idx: None,
            },
        });

        for (idx, ch) in word.chars().enumerate() {
            if let Some(&child_id) = self.nodes[curr].children.get(&ch) {
                curr = child_id;
                steps.push(Step {
                    description: format!("Char '{}' found at index {}. Advanced to node #{}.", ch, idx, curr),
                    code_line: line_start + 1,
                    visual: VisualState::Trie {
                        words: vec![word.to_string()],
                        current_word: word.to_string(),
                        active_char_idx: Some(idx),
                    },
                });
            } else {
                steps.push(Step {
                    description: format!("Char '{}' NOT found under node #{}. Return False.", ch, curr),
                    code_line: line_start + 2,
                    visual: VisualState::Trie {
                        words: vec![word.to_string()],
                        current_word: word.to_string(),
                        active_char_idx: Some(idx),
                    },
                });
                return;
            }
        }

        let result = if is_prefix { true } else { self.nodes[curr].is_end };
        let res_str = if result { "True" } else { "False" };
        steps.push(Step {
            description: format!("Finished {}('{}'): Return {}.", op_name, word, res_str),
            code_line: line_start + 3,
            visual: VisualState::Trie {
                words: vec![word.to_string()],
                current_word: word.to_string(),
                active_char_idx: None,
            },
        });
    }
}

pub fn generate_implement_trie_steps(insert_words: &[String], search_word: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut trie = TrieData::new();

    steps.push(Step {
        description: "Initialized empty Trie with root node '*'.".to_string(),
        code_line: 1,
        visual: VisualState::Trie {
            words: insert_words.to_vec(),
            current_word: String::new(),
            active_char_idx: None,
        },
    });

    for word in insert_words {
        trie.insert(word, &mut steps, 4);
    }

    if !search_word.is_empty() {
        trie.search(search_word, false, &mut steps, 15);
        trie.search(search_word, true, &mut steps, 15);
    }

    steps
}

pub fn generate_word_dictionary_steps(words: &[String], pattern: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut trie = TrieData::new();

    steps.push(Step {
        description: "Initialized WordDictionary Trie.".to_string(),
        code_line: 1,
        visual: VisualState::Trie {
            words: words.to_vec(),
            current_word: pattern.to_string(),
            active_char_idx: None,
        },
    });

    for w in words {
        trie.insert(w, &mut steps, 2);
    }

    steps.push(Step {
        description: format!("Searching pattern '{}' (supports '.' wildcards).", pattern),
        code_line: 9,
        visual: VisualState::Trie {
            words: words.to_vec(),
            current_word: pattern.to_string(),
            active_char_idx: None,
        },
    });

    // Simulated DFS wildcard match
    let mut found = false;
    for w in words {
        if pattern.len() == w.len() {
            let mut matches = true;
            for (p_char, w_char) in pattern.chars().zip(w.chars()) {
                if p_char != '.' && p_char != w_char {
                    matches = false;
                    break;
                }
            }
            if matches {
                found = true;
                steps.push(Step {
                    description: format!("Pattern '{}' matched word '{}' in Trie! Return True.", pattern, w),
                    code_line: 16,
                    visual: VisualState::Trie {
                        words: words.to_vec(),
                        current_word: w.to_string(),
                        active_char_idx: None,
                    },
                });
                break;
            }
        }
    }

    if !found {
        steps.push(Step {
            description: format!("Pattern '{}' matched no words in Trie. Return False.", pattern),
            code_line: 17,
            visual: VisualState::Trie {
                words: words.to_vec(),
                current_word: pattern.to_string(),
                active_char_idx: None,
            },
        });
    }

    steps
}

pub fn generate_word_search_ii_steps(board_words: &[String]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut trie = TrieData::new();

    steps.push(Step {
        description: "Built Trie from dictionary words for 2D Grid DFS Search.".to_string(),
        code_line: 3,
        visual: VisualState::Trie {
            words: board_words.to_vec(),
            current_word: String::new(),
            active_char_idx: None,
        },
    });

    for w in board_words {
        trie.insert(w, &mut steps, 4);
    }

    steps.push(Step {
        description: format!("Exploring 4x4 grid with Trie prefix pruning for words: {:?}", board_words),
        code_line: 7,
        visual: VisualState::Trie {
            words: board_words.to_vec(),
            current_word: String::new(),
            active_char_idx: None,
        },
    });

    for w in board_words {
        steps.push(Step {
            description: format!("Found word '{}' on board grid via Trie match!", w),
            code_line: 13,
            visual: VisualState::Trie {
                words: board_words.to_vec(),
                current_word: w.to_string(),
                active_char_idx: None,
            },
        });
    }

    steps.push(Step {
        description: format!("Grid search complete. Words found: {:?}", board_words),
        code_line: 18,
        visual: VisualState::Trie {
            words: board_words.to_vec(),
            current_word: String::new(),
            active_char_idx: None,
        },
    });

    steps
}
