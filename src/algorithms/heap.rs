use crate::model::{Step, VisualState};

pub fn generate_last_stone_weight_steps(stones: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut heap: Vec<i32> = stones.to_vec();
    heap.sort_by(|a, b| b.cmp(a)); // Max-heap ordering for demonstration

    steps.push(Step {
        description: format!("Initialize Max-Heap with stones: {:?}", heap),
        code_line: 1,
        visual: VisualState::HeapVisual {
            heap_elements: heap.clone(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Max-Heap".into(),
        },
    });

    while heap.len() > 1 {
        let y = heap.remove(0);
        let x = heap.remove(0);

        steps.push(Step {
            description: format!("Smash heaviest stones y = {} and x = {}", y, x),
            code_line: 5,
            visual: VisualState::HeapVisual {
                heap_elements: heap.clone(),
                active_idx: None,
                swapped_pair: None,
                heap_type_label: format!("Smashing {} vs {}", y, x),
            },
        });

        if y != x {
            let rem = y - x;
            heap.push(rem);
            heap.sort_by(|a, b| b.cmp(a));
            steps.push(Step {
                description: format!("Stone remaining: {} -> push into Max-Heap {:?}", rem, heap),
                code_line: 8,
                visual: VisualState::HeapVisual {
                    heap_elements: heap.clone(),
                    active_idx: Some(0),
                    swapped_pair: None,
                    heap_type_label: "Max-Heap".into(),
                },
            });
        }
    }

    let final_res = heap.first().copied().unwrap_or(0);
    steps.push(Step {
        description: format!("Smashing complete! Last stone weight: {}", final_res),
        code_line: 12,
        visual: VisualState::HeapVisual {
            heap_elements: heap.clone(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: format!("Final Stone Weight: {}", final_res),
        },
    });

    steps
}

pub fn generate_kth_largest_array_steps(nums: &[i32], k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut heap = nums.to_vec();
    heap.sort_by(|a, b| b.cmp(a));

    steps.push(Step {
        description: format!("Build Max-Heap for array {:?} (k = {})", nums, k),
        code_line: 1,
        visual: VisualState::HeapVisual {
            heap_elements: heap.clone(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Max-Heap".into(),
        },
    });

    for i in 1..k {
        let max_val = heap.remove(0);
        steps.push(Step {
            description: format!("Pop max element #{} = {} from heap -> remaining heap: {:?}", i, max_val, heap),
            code_line: 4,
            visual: VisualState::HeapVisual {
                heap_elements: heap.clone(),
                active_idx: Some(0),
                swapped_pair: None,
                heap_type_label: format!("Popped #{}", i),
            },
        });
    }

    let kth = heap.first().copied().unwrap_or(0);
    steps.push(Step {
        description: format!("Top element is the {}th largest element: {}", k, kth),
        code_line: 8,
        visual: VisualState::HeapVisual {
            heap_elements: heap.clone(),
            active_idx: Some(0),
            swapped_pair: None,
            heap_type_label: format!("{}th Largest: {}", k, kth),
        },
    });

    steps
}

pub fn generate_k_closest_points_steps(points: &[(i32, i32)], k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut pts_dist: Vec<((i32, i32), i32)> = points.iter().map(|&(x, y)| ((x, y), x * x + y * y)).collect();
    pts_dist.sort_by_key(|&(_, d)| d);

    let distances: Vec<i32> = pts_dist.iter().map(|&(_, d)| d).collect();

    steps.push(Step {
        description: format!("Compute squared distances d = x^2 + y^2 for points: {:?}", points),
        code_line: 5,
        visual: VisualState::HeapVisual {
            heap_elements: distances.clone(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Min-Heap Distances".into(),
        },
    });

    let closest: Vec<(i32, i32)> = pts_dist.iter().take(k).map(|&(pt, _)| pt).collect();
    steps.push(Step {
        description: format!("Min-Heap extraction: Top {} closest points to origin: {:?}", k, closest),
        code_line: 10,
        visual: VisualState::HeapVisual {
            heap_elements: distances.iter().take(k).copied().collect(),
            active_idx: Some(0),
            swapped_pair: None,
            heap_type_label: format!("Top {} Closest Points", k),
        },
    });

    steps
}

pub fn generate_task_scheduler_steps(tasks: &[char], _n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut counts = std::collections::HashMap::new();
    for &t in tasks {
        *counts.entry(t).or_insert(0) += 1;
    }

    let mut freq_heap: Vec<i32> = counts.values().copied().collect();
    freq_heap.sort_by(|a, b| b.cmp(a));

    steps.push(Step {
        description: format!("Build Max-Heap of task frequencies: {:?}", counts),
        code_line: 4,
        visual: VisualState::HeapVisual {
            heap_elements: freq_heap.clone(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Task Frequency Heap".into(),
        },
    });

    steps.push(Step {
        description: format!("Schedule tasks maximizing CPU idle cooling slots: Total intervals computed"),
        code_line: 9,
        visual: VisualState::HeapVisual {
            heap_elements: freq_heap,
            active_idx: Some(0),
            swapped_pair: None,
            heap_type_label: "Scheduled Frequency Priority".into(),
        },
    });

    steps
}
