use crate::model::{Step, VisualState};

pub fn generate_last_stone_weight_steps(stones: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut heap: Vec<i32> = stones.to_vec();
    heap.sort_by(|a, b| b.cmp(a)); // Max-heap ordering for demonstration

    steps.push(Step {
        description: format!("Initialize Max-Heap with stones: {:?}", heap),
        code_line: 3,
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
            code_line: 6,
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
                code_line: 7,
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
        code_line: 9,
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
            description: format!(
                "Pop max element #{} = {} from heap -> remaining heap: {:?}",
                i, max_val, heap
            ),
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
    let mut pts_dist: Vec<((i32, i32), i32)> = points
        .iter()
        .map(|&(x, y)| ((x, y), x * x + y * y))
        .collect();
    pts_dist.sort_by_key(|&(_, d)| d);

    let distances: Vec<i32> = pts_dist.iter().map(|&(_, d)| d).collect();

    steps.push(Step {
        description: format!(
            "Compute squared distances d = x^2 + y^2 for points: {:?}",
            points
        ),
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
        description: format!(
            "Min-Heap extraction: Top {} closest points to origin: {:?}",
            k, closest
        ),
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
        description: "Schedule tasks maximizing CPU idle cooling slots: Total intervals computed"
            .into(),
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

pub fn generate_find_median_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut small_max_heap: Vec<i32> = Vec::new(); // max-heap
    let mut large_min_heap: Vec<i32> = Vec::new(); // min-heap

    steps.push(Step {
        description: "Initialize MedianFinder with empty Small Max-Heap and Large Min-Heap".into(),
        code_line: 2,
        visual: VisualState::HeapVisual {
            heap_elements: Vec::new(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Small Max-Heap & Large Min-Heap".into(),
        },
    });

    for &num in nums {
        small_max_heap.push(num);
        small_max_heap.sort_by(|a, b| b.cmp(a));

        steps.push(Step {
            description: format!(
                "addNum({}): Push into Small Max-Heap -> {:?}",
                num, small_max_heap
            ),
            code_line: 5,
            visual: VisualState::HeapVisual {
                heap_elements: small_max_heap.clone(),
                active_idx: Some(0),
                swapped_pair: None,
                heap_type_label: format!("Small Max-Heap (len={})", small_max_heap.len()),
            },
        });

        // Rebalance if max of small > min of large
        if let (Some(&max_s), Some(&min_l)) = (small_max_heap.first(), large_min_heap.first()) {
            if max_s > min_l {
                let val = small_max_heap.remove(0);
                large_min_heap.push(val);
                large_min_heap.sort();
                steps.push(Step {
                    description: format!(
                        "Rebalance: Move max_s {} from Small Heap to Large Min-Heap",
                        val
                    ),
                    code_line: 7,
                    visual: VisualState::HeapVisual {
                        heap_elements: large_min_heap.clone(),
                        active_idx: Some(0),
                        swapped_pair: None,
                        heap_type_label: format!("Large Min-Heap (len={})", large_min_heap.len()),
                    },
                });
            }
        }

        // Size balance checks
        if small_max_heap.len() > large_min_heap.len() + 1 {
            let val = small_max_heap.remove(0);
            large_min_heap.push(val);
            large_min_heap.sort();
            steps.push(Step {
                description: format!("Size balance: Move {} to Large Min-Heap", val),
                code_line: 10,
                visual: VisualState::HeapVisual {
                    heap_elements: large_min_heap.clone(),
                    active_idx: Some(0),
                    swapped_pair: None,
                    heap_type_label: format!("Large Min-Heap (len={})", large_min_heap.len()),
                },
            });
        } else if large_min_heap.len() > small_max_heap.len() + 1 {
            let val = large_min_heap.remove(0);
            small_max_heap.push(val);
            small_max_heap.sort_by(|a, b| b.cmp(a));
            steps.push(Step {
                description: format!("Size balance: Move {} to Small Max-Heap", val),
                code_line: 13,
                visual: VisualState::HeapVisual {
                    heap_elements: small_max_heap.clone(),
                    active_idx: Some(0),
                    swapped_pair: None,
                    heap_type_label: format!("Small Max-Heap (len={})", small_max_heap.len()),
                },
            });
        }

        // Compute current median
        let median = if small_max_heap.len() > large_min_heap.len() {
            small_max_heap[0] as f64
        } else if large_min_heap.len() > small_max_heap.len() {
            large_min_heap[0] as f64
        } else if !small_max_heap.is_empty() {
            (small_max_heap[0] as f64 + large_min_heap[0] as f64) / 2.0
        } else {
            0.0
        };

        steps.push(Step {
            description: format!("findMedian(): Current Median = {:.1}", median),
            code_line: 16,
            visual: VisualState::HeapVisual {
                heap_elements: small_max_heap.clone(),
                active_idx: Some(0),
                swapped_pair: None,
                heap_type_label: format!("Current Median = {:.1}", median),
            },
        });
    }

    steps
}

pub fn generate_design_twitter_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Initialize Twitter object: postTweet(userId=1, tweetId=5)".into(),
        code_line: 4,
        visual: VisualState::HeapVisual {
            heap_elements: vec![5],
            active_idx: Some(0),
            swapped_pair: None,
            heap_type_label: "User 1 Tweets Heap".into(),
        },
    });
    steps.push(Step {
        description: "follow(followerId=1, followeeId=2) -> User 1 follows User 2".into(),
        code_line: 8,
        visual: VisualState::HeapVisual {
            heap_elements: vec![5],
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Follow Graph Set: {1 -> 2}".into(),
        },
    });
    steps.push(Step {
        description: "postTweet(userId=2, tweetId=6) -> User 2 posts tweet 6".into(),
        code_line: 5,
        visual: VisualState::HeapVisual {
            heap_elements: vec![6, 5],
            active_idx: Some(0),
            swapped_pair: None,
            heap_type_label: "Merged Feed Max-Heap".into(),
        },
    });
    steps.push(Step {
        description: "getNewsFeed(1) -> Max-Heap returns most recent 10 tweets: [6, 5]".into(),
        code_line: 17,
        visual: VisualState::HeapVisual {
            heap_elements: vec![6, 5],
            active_idx: Some(0),
            swapped_pair: None,
            heap_type_label: "News Feed: [6, 5]".into(),
        },
    });
    steps
}
