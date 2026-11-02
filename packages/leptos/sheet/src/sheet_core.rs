//! Shared, style-agnostic Sheet logic used by both the `default` and
//! `new_york` themes.
//!
//! Everything that decides *behavior* (open-state resolution, Escape handling,
//! side to class mapping, id generation) lives here as plain functions or on
//! [`SheetContext`], so it can be unit tested without a DOM.
//!
//! Follow-up: the sheet currently renders in place as a `fixed`-position
//! overlay (same approach as the drawer component). Rendering through a real
//! portal (`leptos::portal::Portal`) to escape stacking contexts / `overflow`
//! clipping of ancestors is not implemented yet.

use leptos::{ev::KeyboardEvent, ev::MouseEvent, prelude::*};
use leptos_style::Style;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Which edge of the viewport the sheet slides in from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    /// Value used for the `data-side` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }

    /// shadcn `sheetVariants` side classes (position + slide animation).
    pub fn class(&self) -> &'static str {
        match self {
            SheetSide::Top => {
                "inset-x-0 top-0 border-b data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top"
            }
            SheetSide::Bottom => {
                "inset-x-0 bottom-0 border-t data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom"
            }
            SheetSide::Left => {
                "inset-y-0 left-0 h-full w-3/4 border-r data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left sm:max-w-sm"
            }
            SheetSide::Right => {
                "inset-y-0 right-0 h-full w-3/4 border-l data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right sm:max-w-sm"
            }
        }
    }
}

/// Theme-specific class strings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SheetStyles {
    pub overlay: &'static str,
    pub content: &'static str,
    pub close: &'static str,
    pub header: &'static str,
    pub footer: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

/// Resolve the effective open state: a controlled value always wins,
/// otherwise the internal (uncontrolled) state is used.
pub fn resolve_open(controlled: Option<bool>, internal: bool) -> bool {
    controlled.unwrap_or(internal)
}

/// Whether a keyboard key should close the sheet.
pub fn is_close_key(key: &str) -> bool {
    key == "Escape" || key == "Esc"
}

/// `data-state` attribute value.
pub fn data_state(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

/// Join a base class string with an optional user class.
pub fn merge_class(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(extra) if !extra.trim().is_empty() => format!("{base} {}", extra.trim()),
        _ => base.to_string(),
    }
}

/// Full class string for the sliding panel.
pub fn content_class(styles: &SheetStyles, side: SheetSide, extra: Option<String>) -> String {
    merge_class(&format!("{} {}", styles.content, side.class()), extra)
}

/// Generate a unique id base for one sheet instance (`sheet-N`).
pub fn next_sheet_id() -> String {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    format!("sheet-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// Context shared by `Sheet` with all its parts.
#[derive(Clone, Copy)]
pub struct SheetContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
    pub title_id: Signal<String>,
    pub description_id: Signal<String>,
}

impl SheetContext {
    /// Build the context. `open` = controlled state (if any), `default_open`
    /// seeds the uncontrolled state.
    pub fn new(
        open: Option<Signal<bool>>,
        default_open: bool,
        on_open_change: Option<Callback<bool>>,
    ) -> Self {
        let internal = RwSignal::new(default_open);
        let open_state =
            Signal::derive(move || resolve_open(open.map(|o| o.get()), internal.get()));
        let set_open = Callback::new(move |value: bool| {
            internal.set(value);
            if let Some(cb) = on_open_change {
                cb.run(value);
            }
        });
        let base = next_sheet_id();
        Self {
            open: open_state,
            set_open,
            title_id: Signal::stored(format!("{base}-title")),
            description_id: Signal::stored(format!("{base}-description")),
        }
    }

    pub fn open(&self) {
        self.set_open.run(true);
    }

    pub fn close(&self) {
        self.set_open.run(false);
    }

    /// Handle a key press; returns `true` if it closed the sheet.
    pub fn handle_key(&self, key: &str) -> bool {
        if is_close_key(key) && self.open.get_untracked() {
            self.close();
            true
        } else {
            false
        }
    }

    /// Handle a click on the overlay (always closes).
    pub fn handle_overlay_click(&self) {
        self.close();
    }
}

fn ctx() -> SheetContext {
    expect_context::<SheetContext>()
}

pub(crate) fn root(
    open: Option<Signal<bool>>,
    default_open: bool,
    on_open_change: Option<Callback<bool>>,
    children: Option<Children>,
) -> impl IntoView {
    let ctx = SheetContext::new(open, default_open, on_open_change);
    provide_context(ctx);
    view! {
        <div
            data-slot="sheet"
            style="display: contents"
            on:keydown=move |e: KeyboardEvent| {
                ctx.handle_key(&e.key());
            }
        >
            {children.map(|c| c())}
        </div>
    }
}

pub(crate) fn trigger(
    class: MaybeProp<String>,
    id: MaybeProp<String>,
    children: Option<Children>,
) -> impl IntoView {
    let ctx = ctx();
    view! {
        <button
            type="button"
            data-slot="sheet-trigger"
            class=move || class.get().unwrap_or_default()
            id=move || id.get()
            aria-haspopup="dialog"
            aria-expanded=move || ctx.open.get().to_string()
            data-state=move || data_state(ctx.open.get())
            on:click=move |_: MouseEvent| ctx.open()
        >
            {children.map(|c| c())}
        </button>
    }
}

pub(crate) fn close(class: MaybeProp<String>, children: Option<Children>) -> impl IntoView {
    let ctx = ctx();
    view! {
        <button
            type="button"
            data-slot="sheet-close"
            class=move || class.get().unwrap_or_default()
            on:click=move |_: MouseEvent| ctx.close()
        >
            {children.map(|c| c())}
        </button>
    }
}

pub(crate) fn content(
    styles: &'static SheetStyles,
    side: MaybeProp<SheetSide>,
    class: MaybeProp<String>,
    style: Signal<Style>,
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = ctx();
    let side = Signal::derive(move || side.get().unwrap_or_default());
    let panel_class = Signal::derive(move || content_class(styles, side.get(), class.get()));
    view! {
        <Show when=move || ctx.open.get()>
            <div
                data-slot="sheet-overlay"
                data-state=move || data_state(ctx.open.get())
                class=styles.overlay
                on:click=move |_: MouseEvent| ctx.handle_overlay_click()
            ></div>
            <div
                data-slot="sheet-content"
                role="dialog"
                aria-modal="true"
                aria-labelledby=move || ctx.title_id.get()
                aria-describedby=move || ctx.description_id.get()
                tabindex="-1"
                data-state=move || data_state(ctx.open.get())
                data-side=move || side.get().as_str()
                class=move || panel_class.get()
                style=move || style.get().to_string()
                on:keydown=move |e: KeyboardEvent| {
                    if ctx.handle_key(&e.key()) {
                        e.stop_propagation();
                    }
                }
            >
                {children.clone().map(|c| c())}
                <button
                    type="button"
                    data-slot="sheet-close"
                    class=styles.close
                    on:click=move |_: MouseEvent| ctx.close()
                >
                    <span aria-hidden="true">"\u{00d7}"</span>
                    <span class="sr-only">"Close"</span>
                </button>
            </div>
        </Show>
    }
}

pub(crate) fn header(
    styles: &'static SheetStyles,
    class: MaybeProp<String>,
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-slot="sheet-header" class=move || merge_class(styles.header, class.get())>
            {children.map(|c| c())}
        </div>
    }
}

