use crate::app::{TerminalQuizState, VisualizerApp};
use crate::model::*;

pub fn generate_offline_ai_response(app: &mut VisualizerApp, query: &str) -> String {
    let q = query.trim().to_lowercase();

    if q == "exit" || q == "quit" {
        if app.terminal_quiz_state != TerminalQuizState::Inactive {
            app.terminal_quiz_state = TerminalQuizState::Inactive;
            return "Exited quiz mode.".to_string();
        }
    }

    match app.terminal_quiz_state {
        TerminalQuizState::Inactive => match q.as_str() {
            "help" | "?" | "commands" => cmd_help(),
            "vars" | "state" | "variables" => cmd_vars(app),
            "code" | "line" | "source" => cmd_code(app),
            "formula" | "invariant" => cmd_formula(app),
            "hint" | "clue" => cmd_hint(app),
            "compare" | "vs" | "approaches" => cmd_compare(app),
            "pattern" | "concept" | "category" => cmd_pattern(app),
            "tips" | "protips" | "interview" => cmd_tips(app),
            "example" | "examples" | "sample" => cmd_example(app),
            "next" | "preview" => cmd_next(app),
            "quiz" => cmd_start_quiz(app),
            "clear" | "cls" | "reset" => String::new(), // Sent to UI to clear history
            _ => "Unknown command. Type 'help' for available commands.".to_string(),
        },
        TerminalQuizState::AskingTime(prob, approach) => handle_quiz_time(app, prob, approach, &q),
        TerminalQuizState::AskingSpace(prob, approach) => handle_quiz_space(app, prob, approach, &q),
    }
}

fn cmd_help() -> String {
    "Available commands:
  help    - Show this list
  vars    - Show live variables for current step
  code    - Show currently executing Python line
  formula - Show core algorithmic invariant
  hint    - Get a progressive hint
  compare - Compare all available approaches
  pattern - Explain the algorithmic pattern
  tips    - Show interview pro tips
  example - Show input/output examples
  next    - Preview the next step
  quiz    - Start an interactive complexity quiz
  clear   - Clear terminal history

  (Tip: Press Enter again to skip typing animation)"
        .to_string()
}

fn cmd_vars(app: &VisualizerApp) -> String {
    let Some(step) = app.steps.get(app.current_step_idx) else {
        return "No active step.".to_string();
    };

    let vars = step.visual.variables(app.selected_approach_id);
    if vars.is_empty() {
        return "No variables in scope.".to_string();
    }

    let mut out = String::from("Live Variables:\n");
    for (k, v) in vars {
        out.push_str(&format!("  {} = {}\n", k, v));
    }
    out.trim_end().to_string()
}

fn cmd_code(app: &VisualizerApp) -> String {
    let Some(step) = app.steps.get(app.current_step_idx) else {
        return "No active step.".to_string();
    };

    let lines = crate::model::approach_code_lines(app.current_problem, app.selected_approach_id);
    if let Some((num, text)) = lines.iter().find(|(num, _)| *num == step.code_line) {
        format!("Line {}:\n  {}", num, text)
    } else {
        "Code line not available for current step.".to_string()
    }
}

fn cmd_formula(app: &VisualizerApp) -> String {
    if let Some(f) = app
        .current_problem
        .formula_for_approach(app.selected_approach_id)
    {
        format!("Core Invariant:\n  {}", f)
    } else if let Some(f) = app.current_problem.formula() {
        format!("Core Invariant:\n  {}", f)
    } else {
        "No specific formula invariant defined for this approach.".to_string()
    }
}

fn cmd_hint(app: &mut VisualizerApp) -> String {
    let details = app.current_problem.details();
    let guide = get_category_guide(details.category);
    let approach = details
        .approach_by_id(app.selected_approach_id)
        .unwrap_or(&details.approaches[0]);

    app.hint_progress += 1;

    match app.hint_progress {
        1 => {
            let patterns = guide.key_patterns.join(", ");
            format!("Hint 1/3 (Pattern):\n  Consider using: {}", patterns)
        }
        2 => {
            format!("Hint 2/3 (Mechanics):\n  {}", approach.description)
        }
        _ => {
            format!("Hint 3/3 (Rationale):\n  {}", approach.rationale)
        }
    }
}

