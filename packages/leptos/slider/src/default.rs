use leptos::wasm_bindgen::JsCast;
use leptos::{ev::Event, prelude::*};
use leptos_style::Style;

const SLIDER_CLASS: &str = "relative flex w-full touch-none select-none items-center";
const SLIDER_TRACK_CLASS: &str =
    "relative h-2 w-full grow overflow-hidden rounded-full bg-secondary";
const SLIDER_RANGE_CLASS: &str = "absolute h-full bg-primary";
const SLIDER_THUMB_CLASS: &str = "block h-5 w-5 rounded-full border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SliderVariant {
    #[default]
    Default,
    Success,
    Warning,
    Destructive,
    Info,
}

impl From<String> for SliderVariant {
    fn from(s: String) -> Self {
        match s.as_str() {
            "success" => SliderVariant::Success,
            "warning" => SliderVariant::Warning,
            "destructive" => SliderVariant::Destructive,
            "info" => SliderVariant::Info,
            _ => SliderVariant::Default,
        }
    }
}

impl SliderVariant {
    fn range_class(&self) -> &'static str {
        match self {
            SliderVariant::Default => "bg-primary",
            SliderVariant::Success => "bg-green-500",
            SliderVariant::Warning => "bg-yellow-500",
            SliderVariant::Destructive => "bg-red-500",
            SliderVariant::Info => "bg-blue-500",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SliderSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl From<String> for SliderSize {
    fn from(s: String) -> Self {
        match s.as_str() {
            "sm" => SliderSize::Sm,
            "lg" => SliderSize::Lg,
            _ => SliderSize::Md,
        }
    }
}

impl SliderSize {
    fn track_class(&self) -> &'static str {
        match self {
            SliderSize::Sm => "h-1",
            SliderSize::Md => "h-2",
            SliderSize::Lg => "h-3",
        }
    }

    fn thumb_class(&self) -> &'static str {
        match self {
            SliderSize::Sm => "h-3 w-3",
            SliderSize::Md => "h-5 w-5",
            SliderSize::Lg => "h-6 w-6",
        }
    }
}

#[component]
pub fn Slider(
    #[prop(into, optional)] value: Signal<f64>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] on_change: Option<Callback<f64>>,
    #[prop(into, optional)] variant: MaybeProp<SliderVariant>,
    #[prop(into, optional)] size: MaybeProp<SliderSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] show_value: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let min_value = min.get().unwrap_or(0.0);
    let max_value = max.get().unwrap_or(100.0);
    let step_value = step.get().unwrap_or(1.0);
    let slider_variant = variant.get().unwrap_or_default();
    let slider_size = size.get().unwrap_or_default();

    let handle_change = {
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                if let Ok(val) = input.value().parse::<f64>() {
                    callback.run(val);
                }
            }
        }
    };

    let progress_percentage = Signal::derive(move || {
        let val = value.get();
        let range = max_value - min_value;
        if range <= 0.0 {
            0.0
        } else {
            ((val - min_value) / range * 100.0).clamp(0.0, 100.0)
        }
    });

    let track_class = slider_size.track_class();
    let thumb_class = slider_size.thumb_class();
    let variant_class = slider_variant.range_class();

    let computed_class = Signal::derive(move || {
        format!(
            "{} {} {}",
            SLIDER_CLASS,
            track_class,
            class.get().unwrap_or_default()
        )
    });

    let computed_range_class =
        Signal::derive(move || format!("{} {} {}", SLIDER_RANGE_CLASS, variant_class, track_class));

    let computed_thumb_class =
        Signal::derive(move || format!("{} {} {}", SLIDER_THUMB_CLASS, thumb_class, track_class));

    view! {
        <div class="w-full space-y-2">
            <div
                class=move || computed_class.get()
                id=id.get().unwrap_or_default()
                style=move || style.get().to_string()
            >
                <div class={format!("{} {}", SLIDER_TRACK_CLASS, track_class)}>
                    <div
                        class=move || computed_range_class.get()
                        style={move || format!("width: {}%", progress_percentage.get())}
                    />
                </div>
                <input
                    r#type="range"
                    min={min_value}
                    max={max_value}
                    step={step_value}
                    value={move || value.get()}
                    disabled=move || disabled.get()
                    class="absolute inset-0 h-full w-full opacity-0 cursor-pointer"
                    on:input=handle_change
                />
                <div
                    class=move || computed_thumb_class.get()
                    style={move || format!("left: {}%", progress_percentage.get())}
                />
            </div>
            <Show
                when=move || show_value.get()
                fallback=|| view! { <div class="hidden"></div> }
            >
                <div class="flex justify-between text-sm text-muted-foreground">
                    <span>{move || format!("{:.0}", min_value)}</span>
                    <span>{move || format!("{:.0}", value.get())}</span>
                    <span>{move || format!("{:.0}", max_value)}</span>
                </div>
            </Show>
        </div>
    }
}

