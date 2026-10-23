//! Shared resizable implementation used by the default and New York variants.

use leptos::html;
use leptos::prelude::*;
use leptos_style::Style;

use crate::resizable::ResizeDirection;

/// Percentage moved per arrow key press when keyboard resizing is enabled.
pub(crate) const KEYBOARD_STEP: f64 = 5.0;

/// Size and constraints for one registered panel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PanelLayout {
    pub size: f64,
    pub min: f64,
    pub max: f64,
}

/// Context shared between a panel group and its panels and handles.
#[derive(Clone, Copy)]
pub(crate) struct GroupContext {
    pub direction: Signal<ResizeDirection>,
    pub panels: RwSignal<Vec<PanelLayout>>,
    pub handle_count: RwSignal<usize>,
    pub is_resizing: RwSignal<bool>,
    pub keyboard_resize: Signal<bool>,
    pub touch_support: Signal<bool>,
    pub group_ref: NodeRef<html::Div>,
    pub on_resize: Signal<Option<Callback<Vec<f64>>>>,
}

impl GroupContext {
    fn notify(&self) {
        if let Some(callback) = self.on_resize.get_untracked() {
            callback.run(self.panels.with_untracked(|p| p.iter().map(|l| l.size).collect()));
        }
    }

    /// Moves the boundary after panel `before` by `delta` percent.
    pub fn resize(&self, before: usize, delta: f64) {
        let mut changed = false;
        self.panels.update(|panels| changed = apply_delta(panels, before, delta));
        if changed {
            self.notify();
        }
    }

    /// Sets panel `index` to `size` percent, giving or taking space from a neighbour.
    pub fn set_size(&self, index: usize, size: f64) {
        let Some(current) = self.panels.with_untracked(|p| p.get(index).map(|l| l.size)) else {
            return;
        };
        let count = self.panels.with_untracked(Vec::len);
        let delta = size - current;
        if index + 1 < count {
            self.resize(index, delta);
        } else if index > 0 {
            self.resize(index - 1, -delta);
        } else {
            self.panels.update(|p| p[0].size = size.clamp(p[0].min, p[0].max));
            self.notify();
        }
    }
}

/// Moves the boundary between panels `before` and `before + 1` by `delta` percent,
/// clamped so both panels stay within their limits. Returns true if sizes changed.
pub(crate) fn apply_delta(panels: &mut [PanelLayout], before: usize, delta: f64) -> bool {
    if before + 1 >= panels.len() || delta == 0.0 || !delta.is_finite() {
        return false;
    }
    let (a, b) = (panels[before], panels[before + 1]);
    let low = (a.min - a.size).max(b.size - b.max);
    let high = (a.max - a.size).min(b.size - b.min);
    if low > high {
        return false;
    }
    let delta = delta.clamp(low, high);
    if delta == 0.0 {
        return false;
    }
    panels[before].size += delta;
    panels[before + 1].size -= delta;
    true
}

fn px_to_percent(ctx: &GroupContext, delta_px: f64) -> f64 {
    let Some(el) = ctx.group_ref.get_untracked() else {
        return 0.0;
    };
    let rect = el.get_bounding_client_rect();
    let total = match ctx.direction.get_untracked() {
        ResizeDirection::Horizontal => rect.width(),
        ResizeDirection::Vertical => rect.height(),
    };
    if total > 0.0 {
        delta_px / total * 100.0
    } else {
        0.0
    }
}

pub(crate) struct GroupProps {
    pub base_class: &'static str,
    pub direction: MaybeProp<ResizeDirection>,
    pub class: MaybeProp<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
    pub keyboard_resize: MaybeProp<bool>,
    pub touch_support: MaybeProp<bool>,
    pub aria_label: MaybeProp<String>,
    pub on_resize: MaybeProp<Callback<Vec<f64>>>,
    pub children: Option<Children>,
}

