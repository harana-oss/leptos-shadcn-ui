//! Pure keyboard-handling helpers for Select (WAI-ARIA select-only combobox).
//!
//! No DOM access here, so the logic is unit testable natively.

/// What the trigger should do in response to a key press.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectKeyAction {
    /// Open the listbox and highlight the given option index (if any).
    Open(Option<usize>),
    /// Move the highlight to the given option index.
    Highlight(usize),
    /// Select the highlighted option (if any) and close.
    SelectActive,
    /// Close and keep focus on the trigger.
    Close,
    /// Close without preventing the default action (e.g. Tab moves focus on).
    CloseAndContinue,
    /// The key is not handled.
    None,
}

impl SelectKeyAction {
    /// Whether the browser default should be prevented for this action.
    pub fn prevents_default(self) -> bool {
        !matches!(self, Self::None | Self::CloseAndContinue)
    }
}

fn first_enabled(disabled: &[bool]) -> Option<usize> {
    disabled.iter().position(|d| !*d)
}

fn last_enabled(disabled: &[bool]) -> Option<usize> {
    disabled.iter().rposition(|d| !*d)
}

/// Moves the highlighted option for `key`, skipping disabled options.
/// Arrow keys do not wrap (matching native `<select>`). Returns `None` if the
/// key is not a navigation key or there is nothing to move to.
pub fn next_index(current: Option<usize>, key: &str, disabled: &[bool]) -> Option<usize> {
    let len = disabled.len();
    match key {
        "Home" | "PageUp" => first_enabled(disabled),
        "End" | "PageDown" => last_enabled(disabled),
        "ArrowDown" => match current {
            Some(c) if c < len => (c + 1..len).find(|&i| !disabled[i]).or(Some(c)),
            _ => first_enabled(disabled),
        },
        "ArrowUp" => match current {
            Some(c) if c < len => (0..c).rev().find(|&i| !disabled[i]).or(Some(c)),
            _ => last_enabled(disabled),
        },
        _ => None,
    }
}

/// Decides what a key press on the trigger does.
///
/// * `open` – whether the listbox is currently open.
/// * `active` – index of the highlighted option.
/// * `selected` – index of the currently selected option.
/// * `disabled` – disabled flag per option, in DOM order.
pub fn key_action(
    key: &str,
    open: bool,
    active: Option<usize>,
    selected: Option<usize>,
    disabled: &[bool],
) -> SelectKeyAction {
    let enabled_selected = selected.filter(|&s| s < disabled.len() && !disabled[s]);
    if !open {
        return match key {
            "Enter" | " " | "ArrowDown" => {
                SelectKeyAction::Open(enabled_selected.or_else(|| first_enabled(disabled)))
            }
            "ArrowUp" => SelectKeyAction::Open(enabled_selected.or_else(|| last_enabled(disabled))),
            "Home" => SelectKeyAction::Open(first_enabled(disabled)),
            "End" => SelectKeyAction::Open(last_enabled(disabled)),
            _ => SelectKeyAction::None,
        };
    }
    match key {
        "Enter" | " " => SelectKeyAction::SelectActive,
        "Escape" => SelectKeyAction::Close,
        "Tab" => SelectKeyAction::CloseAndContinue,
        _ => match next_index(active, key, disabled) {
            Some(i) => SelectKeyAction::Highlight(i),
            None => SelectKeyAction::None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_move_without_wrapping_and_skip_disabled() {
        let d = [false, true, false];
        assert_eq!(next_index(Some(0), "ArrowDown", &d), Some(2));
        assert_eq!(next_index(Some(2), "ArrowDown", &d), Some(2));
        assert_eq!(next_index(Some(2), "ArrowUp", &d), Some(0));
        assert_eq!(next_index(Some(0), "ArrowUp", &d), Some(0));
        assert_eq!(next_index(None, "ArrowDown", &d), Some(0));
        assert_eq!(next_index(None, "ArrowUp", &d), Some(2));
    }

    #[test]
    fn home_end() {
        let d = [true, false, false, true];
        assert_eq!(next_index(Some(2), "Home", &d), Some(1));
        assert_eq!(next_index(Some(1), "End", &d), Some(2));
        assert_eq!(next_index(Some(1), "x", &d), None);
        assert_eq!(next_index(None, "Home", &[]), None);
    }

    #[test]
    fn closed_keys_open() {
        let d = [false, false, false];
        assert_eq!(
            key_action("Enter", false, None, None, &d),
            SelectKeyAction::Open(Some(0))
        );
        assert_eq!(
            key_action(" ", false, None, Some(1), &d),
            SelectKeyAction::Open(Some(1))
        );
        assert_eq!(
            key_action("ArrowDown", false, None, None, &d),
            SelectKeyAction::Open(Some(0))
        );
        assert_eq!(
            key_action("ArrowUp", false, None, None, &d),
            SelectKeyAction::Open(Some(2))
        );
        assert_eq!(
            key_action("Escape", false, None, None, &d),
            SelectKeyAction::None
        );
        // Disabled selected option falls back to the first enabled one.
        assert_eq!(
            key_action("Enter", false, None, Some(0), &[true, false]),
            SelectKeyAction::Open(Some(1))
        );
    }

    #[test]
    fn open_keys() {
        let d = [false, false];
        assert_eq!(
            key_action("Enter", true, Some(0), None, &d),
            SelectKeyAction::SelectActive
        );
        assert_eq!(
            key_action(" ", true, Some(0), None, &d),
            SelectKeyAction::SelectActive
        );
        assert_eq!(
            key_action("Escape", true, Some(0), None, &d),
            SelectKeyAction::Close
        );
        assert_eq!(
            key_action("Tab", true, Some(0), None, &d),
            SelectKeyAction::CloseAndContinue
        );
        assert_eq!(
            key_action("ArrowDown", true, Some(0), None, &d),
            SelectKeyAction::Highlight(1)
        );
        assert_eq!(
            key_action("End", true, Some(0), None, &d),
            SelectKeyAction::Highlight(1)
        );
        assert_eq!(
            key_action("q", true, Some(0), None, &d),
            SelectKeyAction::None
        );
    }

    #[test]
    fn prevent_default_flags() {
        assert!(SelectKeyAction::Close.prevents_default());
        assert!(SelectKeyAction::Open(None).prevents_default());
        assert!(!SelectKeyAction::CloseAndContinue.prevents_default());
        assert!(!SelectKeyAction::None.prevents_default());
    }
}
