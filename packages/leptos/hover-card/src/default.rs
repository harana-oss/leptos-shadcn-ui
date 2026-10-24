use leptos::prelude::*;
use leptos_style::Style;

const HOVER_CARD_ROOT_CLASS: &str = "relative inline-block";
const HOVER_CARD_TRIGGER_CLASS: &str = "inline-flex items-center underline-offset-4 hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 aria-disabled:pointer-events-none aria-disabled:opacity-50";
const HOVER_CARD_CONTENT_CLASS: &str = "absolute z-50 mt-2 w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";

/// Shared state between `HoverCard`, `HoverCardTrigger` and `HoverCardContent`.
#[derive(Debug, Clone, Copy)]
pub struct HoverCardContext {
    pub open: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub content_id: StoredValue<String>,
}

impl HoverCardContext {
    pub fn set_open(&self, value: bool) {
        if value && self.disabled.get_untracked() {
            return;
        }
        if self.open.get_untracked() != value {
            self.open.set(value);
        }
    }
}

/// Returns the enclosing context, or a standalone (closed) state when the
/// part is rendered outside a root or without a reactive owner.
fn use_hover_card_context() -> HoverCardContext {
    use_context::<HoverCardContext>().unwrap_or_else(|| HoverCardContext {
        open: RwSignal::new(false),
        disabled: Signal::derive(|| false),
        content_id: StoredValue::new(next_content_id()),
    })
}

fn next_content_id() -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    format!("hover-card-content-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

fn data_state(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

/// Hover card root. Provides shared open state (via context) to
/// `HoverCardTrigger` and `HoverCardContent`; opens while the pointer or
/// keyboard focus is on the trigger or content.
///
/// The legacy `variant`/`size`/`on_click` props are kept for compatibility.
#[component]
pub fn HoverCard(
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
    provide_context(HoverCardContext {
        open,
        disabled,
        content_id: StoredValue::new(next_content_id()),
    });

    if let Some(cb) = on_open_change {
        Effect::watch(move || open.get(), move |o, _, _| cb.run(*o), false);
    }

    // `variant` and `size` are accepted for backwards compatibility and exposed
    // as data attributes; styling lives on the trigger/content parts.
    let computed_class = Signal::derive(move || {
        format!("{} {}", HOVER_CARD_ROOT_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
            data-state=move || data_state(open.get())
            data-disabled=move || disabled.get().then_some("")
            data-variant=move || variant.get()
            data-size=move || size.get()
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

/// Element that opens the enclosing `HoverCard` on hover or focus.
#[component]
pub fn HoverCardTrigger(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<String>,
    #[prop(into, optional)] lang: MaybeProp<String>,
    #[prop(into, optional)] tabindex: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_hover_card_context();
    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());
    let computed_class = Signal::derive(move || {
        format!("{} {}", HOVER_CARD_TRIGGER_CLASS, class.get().unwrap_or_default())
    });
    let open_card = move || {
        if !is_disabled.get_untracked() {
            ctx.set_open(true);
        }
    };

    view! {
        <a
            class=move || computed_class.get()
            id=move || id.get()
            style=move || style.get()
            role=move || role.get()
            dir=move || dir.get()
            lang=move || lang.get()
            tabindex=move || tabindex.get().unwrap_or_else(|| "0".to_string())
            aria-disabled=move || is_disabled.get().then_some("true")
            aria-expanded=move || ctx.open.get().to_string()
            aria-describedby=move || ctx.open.get().then(|| ctx.content_id.get_value())
            data-state=move || data_state(ctx.open.get())
            on:mouseenter=move |_| open_card()
            on:focus=move |_| open_card()
            on:mouseleave=move |_| ctx.set_open(false)
            on:blur=move |_| ctx.set_open(false)
        >
            {children.map(|c| c())}
        </a>
    }
}

/// Card shown while the enclosing `HoverCard` is open.
#[component]
pub fn HoverCardContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<String>,
    #[prop(into, optional)] lang: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_hover_card_context();
    let computed_class = Signal::derive(move || {
        format!("{} {}", HOVER_CARD_CONTENT_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_else(|| ctx.content_id.get_value())
            style=move || style.get()
            role=move || role.get().unwrap_or_else(|| "tooltip".to_string())
            dir=move || dir.get()
            lang=move || lang.get()
            data-state=move || data_state(ctx.open.get())
            aria-hidden=move || (!ctx.open.get()).to_string()
            hidden=move || !ctx.open.get()
            on:mouseenter=move |_| ctx.set_open(true)
            on:mouseleave=move |_| ctx.set_open(false)
        >
            {children.map(|c| c())}
        </div>
    }
}
