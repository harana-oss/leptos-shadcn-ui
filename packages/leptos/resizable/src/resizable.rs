use leptos::prelude::*;

/// Resize direction for panels
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum ResizeDirection {
    #[default]
    Horizontal,
    Vertical,
}


/// Resizable state management
#[derive(Debug, Clone)]
pub struct ResizableState {
    pub panel_sizes: Vec<f64>,
    pub is_resizing: bool,
    pub resize_direction: ResizeDirection,
    pub collapsed_panels: Vec<usize>,
}

impl Default for ResizableState {
    fn default() -> Self {
        Self {
            panel_sizes: Vec::new(),
            is_resizing: false,
            resize_direction: ResizeDirection::Horizontal,
            collapsed_panels: Vec::new(),
        }
    }
}

/// Resizable configuration
#[derive(Debug, Clone)]
pub struct ResizableConfig {
    pub default_sizes: Vec<f64>,
    pub min_sizes: Vec<f64>,
    pub max_sizes: Vec<f64>,
    pub collapsible: Vec<bool>,
    pub collapsed_sizes: Vec<f64>,
    pub keyboard_resize: bool,
    pub touch_support: bool,
}

impl Default for ResizableConfig {
    fn default() -> Self {
        Self {
            default_sizes: vec![50.0, 50.0],
            min_sizes: vec![10.0, 10.0],
            max_sizes: vec![90.0, 90.0],
            collapsible: vec![false, false],
            collapsed_sizes: vec![0.0, 0.0],
            keyboard_resize: false,
            touch_support: false,
        }
    }
}

/// Resizable context for managing state across components
#[derive(Clone)]
pub struct ResizableContext {
    pub state: RwSignal<ResizableState>,
    pub config: RwSignal<ResizableConfig>,
    pub update_size: Callback<(usize, f64)>,
    pub toggle_collapse: Callback<usize>,
    pub start_resize: Callback<()>,
    pub end_resize: Callback<()>,
}

impl Default for ResizableContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ResizableContext {
    pub fn new() -> Self {
        let state = RwSignal::new(ResizableState::default());
        let config = RwSignal::new(ResizableConfig::default());

        let update_size = {
            let state = state;
            Callback::new(move |(panel_index, size): (usize, f64)| {
                state.update(|s| {
                    if panel_index < s.panel_sizes.len() {
                        s.panel_sizes[panel_index] = size;
                    }
                });
            })
        };

        let toggle_collapse = {
            let state = state;
            Callback::new(move |panel_index: usize| {
                state.update(|s| {
                    if s.collapsed_panels.contains(&panel_index) {
                        s.collapsed_panels.retain(|&i| i != panel_index);
                    } else {
                        s.collapsed_panels.push(panel_index);
                    }
                });
            })
        };

        let start_resize = {
            let state = state;
            Callback::new(move |_| {
                state.update(|s| s.is_resizing = true);
            })
        };

        let end_resize = {
            let state = state;
            Callback::new(move |_| {
                state.update(|s| s.is_resizing = false);
            })
        };

        Self {
            state,
            config,
            update_size,
            toggle_collapse,
            start_resize,
            end_resize,
        }
    }
}

/// Hook for using resizable context
pub fn use_resizable_context() -> ResizableContext {
    expect_context::<ResizableContext>()
}

/// Utility functions for resizable panels
pub mod utils {
    use super::*;

    /// Calculate new panel sizes when resizing
    pub fn calculate_new_sizes(
        current_sizes: &[f64],
        panel_index: usize,
        new_size: f64,
        min_sizes: &[f64],
        max_sizes: &[f64],
    ) -> Vec<f64> {
        let mut new_sizes = current_sizes.to_vec();
        
        if panel_index >= new_sizes.len() {
            return new_sizes;
        }

        // Find the neighbouring panel that gives or takes the space
        let next_panel_index = if panel_index + 1 < new_sizes.len() {
            panel_index + 1
        } else if panel_index > 0 {
            panel_index - 1
        } else {
            return new_sizes;
        };

        let limits = |i: usize| {
            (
                min_sizes.get(i).copied().unwrap_or(0.0),
                max_sizes.get(i).copied().unwrap_or(100.0),
            )
        };
        let (min_a, max_a) = limits(panel_index);
        let (min_b, max_b) = limits(next_panel_index);
        let (size_a, size_b) = (new_sizes[panel_index], new_sizes[next_panel_index]);

        // Clamp the change so both panels stay within their limits and the total is preserved
        let low = (min_a - size_a).max(size_b - max_b);
        let high = (max_a - size_a).min(size_b - min_b);
        if low > high || !new_size.is_finite() {
            return new_sizes;
        }
        let diff = (new_size - size_a).clamp(low, high);
        new_sizes[panel_index] += diff;
        new_sizes[next_panel_index] -= diff;

        new_sizes
    }

    /// Check if a panel can be resized
    pub fn can_resize(
        panel_index: usize,
        _direction: ResizeDirection,
        is_collapsed: bool,
    ) -> bool {
        !is_collapsed && panel_index > 0
    }

    /// Get the resize handle position
    pub fn get_handle_position(
        _panel_index: usize,
        direction: ResizeDirection,
    ) -> String {
        match direction {
            ResizeDirection::Horizontal => "right".to_string(),
            ResizeDirection::Vertical => "bottom".to_string(),
        }
    }

    /// Calculate total size of all panels
    pub fn calculate_total_size(sizes: &[f64]) -> f64 {
        sizes.iter().sum()
    }

    /// Normalize panel sizes to ensure they sum to 100%
    pub fn normalize_sizes(sizes: &mut [f64]) {
        let total: f64 = sizes.iter().sum();
        if total > 0.0 {
            for size in sizes.iter_mut() {
                *size = (*size / total) * 100.0;
            }
        }
    }
}