fn cmd_compare(app: &VisualizerApp) -> String {
    let details = app.current_problem.details();
    if details.approaches.is_empty() {
        return "No approaches available to compare.".to_string();
    }

    let mut out = String::from("Available Approaches:\n");
    for (i, a) in details.approaches.iter().enumerate() {
        out.push_str(&format!(
            "{}. {} (Time: {}, Space: {})\n",
            i + 1,
            a.name,
            a.time_complexity,
            a.space_complexity
        ));
    }
    out.trim_end().to_string()
}

fn cmd_pattern(app: &VisualizerApp) -> String {
    let details = app.current_problem.details();
    let guide = get_category_guide(details.category);
    format!(
        "Pattern: {:?}\n  {}\n\nHow it works:\n  {}",
        details.category, guide.summary, guide.how_it_works
    )
}

fn cmd_tips(app: &VisualizerApp) -> String {
    let details = app.current_problem.details();
    let guide = get_category_guide(details.category);

    if guide.pro_tips.is_empty() {
        return "No tips available for this category.".to_string();
    }

    let mut out = String::from("Interview Pro Tips:\n");
    for tip in guide.pro_tips {
        out.push_str(&format!("- {}\n", tip));
    }
    out.trim_end().to_string()
}

fn cmd_example(app: &VisualizerApp) -> String {
    let details = app.current_problem.details();
    if details.examples.is_empty() {
        return "No examples available.".to_string();
    }

    let ex = &details.examples[0]; // Show first example for brevity
    format!(
        "Example 1:\nInput: {}\nOutput: {}\nExplanation: {}",
        ex.input, ex.output, ex.explanation
    )
}

fn cmd_next(app: &VisualizerApp) -> String {
    if let Some(next_step) = app.steps.get(app.current_step_idx + 1) {
        format!("Next Step Preview:\n  {}", next_step.description)
    } else {
        "You are at the final step.".to_string()
    }
}

fn cmd_start_quiz(app: &mut VisualizerApp) -> String {
    let cat = app.current_problem.details().category;
    let mut candidates: Vec<Problem> = Problem::all()
        .iter()
        .copied()
        .filter(|&p| p.category() == cat && p != app.current_problem)
        .collect();

    if candidates.is_empty() {
        candidates.push(app.current_problem);
    }

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as usize;

    let target_prob = candidates[seed % candidates.len()];
    let approaches = target_prob.details().approaches;
    let approach_idx = seed % approaches.len();
    let target_approach = &approaches[approach_idx];

    app.terminal_quiz_state = TerminalQuizState::AskingTime(target_prob, target_approach.id);

    format!(
        "--- {} Mastery Quiz Started ---\n\nThink back to the problem \"{}\".\nIf you use the \"{}\" approach, what is the Time Complexity?\n(e.g., O(1), O(n), O(n log n), O(n^2))",
        cat.name(),
        target_prob.details().title,
        target_approach.name
    )
}

fn handle_quiz_time(app: &mut VisualizerApp, prob: Problem, approach_id: usize, answer: &str) -> String {
    let approach = prob.details().approach_by_id(approach_id).unwrap();
    let expected = approach.time_complexity.to_lowercase().replace(" ", "");
    let user_ans = answer.replace(" ", "");

    if expected == user_ans || user_ans.contains(&expected) {
        app.terminal_quiz_state = TerminalQuizState::AskingSpace(prob, approach_id);
        format!(
            "Correct! The time complexity is {}.\n\nNext question: What is the Space Complexity?",
            approach.time_complexity
        )
    } else {
        format!("Incorrect. Try again, or type 'exit' to leave quiz mode.\nHint: {}", approach.rationale)
    }
}

fn handle_quiz_space(app: &mut VisualizerApp, prob: Problem, approach_id: usize, answer: &str) -> String {
    let approach = prob.details().approach_by_id(approach_id).unwrap();
    let expected = approach.space_complexity.to_lowercase().replace(" ", "");
    let user_ans = answer.replace(" ", "");

    if expected == user_ans || user_ans.contains(&expected) {
        app.terminal_quiz_state = TerminalQuizState::Inactive;

        let cat = prob.details().category;
        let entry = app.quiz_scores.entry(cat).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += 1;

        format!("Correct! The space complexity is {}.\n\nYou have mastered this approach! Exited quiz mode.", approach.space_complexity)
    } else {
        format!("Incorrect. Try again, or type 'exit' to leave quiz mode.\nHint: {}", approach.rationale)
    }
}
