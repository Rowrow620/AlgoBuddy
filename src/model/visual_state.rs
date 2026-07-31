use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeDecodePhase {
    Init,
    Encoding,
    EncodingComplete,
    Decoding,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductPhase {
    Init,
    PrefixPass,
    SuffixPass,
    Complete,
}

// ── Generic Execution Step ──

#[derive(Debug, Clone)]
pub struct Step {
    pub code_line: usize,
    pub description: String,
    pub visual: VisualState,
}

/// ── Canonical Visual State Representations ──
///
/// Encapsulates all visual layout state data rendered across the 150 AlgoBuddy problems.
/// Structured into canonical visual layout categories:
/// - 1D Array & Sequence Layouts (`Array1D`, `ContainsDuplicate`, `LongestConsecutive`, `Product`)
/// - Pointer & Window Traces (`TwoPointers`, `BinarySearch`, `BestTimeStock`)
/// - Memory & LIFO Inspectors (`Stack`, `TwoSum`, `ValidAnagram`, `GroupAnagrams`, `TopK`, `EncodeDecode`)
/// - Sequential Linkages (`LinkedList`, `MergeLinkedLists`, `LinkedListCycle`)
/// - Hierarchical Views (`TreeVisual`, `HeapVisual`, `Trie`, `DecisionTreeVisual`)
/// - Graph & Grid Networks (`GridGraph`, `NodeGraph`, `ValidSudoku`)
#[derive(Debug, Clone)]
pub enum VisualState {
    ContainsDuplicate {
        nums: Vec<i32>,
        active_idx: Option<usize>,
        seen_set: BTreeSet<i32>,
        duplicate_val: Option<i32>,
        has_duplicate: Option<bool>,
    },
    GroupAnagrams {
        input_strs: Vec<String>,
        active_idx: Option<usize>,
        key_fmt: String,
        groups: BTreeMap<String, Vec<String>>,
    },
    TopK {
        nums: Vec<i32>,
        active_nums_idx: Option<usize>,
        count_map: BTreeMap<i32, usize>,
        buckets: Vec<Vec<i32>>,
        active_bucket_idx: Option<usize>,
        result: Vec<i32>,
    },
    EncodeDecode {
        input_strs: Vec<String>,
        encoded_so_far: String,
        decoded_so_far: Vec<String>,
        pointer: usize,
        active_str_idx: Option<usize>,
        phase: EncodeDecodePhase,
    },
    Product {
        nums: Vec<i32>,
        output: Vec<i64>,
        active_idx: Option<usize>,
        prefix_val: i64,
        suffix_val: i64,
        phase: ProductPhase,
    },
    Trie {
        words: Vec<String>,
        current_word: String,
        active_char_idx: Option<usize>,
    },

    ValidSudoku {
        board: [[char; 9]; 9],
        active_r: Option<usize>,
        active_c: Option<usize>,
        duplicate_pos: Option<(usize, usize)>,
        is_valid: Option<bool>,
    },
    LongestConsecutive {
        nums: Vec<i32>,
        num_set: BTreeSet<i32>,
        current_num: Option<i32>,
        current_seq: Vec<i32>,
        max_length: usize,
        is_seq_start: Option<bool>,
    },
    TwoSum {
        nums: Vec<i32>,
        target: i32,
        active_idx: Option<usize>,
        secondary_idx: Option<usize>,
        map: BTreeMap<i32, usize>,
        found_indices: Option<(usize, usize)>,
    },
    ValidAnagram {
        s: String,
        t: String,
        s_counts: [usize; 26],
        t_counts: [usize; 26],
        active_s_idx: Option<usize>,
        active_t_idx: Option<usize>,
        is_anagram: Option<bool>,
    },
    TwoPointers {
        chars: Vec<char>,
        left: usize,
        right: usize,
        is_valid: Option<bool>,
        skipped: bool,
    },
    Stack {
        chars: Vec<char>,
        active_idx: Option<usize>,
        stack: Vec<char>,
        is_valid: Option<bool>,
    },
    BestTimeStock {
        prices: Vec<i32>,
        left_buy: usize,
        right_sell: usize,
        current_profit: i32,
        max_profit: i32,
    },
    BinarySearch {
        nums: Vec<i32>,
        target: i32,
        left: usize,
        right: usize,
        mid: Option<usize>,
        found_idx: Option<usize>,
    },
    LinkedList {
        nodes: Vec<i32>,
        prev_idx: Option<usize>,
        curr_idx: Option<usize>,
        next_idx: Option<usize>,
        reversed_so_far: Vec<i32>,
    },
    MergeLinkedLists {
        list1: Vec<i32>,
        list2: Vec<i32>,
        p1_idx: Option<usize>,
        p2_idx: Option<usize>,
        merged_so_far: Vec<i32>,
    },
    LinkedListCycle {
        nodes: Vec<i32>,
        cycle_target_idx: Option<usize>,
        slow_idx: Option<usize>,
        fast_idx: Option<usize>,
        has_cycle: Option<bool>,
    },
    TreeVisual {
        tree_nodes: Vec<Option<i32>>,
        active_node_idx: Option<usize>,
        secondary_node_idx: Option<usize>,
        depth_val: Option<i32>,
        max_diameter: Option<i32>,
    },
    HeapVisual {
        heap_elements: Vec<i32>,
        active_idx: Option<usize>,
        swapped_pair: Option<(usize, usize)>,
        heap_type_label: String,
    },
    DecisionTreeVisual {
        current_path: Vec<i32>,
        active_choice: Option<String>,
        completed_results: Vec<Vec<i32>>,
    },
    GridGraph {
        rows: usize,
        cols: usize,
        grid: Vec<Vec<String>>,
        active_cell: Option<(usize, usize)>,
        visited_cells: BTreeSet<(usize, usize)>,
        frontier_cells: BTreeSet<(usize, usize)>,
        message: String,
    },
    NodeGraph {
        nodes: Vec<usize>,
        node_labels: Vec<String>,
        edges: Vec<(usize, usize)>,
        active_node: Option<usize>,
        active_edge: Option<(usize, usize)>,
        visited_nodes: BTreeSet<usize>,
        cycle_edges: BTreeSet<(usize, usize)>,
        topo_order: Vec<usize>,
        message: String,
    },
    Array1D {
        title: String,
        elements: Vec<i32>,
        active_idx: Option<usize>,
        secondary_idx: Option<usize>,
        pointers: Vec<(&'static str, usize)>,
        status_message: String,
        is_success: Option<bool>,
    },
}

impl VisualState {
    pub fn variables(&self) -> Vec<(&'static str, String)> {
        match self {
            VisualState::ContainsDuplicate {
                active_idx,
                duplicate_val,
                has_duplicate,
                seen_set,
                nums,
            } => {
                let mut vars = Vec::new();
                if let Some(i) = active_idx {
                    vars.push(("i", i.to_string()));
                    if let Some(val) = nums.get(*i) {
                        vars.push(("nums[i]", val.to_string()));
                    }
                }
                vars.push(("seen", crate::utils::format_python_set(seen_set)));
                if let Some(dup) = duplicate_val {
                    vars.push(("duplicate", dup.to_string()));
                }
                if let Some(res) = has_duplicate {
                    vars.push(("result", res.to_string()));
                }
                vars
            }
            VisualState::TwoSum {
                active_idx,
                secondary_idx,
                map,
                found_indices,
                nums,
                target,
            } => {
                let mut vars = Vec::new();
                vars.push(("target", target.to_string()));
                if let Some(i) = active_idx {
                    vars.push(("i", i.to_string()));
                    if let Some(val) = nums.get(*i) {
                        vars.push(("nums[i]", val.to_string()));
                        vars.push(("diff", (target - val).to_string()));
                    }
                }
                if let Some(j) = secondary_idx {
                    vars.push(("prevMap[diff]", j.to_string()));
                }
                vars.push(("prevMap", format!("{:?}", map)));
                if let Some((a, b)) = found_indices {
                    vars.push(("result", format!("[{}, {}]", a, b)));
                }
                vars
            }
            VisualState::ValidAnagram {
                s,
                t,
                active_s_idx,
                is_anagram,
                ..
            } => {
                let mut vars = Vec::new();
                vars.push(("s", s.clone()));
                vars.push(("t", t.clone()));
                if let Some(i) = active_s_idx {
                    vars.push(("i", i.to_string()));
                    let s_chars: Vec<char> = s.chars().collect();
                    let t_chars: Vec<char> = t.chars().collect();
                    if let Some(&c_s) = s_chars.get(*i) {
                        vars.push(("s[i]", c_s.to_string()));
                    }
                    if let Some(&c_t) = t_chars.get(*i) {
                        vars.push(("t[i]", c_t.to_string()));
                    }
                }
                if let Some(res) = is_anagram {
                    vars.push(("is_anagram", res.to_string()));
                }
                vars
            }
            VisualState::GroupAnagrams {
                active_idx,
                key_fmt,
                groups,
                input_strs,
            } => {
                let mut vars = Vec::new();
                if let Some(i) = active_idx {
                    vars.push(("i", i.to_string()));
                    if let Some(s) = input_strs.get(*i) {
                        vars.push(("word", s.clone()));
                    }
                }
                if !key_fmt.is_empty() {
                    vars.push(("key", key_fmt.clone()));
                }
                vars.push(("groups_count", groups.len().to_string()));
                vars
            }
            VisualState::TopK {
                active_nums_idx,
                count_map,
                active_bucket_idx,
                result,
                nums,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(i) = active_nums_idx {
                    vars.push(("i", i.to_string()));
                    if let Some(val) = nums.get(*i) {
                        vars.push(("nums[i]", val.to_string()));
                    }
                }
                vars.push(("count_map", format!("{:?}", count_map)));
                if let Some(b) = active_bucket_idx {
                    vars.push(("bucket_freq", b.to_string()));
                }
                vars.push(("result", format!("{:?}", result)));
                vars
            }
            VisualState::EncodeDecode {
                pointer,
                encoded_so_far,
                decoded_so_far,
                input_strs,
                active_str_idx,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(i) = active_str_idx {
                    vars.push(("i", i.to_string()));
                    if let Some(s) = input_strs.get(*i) {
                        vars.push(("str", s.clone()));
                    }
                }
                vars.push(("pointer", pointer.to_string()));
                vars.push(("encoded", encoded_so_far.clone()));
                vars.push(("decoded", format!("{:?}", decoded_so_far)));
                vars
            }
            VisualState::Product {
                active_idx,
                prefix_val,
                suffix_val,
                output,
                nums,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(i) = active_idx {
                    vars.push(("i", i.to_string()));
                    if let Some(n) = nums.get(*i) {
                        vars.push(("nums[i]", n.to_string()));
                    }
                }
                vars.push(("prefix", prefix_val.to_string()));
                vars.push(("suffix", suffix_val.to_string()));
                vars.push(("output", format!("{:?}", output)));
                vars
            }
            VisualState::ValidSudoku {
                active_r,
                active_c,
                is_valid,
                duplicate_pos,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(r) = active_r {
                    vars.push(("row", r.to_string()));
                }
                if let Some(c) = active_c {
                    vars.push(("col", c.to_string()));
                }
                if let Some(pos) = duplicate_pos {
                    vars.push(("dup_pos", format!("({},{})", pos.0, pos.1)));
                }
                if let Some(v) = is_valid {
                    vars.push(("is_valid", v.to_string()));
                }
                vars
            }
            VisualState::LongestConsecutive {
                num_set,
                current_num,
                current_seq,
                max_length,
                ..
            } => {
                let mut vars = Vec::new();
                vars.push(("numSet", crate::utils::format_python_set(num_set)));
                if let Some(n) = current_num {
                    vars.push(("num", n.to_string()));
                }
                vars.push(("curr_streak", current_seq.len().to_string()));
                vars.push(("max_streak", max_length.to_string()));
                vars
            }
            VisualState::TwoPointers {
                left,
                right,
                is_valid,
                ..
            } => {
                let mut vars = Vec::new();
                vars.push(("left", left.to_string()));
                vars.push(("right", right.to_string()));
                if let Some(v) = is_valid {
                    vars.push(("is_valid", v.to_string()));
                }
                vars
            }
            VisualState::Stack {
                active_idx,
                stack,
                is_valid,
                chars,
            } => {
                let mut vars = Vec::new();
                if let Some(i) = active_idx {
                    vars.push(("i", i.to_string()));
                    if let Some(ch) = chars.get(*i) {
                        vars.push(("char", ch.to_string()));
                    }
                }
                vars.push(("stack", format!("{:?}", stack)));
                if let Some(v) = is_valid {
                    vars.push(("is_valid", v.to_string()));
                }
                vars
            }
            VisualState::BestTimeStock {
                left_buy,
                right_sell,
                current_profit,
                max_profit,
                prices,
            } => {
                let mut vars = Vec::new();
                vars.push(("buy_day (l)", left_buy.to_string()));
                vars.push(("sell_day (r)", right_sell.to_string()));
                if let Some(p) = prices.get(*left_buy) {
                    vars.push(("buy_price", format!("${}", p)));
                }
                if let Some(p) = prices.get(*right_sell) {
                    vars.push(("sell_price", format!("${}", p)));
                }
                vars.push(("current_profit", format!("${}", current_profit)));
                vars.push(("max_profit", format!("${}", max_profit)));
                vars
            }
            VisualState::BinarySearch {
                left,
                right,
                mid,
                found_idx,
                target,
                nums,
            } => {
                let mut vars = Vec::new();
                vars.push(("target", target.to_string()));
                vars.push(("left", left.to_string()));
                vars.push(("right", right.to_string()));
                if let Some(m) = mid {
                    vars.push(("mid", m.to_string()));
                    if let Some(val) = nums.get(*m) {
                        vars.push(("nums[mid]", val.to_string()));
                    }
                }
                if let Some(f) = found_idx {
                    vars.push(("found_at", f.to_string()));
                }
                vars
            }
            VisualState::LinkedList {
                curr_idx,
                prev_idx,
                next_idx,
                reversed_so_far,
                nodes,
            } => {
                let mut vars = Vec::new();
                if let Some(i) = curr_idx {
                    vars.push((
                        "curr",
                        format!("node[{}]={}", i, nodes.get(*i).unwrap_or(&0)),
                    ));
                }
                if let Some(p) = prev_idx {
                    vars.push((
                        "prev",
                        format!("node[{}]={}", p, nodes.get(*p).unwrap_or(&0)),
                    ));
                }
                if let Some(n) = next_idx {
                    vars.push((
                        "next",
                        format!("node[{}]={}", n, nodes.get(*n).unwrap_or(&0)),
                    ));
                }
                vars.push(("reversed_list", format!("{:?}", reversed_so_far)));
                vars
            }
            VisualState::MergeLinkedLists {
                p1_idx,
                p2_idx,
                merged_so_far,
                list1,
                list2,
            } => {
                let mut vars = Vec::new();
                if let Some(i) = p1_idx {
                    vars.push(("list1_curr", format!("val={}", list1.get(*i).unwrap_or(&0))));
                }
                if let Some(j) = p2_idx {
                    vars.push(("list2_curr", format!("val={}", list2.get(*j).unwrap_or(&0))));
                }
                vars.push(("merged_list", format!("{:?}", merged_so_far)));
                vars
            }
            VisualState::LinkedListCycle {
                slow_idx,
                fast_idx,
                has_cycle,
                nodes,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(s) = slow_idx {
                    vars.push((
                        "slow_ptr",
                        format!("idx={} (val={})", s, nodes.get(*s).unwrap_or(&0)),
                    ));
                }
                if let Some(f) = fast_idx {
                    vars.push((
                        "fast_ptr",
                        format!("idx={} (val={})", f, nodes.get(*f).unwrap_or(&0)),
                    ));
                }
                if let Some(c) = has_cycle {
                    vars.push(("has_cycle", c.to_string()));
                }
                vars
            }
            VisualState::TreeVisual {
                active_node_idx,
                depth_val,
                max_diameter,
                tree_nodes,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(idx) = active_node_idx {
                    if let Some(Some(val)) = tree_nodes.get(*idx) {
                        vars.push(("curr_node", format!("val={} (idx={})", val, idx)));
                    }
                }
                if let Some(d) = depth_val {
                    vars.push(("depth", d.to_string()));
                }
                if let Some(diam) = max_diameter {
                    vars.push(("max_diameter", diam.to_string()));
                }
                vars
            }
            VisualState::HeapVisual {
                heap_elements,
                active_idx,
                swapped_pair,
                heap_type_label,
            } => {
                let mut vars = Vec::new();
                vars.push(("heap_type", heap_type_label.clone()));
                vars.push(("heap_size", heap_elements.len().to_string()));
                if let Some(i) = active_idx {
                    if let Some(val) = heap_elements.get(*i) {
                        vars.push(("active_elem", format!("val={} (idx={})", val, i)));
                    }
                }
                if let Some((a, b)) = swapped_pair {
                    vars.push(("swapped_pair", format!("indices ({}, {})", a, b)));
                }
                vars.push(("array_rep", format!("{:?}", heap_elements)));
                vars
            }
            VisualState::DecisionTreeVisual {
                current_path,
                active_choice,
                completed_results,
            } => {
                let mut vars = Vec::new();
                vars.push(("current_path", format!("{:?}", current_path)));
                if let Some(c) = active_choice {
                    vars.push(("active_choice", c.clone()));
                }
                vars.push(("total_subsets", completed_results.len().to_string()));
                vars
            }
            VisualState::GridGraph {
                active_cell,
                visited_cells,
                frontier_cells,
                message,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some((r, c)) = active_cell {
                    vars.push(("curr_cell", format!("({}, {})", r, c)));
                }
                vars.push(("visited_count", visited_cells.len().to_string()));
                vars.push(("queue_size", frontier_cells.len().to_string()));
                if !message.is_empty() {
                    vars.push(("status", message.clone()));
                }
                vars
            }
            VisualState::NodeGraph {
                active_node,
                active_edge,
                visited_nodes,
                topo_order,
                message,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(n) = active_node {
                    vars.push(("active_node", n.to_string()));
                }
                if let Some((u, v)) = active_edge {
                    vars.push(("traversing_edge", format!("{} -> {}", u, v)));
                }
                vars.push(("visited_nodes", format!("{:?}", visited_nodes)));
                if !topo_order.is_empty() {
                    vars.push(("topo_order", format!("{:?}", topo_order)));
                }
                if !message.is_empty() {
                    vars.push(("status", message.clone()));
                }
                vars
            }
            VisualState::Trie {
                current_word,
                active_char_idx,
                words,
            } => {
                let mut vars = Vec::new();
                vars.push(("dictionary_words", format!("{:?}", words)));
                vars.push(("inserting_word", current_word.clone()));
                if let Some(idx) = active_char_idx {
                    if let Some(ch) = current_word.chars().nth(*idx) {
                        vars.push(("char", format!("'{}' at idx={}", ch, idx)));
                    }
                }
                vars
            }
            VisualState::Array1D {
                active_idx,
                secondary_idx,
                elements,
                pointers,
                status_message,
                ..
            } => {
                let mut vars = Vec::new();
                if let Some(i) = active_idx {
                    vars.push(("idx", i.to_string()));
                    if let Some(val) = elements.get(*i) {
                        vars.push(("elements[idx]", val.to_string()));
                    }
                }
                if let Some(j) = secondary_idx {
                    vars.push(("secondary_idx", j.to_string()));
                }
                for (ptr_name, ptr_val) in pointers {
                    vars.push((ptr_name, ptr_val.to_string()));
                }
                if !status_message.is_empty() {
                    vars.push(("status", status_message.clone()));
                }
                vars
            }
        }
    }
}
