use eframe::egui;
use web_time::Instant;

use crate::model::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightTab {
    CodeTrace,
    ProblemDetails,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Visualizer,
    RoadmapDashboard,
}

pub struct VisualizerApp {
    // Theme & Settings
    pub(crate) theme: Theme,
    pub(crate) colorblind_mode: ColorblindMode,
    pub(crate) show_settings_modal: bool,
    pub(crate) show_reset_confirm_modal: bool,
    pub(crate) show_unaudited: bool,
    pub(crate) view_mode: ViewMode,
    pub(crate) completed_problems: std::collections::HashSet<u32>,
    pub(crate) favorite_problems: std::collections::HashSet<u32>,

    // Navigation state & Sidebar Visibility
    pub(crate) show_roadmap_sidebar: bool,
    pub(crate) show_right_sidebar: bool,
    pub(crate) current_problem: Problem,
    pub(crate) selected_approach_id: usize,
    pub(crate) selected_difficulty: Option<Difficulty>,
    pub(crate) search_query: String,
    pub(crate) right_tab: RightTab,

    // Dynamic input state maps
    pub(crate) input_strings: std::collections::HashMap<(Problem, &'static str), String>,
    pub(crate) input_integers: std::collections::HashMap<(Problem, &'static str), i32>,

    pub(crate) sudoku_preset_valid: bool,

    // Playback state
    pub(crate) steps: Vec<Step>,
    pub(crate) current_step_idx: usize,
    pub(crate) is_playing: bool,
    pub(crate) playback_speed_ms: u64,
    pub(crate) last_step_time: Instant,

    pub(crate) canvas_zoom: f32,
    pub(crate) last_focused_step_idx: Option<usize>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) is_fullscreen: bool,
}

impl Default for VisualizerApp {
    fn default() -> Self {
        let mut app = Self {
            theme: Theme::DarkVSCode, // Default to user's favorite VS Code Dark style!
            colorblind_mode: ColorblindMode::Off,
            show_settings_modal: false,
            show_reset_confirm_modal: false,
            show_unaudited: false, // Default: Only show 100% Audited problems in Public Release!
            view_mode: ViewMode::Visualizer,
            completed_problems: std::collections::HashSet::new(),
            favorite_problems: std::collections::HashSet::new(),

            show_roadmap_sidebar: true,
            show_right_sidebar: true,
            current_problem: Problem::ContainsDuplicate,
            selected_approach_id: 0,
            selected_difficulty: None,
            search_query: String::new(),
            right_tab: RightTab::CodeTrace,

            input_strings: std::collections::HashMap::new(),
            input_integers: std::collections::HashMap::new(),
            sudoku_preset_valid: true,

            steps: Vec::new(),

            current_step_idx: 0,
            is_playing: false,
            playback_speed_ms: 500,
            last_step_time: Instant::now(),

            canvas_zoom: 1.0,
            last_focused_step_idx: None,
            #[cfg(not(target_arch = "wasm32"))]
            is_fullscreen: false,
        };

        app.recompute_steps();
        app
    }
}

impl VisualizerApp {
    // ── Input State Map Helpers ──