// Range Slider (for dual values)
/// Snaps `raw` to the nearest `step` from `min` and clamps it to `[min, max]`.
fn snap_to_step(raw: f64, min: f64, max: f64, step: f64) -> f64 {
    let snapped = if step > 0.0 {
        min + ((raw - min) / step).round() * step
    } else {
        raw
    };
    snapped.clamp(min, max)
}

/// Moves thumb `thumb` (0 = low, 1 = high) to `value`, never letting the thumbs cross.
fn move_thumb((low, high): (f64, f64), thumb: usize, value: f64) -> (f64, f64) {
    if thumb == 0 {
        (value.min(high), high)
    } else {
        (low, value.max(low))
    }
}

#[component]
pub fn RangeSlider(
    #[prop(into, optional)] values: Signal<(f64, f64)>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] on_change: Option<Callback<(f64, f64)>>,
    #[prop(into, optional)] variant: MaybeProp<SliderVariant>,
    #[prop(into, optional)] size: MaybeProp<SliderSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] show_values: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let min_value = min.get().unwrap_or(0.0);
    let max_value = max.get().unwrap_or(100.0);
    let step_value = step.get().unwrap_or(1.0);
    let slider_variant = variant.get().unwrap_or_default();
    let slider_size = size.get().unwrap_or_default();

    let range_percentage = Signal::derive(move || {
        let (min_v, max_v) = values.get();
        let range = max_value - min_value;
        if range <= 0.0 {
            (0.0, 0.0)
        } else {
            let min_percent = ((min_v - min_value) / range * 100.0).clamp(0.0, 100.0);
            let max_percent = ((max_v - min_value) / range * 100.0).clamp(0.0, 100.0);
            (min_percent, max_percent)
        }
    });

    // Controlled like `Slider`: changes are reported through `on_change` and the
    // parent feeds the new pair back through `values`.
    let emit = move |thumb: usize, value: f64| {
        let current = values.get_untracked();
        let next = move_thumb(
            current,
            thumb,
            snap_to_step(value, min_value, max_value, step_value),
        );
        if next != current
            && let Some(callback) = on_change
        {
            callback.run(next);
        }
    };

    let track_ref = NodeRef::<leptos::html::Div>::new();
    let active_thumb = RwSignal::new(None::<usize>);

    let value_at = move |client_x: f64| -> Option<f64> {
        let rect = track_ref.get_untracked()?.get_bounding_client_rect();
        (rect.width() > 0.0)
            .then(|| min_value + (client_x - rect.left()) / rect.width() * (max_value - min_value))
    };

    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        if disabled.get_untracked() {
            return;
        }
        let Some(value) = value_at(ev.client_x() as f64) else {
            return;
        };
        // Grab whichever thumb is closer to the pointer.
        let (low, high) = values.get_untracked();
        let thumb = if (value - low).abs() <= (value - high).abs() {
            0
        } else {
            1
        };
        if let Some(el) = track_ref.get_untracked() {
            let _ = el.set_pointer_capture(ev.pointer_id());
        }
        active_thumb.set(Some(thumb));
        emit(thumb, value);
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        if let (Some(thumb), Some(value)) =
            (active_thumb.get_untracked(), value_at(ev.client_x() as f64))
        {
            emit(thumb, value);
        }
    };

    let on_key_down = move |thumb: usize, ev: web_sys::KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let current = if thumb == 0 {
            values.get_untracked().0
        } else {
            values.get_untracked().1
        };
        let value = match ev.key().as_str() {
            "ArrowLeft" | "ArrowDown" => current - step_value,
            "ArrowRight" | "ArrowUp" => current + step_value,
            "Home" => min_value,
            "End" => max_value,
            _ => return,
        };
        ev.prevent_default();
        emit(thumb, value);
    };

    let track_class = slider_size.track_class();
    let thumb_class = slider_size.thumb_class();
    let variant_class = slider_variant.range_class();

    let computed_class = Signal::derive(move || {
        format!(
            "{} {} {}",
            SLIDER_CLASS,
            track_class,
            class.get().unwrap_or_default()
        )
    });

    let computed_range_class =
        Signal::derive(move || format!("{} {} {}", SLIDER_RANGE_CLASS, variant_class, track_class));

    let computed_thumb_class = Signal::derive(move || {
        format!(
            "{} {} absolute -translate-x-1/2 {}",
            SLIDER_THUMB_CLASS, thumb_class, track_class
        )
    });

    let thumb = move |index: usize, label: &'static str| {
        let percent = move || {
            if index == 0 {
                range_percentage.get().0
            } else {
                range_percentage.get().1
            }
        };
        let now = move || {
            if index == 0 {
                values.get().0
            } else {
                values.get().1
            }
        };
        view! {
            <div
                class=move || computed_thumb_class.get()
                style=move || format!("left: {}%", percent())
                role="slider"
                aria-label=label
                aria-valuemin=min_value
                aria-valuemax=max_value
                aria-valuenow=now
                aria-disabled=move || disabled.get().to_string()
                tabindex=move || (!disabled.get()).then_some(0)
                on:keydown=move |ev| on_key_down(index, ev)
            />
        }
    };

    view! {
        <div class="w-full space-y-2">
            <div
                node_ref=track_ref
                class=move || computed_class.get()
                id=id.get().unwrap_or_default()
                style=move || style.get().to_string()
                style:touch-action="none"
                data-disabled=move || disabled.get().then_some("")
                on:pointerdown=on_pointer_down
                on:pointermove=on_pointer_move
                on:pointerup=move |_| active_thumb.set(None)
                on:pointercancel=move |_| active_thumb.set(None)
                on:lostpointercapture=move |_| active_thumb.set(None)
            >
                <div class={format!("{} {}", SLIDER_TRACK_CLASS, track_class)}>
                    <div
                        class=move || computed_range_class.get()
                        style={move || {
                            let (min_p, max_p) = range_percentage.get();
                            format!("left: {}%; width: {}%", min_p, max_p - min_p)
                        }}
                    />
                </div>
                {thumb(0, "Minimum")}
                {thumb(1, "Maximum")}
            </div>
            <Show
                when=move || show_values.get()
                fallback=|| view! { <div class="hidden"></div> }
            >
                <div class="flex justify-between text-sm text-muted-foreground">
                    <span>{move || format!("{:.0}", min_value)}</span>
                    <span>{move || format!("{:.0} - {:.0}", values.get().0, values.get().1)}</span>
                    <span>{move || format!("{:.0}", max_value)}</span>
                </div>
            </Show>
        </div>
    }
}

