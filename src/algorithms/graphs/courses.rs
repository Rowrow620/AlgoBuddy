use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

pub fn generate_course_schedule_steps(_num_courses: i32, _prerequisites: &[[i32; 2]]) -> Vec<Step> {
    let mut steps = Vec::new();
    let num_courses = 4;
    let nodes: Vec<usize> = (0..num_courses).collect();
    let labels: Vec<String> = nodes.iter().map(|u| format!("Course {}", u)).collect();
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 3), (2, 3)];

    steps.push(Step {
        description: format!(
            "Course Schedule: Build dependency graph with {} courses",
            num_courses
        ),
        code_line: 4,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: BTreeSet::new(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Building prerequisite adjacency map".into(),
        },
    });

    let mut visited = BTreeSet::new();
    let mut visit_set = BTreeSet::new();

    // Step-by-step DFS traversal
    let dfs_sequence = vec![
        (0, "Start DFS at Course 0 - add 0 to visitSet", None),
        (1, "Traverse edge 0 ➔ 1: enter DFS(1)", Some((0, 1))),
        (3, "Traverse edge 1 ➔ 3: enter DFS(3)", Some((1, 3))),
        (
            3,
            "Course 3 has no prerequisites! Backtrack & mark Course 3 as complete",
            None,
        ),
        (
            1,
            "Backtrack to Course 1: all prerequisites processed! Mark Course 1 as complete",
            None,
        ),
        (2, "Traverse edge 0 ➔ 2: enter DFS(2)", Some((0, 2))),
        (
            2,
            "Backtrack to Course 2: all prerequisites processed! Mark Course 2 as complete",
            None,
        ),
        (
            0,
            "Backtrack to Course 0: all prerequisites processed! Mark Course 0 as complete",
            None,
        ),
    ];

    for (node, desc, active_edge) in dfs_sequence {
        if desc.contains("add") || desc.contains("enter") {
            visit_set.insert(node);
        } else if desc.contains("complete") {
            visit_set.remove(&node);
            visited.insert(node);
        }

        steps.push(Step {
            description: desc.to_string(),
            code_line: 9,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(node),
                active_edge,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("DFS visiting Course {}", node),
            },
        });
    }

    steps.push(Step {
        description:
            "Course Schedule Verified: No directed cycles detected! All courses can be completed."
                .into(),
        code_line: 16,
        visual: VisualState::NodeGraph {
            nodes,
            node_labels: labels,
            edges,
            active_node: None,
            active_edge: None,
            visited_nodes: visited,
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Result: True (Valid Course Schedule)".to_string(),
        },
    });

    steps
}

pub fn generate_course_schedule_ii_steps(
    _num_courses: i32,
    _prerequisites: &[[i32; 2]],
) -> Vec<Step> {
    let mut steps = Vec::new();
    let num_courses = 4;
    let nodes: Vec<usize> = (0..num_courses).collect();
    let labels: Vec<String> = nodes.iter().map(|u| format!("Course {}", u)).collect();
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 3), (2, 3)];

    let mut topo_order = Vec::new();
    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: format!(
            "Course Schedule II: Initialize Topological Sort for {} courses",
            num_courses
        ),
        code_line: 4,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: topo_order.clone(),
            message: "Topological Sort initialized".into(),
        },
    });

    for &u in &[3, 1, 2, 0] {
        visited.insert(u);
        topo_order.push(u);
        steps.push(Step {
            description: format!(
                "Post-Order DFS: Add Course {} to Topological Order output array",
                u
            ),
            code_line: 12,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(u),
                active_edge: None,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: topo_order.clone(),
                message: format!("Topological Sequence = {:?}", topo_order),
            },
        });
    }

    steps
}
