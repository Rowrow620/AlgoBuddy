use eframe::egui::{self, Color32, Frame, RichText, Rounding, Stroke};
use crate::algorithms::{
    bucket_sort::generate_bucket_sort_steps,
    min_heap::generate_min_heap_steps,
    sorting::generate_sorting_steps,
    encode_decode::generate_encode_decode_steps,
    product_except_self::generate_product_steps,
    two_sum::generate_two_sum_steps,
    valid_anagram::generate_valid_anagram_steps,
    valid_palindrome::generate_valid_palindrome_steps,
    valid_parentheses::generate_valid_parentheses_steps,
    best_time_stock::generate_best_time_stock_steps,
    binary_search::generate_binary_search_steps,
    reverse_linked_list::generate_reverse_linked_list_steps,
};
use crate::model::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightTab {
    CodeTrace,
    ProblemDetails,
}

pub struct VisualizerApp {
    // Navigation state
    current_problem: Problem,
    selected_approach_id: usize,
    selected_difficulty: Option<Difficulty>,
    search_query: String,
    right_tab: RightTab,

    // Inputs per problem
    two_sum_nums_input: String,
    two_sum_target_input: i32,

    valid_anagram_s_input: String,
    valid_anagram_t_input: String,

    topk_nums_input: String,
    topk_k_input: usize,
    topk_nums: Vec<i32>,
    topk_k: usize,

    ed_strs_input: String,
    ed_strs: Vec<String>,

    prod_nums_input: String,
    prod_nums: Vec<i32>,

    palindrome_s_input: String,
    parentheses_s_input: String,

    stock_prices_input: String,
    binary_search_nums_input: String,
    binary_search_target_input: i32,
    linked_list_nodes_input: String,

    // Playback state
    steps: Vec<Step>,
    current_step_idx: usize,
    is_playing: bool,
    playback_speed_ms: u64,
    last_step_time: std::time::Instant,
}

impl Default for VisualizerApp {
    fn default() -> Self {
        let mut app = Self {
            current_problem: Problem::TwoSum,
            selected_approach_id: 0,
            selected_difficulty: None,
            search_query: String::new(),
            right_tab: RightTab::CodeTrace,

            two_sum_nums_input: "2, 7, 11, 15".to_string(),
            two_sum_target_input: 9,

            valid_anagram_s_input: "anagram".to_string(),
            valid_anagram_t_input: "nagaram".to_string(),

            topk_nums_input: "1, 1, 1, 2, 2, 3".to_string(),
            topk_k_input: 2,
            topk_nums: vec![1, 1, 1, 2, 2, 3],
            topk_k: 2,

            ed_strs_input: "Hello, World".to_string(),
            ed_strs: vec!["Hello".to_string(), "World".to_string()],

            prod_nums_input: "1, 2, 4, 6".to_string(),
            prod_nums: vec![1, 2, 4, 6],

            palindrome_s_input: "Was it a car or a cat I saw?".to_string(),
            parentheses_s_input: "([{}])".to_string(),

            stock_prices_input: "10, 1, 5, 6, 7, 1".to_string(),
            binary_search_nums_input: "-1, 0, 2, 4, 6, 8".to_string(),
            binary_search_target_input: 4,
            linked_list_nodes_input: "0, 1, 2, 3".to_string(),

            steps: Vec::new(),
            current_step_idx: 0,
            is_playing: false,
            playback_speed_ms: 600,
            last_step_time: std::time::Instant::now(),
        };
        app.recompute_steps();
        app
    }
}

impl VisualizerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn recompute_steps(&mut self) {
        let app_id = self.selected_approach_id;
        self.steps = match self.current_problem {
            Problem::TwoSum => {
                let parsed: Vec<i32> = self.two_sum_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![2, 7, 11, 15] } else { parsed };
                generate_two_sum_steps(&nums, self.two_sum_target_input, app_id)
            }
            Problem::ValidAnagram => {
                generate_valid_anagram_steps(&self.valid_anagram_s_input, &self.valid_anagram_t_input, app_id)
            }
            Problem::TopKFrequent => {
                let parsed: Vec<i32> = self.topk_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                if !parsed.is_empty() { self.topk_nums = parsed; }
                let unique = self.topk_nums.iter().collect::<std::collections::HashSet<_>>().len();
                self.topk_k = self.topk_k_input.clamp(1, unique.max(1));
                self.topk_k_input = self.topk_k;

                match app_id {
                    0 => generate_bucket_sort_steps(&self.topk_nums, self.topk_k),
                    1 => generate_min_heap_steps(&self.topk_nums, self.topk_k),
                    _ => generate_sorting_steps(&self.topk_nums, self.topk_k),
                }
            }
            Problem::ProductExceptSelf => {
                let parsed: Vec<i32> = self.prod_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                if !parsed.is_empty() { self.prod_nums = parsed; }
                generate_product_steps(&self.prod_nums)
            }
            Problem::EncodeDecode => {
                self.ed_strs = self.ed_strs_input.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if self.ed_strs.is_empty() { self.ed_strs = vec!["".to_string()]; }
                generate_encode_decode_steps(&self.ed_strs)
            }
            Problem::ValidPalindrome => {
                generate_valid_palindrome_steps(&self.palindrome_s_input, app_id)
            }
            Problem::BestTimeStock => {
                let parsed: Vec<i32> = self.stock_prices_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let prices = if parsed.is_empty() { vec![10, 1, 5, 6, 7, 1] } else { parsed };
                generate_best_time_stock_steps(&prices)
            }
            Problem::ValidParentheses => {
                generate_valid_parentheses_steps(&self.parentheses_s_input)
            }
            Problem::BinarySearch => {
                let parsed: Vec<i32> = self.binary_search_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![-1, 0, 2, 4, 6, 8] } else { parsed };
                generate_binary_search_steps(&nums, self.binary_search_target_input)
            }
            Problem::ReverseLinkedList => {
                let parsed: Vec<i32> = self.linked_list_nodes_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nodes = if parsed.is_empty() { vec![0, 1, 2, 3] } else { parsed };
                generate_reverse_linked_list_steps(&nodes)
            }
        };
        self.current_step_idx = 0;
        self.is_playing = false;
    }

    fn select_problem(&mut self, problem: Problem) {
        if self.current_problem != problem {
            self.current_problem = problem;
            self.selected_approach_id = 0;
            self.recompute_steps();
        }
    }
}

