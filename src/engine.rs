use crate::app::VisualizerApp;
use crate::model::*;

mod dispatch;
mod input;

pub(crate) fn recompute_steps(app: &mut VisualizerApp) {
    let details = app.current_problem.details();
    if details.approach_by_id(app.selected_approach_id).is_none() {
        app.selected_approach_id = details.default_approach_id();
    }
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
mod catalog_tests;

pub(crate) fn select_problem(app: &mut VisualizerApp, problem: Problem) {
    if app.current_problem != problem {
        app.current_problem = problem;
        app.selected_approach_id = problem.details().default_approach_id();
        app.recompute_steps();
    }
}

pub(crate) fn select_approach(app: &mut VisualizerApp, approach_id: usize) -> bool {
    if app.selected_approach_id == approach_id
        || app
            .current_problem
            .details()
            .approach_by_id(approach_id)
            .is_none()
    {
        return false;
    }

    app.selected_approach_id = approach_id;
    app.recompute_steps();
    true
}
