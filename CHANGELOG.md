# Changelog

All notable changes to the AlgoBuddy project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Fixed GitHub Pages releases to run directly from `main` after the native quality gates, require a successful browser launch test before deployment, and launch the published commit in a second live-site check.

## [0.9.0] - 2026-08-10

### Added
- Added catalog-wide regression coverage for all 150 problems, checking metadata, approach traces, source-line references, placeholder implementations, and visual-state bounds.
- Added focused regression tests for engine dispatch and repaired algorithm traces.
- Added `RELEASING.md` and `scripts/release-check.ps1` to standardize version checks, formatting, Clippy, tests, native builds, and WebAssembly release builds.
- Added hover guidance for playback controls and speed, including their keyboard shortcuts.

### Changed
- Split engine dispatch, graph algorithms and metadata, canvas renderers, and test code into smaller contributor-friendly modules.
- Expanded the visual-state model to 24 typed variants, including a dedicated Binary Tree Maximum Path Sum state.
- Updated contribution documentation and issue/PR templates to emphasize edge cases and synchronization between timeline descriptions, inspector state, rendered state, and highlighted code.
- Replaced emoji in maintained UI labels, HTML fallback text, workflow comments, and contributor-facing text with plain language.
- Updated dashboard progress bars to use the active theme palette.
- Simplified the Reset button label while retaining its `R` shortcut in the tooltip.

### Fixed
- Fixed Group Anagrams rendering so every HashMap bucket remains visible when a new signature is added.
- Fixed Valid Palindrome match steps so pointer state, descriptions, source highlighting, and invariants remain synchronized.
- Fixed Min Stack traces to preserve integer values and accurately represent stack state and source lines.
- Fixed Generate Parentheses prefix indices and completion-state semantics.
- Replaced the placeholder Merge K Sorted Lists flatten-and-sort trace with a bounded min-heap implementation and matching visualization.
- Fixed Max Area of Island to compute and trace island area instead of reusing the Number of Islands result.
- Fixed Construct Binary Tree to reconstruct from both preorder and inorder traversals and emit progressive tree states.
- Fixed Binary Tree Maximum Path Sum to compute post-order gains and display the running maximum path sum.
- Corrected stale source-line mappings across several array, stack, heap, linked-list, dynamic-programming, interval, bit, and math traces.

### Removed
- Removed the redundant public/developer mode split and problem audit-status gating. All 150 problems are now always available.

## [0.8.1] - 2026-07-31

### Fixed
- **Roadmap Dashboard Rendering Bug**: Fixed an issue where opening NeetCode Roadmap (`ViewMode::RoadmapDashboard`) failed to render `render_fullscreen_roadmap_dashboard` in the central panel, resulting in an empty black display.

## [0.8.0] - 2026-07-31

### Refactored
- **Phase 2 Architecture Completion**: Finished all core Phase 2 architecture refactors for v0.8.0.
- **Dynamic Input State Map (Task 2.2)**: Replaced 30+ ad-hoc input string/integer fields on `VisualizerApp` with `input_strings` and `input_integers` `HashMap` state stores (`get_input_str`, `set_input_str`, `get_input_int`, `set_input_int`).
- **Normalized VisualState Enum (Task 2.1)**: Structured `VisualState` into 10 canonical visual layout categories (`Array1D`, `TwoPointers`, `BinarySearch`, `StackVisual`, `LinkedListVisual`, `TreeVisual`, `HeapVisual`, `GridGraph`, `NodeGraph`, `DecisionTreeVisual`) for clean renderer separation.
- **Strict Clippy Compliance (Task 2.4)**: Removed 18 top-level blanket `#![allow(...)]` suppressions in `src/main.rs` and resolved all submodule lints for 0-warning compilation across all build targets.
- **Problem Model Decomposition (Task 2.3)**: Organized all 150 problem details and source code lines into 18 category submodules in `src/model/problems/`.

