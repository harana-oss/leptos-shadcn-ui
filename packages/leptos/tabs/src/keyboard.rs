//! Pure keyboard-navigation helpers for Tabs (WAI-ARIA tabs pattern).
//!
//! These functions contain no DOM access so they can be unit tested natively.

/// Orientation of a tablist; decides which arrow keys move focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    /// Parses `"vertical"` (case-insensitive); anything else is horizontal.
    pub fn parse(value: &str) -> Self {
        if value.eq_ignore_ascii_case("vertical") {
            Self::Vertical
        } else {
            Self::Horizontal
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Returns the index of the trigger that should receive focus after `key` is
/// pressed while the trigger at `current` is focused.
///
/// `disabled[i]` says whether trigger `i` is disabled; disabled triggers are
/// skipped. Arrow keys wrap around. Returns `None` when the key is not a
/// navigation key for this orientation or no enabled trigger exists.
pub fn next_index(
    current: Option<usize>,
    key: &str,
    disabled: &[bool],
    orientation: Orientation,
) -> Option<usize> {
    let len = disabled.len();
    if disabled.iter().all(|d| *d) {
        return None;
    }
    let (prev_key, next_key) = match orientation {
        Orientation::Horizontal => ("ArrowLeft", "ArrowRight"),
        Orientation::Vertical => ("ArrowUp", "ArrowDown"),
    };
    let first = || (0..len).find(|&i| !disabled[i]);
    let last = || (0..len).rev().find(|&i| !disabled[i]);
    let step = |start: usize, forward: bool| {
        (1..=len)
            .map(|offset| {
                if forward {
                    (start + offset) % len
                } else {
                    (start + len * 2 - offset) % len
                }
            })
            .find(|&i| !disabled[i])
    };

    match key {
        "Home" => first(),
        "End" => last(),
        k if k == next_key => match current {
            Some(c) if c < len => step(c, true),
            _ => first(),
        },
        k if k == prev_key => match current {
            Some(c) if c < len => step(c, false),
            _ => last(),
        },
        _ => None,
    }
}

/// Index of the trigger that gets `tabindex="0"` (roving tabindex): the
/// selected trigger if it is enabled, otherwise the first enabled trigger.
pub fn tab_stop_index(selected: Option<usize>, disabled: &[bool]) -> Option<usize> {
    match selected {
        Some(s) if s < disabled.len() && !disabled[s] => Some(s),
        _ => disabled.iter().position(|d| !*d),
    }
}

/// Turns an arbitrary tab value into a string safe for use inside an HTML id.
pub fn sanitize_id_part(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const H: Orientation = Orientation::Horizontal;
    const V: Orientation = Orientation::Vertical;

    #[test]
    fn arrow_right_moves_forward_and_wraps() {
        let d = [false, false, false];
        assert_eq!(next_index(Some(0), "ArrowRight", &d, H), Some(1));
        assert_eq!(next_index(Some(2), "ArrowRight", &d, H), Some(0));
    }

    #[test]
    fn arrow_left_moves_back_and_wraps() {
        let d = [false, false, false];
        assert_eq!(next_index(Some(1), "ArrowLeft", &d, H), Some(0));
        assert_eq!(next_index(Some(0), "ArrowLeft", &d, H), Some(2));
    }

    #[test]
    fn skips_disabled() {
        let d = [false, true, false, true];
        assert_eq!(next_index(Some(0), "ArrowRight", &d, H), Some(2));
        assert_eq!(next_index(Some(2), "ArrowRight", &d, H), Some(0));
        assert_eq!(next_index(Some(0), "ArrowLeft", &d, H), Some(2));
    }

    #[test]
    fn home_end_skip_disabled() {
        let d = [true, false, false, true];
        assert_eq!(next_index(Some(2), "Home", &d, H), Some(1));
        assert_eq!(next_index(Some(1), "End", &d, H), Some(2));
    }

    #[test]
    fn orientation_selects_arrow_keys() {
        let d = [false, false];
        assert_eq!(next_index(Some(0), "ArrowDown", &d, H), None);
        assert_eq!(next_index(Some(0), "ArrowDown", &d, V), Some(1));
        assert_eq!(next_index(Some(1), "ArrowUp", &d, V), Some(0));
        assert_eq!(next_index(Some(0), "ArrowRight", &d, V), None);
    }

    #[test]
    fn unknown_key_and_empty_lists() {
        assert_eq!(next_index(Some(0), "a", &[false], H), None);
        assert_eq!(next_index(None, "Home", &[], H), None);
        assert_eq!(next_index(Some(0), "End", &[true, true], H), None);
        assert_eq!(next_index(None, "ArrowRight", &[true, false], H), Some(1));
    }

    #[test]
    fn tab_stop_prefers_enabled_selection() {
        assert_eq!(tab_stop_index(Some(1), &[false, false]), Some(1));
        assert_eq!(tab_stop_index(Some(0), &[true, false]), Some(1));
        assert_eq!(tab_stop_index(None, &[true, true]), None);
    }

    #[test]
    fn sanitizes_ids() {
        assert_eq!(sanitize_id_part("a b/c"), "a-b-c");
        assert_eq!(Orientation::parse("Vertical"), V);
        assert_eq!(Orientation::parse("x").as_str(), "horizontal");
    }
}
