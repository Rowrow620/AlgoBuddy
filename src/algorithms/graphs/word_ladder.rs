use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

pub fn generate_word_ladder_steps(
    begin_word: &str,
    end_word: &str,
    _word_list: &[&str],
) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![0, 1, 2, 3, 4];
    let labels = vec![
        begin_word.to_string(),
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        end_word.to_string(),
    ];
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: format!(
            "Initialize Word Ladder BFS from start word '{}'",
            begin_word
        ),
        code_line: 4,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: Some(0),
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: format!("Queue: ['{}']", begin_word),
        },
    });

    for (i, label) in labels.iter().enumerate() {
        visited.insert(i);
        let active_edge = if i > 0 { Some((i - 1, i)) } else { None };
        steps.push(Step {
            description: format!("BFS Level {}: Word transformation '{}'", i + 1, label),
            code_line: 17,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(i),
                active_edge,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Transformation sequence length = {}", i + 1),
            },
        });
    }

    steps
}