// ── UI Theme ──

const BG_DARK: Color32 = Color32::from_rgb(11, 15, 25);
const SIDEBAR_BG: Color32 = Color32::from_rgb(15, 23, 42);
const STEP_BOX_BG: Color32 = Color32::from_rgb(30, 41, 59);
const CELL_BG: Color32 = Color32::from_rgb(30, 41, 59);
const CELL_BORDER: Color32 = Color32::from_rgb(51, 65, 85);
const MUTED: Color32 = Color32::from_rgb(156, 163, 175);
const DIM: Color32 = Color32::from_rgb(100, 116, 139);

const CYAN: Color32 = Color32::from_rgb(56, 189, 248);
const PURPLE: Color32 = Color32::from_rgb(168, 85, 247);
const EMERALD: Color32 = Color32::from_rgb(16, 185, 129);
const EMERALD_TEXT: Color32 = Color32::from_rgb(52, 211, 153);
const AMBER: Color32 = Color32::from_rgb(245, 158, 11);
const PINK: Color32 = Color32::from_rgb(236, 72, 153);
const RED: Color32 = Color32::from_rgb(244, 63, 94);
const CODE_ACTIVE_BG: Color32 = Color32::from_rgb(14, 116, 144);

fn difficulty_color(d: Difficulty) -> Color32 {
    match d {
        Difficulty::Easy => EMERALD_TEXT,
        Difficulty::Medium => AMBER,
        Difficulty::Hard => RED,
    }
}

