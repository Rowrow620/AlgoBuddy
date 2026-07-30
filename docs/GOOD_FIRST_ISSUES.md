# AlgoBuddy — Good First Issue Templates

> **Target Repository**: [Rowrow620/AlgoBuddy](https://github.com/Rowrow620/AlgoBuddy)  
> **Create New Issue**: [https://github.com/Rowrow620/AlgoBuddy/issues/new](https://github.com/Rowrow620/AlgoBuddy/issues/new)

---

## Issue 1: Reset Progress Confirmation Modal

* **Title**: `[good first issue] Add confirmation popup before resetting NeetCode Dashboard progress`
* **Labels**: `good first issue`, `ui`, `help wanted`

```markdown
## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. We know that submitting a first pull request can feel intimidating. The goal of this issue is to guide you step-by-step through making your first contribution to the AlgoBuddy repository.

## Description of the Issue
On the NeetCode 150 Mastery Dashboard, clicking the "Reset Progress" button currently clears all completed problem checkmarks immediately without asking for confirmation. If a user accidentally clicks this button, their saved progress is wiped instantly.

## Proposed Solution
Add a modal dialog confirmation window that appears when "Reset Progress" is clicked, asking the user to confirm before wiping their completed problem checkmarks.

## Implementation Steps
1. Open `src/app.rs` and add a boolean field `show_reset_confirm_modal: bool` to the `VisualizerApp` struct.
2. In the dashboard rendering section of `src/app.rs`, update the "Reset Progress" button handler to set `self.show_reset_confirm_modal = true` instead of clearing progress directly.
3. Implement a helper function `render_reset_confirm_modal(&mut self, ctx: &egui::Context)` in `src/app.rs`:
   ```rust
   if self.show_reset_confirm_modal {
       egui::Window::new("Confirm Reset")
           .collapsible(false)
           .resizable(false)
           .show(ctx, |ui| {
               ui.label("Are you sure you want to reset all completed problem checkmarks?");
               ui.horizontal(|ui| {
                   if ui.button("Cancel").clicked() {
                       self.show_reset_confirm_modal = false;
                   }
                   if ui.button("Confirm Reset").clicked() {
                       self.completed_problems.clear();
                       self.show_reset_confirm_modal = false;
                   }
               });
           });
   }
   ```
4. Verify by running `cargo run` and testing both "Cancel" and "Confirm Reset" buttons.

## Acceptance Criteria
To merge a pull request for this issue:
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Issue Solved: Clicking "Reset Progress" opens a confirmation popup; progress is only wiped when confirmed.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `issue-1-reset-modal`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
```

---

## Issue 2: Search Clear ("x") Button for Sidebar

* **Title**: `[good first issue] Add search clear ("x") button to Roadmap search bar`
* **Labels**: `good first issue`, `ui`, `help wanted`

```markdown
## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. The goal of this issue is to guide you step-by-step through making your first contribution to the AlgoBuddy repository.

## Description of the Issue
When filtering problems in the Roadmap sidebar, clearing the search query currently requires manually backspacing all characters in the text input box.

## Proposed Solution
Add a small "x" button next to the search filter text box that clears the active search string in a single click.

## Implementation Steps
1. Open `src/app.rs` and locate the sidebar search input rendering code.
2. Wrap the search input in a horizontal layout and conditionally display a clear button:
   ```rust
   ui.horizontal(|ui| {
       ui.add(egui::TextEdit::singleline(&mut self.search_filter).hint_text("Search..."));
       if !self.search_filter.is_empty() {
           if ui.small_button("x").on_hover_text("Clear search").clicked() {
               self.search_filter.clear();
           }
       }
   });
   ```
3. Test locally using `cargo run` by typing a query and clicking "x".

## Acceptance Criteria
To merge a pull request for this issue:
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Issue Solved: The clear button appears when text is entered and clears the search box when clicked.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `issue-2-search-clear`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
```

---

## Issue 3: Category Completion Ratios on Dashboard

* **Title**: `[good first issue] Add category completion ratios to NeetCode Mastery Dashboard`
* **Labels**: `good first issue`, `dashboard`, `help wanted`

```markdown
## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. The goal of this issue is to guide you step-by-step through making your first contribution to the AlgoBuddy repository.

## Description of the Issue
On the NeetCode 150 Mastery Dashboard, category headers display the topic title (e.g. "Arrays & Hashing"), but do not show how many problems within that category have been completed.

## Proposed Solution
Display completion ratios (e.g. `Arrays & Hashing (9/9)`, `Two Pointers (5/5)`) inside each category header card on the dashboard.

## Implementation Steps
1. Open `src/app.rs` and locate `render_dashboard`.
2. For each category card, calculate the completed problem count:
   ```rust
   let completed_count = category_problems
       .iter()
       .filter(|p| self.completed_problems.contains(&p.id()))
       .count();
   let total_count = category_problems.len();

   let title_text = format!("{} ({}/{})", category.name(), completed_count, total_count);
   ui.heading(RichText::new(title_text).color(p.cyan).strong());
   ```
3. Run `cargo run` and open the NeetCode 150 Dashboard to verify ratios display accurately.

## Acceptance Criteria
To merge a pull request for this issue:
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Issue Solved: Category headers display correct completion ratios.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `issue-3-category-ratios`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
```

---

## Issue 4: Step Generation Execution Duration Badge

* **Title**: `[good first issue] Display step generation execution time in Scope Inspector`
* **Labels**: `good first issue`, `ui`, `performance`

```markdown
## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. The goal of this issue is to guide you step-by-step through making your first contribution to the AlgoBuddy repository.

## Description of the Issue
The Scope Inspector panel displays active variables and algorithm invariants, but does not indicate how long step generation took.

## Proposed Solution
Measure the execution duration of `recompute_steps()` using `web_time::Instant` and display a small duration badge in the footer of the Scope Inspector.

## Implementation Steps
1. Open `src/app.rs` and add `last_compute_duration: std::time::Duration` to `VisualizerApp`.
2. Record duration inside `recompute_steps()`:
   ```rust
   let start = Instant::now();
   // step generation logic ...
   self.last_compute_duration = start.elapsed();
   ```
3. Render the duration text in the Scope Inspector footer:
   ```rust
   ui.label(
       RichText::new(format!("Steps Generated in {:.2?}", self.last_compute_duration))
           .size(11.0)
           .color(p.text_muted),
   );
   ```
4. Verify by running `cargo run` and switching between problems.

## Acceptance Criteria
To merge a pull request for this issue:
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Issue Solved: Step generation duration is displayed in milliseconds.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `issue-4-compute-duration`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
```

---

## Issue 5: Rustdoc Comments for VisualState Structs

* **Title**: `[good first issue] Add Rustdoc comments to VisualState and Taxonomy structs`
* **Labels**: `good first issue`, `documentation`, `help wanted`

```markdown
## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. The goal of this issue is to guide you step-by-step through making your first contribution to the AlgoBuddy repository.

## Description of the Issue
Improve developer documentation by adding `///` Rustdoc comments to public structs and enums in `src/model/visual_state.rs` and `src/model/taxonomy.rs`.

## Proposed Solution
Add comprehensive `///` doc comments above variants and fields in `src/model/visual_state.rs` and `src/model/taxonomy.rs`.

## Implementation Steps
1. Open `src/model/visual_state.rs` and add documentation comments above each `VisualState` enum variant:
   ```rust
   /// Single-dimensional array visual state snapshot.
   /// Used for Two Pointers, Sliding Window, and 1D DP visualizers.
   Array1D {
       /// Title displayed at top of renderer
       title: String,
       /// Array element values
       elements: Vec<i32>,
       /// Active element index highlight
       active_idx: Option<usize>,
       // ...
   },

   /// 2D Matrix Grid visual state snapshot.
   /// Used for 2D DP, Flood Fill, and Grid Traversal visualizers.
   TwoDGrid { ... }
   ```
2. Verify documentation builds without warnings by running `cargo doc --open`.

## Acceptance Criteria
To merge a pull request for this issue:
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Documentation Quality: All `VisualState` variants and taxonomy structs have `///` comments.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `issue-5-rustdoc-comments`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
```
