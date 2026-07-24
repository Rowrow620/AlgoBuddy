use crate::model::{Step, VisualState};

pub fn generate_best_time_stock_steps(prices: &[i32]) -> Vec<Step> {
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
    let mut max_p = 0;

    // 1. Init pointers (code_line 3-4)
    steps.push(Step {
        code_line: 3,
        description: format!("Initialized buy pointer l=0 (price={}) and sell pointer r=1 (price={}). maxP = 0.", prices[l], prices[r]),
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
            let profit = sell_price - buy_price;
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
            steps.push(Step {
                code_line: 10,
                description: format!(
                    "Loss or zero profit: Buy day {} ({}) >= Sell day {} ({}). Shifted buying day l to r (l={}).",
                    l, buy_price, r, sell_price, r
                ),
                visual: VisualState::BestTimeStock {
                    prices: prices_vec.clone(),
                    left_buy: l,
                    right_sell: r,
                    current_profit: 0,
                    max_profit: max_p,
                },
            });
            l = r;
        }

        r += 1;
        if r < prices.len() {
            steps.push(Step {
                code_line: 11,
                description: format!("Advanced sell pointer r to day {} (price={}).", r, prices[r]),
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
        description: format!("Completed scanning prices. Maximum achievable profit = {}.", max_p),
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