pub(crate) fn render_group(props: GroupProps) -> impl IntoView {
    let GroupProps {
        base_class,
        direction,
        class,
        id,
        style,
        keyboard_resize,
        touch_support,
        aria_label,
        on_resize,
        children,
    } = props;

    let ctx = GroupContext {
        direction: Signal::derive(move || direction.get().unwrap_or_default()),
        panels: RwSignal::new(Vec::new()),
        handle_count: RwSignal::new(0),
        is_resizing: RwSignal::new(false),
        keyboard_resize: Signal::derive(move || keyboard_resize.get().unwrap_or(false)),
        touch_support: Signal::derive(move || touch_support.get().unwrap_or(false)),
        group_ref: NodeRef::new(),
        on_resize: Signal::derive(move || on_resize.get()),
    };
    provide_context(ctx);

    let computed_class = Signal::derive(move || {
        let mut classes = vec![base_class.to_string(), "flex".to_string()];
        classes.push(
            match ctx.direction.get() {
                ResizeDirection::Horizontal => "flex-row",
                ResizeDirection::Vertical => "flex-col",
            }
            .to_string(),
        );
        if ctx.is_resizing.get() {
            classes.push("resizing".to_string());
        }
        classes.push(class.get().unwrap_or_default());
        classes.join(" ")
    });

    view! {
        <div
            node_ref=ctx.group_ref
            class=computed_class
            id=move || id.get()
            style=move || style.get().to_string()
            aria-label=move || aria_label.get()
            aria-orientation=move || match ctx.direction.get() {
                ResizeDirection::Horizontal => "horizontal",
                ResizeDirection::Vertical => "vertical",
            }
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}

pub(crate) struct PanelProps {
    pub base_class: &'static str,
    pub button_class: &'static str,
    pub content_class: &'static str,
    pub default_size: MaybeProp<f64>,
    pub min_size: MaybeProp<f64>,
    pub max_size: MaybeProp<f64>,
    pub collapsible: MaybeProp<bool>,
    pub collapsed_size: MaybeProp<f64>,
    pub collapsed: MaybeProp<bool>,
    pub class: MaybeProp<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
    pub aria_label: MaybeProp<String>,
    pub on_resize: MaybeProp<Callback<f64>>,
    pub children: Option<Children>,
}

pub(crate) fn render_panel(props: PanelProps) -> impl IntoView {
    let PanelProps {
        base_class,
        button_class,
        content_class,
        default_size,
        min_size,
        max_size,
        collapsible,
        collapsed_size,
        collapsed,
        class,
        id,
        style,
        aria_label,
        on_resize,
        children,
    } = props;

    let min = min_size.get_untracked().unwrap_or(10.0);
    let max = max_size.get_untracked().unwrap_or(90.0).max(min);
    let collapsed_size_val = collapsed_size.get_untracked().unwrap_or(0.0);
    let initial = default_size.get_untracked().unwrap_or(50.0).clamp(min, max);
    let initially_collapsed = collapsed.get_untracked().unwrap_or(false);

    let group = use_context::<GroupContext>();
    let index = group.map(|ctx| {
        let mut index = 0;
        ctx.panels.update(|p| {
            index = p.len();
            p.push(PanelLayout { size: initial, min, max });
        });
        index
    });

    // Standalone panels (no group) keep their size locally.
    let local_size = RwSignal::new(initial);
    let is_collapsed = RwSignal::new(initially_collapsed);
    let size_before_collapse = StoredValue::new(initial);

    let current_size = Signal::derive(move || match (group, index) {
        (Some(ctx), Some(i)) => ctx.panels.with(|p| p.get(i).map(|l| l.size).unwrap_or(initial)),
        _ => local_size.get(),
    });

    let set_size = move |size: f64| match (group, index) {
        (Some(ctx), Some(i)) => {
            // Collapsing must be able to go below `min`, so relax the limit while applying it.
            ctx.panels.update(|p| p[i].min = p[i].min.min(size));
            ctx.set_size(i, size);
            ctx.panels.update(|p| p[i].min = min);
        }
        _ => local_size.set(size),
    };

    if initially_collapsed {
        set_size(collapsed_size_val);
    }

    // Report size changes to the panel's own callback.
    Effect::new(move |prev: Option<f64>| {
        let size = current_size.get();
        if let Some(prev) = prev
            && prev != size
                && let Some(callback) = on_resize.get_untracked() {
                    callback.run(size);
                }
        size
    });

    let toggle_collapse = move |_| {
        if !collapsible.get_untracked().unwrap_or(false) {
            return;
        }
        if is_collapsed.get_untracked() {
            is_collapsed.set(false);
            set_size(size_before_collapse.get_value());
        } else {
            size_before_collapse.set_value(current_size.get_untracked());
            is_collapsed.set(true);
            set_size(collapsed_size_val);
        }
    };

    let computed_class = Signal::derive(move || {
        let mut classes = vec![base_class.to_string(), "relative".to_string()];
        if is_collapsed.get() {
            classes.push("collapsed".to_string());
        }
        if group.is_some_and(|ctx| ctx.is_resizing.get()) {
            classes.push("resizing".to_string());
        }
        classes.push(class.get().unwrap_or_default());
        classes.join(" ")
    });

    let computed_style = Signal::derive(move || {
        let mut style_str = style.get().to_string();
        if !style_str.is_empty() && !style_str.trim_end().ends_with(';') {
            style_str.push(';');
        }
        style_str.push_str(&format!(" flex: 0 0 {}%; overflow: hidden;", current_size.get()));
        style_str
    });

    view! {
        <div
            class=computed_class
            id=move || id.get()
            style=computed_style
            aria-label=move || aria_label.get()
            role="region"
            data-panel-size=move || format!("{:.1}", current_size.get())
        >
            <Show when=move || collapsible.get().unwrap_or(false)>
                <button
                    type="button"
                    class=button_class
                    on:click=toggle_collapse
                    aria-expanded=move || (!is_collapsed.get()).to_string()
                    aria-label=move || if is_collapsed.get() { "Expand panel" } else { "Collapse panel" }
                >
                    {move || if is_collapsed.get() { "\u{2192}" } else { "\u{2190}" }}
                </button>
            </Show>
            <div class=content_class style:display=move || is_collapsed.get().then_some("none")>
                {children.map(|c| c())}
            </div>
        </div>
    }
}

pub(crate) struct HandleProps {
    pub base_class: &'static str,
    pub grip_class: &'static str,
    pub dots_class: &'static str,
    pub with_handle: MaybeProp<bool>,
    pub class: MaybeProp<String>,
    pub disabled: MaybeProp<bool>,
    pub aria_label: MaybeProp<String>,
    pub role: MaybeProp<String>,
    pub keyboard_resize: MaybeProp<bool>,
    pub touch_support: MaybeProp<bool>,
}

pub(crate) fn render_handle(props: HandleProps) -> impl IntoView {
    let HandleProps {
        base_class,
        grip_class,
        dots_class,
        with_handle,
        class,
        disabled,
        aria_label,
        role,
        keyboard_resize,
        touch_support,
    } = props;

    let group = use_context::<GroupContext>();
    // A handle sits after the panels registered before it.
    let before = group.map(|ctx| {
        ctx.handle_count.update(|c| *c += 1);
        ctx.panels.with_untracked(Vec::len).saturating_sub(1)
    });

    let is_dragging = RwSignal::new(false);
    let is_hovering = RwSignal::new(false);
    let last_pos = StoredValue::new(0.0_f64);

    let is_disabled = move || disabled.get().unwrap_or(false) || group.is_none();
    let keyboard_enabled = move || {
        keyboard_resize
            .get()
            .or_else(|| group.map(|ctx| ctx.keyboard_resize.get()))
            .unwrap_or(false)
    };
    let touch_enabled = move || {
        touch_support
            .get()
            .or_else(|| group.map(|ctx| ctx.touch_support.get()))
            .unwrap_or(true)
    };
    let direction = move || group.map(|ctx| ctx.direction.get()).unwrap_or_default();

    let pointer_pos = move |ev: &web_sys::PointerEvent| match direction() {
        ResizeDirection::Horizontal => ev.client_x() as f64,
        ResizeDirection::Vertical => ev.client_y() as f64,
    };

    let stop_drag = move || {
        if is_dragging.get_untracked() {
            is_dragging.set(false);
            if let Some(ctx) = group {
                ctx.is_resizing.set(false);
            }
        }
    };

    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        if is_disabled() || (ev.pointer_type() == "touch" && !touch_enabled()) {
            return;
        }
        ev.prevent_default();
        if let Some(target) = ev.current_target() {
            use leptos::wasm_bindgen::JsCast;
            if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                let _ = el.set_pointer_capture(ev.pointer_id());
            }
        }
        last_pos.set_value(pointer_pos(&ev));
        is_dragging.set(true);
        if let Some(ctx) = group {
            ctx.is_resizing.set(true);
        }
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        if !is_dragging.get_untracked() {
            return;
        }
        let (Some(ctx), Some(before)) = (group, before) else {
            return;
        };
        let pos = pointer_pos(&ev);
        let delta = px_to_percent(&ctx, pos - last_pos.get_value());
        last_pos.set_value(pos);
        ctx.resize(before, delta);
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if is_disabled() || !keyboard_enabled() {
            return;
        }
        let (Some(ctx), Some(before)) = (group, before) else {
            return;
        };
        let horizontal = direction() == ResizeDirection::Horizontal;
        let delta = match ev.key().as_str() {
            "ArrowLeft" if horizontal => -KEYBOARD_STEP,
            "ArrowRight" if horizontal => KEYBOARD_STEP,
            "ArrowUp" if !horizontal => -KEYBOARD_STEP,
            "ArrowDown" if !horizontal => KEYBOARD_STEP,
            "Home" => -100.0,
            "End" => 100.0,
            _ => return,
        };
        ev.prevent_default();
        ctx.resize(before, delta);
    };

    let value_now = move || {
        let (ctx, before) = (group?, before?);
        ctx.panels.with(|p| p.get(before).map(|l| format!("{:.0}", l.size)))
    };

    let computed_class = Signal::derive(move || {
        let mut classes = vec![base_class.to_string()];
        if with_handle.get().unwrap_or(true) {
            classes.push("with-handle".to_string());
        }
        classes.push(
            match direction() {
                ResizeDirection::Horizontal => "cursor-col-resize",
                ResizeDirection::Vertical => "cursor-row-resize",
            }
            .to_string(),
        );
        if !touch_enabled() {
            classes.push("touch-none".to_string());
        }
        if is_dragging.get() {
            classes.push("resizing".to_string());
        }
        if is_hovering.get() {
            classes.push("hovering".to_string());
        }
        if is_disabled() {
            classes.push("disabled".to_string());
        }
        classes.push(class.get().unwrap_or_default());
        classes.join(" ")
    });

    view! {
        <div
            class=computed_class
            aria-label=move || aria_label.get()
            role=move || role.get().unwrap_or_else(|| "separator".to_string())
            // A separator's orientation is perpendicular to the panel layout.
            aria-orientation=move || match direction() {
                ResizeDirection::Horizontal => "vertical",
                ResizeDirection::Vertical => "horizontal",
            }
            aria-valuenow=value_now
            aria-valuemin="0"
            aria-valuemax="100"
            aria-disabled=move || is_disabled().to_string()
            tabindex=move || keyboard_enabled().then_some(0)
            style:touch-action="none"
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=move |_| stop_drag()
            on:pointercancel=move |_| stop_drag()
            on:lostpointercapture=move |_| stop_drag()
            on:pointerenter=move |_| is_hovering.set(true)
            on:pointerleave=move |_| is_hovering.set(false)
            on:keydown=on_key_down
        >
            <Show when=move || with_handle.get().unwrap_or(true)>
                <div class=grip_class>
                    <div class=dots_class></div>
                </div>
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(sizes: &[f64]) -> Vec<PanelLayout> {
        sizes
            .iter()
            .map(|&size| PanelLayout { size, min: 10.0, max: 90.0 })
            .collect()
    }

    #[test]
    fn apply_delta_moves_boundary() {
        let mut panels = layout(&[50.0, 50.0]);
        assert!(apply_delta(&mut panels, 0, 20.0));
        assert_eq!(panels[0].size, 70.0);
        assert_eq!(panels[1].size, 30.0);
    }

    #[test]
    fn apply_delta_clamps_to_limits() {
        let mut panels = layout(&[50.0, 50.0]);
        apply_delta(&mut panels, 0, 100.0);
        assert_eq!(panels[0].size, 90.0);
        assert_eq!(panels[1].size, 10.0);
        apply_delta(&mut panels, 0, -500.0);
        assert_eq!(panels[0].size, 10.0);
        assert_eq!(panels[1].size, 90.0);
    }

    #[test]
    fn apply_delta_preserves_total() {
        let mut panels = layout(&[30.0, 40.0, 30.0]);
        apply_delta(&mut panels, 1, 15.0);
        let total: f64 = panels.iter().map(|p| p.size).sum();
        assert!((total - 100.0).abs() < 1e-9);
        assert_eq!(panels[0].size, 30.0);
    }

    #[test]
    fn apply_delta_ignores_invalid_input() {
        let mut panels = layout(&[50.0, 50.0]);
        assert!(!apply_delta(&mut panels, 1, 10.0));
        assert!(!apply_delta(&mut panels, 0, 0.0));
        assert!(!apply_delta(&mut panels, 0, f64::NAN));
        assert_eq!(panels, layout(&[50.0, 50.0]));
    }

    #[test]
    fn group_context_resize_updates_sizes_and_notifies() {
        let owner = Owner::new();
        owner.with(|| {
            let reported = RwSignal::new(Vec::<f64>::new());
            let ctx = GroupContext {
                direction: Signal::stored(ResizeDirection::Horizontal),
                panels: RwSignal::new(layout(&[50.0, 50.0])),
                handle_count: RwSignal::new(1),
                is_resizing: RwSignal::new(false),
                keyboard_resize: Signal::stored(true),
                touch_support: Signal::stored(true),
                group_ref: NodeRef::new(),
                on_resize: Signal::stored(Some(Callback::new(move |s| reported.set(s)))),
            };
            ctx.resize(0, -KEYBOARD_STEP);
            assert_eq!(reported.get_untracked(), vec![45.0, 55.0]);
            ctx.set_size(1, 80.0);
            assert_eq!(reported.get_untracked(), vec![20.0, 80.0]);
        });
    }
}
