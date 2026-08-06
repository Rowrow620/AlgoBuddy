use crate::app::VisualizerApp;
use crate::model::*;

mod dispatch;
mod input;

pub(crate) fn recompute_steps(app: &mut VisualizerApp) {
    app.steps = dispatch::generate_steps(app);
    reset_playback(app);
}

fn reset_playback(app: &mut VisualizerApp) {
    app.current_step_idx = 0;
    app.last_focused_step_idx = None;
    app.is_playing = false;
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests;

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod audit_tests;

pub(crate) fn select_problem(app: &mut VisualizerApp, problem: Problem) {
    if app.current_problem != problem {
        app.current_problem = problem;
        app.selected_approach_id = 0;
        app.recompute_steps();
    }
}
