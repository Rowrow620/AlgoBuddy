use std::collections::BTreeSet;
use crate::model::{Step, VisualState};

pub fn generate_reconstruct_itinerary_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![0, 1, 2, 3, 4];
    let labels = vec!["JFK".to_string(), "MUC".to_string(), "LHR".to_string(), "SFO".to_string(), "SJC".to_string()];
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: "Reconstruct Itinerary: Hierholzer's algorithm initialized at JFK".into(),
        code_line: 13,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: Some(0),
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Flight Itinerary: Starting at JFK".into(),
        },
    });

    for (i, label) in labels.iter().enumerate() {
        visited.insert(i);
        let active_edge = if i > 0 { Some((i - 1, i)) } else { None };
        steps.push(Step {
            description: format!("Flight Leg {}: Fly to {}", i + 1, label),
            code_line: 11,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(i),
                active_edge,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Itinerary Progress: Reach {}", label),
            },
        });
    }

    steps
}

pub fn generate_min_cost_points_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![0, 1, 2, 3, 4];
    let labels = vec!["P0(0,0)".to_string(), "P1(2,2)".to_string(), "P2(3,10)".to_string(), "P3(5,2)".to_string(), "P4(7,0)".to_string()];
    let edges = vec![(0, 1), (1, 3), (3, 4), (1, 2)];

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: "Prim's Minimum Spanning Tree: Initialize greedy point connection".into(),
        code_line: 10,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: Some(0),
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Prim's MST initialized".into(),
        },
    });

    for &u in &nodes {
        visited.insert(u);
        steps.push(Step {
            description: format!("Prim's MST: Add point P{} to MST, total cost accumulator updated", u),
            code_line: 14,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(u),
                active_edge: None,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("MST Connected Points = {}", visited.len()),
            },
        });
    }

    steps.push(Step {
        description: "Prim's Minimum Spanning Tree Complete! Total Manhattan Cost = 20".into(),
        code_line: 17,
        visual: VisualState::NodeGraph {
            nodes,
            node_labels: labels,
            edges,
            active_node: None,
            active_edge: None,
            visited_nodes: visited,
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Min Connection Cost = 20".into(),
        },
    });

    steps
}

pub fn generate_network_delay_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![1, 2, 3, 4];
    let labels = vec!["Node 1".to_string(), "Node 2".to_string(), "Node 3".to_string(), "Node 4".to_string()];
    let edges = vec![(2, 1), (2, 3), (3, 4)];

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: "Dijkstra Network Delay Time: Signal sent from source Node K=2".into(),
        code_line: 5,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: Some(2),
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Source K=2 initialized (t=0)".into(),
        },
    });

    for &u in &[2, 1, 3, 4] {
        visited.insert(u);
        steps.push(Step {
            description: format!("Dijkstra Min-Heap: Signal reaches Node {} at time t", u),
            code_line: 9,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(u),
                active_edge: None,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Reached Node {}", u),
            },
        });
    }

    steps
}

pub fn generate_swim_rising_water_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let grid = vec![
        vec!["0".to_string(), "2".to_string()],
        vec!["1".to_string(), "3".to_string()],
    ];

    steps.push(Step {
        description: "Swim in Rising Water Dijkstra: Water level t=0 at top-left (0,0)".into(),
        code_line: 3,
        visual: VisualState::GridGraph {
            rows: 2,
            cols: 2,
            grid: grid.clone(),
            active_cell: Some((0, 0)),
            visited_cells: [(0, 0)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "Water Level t=0".into(),
        },
    });

    steps.push(Step {
        description: "Swim in Rising Water: Water rises to t=3, path reaches (1,1)!".into(),
        code_line: 8,
        visual: VisualState::GridGraph {
            rows: 2,
            cols: 2,
            grid,
            active_cell: Some((1, 1)),
            visited_cells: [(0, 0), (1, 0), (1, 1)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "Reached Destination at Time t=3".into(),
        },
    });

    steps
}

pub fn generate_alien_dictionary_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![0, 1, 2, 3, 4];
    let labels = vec!["w".to_string(), "e".to_string(), "r".to_string(), "t".to_string(), "f".to_string()];
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: "Alien Dictionary DAG: Extract character precedence edges from word list".into(),
        code_line: 8,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Alien precedence DAG built".into(),
        },
    });

    for &u in &nodes {
        visited.insert(u);
        steps.push(Step {
            description: format!("Post-Order DFS: Visit alien character '{}'", labels[u]),
            code_line: 15,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(u),
                active_edge: None,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Processed character '{}'", labels[u]),
            },
        });
    }

    steps.push(Step {
        description: "Alien Dictionary Topological Sort Complete! Order: \"wertf\"".into(),
        code_line: 18,
        visual: VisualState::NodeGraph {
            nodes,
            node_labels: labels,
            edges,
            active_node: None,
            active_edge: None,
            visited_nodes: visited,
            cycle_edges: BTreeSet::new(),
            topo_order: vec![0, 1, 2, 3, 4],
            message: "Alien Alphabet = \"wertf\"".into(),
        },
    });

    steps
}

pub fn generate_cheapest_flights_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![0, 1, 2, 3];
    let labels = vec!["Src:0".to_string(), "1".to_string(), "2".to_string(), "Dst:3".to_string()];
    let edges = vec![(0, 1), (1, 2), (2, 0), (1, 3), (2, 3)];

    let mut visited = BTreeSet::new();
    visited.insert(0);

    steps.push(Step {
        description: "Bellman-Ford K-Stops: Iteration 1 (At most 1 stop allowed)".into(),
        code_line: 4,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: Some(0),
            active_edge: Some((0, 1)),
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Edge relaxation for K=1 stops".into(),
        },
    });

    visited.insert(1);
    visited.insert(3);
    steps.push(Step {
        description: "Cheapest Flight Path Found: 0 ➔ 1 ➔ 3 with 1 stop. Total Price = 700".into(),
        code_line: 10,
        visual: VisualState::NodeGraph {
            nodes,
            node_labels: labels,
            edges,
            active_node: Some(3),
            active_edge: Some((1, 3)),
            visited_nodes: visited,
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Cheapest Price = 700".into(),
        },
    });

    steps
}
