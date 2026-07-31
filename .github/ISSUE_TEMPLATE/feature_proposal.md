---
name: Feature Proposal / RFC
about: Pitch a new feature, visualizer improvement, or architecture enhancement and propose its technical implementation plan
title: 'proposal: '
labels: 'enhancement, proposal, rfc'
assignees: ''
---

## Feature Pitch & Overview
[A clear and concise 2-3 sentence summary of the proposed feature or visualizer enhancement]

## Motivation & User Impact
[What problem does this feature solve, or how does it improve the visual learning experience for algorithm students?]

## Proposed Technical Implementation Plan
Describe how this feature will interact with the AlgoBuddy codebase:

1. **Affected Modules**:
   - [ ] `src/ui/` (UI layout, panels, canvas renderers, theme palette)
   - [ ] `src/engine.rs` (Deterministic timeline generation)
   - [ ] `src/model/` (Problem definitions, taxonomy, visual state enum)
   - [ ] `src/algorithms/` (Step snapshot generators)

2. **Data Structures & State**:
   [Describe any new fields in `VisualizerApp` or variants added to `VisualState`]

3. **Step-by-Step Implementation Steps**:
   - Step 1: ...
   - Step 2: ...

## Alternatives & Trade-offs
[Describe any alternative designs, UI layouts, or architectural approaches evaluated]

## Questions & Feedback Needed
[List any open design questions or specific areas where you would like maintainer feedback before starting code execution]
