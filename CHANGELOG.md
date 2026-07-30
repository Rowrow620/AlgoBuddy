# Changelog

All notable changes to the AlgoBuddy project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