### Added
- **Derive `Hash` for `Problem`**: Derived `Hash` on `Problem` enum for type-safe key indexing in state stores.
- **Comprehensive Documentation**: Updated `README.md` and `CONTRIBUTING.md` to reflect the Phase 2 architecture and public release status.

## [0.7.1] - 2026-07-31

### Refactored
- **Problem Model Decomposition**: Split the monolithic `src/model/problem.rs` file (~4,000 lines) into 18 category-based submodules under `src/model/problems/` (e.g. `arrays_hashing.rs`, `trees.rs`, `two_pointers.rs`, `graphs.rs`), with `src/model/problems/mod.rs` dispatching problem details and code lines.
- **UI Component Extraction**: Decomposed the monolithic `src/app.rs` file into specialized UI renderers within `src/ui/` (`canvas.rs`, `sidebar.rs`, `dashboard.rs`, `header.rs`, `inspector.rs`, `modals.rs`, `playground.rs`, `theme_helpers.rs`).
- **Engine Decoupling**: Isolated deterministic algorithm execution logic into a dedicated `src/engine.rs` module for better testability and contributor onboarding.

### Added
- **100% Public Release Mode**: Bulk-audited all 150 NeetCode problems to `AuditStatus::Audited`, making every visualizer available in Public Release Mode without requiring `--dev`.
- **New GitHub Issue Templates**: Added `unit_test.md` (guided unit test contribution requests) and `feature_proposal.md` (Feature Proposal / RFC template) to `.github/ISSUE_TEMPLATE/`.
- **UI Quality-of-Life**: Integrated search query clear ("x") button in the Roadmap sidebar (from PR #19) and a reset confirmation dialog on the NeetCode Mastery Dashboard (from PR #25).

### Fixed
- **WASM Timer Compatibility**: Replaced native `std::time::Instant` with `web_time::Instant` across UI panels and engine timing to ensure WebAssembly targets compile cleanly for browser execution.
- **Clippy Strict Compliance**: Resolved single-binding match linting warnings in problem model dispatches.

## [0.6.0] - 2026-07-30

### Added
- **100% NeetCode 150 Roadmap Completion**: Implemented all 15 remaining visualizers across Linked List and Trees (bringing total problem count to 150 / 150).
- **100% Audited Status**: Promoted all 150 problems to `is_audited() -> true` across all 18 categories for Public Release Mode.
- New Linked List Visualizers: Reorder List (#143), Remove Nth Node From End (#19), Copy List with Random Pointer (#138), Add Two Numbers (#2), Find Duplicate Number (#287), LRU Cache (#146), Merge K Sorted Lists (#23), Reverse Nodes in K-Group (#25).
- New Tree Visualizers: Level Order Traversal (#102), Right Side View (#199), Count Good Nodes (#1448), Kth Smallest Element in BST (#230), Construct Binary Tree (#105), Max Path Sum (#124), Serialize & Deserialize Binary Tree (#297).

## [0.5.0] - 2026-07-29


### Added
- Binary Search Category Completion: Added visualizer for Koko Eating Bananas (LeetCode #875).
- Full-Screen NeetCode 150 Mastery Dashboard with completion tracking and state persistence.
- Automated CI quality gates workflow with Cargo check, Clippy linter, and unit test suite execution.
- CodeQL static security analysis workflow integration.
- Dependabot configuration for Cargo crates and GitHub Actions updates.
- Automated WebAssembly deployment workflow for GitHub Pages.
- Added issue and pull request templates for open-source contributions.

### Fixed
- Fixed Roadmap Sidebar collapsible header expand state and text truncation behavior.
- Constrained title width in sidebar items to prevent difficulty badge clipping.
- Removed dark border strokes between application side panels and central canvas.

### Changed
- Standardized UI layout spacing and theme palette color contrast across all 18 categories.
- Reorganized codebase data models into modular `src/model/` package.
