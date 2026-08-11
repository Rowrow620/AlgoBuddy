use crate::model::{Step, VisualState};

pub(crate) const BRUTE_FORCE_VISUALIZATION_LIMIT: usize = 40;
pub(crate) const SLIDING_WINDOW_VISUALIZATION_LIMIT: usize = 512;

pub fn generate_best_time_stock_steps(prices: &[i32], approach_id: usize) -> Vec<Step> {
    let limit = if approach_id == 1 {
        BRUTE_FORCE_VISUALIZATION_LIMIT
    } else {
        SLIDING_WINDOW_VISUALIZATION_LIMIT
    };
    if prices.len() > limit {
        let approach = if approach_id == 1 {
            "Brute Force Pair Scan"
        } else {
            "Sliding Window"
        };
        let message = format!(
            "{approach} traces accept at most {limit} prices because each step stores an array snapshot."
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    match approach_id {
        0 => generate_best_time_stock_sliding_window(prices),
        1 => generate_best_time_stock_brute_force(prices),
        _ => Vec::new(),
    }
}

fn generate_best_time_stock_sliding_window(prices: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let prices_vec = prices.to_vec();

    if prices.len() < 2 {
        steps.push(Step {
            code_line: 12,
            description: "Array length < 2. Cannot buy and sell. Max profit = 0.".to_string(),
            visual: VisualState::BestTimeStock {
                prices: prices_vec,
                left_buy: 0,
                right_sell: 0,
                current_profit: 0,
                max_profit: 0,
            },
        });
        return steps;
    }

    let mut l = 0;
    let mut r = 1;
    let mut max_p = 0i64;

    // 1. Init pointers (code_line 3-4)
    steps.push(Step {
        code_line: 3,
        description: format!(
            "Initialized buy pointer l=0 (price={}) and sell pointer r=1 (price={}). maxP = 0.",
            prices[l], prices[r]
        ),
        visual: VisualState::BestTimeStock {
            prices: prices_vec.clone(),
            left_buy: l,
            right_sell: r,
            current_profit: 0,
            max_profit: max_p,
        },
    });

    // 2. Loop r < len(prices) (code_line 5)
    while r < prices.len() {
        let buy_price = prices[l];
        let sell_price = prices[r];

        if buy_price < sell_price {
            let profit = i64::from(sell_price) - i64::from(buy_price);
            max_p = max_p.max(profit);

            steps.push(Step {
                code_line: 7,
                description: format!(
                    "Profitable transaction: Buy at day {} ({}), Sell at day {} ({}). Profit = {} - {} = {}. Updated maxP = {}.",
                    l, buy_price, r, sell_price, sell_price, buy_price, profit, max_p
                ),
                visual: VisualState::BestTimeStock {
                    prices: prices_vec.clone(),
                    left_buy: l,
                    right_sell: r,
                    current_profit: profit,
                    max_profit: max_p,
                },
            });
        } else {
            let previous_buy = l;
            l = r;
            steps.push(Step {
                code_line: 10,
                description: format!(
                    "Loss or zero profit: Buy day {} ({}) >= Sell day {} ({}). Shifted buying day l to r (l={}).",
                    previous_buy, buy_price, r, sell_price, l
                ),
                visual: VisualState::BestTimeStock {
                    prices: prices_vec.clone(),
                    left_buy: l,
                    right_sell: r,
                    current_profit: 0,
                    max_profit: max_p,
                },
            });
        }

        r += 1;
        if r < prices.len() {
            steps.push(Step {
                code_line: 11,
                description: format!(
                    "Advanced sell pointer r to day {} (price={}).",
                    r, prices[r]
                ),
                visual: VisualState::BestTimeStock {
                    prices: prices_vec.clone(),
                    left_buy: l,
                    right_sell: r,
                    current_profit: 0,
                    max_profit: max_p,
                },
            });
        }
    }

    steps.push(Step {
        code_line: 12,
        description: format!(
            "Completed scanning prices. Maximum achievable profit = {}.",
            max_p
        ),
        visual: VisualState::BestTimeStock {
            prices: prices_vec,
            left_buy: l,
            right_sell: prices.len() - 1,
            current_profit: 0,
            max_profit: max_p,
        },
    });

    steps
}

fn generate_best_time_stock_brute_force(prices: &[i32]) -> Vec<Step> {
    let prices_vec = prices.to_vec();
    if prices.len() < 2 {
        return vec![Step {
            code_line: 8,
            description: "Fewer than two prices means no transaction is possible. Return 0."
                .to_string(),
            visual: VisualState::BestTimeStock {
                prices: prices_vec,
                left_buy: 0,
                right_sell: 0,
                current_profit: 0,
                max_profit: 0,
            },
        }];
    }

    let mut steps = Vec::new();
    let mut max_profit = 0i64;
    let mut best_buy = 0;
    let mut best_sell = 1;

    steps.push(Step {
        code_line: 3,
        description: "Initialized max_profit = 0 before checking every buy/sell pair.".to_string(),
        visual: VisualState::BestTimeStock {
            prices: prices_vec.clone(),
            left_buy: best_buy,
            right_sell: best_sell,
            current_profit: 0,
            max_profit,
        },
    });

    for buy in 0..prices.len() {
        for sell in (buy + 1)..prices.len() {
            let profit = i64::from(prices[sell]) - i64::from(prices[buy]);
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Buy on day {} at {} and sell on day {} at {}: profit = {}.",
                    buy, prices[buy], sell, prices[sell], profit
                ),
                visual: VisualState::BestTimeStock {
                    prices: prices_vec.clone(),
                    left_buy: buy,
                    right_sell: sell,
                    current_profit: profit,
                    max_profit,
                },
            });

            if profit > max_profit {
                max_profit = profit;
                best_buy = buy;
                best_sell = sell;
            }

            steps.push(Step {
                code_line: 7,
                description: format!(
                    "Compared this transaction with the current best. max_profit = {}.",
                    max_profit
                ),
                visual: VisualState::BestTimeStock {
                    prices: prices_vec.clone(),
                    left_buy: buy,
                    right_sell: sell,
                    current_profit: profit,
                    max_profit,
                },
            });
        }
    }

    let selected_pair_profit = i64::from(prices[best_sell]) - i64::from(prices[best_buy]);
    steps.push(Step {
        code_line: 8,
        description: format!("Checked every valid transaction. Return {}.", max_profit),
        visual: VisualState::BestTimeStock {
            prices: prices_vec,
            left_buy: best_buy,
            right_sell: best_sell,
            current_profit: selected_pair_profit,
            max_profit,
        },
    });

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_max_profit(steps: &[Step]) -> Option<i64> {
        match &steps.last()?.visual {
            VisualState::BestTimeStock { max_profit, .. } => Some(*max_profit),
            VisualState::TraceUnavailable { .. } => None,
            _ => None,
        }
    }

    #[test]
    fn both_approaches_find_the_same_profit() {
        for prices in [vec![7, 1, 5, 3, 6, 4], vec![7, 6, 4, 3, 1], vec![2, 4, 1]] {
            let sliding_window = generate_best_time_stock_steps(&prices, 0);
            let brute_force = generate_best_time_stock_steps(&prices, 1);
            assert_eq!(
                final_max_profit(&sliding_window),
                final_max_profit(&brute_force)
            );
        }

        let extremes = [i32::MIN, i32::MAX];
        for approach_id in [0, 1] {
            assert_eq!(
                final_max_profit(&generate_best_time_stock_steps(&extremes, approach_id)),
                Some(4_294_967_295)
            );
        }
    }

    #[test]
    fn brute_force_trace_rejects_oversized_inputs() {
        let prices = vec![1; BRUTE_FORCE_VISUALIZATION_LIMIT + 1];
        let steps = generate_best_time_stock_steps(&prices, 1);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }

    #[test]
    fn sliding_window_trace_rejects_oversized_inputs() {
        let prices = vec![1; SLIDING_WINDOW_VISUALIZATION_LIMIT + 1];
        assert!(matches!(
            generate_best_time_stock_steps(&prices, 0).as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }

    #[test]
    fn brute_force_final_state_keeps_no_transaction_separate_from_pair_profit() {
        let steps = generate_best_time_stock_steps(&[7, 6], 1);
        assert!(matches!(
            &steps.last().expect("trace must not be empty").visual,
            VisualState::BestTimeStock {
                current_profit: -1,
                max_profit: 0,
                ..
            }
        ));
    }
}