    pub fn get_input_str<'a>(
        &'a self,
        problem: Problem,
        key: &'static str,
        fallback: &'a str,
    ) -> &'a str {
        self.input_strings
            .get(&(problem, key))
            .map(|s| s.as_str())
            .unwrap_or(fallback)
    }

    pub fn get_input_str_mut(
        &mut self,
        problem: Problem,
        key: &'static str,
        fallback: &str,
    ) -> &mut String {
        self.input_strings
            .entry((problem, key))
            .or_insert_with(|| fallback.to_string())
    }

    pub fn set_input_str(&mut self, problem: Problem, key: &'static str, val: impl Into<String>) {
        self.input_strings.insert((problem, key), val.into());
    }

    pub fn get_input_int(&self, problem: Problem, key: &'static str, fallback: i32) -> i32 {
        self.input_integers
            .get(&(problem, key))
            .copied()
            .unwrap_or(fallback)
    }

    pub fn get_input_int_mut(
        &mut self,
        problem: Problem,
        key: &'static str,
        fallback: i32,
    ) -> &mut i32 {
        self.input_integers
            .entry((problem, key))
            .or_insert(fallback)
    }

    pub fn set_input_int(&mut self, problem: Problem, key: &'static str, val: i32) {
        self.input_integers.insert((problem, key), val);
    }

    pub fn set_show_unaudited(&mut self, show: bool) {
        self.show_unaudited = show;
    }

    pub fn visible_problems(&self) -> Vec<Problem> {
        Problem::all()
            .iter()
            .copied()
            .filter(|p| p.is_audited() || self.show_unaudited)
            .collect()
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(target_arch = "wasm32")]
        if let Some(loading) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"))
        {
            loading.remove();
        }

        let mut app = Self::default();
        if let Some(storage) = cc.storage {
            if let Some(saved_completed) = eframe::get_value::<std::collections::HashSet<u32>>(
                storage,
                "algobuddy_completed_problems",
            ) {
                app.completed_problems = saved_completed;
            }
            if let Some(saved_favs) = eframe::get_value::<std::collections::HashSet<u32>>(
                storage,
                "algobuddy_favorite_problems",
            ) {
                app.favorite_problems = saved_favs;
            }
            if let Some(theme) = eframe::get_value::<Theme>(storage, "algobuddy_theme") {
                app.theme = theme;
            }
            if let Some(mode) =
                eframe::get_value::<ColorblindMode>(storage, "algobuddy_colorblind_mode")
            {
                app.colorblind_mode = mode;
            }
            if let Some(speed) = eframe::get_value::<u64>(storage, "algobuddy_playback_speed_ms") {
                app.playback_speed_ms = speed;
            }
            if let Some(show_left) =
                eframe::get_value::<bool>(storage, "algobuddy_show_roadmap_sidebar")
            {
                app.show_roadmap_sidebar = show_left;
            }
            if let Some(show_right) =
                eframe::get_value::<bool>(storage, "algobuddy_show_right_sidebar")
            {
                app.show_right_sidebar = show_right;
            }
            if let Some(show_un) = eframe::get_value::<bool>(storage, "algobuddy_show_unaudited") {
                app.show_unaudited = show_un;
            }
        }
        app
    }

    pub(crate) fn current_palette(&self) -> ThemePalette {
        self.theme.palette(self.colorblind_mode)
    }

    pub(crate) fn parse_tree_input(&self) -> Vec<Option<i32>> {
        let input_str =
            self.get_input_str(self.current_problem, "tree_nodes", "1, 2, 3, 4, 5, 6, 7");
        crate::utils::parse_tree_nodes(
            input_str,
            &[
                Some(4),
                Some(2),
                Some(7),
                Some(1),
                Some(3),
                Some(6),
                Some(9),
            ],
        )
    }

    pub(crate) fn get_sudoku_board(&self) -> [[char; 9]; 9] {
        if self.sudoku_preset_valid {
            [
                ['1', '2', '.', '.', '3', '.', '.', '.', '.'],
                ['4', '.', '.', '5', '.', '.', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '.', '3'],
                ['5', '.', '.', '.', '6', '.', '.', '.', '4'],
                ['.', '.', '.', '8', '.', '3', '.', '.', '5'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '.', '.', '.', '.', '.', '2', '.', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '8'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]
        } else {
            [
                ['1', '2', '.', '.', '3', '.', '.', '.', '.'],
                ['4', '.', '.', '5', '.', '.', '.', '.', '.'],
                ['.', '9', '1', '.', '.', '.', '.', '.', '3'],
                ['5', '.', '.', '.', '6', '.', '.', '.', '4'],
                ['.', '.', '.', '8', '.', '3', '.', '.', '5'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '.', '.', '.', '.', '.', '2', '.', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '8'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]
        }
    }

    pub(crate) fn recompute_steps(&mut self) {
        crate::engine::recompute_steps(self);
    }

    pub(crate) fn select_problem(&mut self, problem: Problem) {
        crate::engine::select_problem(self, problem);
    }
}

impl eframe::App for VisualizerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(
            storage,
            "algobuddy_completed_problems",
            &self.completed_problems,
        );
        eframe::set_value(
            storage,
            "algobuddy_favorite_problems",
            &self.favorite_problems,
        );
        eframe::set_value(storage, "algobuddy_theme", &self.theme);
        eframe::set_value(storage, "algobuddy_colorblind_mode", &self.colorblind_mode);
        eframe::set_value(
            storage,
            "algobuddy_playback_speed_ms",
            &self.playback_speed_ms,
        );
        eframe::set_value(
            storage,
            "algobuddy_show_roadmap_sidebar",
            &self.show_roadmap_sidebar,
        );
        eframe::set_value(
            storage,
            "algobuddy_show_right_sidebar",
            &self.show_right_sidebar,
        );
        eframe::set_value(storage, "algobuddy_show_unaudited", &self.show_unaudited);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── Keyboard Shortcuts (Only active when not typing in text fields) ──
        #[cfg(not(target_arch = "wasm32"))]
        let mut toggle_fs = false;
        if !ctx.wants_keyboard_input() {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Space) {
                    if self.current_step_idx >= self.steps.len().saturating_sub(1) {
                        self.current_step_idx = 0;
                    }
                    self.is_playing = !self.is_playing;
                    self.last_step_time = Instant::now();
                }
                if i.key_pressed(egui::Key::ArrowLeft) {
                    self.is_playing = false;
                    self.current_step_idx = self.current_step_idx.saturating_sub(1);
                }
                if i.key_pressed(egui::Key::ArrowRight) {
                    self.is_playing = false;
                    if self.current_step_idx < self.steps.len().saturating_sub(1) {
                        self.current_step_idx += 1;
                    }
                }
                if i.key_pressed(egui::Key::R) {
                    self.is_playing = false;
                    self.current_step_idx = 0;
                }
                if i.key_pressed(egui::Key::ArrowUp)
                    || i.key_pressed(egui::Key::Plus)
                    || i.key_pressed(egui::Key::Equals)
                {
                    self.playback_speed_ms = self.playback_speed_ms.saturating_sub(100).max(100);
                }
                if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::Minus) {
                    self.playback_speed_ms = (self.playback_speed_ms + 100).min(1500);
                }
                #[cfg(not(target_arch = "wasm32"))]
                if i.key_pressed(egui::Key::F11) {
                    toggle_fs = true;
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        if toggle_fs {
            self.is_fullscreen = !self.is_fullscreen;
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.is_fullscreen));
        }

        if self.is_playing {
            if self.last_step_time.elapsed().as_millis() as u64 >= self.playback_speed_ms {
                if self.current_step_idx < self.steps.len().saturating_sub(1) {
                    self.current_step_idx += 1;
                } else {
                    self.is_playing = false;
                }
                self.last_step_time = Instant::now();
            }
            ctx.request_repaint();
        }

        let p = self.current_palette();

        crate::ui::modals::render_settings_modal(self, ctx);
        crate::ui::modals::render_reset_confirm_modal(self, ctx);

        if self.view_mode == ViewMode::RoadmapDashboard {
            crate::ui::sidebar::render_roadmap_sidebar(self, ctx, &p);
            return;
        }

        if self.show_roadmap_sidebar {
            crate::ui::sidebar::render_roadmap_sidebar(self, ctx, &p);
        }

        // ── Top Header Panel ──
        crate::ui::header::render_header_panel(self, ctx, &p);

        // ── Right Sidebar: Tabbed Code Trace & Problem Details ──
        crate::ui::inspector::render_right_sidebar_inspector(self, ctx, &p);
        // ── Central Canvas ──
        self.render_central_canvas(ctx, &p);
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    #[test]
    fn test_all_problems_recompute_steps() {
        let mut app = VisualizerApp::default();
        let all_problems = Problem::all();

        // Verify the number of implemented problems dynamically
        assert_eq!(
            all_problems.len(),
            150,
            "Expected 150 problems in Problem::all()!"
        );

        let mut failed_problems = Vec::new();

        for &problem in all_problems {
            app.current_problem = problem;
            // Test primary approach (0) and secondary approach (1)
            for approach_id in 0..=1 {
                app.selected_approach_id = approach_id;
                app.recompute_steps();

                if app.steps.is_empty() {
                    failed_problems.push(format!(
                        "{:?} (approach {}): steps vector is empty",
                        problem, approach_id
                    ));
                    continue;
                }

                for (idx, step) in app.steps.iter().enumerate() {
                    if step.description.trim().is_empty() {
                        failed_problems.push(format!(
                            "{:?} (approach {}, step {}): empty description",
                            problem, approach_id, idx
                        ));
                    }
                }
            }
        }

        if !failed_problems.is_empty() {
            panic!(
                "Problem step generation failed for {} problem/approach combinations:\n{}",
                failed_problems.len(),
                failed_problems.join("\n")
            );
        }
    }

    #[test]
    fn test_two_sum_logic_correctness() {
        let mut app = VisualizerApp::default();
        app.current_problem = Problem::TwoSum;
        app.set_input_str(Problem::TwoSum, "nums", "2, 7, 11, 15");
        app.set_input_int(Problem::TwoSum, "target", 9);
        app.selected_approach_id = 0; // Hash map
        app.recompute_steps();

        let last_step = app.steps.last().expect("Steps should not be empty");
        if let VisualState::TwoSum { found_indices, .. } = &last_step.visual {
            assert_eq!(
                *found_indices,
                Some((0, 1)),
                "Two Sum failed to find solution pair (0, 1)"
            );
        } else {
            panic!("Expected VisualState::TwoSum");
        }
    }

    #[test]
    fn test_contains_duplicate_logic_correctness() {
        let mut app = VisualizerApp::default();
        app.current_problem = Problem::ContainsDuplicate;
        app.set_input_str(Problem::ContainsDuplicate, "nums", "1, 2, 3, 1");
        app.recompute_steps();

        let last_step = app.steps.last().expect("Steps should not be empty");
        if let VisualState::ContainsDuplicate {
            has_duplicate,
            duplicate_val,
            ..
        } = &last_step.visual
        {
            assert_eq!(
                *has_duplicate,
                Some(true),
                "Contains Duplicate should return true for [1, 2, 3, 1]"
            );
            assert_eq!(*duplicate_val, Some(1), "Duplicate value should be 1");
        } else {
            panic!("Expected VisualState::ContainsDuplicate");
        }
    }

    #[test]
    fn test_valid_anagram_logic_correctness() {
        let mut app = VisualizerApp::default();
        app.current_problem = Problem::ValidAnagram;
        app.set_input_str(Problem::ValidAnagram, "s", "anagram");
        app.set_input_str(Problem::ValidAnagram, "t", "nagaram");
        app.recompute_steps();

        let last_step = app.steps.last().expect("Steps should not be empty");
        assert!(
            last_step.description.contains("VALID")
                || last_step.description.contains("true")
                || last_step.description.contains("match"),
            "Valid Anagram description should indicate a valid anagram match"
        );
    }

    #[test]
    fn test_boundary_and_edge_case_safety() {
        let mut app = VisualizerApp::default();

        // Edge Case 1: Empty input string fallback for Contains Duplicate
        app.current_problem = Problem::ContainsDuplicate;
        app.set_input_str(Problem::ContainsDuplicate, "nums", "");
        app.recompute_steps();
        assert!(
            !app.steps.is_empty(),
            "Contains Duplicate failed on empty input string"
        );

        // Edge Case 2: Target not found for Two Sum
        app.current_problem = Problem::TwoSum;
        app.set_input_str(Problem::TwoSum, "nums", "1, 2, 3");
        app.set_input_int(Problem::TwoSum, "target", 999);
        app.recompute_steps();
        assert!(
            !app.steps.is_empty(),
            "Two Sum failed on non-existent target"
        );

        // Edge Case 3: Empty string input for Valid Anagram
        app.current_problem = Problem::ValidAnagram;
        app.set_input_str(Problem::ValidAnagram, "s", "");
        app.set_input_str(Problem::ValidAnagram, "t", "");
        app.recompute_steps();
        assert!(
            !app.steps.is_empty(),
            "Valid Anagram failed on empty string input"
        );

        // Edge Case 4: Length/Content mismatch for Valid Anagram
        app.set_input_str(Problem::ValidAnagram, "s", "rat");
        app.set_input_str(Problem::ValidAnagram, "t", "car");
        app.recompute_steps();
        assert!(
            !app.steps.is_empty(),
            "Valid Anagram failed on mismatch input"
        );
    }
}
