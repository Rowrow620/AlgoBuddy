use eframe::egui::Color32;

// ── Themes & Accessibility ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    DarkVSCode,
    DarkCyber,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::DarkVSCode => "VS Code Dark",
            Theme::DarkCyber => "Cyber Navy (Dark)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorblindMode {
    Off,
    RedGreenSafe,
    HighContrast,
}

impl ColorblindMode {
    pub fn label(&self) -> &'static str {
        match self {
            ColorblindMode::Off => "Off (Standard)",
            ColorblindMode::RedGreenSafe => "Protan / Deuteran (Blue-Orange)",
            ColorblindMode::HighContrast => "High Contrast B&W",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub bg_dark: Color32,
    pub sidebar_bg: Color32,
    pub step_box_bg: Color32,
    pub cell_bg: Color32,
    pub cell_border: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub text_dim: Color32,
    pub cyan: Color32,
    pub purple: Color32,
    pub emerald: Color32,
    pub emerald_text: Color32,
    pub amber: Color32,
    pub pink: Color32,
    pub red: Color32,
    pub code_active_bg: Color32,
}

impl Theme {
    pub fn palette(&self, cb: ColorblindMode) -> ThemePalette {
        let (base_emerald, base_emerald_text, base_red) = match cb {
            ColorblindMode::Off => (
                Color32::from_rgb(16, 185, 129),
                Color32::from_rgb(52, 211, 153),
                Color32::from_rgb(244, 63, 94),
            ),
            ColorblindMode::RedGreenSafe => (
                Color32::from_rgb(37, 99, 235),
                Color32::from_rgb(96, 165, 250),
                Color32::from_rgb(234, 88, 12),
            ),
            ColorblindMode::HighContrast => (
                Color32::from_rgb(255, 255, 255),
                Color32::from_rgb(255, 255, 255),
                Color32::from_rgb(255, 255, 0),
            ),
        };

        let mut palette = match self {
            Theme::DarkVSCode => ThemePalette {
                bg_dark: Color32::from_rgb(24, 24, 24),
                sidebar_bg: Color32::from_rgb(30, 30, 30),
                step_box_bg: Color32::from_rgb(37, 37, 38),
                cell_bg: Color32::from_rgb(45, 45, 48),
                cell_border: Color32::from_rgb(60, 60, 60),
                text_primary: Color32::from_rgb(220, 220, 220),
                text_muted: Color32::from_rgb(160, 160, 160),
                text_dim: Color32::from_rgb(110, 110, 110),
                cyan: Color32::from_rgb(86, 156, 214),
                purple: Color32::from_rgb(197, 134, 192),
                emerald: base_emerald,
                emerald_text: base_emerald_text,
                amber: Color32::from_rgb(206, 145, 120),
                pink: Color32::from_rgb(220, 100, 170),
                red: base_red,
                code_active_bg: Color32::from_rgb(9, 71, 113),
            },
            Theme::DarkCyber => ThemePalette {
                bg_dark: Color32::from_rgb(11, 15, 25),
                sidebar_bg: Color32::from_rgb(15, 23, 42),
                step_box_bg: Color32::from_rgb(30, 41, 59),
                cell_bg: Color32::from_rgb(30, 41, 59),
                cell_border: Color32::from_rgb(51, 65, 85),
                text_primary: Color32::from_rgb(248, 250, 252),
                text_muted: Color32::from_rgb(156, 163, 175),
                text_dim: Color32::from_rgb(100, 116, 139),
                cyan: Color32::from_rgb(56, 189, 248),
                purple: Color32::from_rgb(168, 85, 247),
                emerald: base_emerald,
                emerald_text: base_emerald_text,
                amber: Color32::from_rgb(245, 158, 11),
                pink: Color32::from_rgb(236, 72, 153),
                red: base_red,
                code_active_bg: Color32::from_rgb(14, 116, 144),
            },
        };

        if cb == ColorblindMode::HighContrast {
            palette.cyan = Color32::WHITE;
            palette.purple = Color32::WHITE;
            palette.emerald = Color32::WHITE;
            palette.emerald_text = Color32::WHITE;
            palette.amber = Color32::WHITE;
            palette.pink = Color32::WHITE;
            palette.red = Color32::WHITE;
            palette.code_active_bg = Color32::from_rgb(60, 60, 60);
        }

        palette
    }
}

// ── Difficulty Level ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}

// ── Roadmap Categories (NeetCode 150 Hierarchy) ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    ArraysAndHashing,
    TwoPointers,
    Stack,
    BinarySearch,
    SlidingWindow,
    LinkedList,
    Trees,
    Tries,
    Backtracking,
    HeapPriorityQueue,
    Graphs,
    OneDDp,
    Intervals,
    Greedy,
    AdvancedGraphs,
    TwoDDp,
    BitManipulation,
    MathAndGeometry,
}

impl Category {
    pub fn name(&self) -> &'static str {
        match self {
            Category::ArraysAndHashing => "Arrays & Hashing",
            Category::TwoPointers => "Two Pointers",
            Category::Stack => "Stack",
            Category::BinarySearch => "Binary Search",
            Category::SlidingWindow => "Sliding Window",
            Category::LinkedList => "Linked List",
            Category::Trees => "Trees",
            Category::Tries => "Tries",
            Category::Backtracking => "Backtracking",
            Category::HeapPriorityQueue => "Heap / Priority Queue",
            Category::Graphs => "Graphs",
            Category::OneDDp => "1-D DP",
            Category::Intervals => "Intervals",
            Category::Greedy => "Greedy",
            Category::AdvancedGraphs => "Advanced Graphs",
            Category::TwoDDp => "2-D DP",
            Category::BitManipulation => "Bit Manipulation",
            Category::MathAndGeometry => "Math & Geometry",
        }
    }

    pub fn all() -> &'static [Category] {
        &[
            Category::ArraysAndHashing,
            Category::TwoPointers,
            Category::Stack,
            Category::BinarySearch,
            Category::SlidingWindow,
            Category::LinkedList,
            Category::Trees,
            Category::Tries,
            Category::Backtracking,
            Category::HeapPriorityQueue,
            Category::Graphs,
            Category::OneDDp,
            Category::Intervals,
            Category::Greedy,
            Category::AdvancedGraphs,
            Category::TwoDDp,
            Category::BitManipulation,
            Category::MathAndGeometry,
        ]
    }
}
