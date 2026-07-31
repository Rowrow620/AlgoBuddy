use crate::model::{Difficulty, ThemePalette};
use eframe::egui::Color32;

pub fn difficulty_color(d: Difficulty, p: &ThemePalette) -> Color32 {
    match d {
        Difficulty::Easy => p.emerald_text,
        Difficulty::Medium => p.amber,
        Difficulty::Hard => p.red,
    }
}
