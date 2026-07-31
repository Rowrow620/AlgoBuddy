# Changelog

All notable changes to the AlgoBuddy project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - Unreleased

### Refactored
- **Architecture Modularity**: Successfully extracted the monolithic `src/app.rs` into specialized UI renderers within the `src/ui/` module (e.g., `canvas.rs`, `sidebar.rs`, `dashboard.rs`).
- **Engine Decoupling**: Isolated deterministic algorithm execution logic into a dedicated `src/engine.rs` module for better testability and contributor onboarding.

### Added
- **UI Quality-of-Life**: Added a clear ("x") button to the Roadmap search bar (from PR #19) and a reset confirmation dialog for the NeetCode Mastery Dashboard (from PR #25).

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
- Open Source community guidelines, Security Policy, Code of Conduct, and Issue / Pull Request templates.

### Fixed
- Fixed Roadmap Sidebar collapsible header expand state and text truncation behavior.
- Constrained title width in sidebar items to prevent difficulty badge clipping.
- Removed dark border strokes between application side panels and central canvas.

### Changed
- Standardized UI layout spacing and theme palette color contrast across all 18 categories.
- Reorganized codebase data models into modular `src/model/` package.