pub(crate) fn footer(
    styles: &'static SheetStyles,
    class: MaybeProp<String>,
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-slot="sheet-footer" class=move || merge_class(styles.footer, class.get())>
            {children.map(|c| c())}
        </div>
    }
}

pub(crate) fn title(
    styles: &'static SheetStyles,
    class: MaybeProp<String>,
    children: Option<Children>,
) -> impl IntoView {
    let ctx = ctx();
    view! {
        <h2
            data-slot="sheet-title"
            id=move || ctx.title_id.get()
            class=move || merge_class(styles.title, class.get())
        >
            {children.map(|c| c())}
        </h2>
    }
}

pub(crate) fn description(
    styles: &'static SheetStyles,
    class: MaybeProp<String>,
    children: Option<Children>,
) -> impl IntoView {
    let ctx = ctx();
    view! {
        <p
            data-slot="sheet-description"
            id=move || ctx.description_id.get()
            class=move || merge_class(styles.description, class.get())
        >
            {children.map(|c| c())}
        </p>
    }
}

/// Defines the public Sheet components for a theme, given a `SheetStyles` static.
macro_rules! define_sheet_components {
    ($styles:path) => {
        use leptos::prelude::*;
        use leptos_style::Style;

        pub use $crate::sheet_core::{SheetContext, SheetSide};

        /// Sheet root. Provides open state, callbacks and generated
        /// title/description ids to its parts via [`SheetContext`].
        ///
        /// Controlled: pass `open` (+ `on_open_change`).
        /// Uncontrolled: omit `open`, optionally set `default_open`.
        ///
        /// Follow-up: no portal support yet; content renders in place as a
        /// fixed-position overlay.
        #[component]
        pub fn Sheet(
            #[prop(optional)] open: Option<Signal<bool>>,
            #[prop(optional)] default_open: bool,
            #[prop(into, default = None)] on_open_change: Option<Callback<bool>>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            $crate::sheet_core::root(open, default_open, on_open_change, children)
        }

        /// Button that opens the sheet.
        #[component]
        pub fn SheetTrigger(
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(into, optional)] id: MaybeProp<String>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            $crate::sheet_core::trigger(class, id, children)
        }

        /// Button that closes the sheet.
        #[component]
        pub fn SheetClose(
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            $crate::sheet_core::close(class, children)
        }

        /// Overlay + sliding panel (`role="dialog"`, `aria-modal="true"`).
        /// Overlay click and Escape close the sheet.
        #[component]
        pub fn SheetContent(
            #[prop(into, optional)] side: MaybeProp<SheetSide>,
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(into, optional)] style: Signal<Style>,
            #[prop(optional)] children: Option<ChildrenFn>,
        ) -> impl IntoView {
            $crate::sheet_core::content(&$styles, side, class, style, children)
        }

        #[component]
        pub fn SheetHeader(
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            $crate::sheet_core::header(&$styles, class, children)
        }

        #[component]
        pub fn SheetFooter(
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            $crate::sheet_core::footer(&$styles, class, children)
        }

        /// Title; its generated id is referenced by the content's `aria-labelledby`.
        #[component]
        pub fn SheetTitle(
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            $crate::sheet_core::title(&$styles, class, children)
        }

        /// Description; its generated id is referenced by `aria-describedby`.
        #[component]
        pub fn SheetDescription(
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            $crate::sheet_core::description(&$styles, class, children)
        }
    };
}
pub(crate) use define_sheet_components;