impl eframe::App for VisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_playing {
            if self.last_step_time.elapsed().as_millis() as u64 >= self.playback_speed_ms {
                if self.current_step_idx < self.steps.len().saturating_sub(1) {
                    self.current_step_idx += 1;
                } else {
                    self.is_playing = false;
                }
                self.last_step_time = std::time::Instant::now();
            }
            ctx.request_repaint();
        }

        // ── Left Sidebar: NeetCode Roadmap Navigation ──
        egui::SidePanel::left("roadmap_sidebar")
            .default_width(280.0)
            .frame(Frame::none().inner_margin(12.0).fill(SIDEBAR_BG))
            .show(ctx, |ui| {
                ui.heading(RichText::new("LeetCode Roadmap").color(CYAN).strong().size(18.0));
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("🔍").font(egui::FontId::proportional(12.0)));
                    ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text("Search problem...").desired_width(180.0));
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Diff:").font(egui::FontId::proportional(11.0)).color(MUTED));
                    if ui.selectable_label(self.selected_difficulty.is_none(), "All").clicked() { self.selected_difficulty = None; }
                    if ui.selectable_label(self.selected_difficulty == Some(Difficulty::Easy), "Easy").clicked() { self.selected_difficulty = Some(Difficulty::Easy); }
                    if ui.selectable_label(self.selected_difficulty == Some(Difficulty::Medium), "Med").clicked() { self.selected_difficulty = Some(Difficulty::Medium); }
                    if ui.selectable_label(self.selected_difficulty == Some(Difficulty::Hard), "Hard").clicked() { self.selected_difficulty = Some(Difficulty::Hard); }
                });

                ui.separator();
                ui.add_space(4.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for &category in Category::all() {
                        let problems_in_cat: Vec<Problem> = Problem::all()
                            .iter()
                            .copied()
                            .filter(|p| p.category() == category)
                            .filter(|p| {
                                if let Some(diff) = self.selected_difficulty { p.difficulty() == diff } else { true }
                            })
                            .filter(|p| {
                                if self.search_query.trim().is_empty() {
                                    true
                                } else {
                                    let q = self.search_query.to_lowercase();
                                    p.title().to_lowercase().contains(&q) || p.id().to_string().contains(&q)
                                }
                            })
                            .collect();

                        let total_in_cat = Problem::all().iter().filter(|p| p.category() == category).count();
                        let header_text = format!("{} ({})", category.name(), problems_in_cat.len());

                        let is_active_cat = problems_in_cat.contains(&self.current_problem);
                        let header_color = if is_active_cat { CYAN } else { Color32::WHITE };

                        egui::CollapsingHeader::new(RichText::new(header_text).color(header_color).strong())
                            .default_open(is_active_cat || !problems_in_cat.is_empty())
                            .show(ui, |ui| {
                                if problems_in_cat.is_empty() {
                                    if total_in_cat == 0 {
                                        ui.label(RichText::new("  (Coming Soon)").italics().font(egui::FontId::proportional(11.0)).color(DIM));
                                    } else {
                                        ui.label(RichText::new("  (Filtered Out)").italics().font(egui::FontId::proportional(11.0)).color(DIM));
                                    }
                                } else {
                                    for prob in problems_in_cat {
                                        let is_selected = self.current_problem == prob;
                                        let diff_color = difficulty_color(prob.difficulty());

                                        ui.horizontal(|ui| {
                                            ui.add_space(8.0);
                                            let label = format!("#{} {}", prob.id(), prob.title());
                                            if ui.selectable_label(is_selected, RichText::new(label).color(if is_selected { CYAN } else { Color32::WHITE })).clicked() {
                                                self.select_problem(prob);
                                            }
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(prob.difficulty().label()).font(egui::FontId::monospace(10.0)).color(diff_color));
                                            });
                                        });
                                    }
                                }
                            });
                    }
                });
            });

        // ── Top Header Panel ──
        egui::TopBottomPanel::top("header_panel")
            .frame(Frame::none().inner_margin(12.0).fill(BG_DARK))
            .show(ctx, |ui| {
                let p = self.current_problem;
                let details = p.details();

                ui.horizontal(|ui| {
                    ui.heading(
                        RichText::new(format!("#{} {}", p.id(), p.title()))
                            .font(egui::FontId::proportional(18.0))
                            .strong()
                            .color(CYAN),
                    );

                    let d_color = difficulty_color(p.difficulty());
                    egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(4.0)).inner_margin(4.0).show(ui, |ui| {
                        ui.label(RichText::new(p.difficulty().label()).font(egui::FontId::monospace(11.0)).strong().color(d_color));
                    });

                    ui.label(RichText::new(format!("Category: {}", p.category().name())).font(egui::FontId::proportional(12.0)).color(MUTED));

                    if let Some(active_approach) = details.approaches.get(self.selected_approach_id) {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("Time: {} | Space: {}", active_approach.time_complexity, active_approach.space_complexity)).font(egui::FontId::monospace(12.0)).color(EMERALD_TEXT).strong());
                        });
                    }
                });

                ui.add_space(6.0);

                // Multi-Approach Selector Row
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Approach:").strong().color(Color32::WHITE));
                    for approach in details.approaches {
                        let is_sel = self.selected_approach_id == approach.id;
                        let btn_label = format!("{} ({})", approach.name, approach.time_complexity);
                        if ui.selectable_label(is_sel, RichText::new(btn_label).color(if is_sel { CYAN } else { Color32::WHITE }).strong()).clicked() {
                            self.selected_approach_id = approach.id;
                            self.recompute_steps();
                        }
                    }
                });

                ui.add_space(6.0);

                // Per-problem Controls & Inputs
                ui.horizontal(|ui| {
                    match self.current_problem {
                        Problem::TwoSum => {
                            ui.label(RichText::new("nums:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.two_sum_nums_input).desired_width(160.0));
                            ui.label(RichText::new("target:").strong());
                            if ui.add(egui::DragValue::new(&mut self.two_sum_target_input).speed(1.0)).changed() { self.recompute_steps(); }
                        }
                        Problem::ValidAnagram => {
                            ui.label(RichText::new("s:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.valid_anagram_s_input).desired_width(120.0));
                            ui.label(RichText::new("t:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.valid_anagram_t_input).desired_width(120.0));
                        }
                        Problem::TopKFrequent => {
                            ui.label(RichText::new("nums:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.topk_nums_input).desired_width(140.0));
                            ui.label(RichText::new("k:").strong());
                            if ui.add(egui::DragValue::new(&mut self.topk_k_input).speed(1.0).range(1..=10)).changed() { self.recompute_steps(); }
                        }
                        Problem::ProductExceptSelf => {
                            ui.label(RichText::new("nums:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.prod_nums_input).desired_width(200.0));
                        }
                        Problem::EncodeDecode => {
                            ui.label(RichText::new("Strings (comma-separated):").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.ed_strs_input).desired_width(260.0));
                        }
                        Problem::ValidPalindrome => {
                            ui.label(RichText::new("String s:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.palindrome_s_input).desired_width(300.0));
                        }
                        Problem::BestTimeStock => {
                            ui.label(RichText::new("prices:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.stock_prices_input).desired_width(220.0));
                        }
                        Problem::ValidParentheses => {
                            ui.label(RichText::new("String s:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.parentheses_s_input).desired_width(200.0));
                        }
                        Problem::BinarySearch => {
                            ui.label(RichText::new("nums (sorted):").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.binary_search_nums_input).desired_width(200.0));
                            ui.label(RichText::new("target:").strong());
                            if ui.add(egui::DragValue::new(&mut self.binary_search_target_input).speed(1.0)).changed() { self.recompute_steps(); }
                        }
                        Problem::ReverseLinkedList => {
                            ui.label(RichText::new("head nodes:").strong());
                            ui.add(egui::TextEdit::singleline(&mut self.linked_list_nodes_input).desired_width(200.0));
                        }
                    }

                    if ui.button(RichText::new("Apply").strong().color(Color32::WHITE)).clicked() {
                        self.recompute_steps();
                    }

                    ui.separator();
                    ui.label(RichText::new("Presets:").strong());
                    match self.current_problem {
                        Problem::TwoSum => {
                            if ui.button("[2,7,11,15] t=9").clicked() { self.two_sum_nums_input = "2,7,11,15".into(); self.two_sum_target_input = 9; self.recompute_steps(); }
                        }
                        Problem::ValidAnagram => {
                            if ui.button("anagram / nagaram").clicked() { self.valid_anagram_s_input = "anagram".into(); self.valid_anagram_t_input = "nagaram".into(); self.recompute_steps(); }
                        }
                        Problem::TopKFrequent => {
                            if ui.button("[1,1,1,2,2,3] k=2").clicked() { self.topk_nums_input = "1,1,1,2,2,3".into(); self.topk_k_input = 2; self.recompute_steps(); }
                        }
                        Problem::ProductExceptSelf => {
                            if ui.button("[1,2,4,6]").clicked() { self.prod_nums_input = "1,2,4,6".into(); self.recompute_steps(); }
                        }
                        Problem::EncodeDecode => {
                            if ui.button("[Hello, World]").clicked() { self.ed_strs_input = "Hello, World".into(); self.recompute_steps(); }
                        }
                        Problem::ValidPalindrome => {
                            if ui.button("Was it a car...").clicked() { self.palindrome_s_input = "Was it a car or a cat I saw?".into(); self.recompute_steps(); }
                            if ui.button("tab a cat").clicked() { self.palindrome_s_input = "tab a cat".into(); self.recompute_steps(); }
                        }
                        Problem::BestTimeStock => {
                            if ui.button("[10,1,5,6,7,1]").clicked() { self.stock_prices_input = "10,1,5,6,7,1".into(); self.recompute_steps(); }
                            if ui.button("[10,8,7,5,2]").clicked() { self.stock_prices_input = "10,8,7,5,2".into(); self.recompute_steps(); }
                        }
                        Problem::ValidParentheses => {
                            if ui.button("([{}])").clicked() { self.parentheses_s_input = "([{}])".into(); self.recompute_steps(); }
                            if ui.button("[(])").clicked() { self.parentheses_s_input = "[(])".into(); self.recompute_steps(); }
                        }
                        Problem::BinarySearch => {
                            if ui.button("[-1,0,2,4,6,8] t=4").clicked() { self.binary_search_nums_input = "-1,0,2,4,6,8".into(); self.binary_search_target_input = 4; self.recompute_steps(); }
                            if ui.button("[-1,0,2,4,6,8] t=3").clicked() { self.binary_search_nums_input = "-1,0,2,4,6,8".into(); self.binary_search_target_input = 3; self.recompute_steps(); }
                        }
                        Problem::ReverseLinkedList => {
                            if ui.button("[0,1,2,3]").clicked() { self.linked_list_nodes_input = "0,1,2,3".into(); self.recompute_steps(); }
                            if ui.button("[7,14,21]").clicked() { self.linked_list_nodes_input = "7,14,21".into(); self.recompute_steps(); }
                        }
                    }
                });

                ui.add_space(6.0);

                // Playback Control Bar
                ui.horizontal(|ui| {
                    let play_text = if self.is_playing { "Pause" } else { "Play" };
                    if ui.button(RichText::new(play_text).strong()).clicked() {
                        if self.current_step_idx >= self.steps.len().saturating_sub(1) {
                            self.current_step_idx = 0;
                        }
                        self.is_playing = !self.is_playing;
                        self.last_step_time = std::time::Instant::now();
                    }
                    if ui.button("Prev").clicked() { self.is_playing = false; self.current_step_idx = self.current_step_idx.saturating_sub(1); }
                    if ui.button("Next").clicked() { self.is_playing = false; if self.current_step_idx < self.steps.len().saturating_sub(1) { self.current_step_idx += 1; } }
                    if ui.button("Reset").clicked() { self.is_playing = false; self.current_step_idx = 0; }

                    ui.separator();
                    ui.label(RichText::new(format!("Step {} / {}", self.current_step_idx + 1, self.steps.len())).strong());
                    let max_idx = self.steps.len().saturating_sub(1);
                    ui.add(egui::Slider::new(&mut self.current_step_idx, 0..=max_idx).show_value(false));

                    ui.separator();
                    ui.label("Speed:");
                    ui.add(egui::Slider::new(&mut self.playback_speed_ms, 100..=1500).text("ms"));
                });
            });

        // ── Right Sidebar: Tabbed Code Trace & Problem Details ──
        egui::SidePanel::right("right_sidebar")
            .default_width(400.0)
            .frame(Frame::none().inner_margin(12.0).fill(SIDEBAR_BG))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.right_tab == RightTab::CodeTrace, RichText::new("💻 Code Trace").strong()).clicked() {
                        self.right_tab = RightTab::CodeTrace;
                    }
                    if ui.selectable_label(self.right_tab == RightTab::ProblemDetails, RichText::new("📄 Problem Statement & Examples").strong()).clicked() {
                        self.right_tab = RightTab::ProblemDetails;
                    }
                });

                ui.separator();
                ui.add_space(6.0);

                match self.right_tab {
                    RightTab::CodeTrace => {
                        if let Some(step) = self.steps.get(self.current_step_idx) {
                            egui::Frame::group(ui.style())
                                .fill(STEP_BOX_BG)
                                .rounding(Rounding::same(8.0))
                                .inner_margin(10.0)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(format!("STEP {} / {}", self.current_step_idx + 1, self.steps.len()))
                                            .font(egui::FontId::monospace(11.0))
                                            .color(CYAN).strong(),
                                    );
                                    ui.add_space(4.0);
                                    ui.label(RichText::new(&step.description).font(egui::FontId::proportional(13.0)).color(Color32::WHITE));
                                });

                            ui.add_space(12.0);
                            ui.label(RichText::new("Python Implementation").strong().color(MUTED));
                            ui.add_space(6.0);

                            let code_lines = approach_code_lines(self.current_problem, self.selected_approach_id);

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for (line_num, line_text) in &code_lines {
                                    let is_active = step.code_line == *line_num;
                                    let text_color = if is_active { Color32::WHITE } else { MUTED };
                                    let bg = if is_active { CODE_ACTIVE_BG } else { Color32::TRANSPARENT };

                                    egui::Frame::none()
                                        .fill(bg)
                                        .rounding(Rounding::same(4.0))
                                        .inner_margin(3.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(format!("{:2} | ", line_num)).font(egui::FontId::monospace(11.0)).color(DIM));
                                                let mut rt = RichText::new(*line_text).font(egui::FontId::monospace(12.0)).color(text_color);
                                                if is_active { rt = rt.strong(); }
                                                ui.label(rt);
                                            });
                                        });
                                }
                            });
                        }
                    }
                    RightTab::ProblemDetails => {
                        let details = self.current_problem.details();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.heading(RichText::new(format!("#{} {}", details.id, details.title)).color(CYAN).strong().size(18.0));
                            ui.add_space(4.0);

                            ui.horizontal(|ui| {
                                let d_color = difficulty_color(details.difficulty);
                                ui.label(RichText::new(details.difficulty.label()).color(d_color).strong());
                                ui.label(RichText::new(format!("• Category: {}", details.category.name())).color(MUTED));
                            });

                            ui.add_space(10.0);
                            ui.label(RichText::new("Description").strong().color(Color32::WHITE));
                            ui.add_space(4.0);
                            ui.label(RichText::new(details.statement).font(egui::FontId::proportional(13.0)).color(Color32::from_rgb(226, 232, 240)));

                            ui.add_space(14.0);
                            ui.label(RichText::new("Examples").strong().color(Color32::WHITE));
                            ui.add_space(4.0);

                            for (ex_idx, ex) in details.examples.iter().enumerate() {
                                egui::Frame::group(ui.style())
                                    .fill(STEP_BOX_BG)
                                    .rounding(Rounding::same(8.0))
                                    .inner_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.label(RichText::new(format!("Example {}", ex_idx + 1)).strong().color(AMBER));
                                        ui.add_space(4.0);
                                        ui.label(RichText::new(format!("Input: {}", ex.input)).font(egui::FontId::monospace(12.0)).color(CYAN));
                                        ui.label(RichText::new(format!("Output: {}", ex.output)).font(egui::FontId::monospace(12.0)).color(EMERALD_TEXT));
                                        if !ex.explanation.is_empty() {
                                            ui.label(RichText::new(format!("Explanation: {}", ex.explanation)).font(egui::FontId::proportional(12.0)).color(MUTED));
                                        }
                                    });
                                ui.add_space(6.0);
                            }

                            ui.add_space(10.0);
                            ui.label(RichText::new("Constraints").strong().color(Color32::WHITE));
                            ui.add_space(4.0);
                            for constraint in details.constraints {
                                ui.label(RichText::new(format!("• {}", constraint)).font(egui::FontId::monospace(12.0)).color(MUTED));
                            }

                            ui.add_space(14.0);
                            if ui.button(RichText::new("🌐 Open on LeetCode.com ↗").strong().color(CYAN)).clicked() {
                                let _ = open::that(details.leetcode_url);
                            }
                        });
                    }
                }
            });

        // ── Central Canvas ──
        egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(16.0).fill(BG_DARK))
            .show(ctx, |ui| {
                if let Some(step) = self.steps.get(self.current_step_idx) {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        match &step.visual {
                            VisualState::TwoSum { nums, target, active_idx, secondary_idx, map, found_indices } => {
                                self.render_two_sum(ui, nums, *target, *active_idx, *secondary_idx, map, *found_indices);
                            }
                            VisualState::ValidAnagram { s, t, s_counts, t_counts, active_s_idx, active_t_idx, is_anagram } => {
                                self.render_valid_anagram(ui, s, t, s_counts, t_counts, *active_s_idx, *active_t_idx, *is_anagram);
                            }
                            VisualState::TwoPointers { chars, left, right, is_valid, skipped } => {
                                self.render_two_pointers(ui, chars, *left, *right, *is_valid, *skipped);
                            }
                            VisualState::Stack { chars, active_idx, stack, is_valid } => {
                                self.render_stack(ui, chars, *active_idx, stack, *is_valid);
                            }
                            VisualState::BestTimeStock { prices, left_buy, right_sell, current_profit, max_profit } => {
                                self.render_stock(ui, prices, *left_buy, *right_sell, *current_profit, *max_profit);
                            }
                            VisualState::BinarySearch { nums, target, left, right, mid, found_idx } => {
                                self.render_binary_search(ui, nums, *target, *left, *right, *mid, *found_idx);
                            }
                            VisualState::LinkedList { nodes, prev_idx, curr_idx, next_idx, reversed_so_far } => {
                                self.render_linked_list(ui, nodes, *prev_idx, *curr_idx, *next_idx, reversed_so_far);
                            }
                            VisualState::TopK { nums, active_nums_idx, count_map, buckets, active_bucket_idx, result } => {
                                self.render_topk(ui, nums, *active_nums_idx, count_map, buckets, *active_bucket_idx, result);
                            }
                            VisualState::EncodeDecode { input_strs, encoded_so_far, decoded_so_far, pointer, active_str_idx, phase } => {
                                self.render_encode_decode(ui, input_strs, encoded_so_far, decoded_so_far, *pointer, *active_str_idx, phase);
                            }
                            VisualState::Product { nums, output, active_idx, prefix_val, suffix_val, phase } => {
                                self.render_product(ui, nums, output, *active_idx, *prefix_val, *suffix_val, phase);
                            }
                        }
                    });
                }
            });
    }
}

