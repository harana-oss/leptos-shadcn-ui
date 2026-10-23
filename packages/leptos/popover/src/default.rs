use leptos::prelude::*;
use leptos_style::Style;

const POPOVER_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
const POPOVER_ROOT_CLASS: &str = "relative inline-block";
pub(crate) const POPOVER_CONTENT_CLASS: &str = "absolute z-50 mt-2 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";

/// Shared state between `Popover`, `PopoverTrigger` and `PopoverContent`.
#[derive(Debug, Clone, Copy)]
pub struct PopoverContext {
    pub open: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub variant: Signal<Option<String>>,
    pub size: Signal<Option<String>>,
    pub content_id: StoredValue<String>,
}

impl PopoverContext {
    /// Toggle the open state unless the popover is disabled.
    pub fn toggle(&self) {
        if !self.disabled.get_untracked() {
            self.open.update(|o| *o = !*o);
        }
    }
}

/// Returns the enclosing context, or a standalone (closed) state when the
/// part is rendered outside a root or without a reactive owner.
fn use_popover_context() -> PopoverContext {
    use_context::<PopoverContext>().unwrap_or_else(|| PopoverContext {
        open: RwSignal::new(false),
        disabled: Signal::derive(|| false),
        variant: Signal::derive(|| None),
        size: Signal::derive(|| None),
        content_id: StoredValue::new(next_content_id()),
    })
}

fn next_content_id() -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    format!("popover-content-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

fn data_state(open: bool) -> &'static str {
    if open {
        "open"
    } else {
        "closed"
    }
}

fn trigger_variant_class(variant: Option<String>) -> &'static str {
    match variant.unwrap_or_default().as_str() {
        "destructive" => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        "outline" => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        "secondary" => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        "ghost" => "hover:bg-accent hover:text-accent-foreground",
        "link" => "text-primary underline-offset-4 hover:underline",
        _ => "bg-primary text-primary-foreground hover:bg-primary/90",
    }
}

fn trigger_size_class(size: Option<String>) -> &'static str {
    match size.unwrap_or_default().as_str() {
        "sm" => "h-9 rounded-md px-3",
        "lg" => "h-11 rounded-md px-8",
        "icon" => "h-10 w-10",
        _ => "h-10 px-4 py-2",
    }
}

/// Popover root. Provides shared open state (via context) to `PopoverTrigger`
/// and `PopoverContent`.
///
/// `variant` and `size` style the nested trigger; `on_click` fires for clicks
/// inside the root. Pass `open` for controlled state; otherwise state is internal.
#[component]
pub fn Popover(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = open.unwrap_or_else(|| RwSignal::new(false));
    provide_context(PopoverContext {
        open,
        disabled,
        variant: Signal::derive(move || variant.get()),
        size: Signal::derive(move || size.get()),
        content_id: StoredValue::new(next_content_id()),
    });

    if let Some(cb) = on_open_change {
        Effect::watch(move || open.get(), move |o, _, _| cb.run(*o), false);
    }

    let computed_class =
        Signal::derive(move || format!("{} {}", POPOVER_ROOT_CLASS, class.get().unwrap_or_default()));

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
            data-state=move || data_state(open.get())
            data-disabled=move || disabled.get().then_some("")
            on:click=move |_| {
                if let Some(cb) = &on_click {
                    cb.run(());
                }
            }
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Button that toggles the enclosing `Popover`.
#[component]
pub fn PopoverTrigger(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_popover_context();
    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());
    let computed_class = Signal::derive(move || {
        format!(
            "{} {} {} {}",
            POPOVER_CLASS,
            trigger_variant_class(ctx.variant.get()),
            trigger_size_class(ctx.size.get()),
            class.get().unwrap_or_default()
        )
    });

    view! {
        <button
            type="button"
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get()
            disabled=move || is_disabled.get()
            aria-haspopup="dialog"
            aria-expanded=move || ctx.open.get().to_string()
            aria-controls=move || ctx.content_id.get_value()
            data-state=move || data_state(ctx.open.get())
            on:click=move |_| {
                if !is_disabled.get_untracked() {
                    ctx.open.update(|o| *o = !*o);
                }
            }
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Floating panel shown while the enclosing `Popover` is open.
#[component]
pub fn PopoverContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_popover_context();
    let computed_class =
        Signal::derive(move || format!("{} {}", POPOVER_CONTENT_CLASS, class.get().unwrap_or_default()));

    view! {
        <div
            role="dialog"
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_else(|| ctx.content_id.get_value())
            style=move || style.get()
            data-state=move || data_state(ctx.open.get())
            aria-hidden=move || (!ctx.open.get()).to_string()
            hidden=move || !ctx.open.get()
        >
            {children.map(|c| c())}
        </div>
    }
}