// Slider Root with Context
#[derive(Clone, Copy)]
pub struct SliderContextValue {
    pub value: RwSignal<f64>,
    pub min: RwSignal<f64>,
    pub max: RwSignal<f64>,
    pub step: RwSignal<f64>,
    pub disabled: RwSignal<bool>,
    pub variant: RwSignal<SliderVariant>,
    pub size: RwSignal<SliderSize>,
}

#[component]
pub fn SliderRoot(
    #[prop(into, optional)] value: Signal<f64>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] variant: MaybeProp<SliderVariant>,
    #[prop(into, optional)] size: MaybeProp<SliderSize>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let value_signal = RwSignal::new(value.get());
    let min_signal = RwSignal::new(min.get().unwrap_or(0.0));
    let max_signal = RwSignal::new(max.get().unwrap_or(100.0));
    let step_signal = RwSignal::new(step.get().unwrap_or(1.0));
    let disabled_signal = RwSignal::new(disabled.get());
    let variant_signal = RwSignal::new(variant.get().unwrap_or_default());
    let size_signal = RwSignal::new(size.get().unwrap_or_default());

    // Update signals when props change
    Effect::new(move |_| {
        value_signal.set(value.get());
    });
    Effect::new(move |_| {
        min_signal.set(min.get().unwrap_or(0.0));
    });
    Effect::new(move |_| {
        max_signal.set(max.get().unwrap_or(100.0));
    });
    Effect::new(move |_| {
        step_signal.set(step.get().unwrap_or(1.0));
    });
    Effect::new(move |_| {
        disabled_signal.set(disabled.get());
    });
    Effect::new(move |_| {
        variant_signal.set(variant.get().unwrap_or_default());
    });
    Effect::new(move |_| {
        size_signal.set(size.get().unwrap_or_default());
    });

    let context_value = SliderContextValue {
        value: value_signal,
        min: min_signal,
        max: max_signal,
        step: step_signal,
        disabled: disabled_signal,
        variant: variant_signal,
        size: size_signal,
    };

    provide_context(context_value);

    view! {
        <div class="w-full">
            {children.map(|c| c())}
        </div>
    }
}

#[cfg(test)]
mod range_slider_tests {
    use super::{move_thumb, snap_to_step};

    #[test]
    fn snap_to_step_rounds_and_clamps() {
        assert_eq!(snap_to_step(12.4, 0.0, 100.0, 5.0), 10.0);
        assert_eq!(snap_to_step(12.6, 0.0, 100.0, 5.0), 15.0);
        assert_eq!(snap_to_step(-20.0, 0.0, 100.0, 5.0), 0.0);
        assert_eq!(snap_to_step(250.0, 0.0, 100.0, 5.0), 100.0);
        assert_eq!(snap_to_step(33.3, 0.0, 100.0, 0.0), 33.3);
    }

    #[test]
    fn move_thumb_keeps_thumbs_ordered() {
        assert_eq!(move_thumb((20.0, 80.0), 0, 30.0), (30.0, 80.0));
        assert_eq!(move_thumb((20.0, 80.0), 1, 60.0), (20.0, 60.0));
        assert_eq!(move_thumb((20.0, 80.0), 0, 90.0), (80.0, 80.0));
        assert_eq!(move_thumb((20.0, 80.0), 1, 10.0), (20.0, 20.0));
    }
}