// ── Visual Canvas Renderers ──

impl VisualizerApp {
    fn render_stock(&self, ui: &mut egui::Ui, prices: &[i32], left_buy: usize, right_sell: usize, current_profit: i32, max_profit: i32) {
        ui.heading(RichText::new("Sliding Window / Buy & Sell Stock Trace").color(CYAN).size(16.0));
        ui.add_space(8.0);

        // Prices array cards
        ui.group(|ui| {
            ui.label(RichText::new("STOCK PRICES ARRAY (Days 0..N-1)").font(egui::FontId::monospace(11.0)).color(MUTED));
            ui.horizontal(|ui| {
                for (i, &price) in prices.iter().enumerate() {
                    let is_buy = i == left_buy;
                    let is_sell = i == right_sell;

                    let fill = if is_buy && is_sell {
                        PURPLE
                    } else if is_buy {
                        CYAN
                    } else if is_sell {
                        PINK
                    } else {
                        CELL_BG
                    };

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(10.0).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let label = if is_buy && is_sell {
                                "Buy & Sell"
                            } else if is_buy {
                                "Buy (l)"
                            } else if is_sell {
                                "Sell (r)"
                            } else {
                                ""
                            };
                            ui.label(RichText::new(format!("day {} {}", i, label)).font(egui::FontId::proportional(10.0)).color(MUTED));
                            ui.label(RichText::new(format!("${}", price)).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    });
                }
            });
        });

