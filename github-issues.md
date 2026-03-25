# GitHub Issues to Create

## Issue 1: Document or remove 1000 entry cap on --all flag

**Title:** `--all` flag caps at 1000 entries but doesn't document this limit

**Labels:** documentation, enhancement

**Body:**
The `--all` flag promises to show "all reflog entries" but silently caps at 1000:

```rust
let limit = if show_all { "1000" } else { "50" };
```

**Options:**
1. Update help text to mention the limit: `--all        # Show all reflog entries (max 1000)`
2. Remove the cap entirely and truly show all entries

**Priority:** Medium - affects user expectations but not functionality

---

## Issue 2: Extract duplicate diff refresh logic into helper method

**Title:** Refactor: Extract duplicate diff refresh logic

**Labels:** refactoring, code-quality

**Body:**
The "update diff if showing" logic is duplicated in 6 places:
- `next()`
- `previous()`
- `Home` key handler
- `End` key handler
- `PageDown` key handler
- `PageUp` key handler

**Suggestion:**
Extract to a helper method:
```rust
fn update_diff_if_visible(&mut self) -> Result<()> {
    if self.show_diff {
        let idx = self.selected_index();
        if let Some(entry) = self.entries.get(idx) {
            self.diff_content = self.git_manager.get_diff_stat(&entry.hash)?;
            self.diff_scroll_offset = 0;
        }
    }
    Ok(())
}
```

Then call it after any selection change.

**Priority:** Low - code quality improvement, no user-facing impact

---

## Issue 3: Optimize selected_index() calls in UI rendering

**Title:** Optimize: Pull selected_index() out of map closure

**Labels:** performance, code-quality

**Body:**
In `ui()`, `app.selected_index()` is called once per list item inside the `.map()` closure:

```rust
.map(|(i, entry)| {
    let selected_idx = app.selected_index();  // Called N times
    let is_selected = i == selected_idx;
    // ...
})
```

**Suggestion:**
Pull it out before the iterator:
```rust
let selected_idx = app.selected_index();
let items: Vec<ListItem> = app.entries.iter()
    .enumerate()
    .map(|(i, entry)| {
        let is_selected = i == selected_idx;
        // ...
    })
```

**Priority:** Low - micro-optimization, the method is cheap (just `unwrap_or`)

---

**Credit:** Issues identified by code reviewer feedback