        ui.add_space(20.0);

        // Profit metrics
        ui.horizontal(|ui| {
            egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, PINK)).inner_margin(12.0).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Current Profit (prices[r] - prices[l])").font(egui::FontId::proportional(11.0)).color(MUTED));
                    ui.label(RichText::new(format!("${}", current_profit)).font(egui::FontId::monospace(18.0)).strong().color(PINK));
                });
            });

            ui.add_space(16.0);

            egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, EMERALD_TEXT)).inner_margin(12.0).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Maximum Achieved Profit (maxP)").font(egui::FontId::proportional(11.0)).color(MUTED));
                    ui.label(RichText::new(format!("${}", max_profit)).font(egui::FontId::monospace(18.0)).strong().color(EMERALD_TEXT));
                });
            });
        });
    }

    fn render_binary_search(&self, ui: &mut egui::Ui, nums: &[i32], target: i32, left: usize, right: usize, mid: Option<usize>, found_idx: Option<usize>) {
        ui.heading(RichText::new(format!("Binary Search bounds (l={}, r={}) | Target = {}", left, right, target)).color(CYAN).size(16.0));
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new("SORTED ARRAY").font(egui::FontId::monospace(11.0)).color(MUTED));
            ui.horizontal(|ui| {
                for (i, &num) in nums.iter().enumerate() {
                    let is_found = found_idx == Some(i);
                    let is_mid = mid == Some(i);
                    let in_range = i >= left && i <= right;

                    let fill = if is_found {
                        EMERALD
                    } else if is_mid {
                        AMBER
                    } else if in_range {
                        CELL_BG
                    } else {
                        DIM
                    };

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(10.0).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut ptr_label = String::new();
                            if i == left { ptr_label.push_str("L "); }
                            if is_mid { ptr_label.push_str("MID "); }
                            if i == right { ptr_label.push_str("R"); }

                            ui.label(RichText::new(format!("i={} {}", i, ptr_label)).font(egui::FontId::proportional(10.0)).color(Color32::WHITE));
                            ui.label(RichText::new(num.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    });
                }
            });
        });

        if let Some(f) = found_idx {
            ui.add_space(20.0);
            ui.heading(RichText::new(format!("Target {} Found at Index {}!", target, f)).color(EMERALD_TEXT).size(18.0));
        }
    }

    fn render_linked_list(&self, ui: &mut egui::Ui, nodes: &[i32], prev_idx: Option<usize>, curr_idx: Option<usize>, next_idx: Option<usize>, reversed_so_far: &[i32]) {
        ui.heading(RichText::new("Singly-Linked List Pointer Reversal").color(CYAN).size(16.0));
        ui.add_space(8.0);

        // Original chain with pointers
        ui.group(|ui| {
            ui.label(RichText::new("ORIGINAL LINKED LIST NODES").font(egui::FontId::monospace(11.0)).color(MUTED));
            ui.horizontal(|ui| {
                for (i, &val) in nodes.iter().enumerate() {
                    let is_prev = prev_idx == Some(i);
                    let is_curr = curr_idx == Some(i);
                    let is_nxt = next_idx == Some(i);

                    let fill = if is_curr {
                        CYAN
                    } else if is_prev {
                        PURPLE
                    } else if is_nxt {
                        PINK
                    } else {
                        CELL_BG
                    };

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(10.0).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut label = String::new();
                            if is_prev { label.push_str("prev "); }
                            if is_curr { label.push_str("curr "); }
                            if is_nxt { label.push_str("nxt "); }

                            ui.label(RichText::new(format!("idx {} {}", i, label)).font(egui::FontId::proportional(10.0)).color(Color32::WHITE));
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    });
                }
                ui.label(RichText::new("None").font(egui::FontId::monospace(14.0)).color(DIM));
            });
        });

        ui.add_space(20.0);

        // Reversed list so far
        ui.group(|ui| {
            ui.label(RichText::new("REVERSED LINKED LIST (Constructed from head)").font(egui::FontId::monospace(11.0)).color(EMERALD_TEXT));
            ui.horizontal(|ui| {
                if reversed_so_far.is_empty() {
                    ui.label(RichText::new("None").font(egui::FontId::monospace(14.0)).color(DIM));
                } else {
                    for (i, &val) in reversed_so_far.iter().enumerate() {
                        let fill = if i == 0 { EMERALD } else { CELL_BG };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, EMERALD_TEXT)).inner_margin(10.0).show(ui, |ui| {
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    }
                    ui.label(RichText::new("None").font(egui::FontId::monospace(14.0)).color(DIM));
                }
            });
        });
    }

    fn render_two_sum(&self, ui: &mut egui::Ui, nums: &[i32], target: i32, active_idx: Option<usize>, secondary_idx: Option<usize>, map: &std::collections::BTreeMap<i32, usize>, found: Option<(usize, usize)>) {
        ui.heading(RichText::new(format!("Target Sum: {}", target)).color(CYAN).size(16.0));
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new("NUMS ARRAY").font(egui::FontId::monospace(11.0)).color(MUTED));
            ui.horizontal(|ui| {
                for (i, &num) in nums.iter().enumerate() {
                    let is_found = found.map_or(false, |(a, b)| a == i || b == i);
                    let is_primary = active_idx == Some(i);
                    let is_sec = secondary_idx == Some(i);

                    let fill = if is_found {
                        EMERALD
                    } else if is_primary {
                        AMBER
                    } else if is_sec {
                        PINK
                    } else {
                        CELL_BG
                    };

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(10.0).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let label = if is_primary { "i" } else if is_sec { "j" } else { "" };
                            ui.label(RichText::new(format!("i={} {}", i, label)).font(egui::FontId::proportional(10.0)).color(MUTED));
                            ui.label(RichText::new(num.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    });
                }
            });
        });

        ui.add_space(20.0);

        if self.selected_approach_id == 0 {
            ui.group(|ui| {
                ui.label(RichText::new("PREVMAP {value -> index}").font(egui::FontId::monospace(11.0)).color(MUTED));
                ui.horizontal(|ui| {
                    if map.is_empty() {
                        ui.label(RichText::new("Empty {}").italics().color(DIM));
                    } else {
                        for (&val, &idx) in map {
                            egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, PURPLE)).inner_margin(8.0).show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(format!("val: {}", val)).font(egui::FontId::proportional(12.0)).color(Color32::WHITE));
                                    ui.label(RichText::new(format!("idx: {}", idx)).font(egui::FontId::monospace(14.0)).strong().color(PURPLE));
                                });
                            });
                        }
                    }
                });
            });
        }

        if let Some((a, b)) = found {
            ui.add_space(20.0);
            ui.heading(RichText::new(format!("Result Pair Found! Indices: [{}, {}]", a, b)).color(EMERALD_TEXT).size(18.0));
        }
    }

    fn render_valid_anagram(&self, ui: &mut egui::Ui, s: &str, t: &str, s_counts: &[usize; 26], t_counts: &[usize; 26], active_s: Option<usize>, active_t: Option<usize>, is_anagram: Option<bool>) {
        ui.heading(RichText::new("Character Comparison").color(CYAN).size(16.0));
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new(format!("STRING s: \"{}\"", s)).font(egui::FontId::monospace(12.0)).color(MUTED));
                ui.horizontal(|ui| {
                    for (i, c) in s.chars().enumerate() {
                        let fill = if active_s == Some(i) { AMBER } else { CELL_BG };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(6.0)).inner_margin(8.0).show(ui, |ui| {
                            ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label(RichText::new(format!("STRING t: \"{}\"", t)).font(egui::FontId::monospace(12.0)).color(MUTED));
                ui.horizontal(|ui| {
                    for (i, c) in t.chars().enumerate() {
                        let fill = if active_t == Some(i) { PINK } else { CELL_BG };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(6.0)).inner_margin(8.0).show(ui, |ui| {
                            ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });
        });

        ui.add_space(20.0);

        if self.selected_approach_id == 0 {
            ui.group(|ui| {
                ui.label(RichText::new("ALPHABET FREQUENCY LOG").font(egui::FontId::monospace(11.0)).color(MUTED));
                ui.horizontal_wrapped(|ui| {
                    for i in 0..26 {
                        let ch = (b'a' + i as u8) as char;
                        if s_counts[i] > 0 || t_counts[i] > 0 {
                            let match_color = if s_counts[i] == t_counts[i] { EMERALD_TEXT } else { RED };
                            egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(6.0)).stroke(Stroke::new(1.0_f32, match_color)).inner_margin(6.0).show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(ch.to_string()).font(egui::FontId::monospace(14.0)).strong().color(CYAN));
                                    ui.label(RichText::new(format!("s:{}", s_counts[i])).font(egui::FontId::monospace(11.0)).color(MUTED));
                                    ui.label(RichText::new(format!("t:{}", t_counts[i])).font(egui::FontId::monospace(11.0)).color(MUTED));
                                });
                            });
                        }
                    }
                });
            });
        }

        if let Some(res) = is_anagram {
            ui.add_space(20.0);
            if res {
                ui.heading(RichText::new("Valid Anagram!").color(EMERALD_TEXT).size(18.0));
            } else {
                ui.heading(RichText::new("Not an Anagram").color(RED).size(18.0));
            }
        }
    }

    fn render_two_pointers(&self, ui: &mut egui::Ui, chars: &[char], left: usize, right: usize, is_valid: Option<bool>, skipped: bool) {
        ui.heading(RichText::new("Two Pointers Convergence").color(CYAN).size(16.0));
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            for (i, &c) in chars.iter().enumerate() {
                let is_left = i == left;
                let is_right = i == right;

                let fill = if is_left && is_right {
                    PURPLE
                } else if is_left {
                    CYAN
                } else if is_right {
                    PINK
                } else if skipped && (i < left || i > right) {
                    DIM
                } else {
                    CELL_BG
                };

                egui::Frame::none().fill(fill).rounding(Rounding::same(6.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(8.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        let ptr_label = if is_left && is_right {
                            "L & R"
                        } else if is_left {
                            "L ->"
                        } else if is_right {
                            "<- R"
                        } else {
                            " "
                        };
                        ui.label(RichText::new(ptr_label).font(egui::FontId::monospace(10.0)).strong().color(Color32::WHITE));
                        ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                    });
                });
            }
        });

        if let Some(valid) = is_valid {
            ui.add_space(20.0);
            if valid {
                ui.heading(RichText::new("Valid Palindrome!").color(EMERALD_TEXT).size(18.0));
            } else {
                ui.heading(RichText::new("Invalid Palindrome Mismatch").color(RED).size(18.0));
            }
        }
    }

    fn render_stack(&self, ui: &mut egui::Ui, chars: &[char], active_idx: Option<usize>, stack: &[char], is_valid: Option<bool>) {
        ui.heading(RichText::new("Vertical Stack Push / Pop Trace").color(CYAN).size(16.0));
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new("EXPRESSION").font(egui::FontId::monospace(11.0)).color(MUTED));
                ui.horizontal(|ui| {
                    for (i, &c) in chars.iter().enumerate() {
                        let fill = if active_idx == Some(i) { AMBER } else { CELL_BG };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(6.0)).inner_margin(8.0).show(ui, |ui| {
                            ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });

            ui.add_space(30.0);

            ui.group(|ui| {
                ui.label(RichText::new("STACK (Top on right/bottom)").font(egui::FontId::monospace(11.0)).color(MUTED));
                ui.vertical(|ui| {
                    if stack.is_empty() {
                        ui.label(RichText::new("Stack is Empty []").italics().color(DIM));
                    } else {
                        for (idx, &c) in stack.iter().rev().enumerate() {
                            let is_top = idx == 0;
                            let fill = if is_top { PURPLE } else { CELL_BG };
                            egui::Frame::none().fill(fill).rounding(Rounding::same(6.0)).stroke(Stroke::new(1.0_f32, CYAN)).inner_margin(8.0).show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    if is_top {
                                        ui.label(RichText::new("TOP ->").font(egui::FontId::monospace(10.0)).color(AMBER));
                                    }
                                    ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                                });
                            });
                        }
                    }
                });
            });
        });

        if let Some(valid) = is_valid {
            ui.add_space(20.0);
            if valid {
                ui.heading(RichText::new("Valid Parentheses Expression!").color(EMERALD_TEXT).size(18.0));
            } else {
                ui.heading(RichText::new("Invalid Parentheses Expression").color(RED).size(18.0));
            }
        }
    }

    fn render_topk(&self, ui: &mut egui::Ui, nums: &[i32], active_nums_idx: Option<usize>, count_map: &std::collections::BTreeMap<i32, usize>, buckets: &[Vec<i32>], active_bucket_idx: Option<usize>, result: &[i32]) {
        ui.heading(RichText::new("1. Input Array & Frequency Map").color(CYAN).size(16.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new("NUMS ARRAY").font(egui::FontId::monospace(11.0)).color(MUTED));
                ui.horizontal(|ui| {
                    for (idx, &val) in nums.iter().enumerate() {
                        let fill = if active_nums_idx == Some(idx) { AMBER } else { CELL_BG };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(10.0).show(ui, |ui| {
                            ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });
            ui.add_space(20.0);
            ui.group(|ui| {
                ui.label(RichText::new("COUNT MAP {num: frequency}").font(egui::FontId::monospace(11.0)).color(MUTED));
                ui.horizontal(|ui| {
                    if count_map.is_empty() {
                        ui.label(RichText::new("Empty {}").italics().color(DIM));
                    } else {
                        for (&num, &cnt) in count_map.iter() {
                            egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, PURPLE)).inner_margin(8.0).show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(format!("num: {}", num)).font(egui::FontId::proportional(12.0)).color(Color32::from_rgb(209, 213, 219)));
                                    ui.label(RichText::new(format!("{}", cnt)).font(egui::FontId::monospace(16.0)).strong().color(PURPLE));
                                });
                            });
                        }
                    }
                });
            });
        });

        ui.add_space(24.0);

        ui.heading(RichText::new("2. Frequency Buckets (Index = Count)").color(PURPLE).size(16.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            for (idx, items) in buckets.iter().enumerate() {
                let is_active = active_bucket_idx == Some(idx);
                let fill = if is_active { PINK } else { SIDEBAR_BG };
                egui::Frame::none().fill(fill).rounding(Rounding::same(10.0)).stroke(Stroke::new(1.0_f32, if is_active { Color32::WHITE } else { CELL_BORDER })).inner_margin(12.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("freq[{}]", idx)).font(egui::FontId::monospace(12.0)).strong().color(MUTED));
                        ui.separator();
                        if items.is_empty() {
                            ui.label(RichText::new("—").color(Color32::from_rgb(71, 85, 105)));
                        } else {
                            for &item in items {
                                egui::Frame::none().fill(CYAN).rounding(Rounding::same(6.0)).inner_margin(6.0).show(ui, |ui| {
                                    ui.label(RichText::new(item.to_string()).font(egui::FontId::monospace(14.0)).strong().color(Color32::BLACK));
                                });
                            }
                        }
                    });
                });
            }
        });

        ui.add_space(24.0);

        ui.heading(RichText::new(format!("3. Result Collector (Target k = {})", self.topk_k)).color(EMERALD_TEXT).size(16.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if result.is_empty() {
                ui.label(RichText::new("Result array is empty...").italics().color(DIM));
            } else {
                for &val in result {
                    egui::Frame::none().fill(EMERALD).rounding(Rounding::same(10.0)).inner_margin(12.0).show(ui, |ui| {
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(18.0)).strong().color(Color32::WHITE));
                    });
                }
            }
        });
    }

    fn render_encode_decode(&self, ui: &mut egui::Ui, input_strs: &[String], encoded_so_far: &str, decoded_so_far: &[String], pointer: usize, active_str_idx: Option<usize>, phase: &EncodeDecodePhase) {
        ui.heading(RichText::new("1. Input Strings").color(CYAN).size(16.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            for (idx, s) in input_strs.iter().enumerate() {
                let is_active = active_str_idx == Some(idx);
                let fill = if is_active { AMBER } else { CELL_BG };
                egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(10.0).show(ui, |ui| {
                    ui.label(RichText::new(format!("\"{}\"", s)).font(egui::FontId::monospace(14.0)).strong().color(Color32::WHITE));
                });
            }
        });

        ui.add_space(24.0);

        ui.heading(RichText::new("2. Encoded String").color(PURPLE).size(16.0));
        ui.add_space(8.0);
        if encoded_so_far.is_empty() {
            ui.label(RichText::new("\"\" (empty)").italics().color(DIM));
        } else {
            ui.horizontal_wrapped(|ui| {
                for (i, ch) in encoded_so_far.chars().enumerate() {
                    let is_ptr = *phase == EncodeDecodePhase::Decoding && i == pointer;
                    let fill = if is_ptr { PINK } else if ch == '#' { PURPLE } else { CELL_BG };
                    egui::Frame::none().fill(fill).rounding(Rounding::same(4.0)).inner_margin(6.0).show(ui, |ui| {
                        ui.label(RichText::new(ch.to_string()).font(egui::FontId::monospace(14.0)).strong().color(Color32::WHITE));
                    });
                }
            });
        }

        ui.add_space(24.0);

        ui.heading(RichText::new("3. Decoded Strings").color(EMERALD_TEXT).size(16.0));
        ui.add_space(8.0);
        if decoded_so_far.is_empty() {
            ui.label(RichText::new("Decoded list is empty...").italics().color(DIM));
        } else {
            ui.horizontal(|ui| {
                for s in decoded_so_far {
                    egui::Frame::none().fill(EMERALD).rounding(Rounding::same(10.0)).inner_margin(12.0).show(ui, |ui| {
                        ui.label(RichText::new(format!("\"{}\"", s)).font(egui::FontId::monospace(14.0)).strong().color(Color32::WHITE));
                    });
                }
            });
        }
    }

    fn render_product(&self, ui: &mut egui::Ui, nums: &[i32], output: &[i64], active_idx: Option<usize>, prefix_val: i64, suffix_val: i64, phase: &ProductPhase) {
        ui.heading(RichText::new("1. Input Array (nums)").color(CYAN).size(16.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            for (idx, &val) in nums.iter().enumerate() {
                let is_active = active_idx == Some(idx);
                let fill = if is_active { AMBER } else { CELL_BG };
                egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CELL_BORDER)).inner_margin(10.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("i={}", idx)).font(egui::FontId::proportional(10.0)).color(MUTED));
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                    });
                });
            }
        });

        ui.add_space(24.0);

        ui.heading(RichText::new("2. Running Prefix / Suffix Values").color(PURPLE).size(16.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, CYAN)).inner_margin(12.0).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("prefix").font(egui::FontId::monospace(12.0)).color(MUTED));
                    ui.label(RichText::new(prefix_val.to_string()).font(egui::FontId::monospace(18.0)).strong().color(CYAN));
                });
            });
            ui.add_space(16.0);
            egui::Frame::none().fill(CELL_BG).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, PINK)).inner_margin(12.0).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("suffix").font(egui::FontId::monospace(12.0)).color(MUTED));
                    ui.label(RichText::new(suffix_val.to_string()).font(egui::FontId::monospace(18.0)).strong().color(PINK));
                });
            });
            ui.add_space(16.0);
            let phase_label = match phase {
                ProductPhase::Init => "Initializing",
                ProductPhase::PrefixPass => "Prefix Pass (left to right)",
                ProductPhase::SuffixPass => "Suffix Pass (right to left)",
                ProductPhase::Complete => "Complete",
            };
            ui.label(RichText::new(format!("Phase: {}", phase_label)).font(egui::FontId::proportional(14.0)).strong().color(Color32::WHITE));
        });

        ui.add_space(24.0);

        ui.heading(RichText::new("3. Output Array").color(EMERALD_TEXT).size(16.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            for (idx, &val) in output.iter().enumerate() {
                let is_active = active_idx == Some(idx);
                let fill = if is_active {
                    match phase {
                        ProductPhase::PrefixPass => CYAN,
                        ProductPhase::SuffixPass => PINK,
                        _ => EMERALD,
                    }
                } else {
                    CELL_BG
                };
                egui::Frame::none().fill(fill).rounding(Rounding::same(8.0)).stroke(Stroke::new(1.0_f32, EMERALD_TEXT)).inner_margin(10.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("o[{}]", idx)).font(egui::FontId::proportional(10.0)).color(MUTED));
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(16.0)).strong().color(Color32::WHITE));
                    });
                });
            }
        });
    }
}
