//! Leptos port of the shadcn/ui (v4) Sonner toaster.
//!
//! Call [`provide_sonner`] near the app root (or let [`Toaster`] provide one),
//! then use [`use_sonner`] anywhere below it:
//!
//! ```ignore
//! let toast = use_sonner();
//! let id = toast.success("Saved");
//! toast.dismiss(id);
//! ```

use leptos::prelude::*;

mod queue;
pub use queue::*;

#[cfg(test)]
mod tests;

pub const TOASTER_CLASS: &str = "toaster group fixed z-[100] flex w-full max-w-[356px] flex-col gap-2 p-0 pointer-events-none";
pub const TOASTER_STYLE: &str = "--normal-bg: var(--popover); --normal-text: var(--popover-foreground); --normal-border: var(--border); --border-radius: var(--radius);";
pub const TOAST_CLASS: &str = "group toast pointer-events-auto relative flex w-full items-start gap-1.5 rounded-(--border-radius) border border-(--normal-border) bg-(--normal-bg) p-4 text-(--normal-text) text-[13px] shadow-lg";
pub const TOAST_ICON_CLASS: &str = "flex size-4 shrink-0 items-center justify-center [&>svg]:size-4";
pub const TOAST_CONTENT_CLASS: &str = "flex flex-1 flex-col gap-0.5";
pub const TOAST_TITLE_CLASS: &str = "font-medium leading-normal";
pub const TOAST_DESCRIPTION_CLASS: &str = "text-muted-foreground leading-snug";
pub const TOAST_CLOSE_CLASS: &str = "absolute top-1.5 right-1.5 inline-flex size-5 items-center justify-center rounded-md text-muted-foreground opacity-0 transition-opacity hover:text-foreground group-hover:opacity-100 focus-visible:opacity-100";

/// Where the toaster is anchored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToasterPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl ToasterPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToasterPosition::TopLeft => "top-left",
            ToasterPosition::TopCenter => "top-center",
            ToasterPosition::TopRight => "top-right",
            ToasterPosition::BottomLeft => "bottom-left",
            ToasterPosition::BottomCenter => "bottom-center",
            ToasterPosition::BottomRight => "bottom-right",
        }
    }

    /// Tailwind placement classes.
    pub fn class(&self) -> &'static str {
        match self {
            ToasterPosition::TopLeft => "top-4 left-4",
            ToasterPosition::TopCenter => "top-4 left-1/2 -translate-x-1/2",
            ToasterPosition::TopRight => "top-4 right-4",
            ToasterPosition::BottomLeft => "bottom-4 left-4 flex-col-reverse",
            ToasterPosition::BottomCenter => "bottom-4 left-1/2 -translate-x-1/2 flex-col-reverse",
            ToasterPosition::BottomRight => "bottom-4 right-4 flex-col-reverse",
        }
    }
}

/// Options for a toast call.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToastOptions {
    pub description: Option<String>,
    /// Override duration (ms). `Some(0)` keeps the toast until dismissed.
    pub duration_ms: Option<u64>,
}

#[cfg(target_arch = "wasm32")]
type TimerHandle = leptos::leptos_dom::helpers::TimeoutHandle;
#[cfg(not(target_arch = "wasm32"))]
type TimerHandle = ();

/// Wall-clock time in ms since the UNIX epoch (target-specific source).
#[cfg(target_arch = "wasm32")]
pub fn real_now_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// Wall-clock time in ms since the UNIX epoch (target-specific source).
#[cfg(not(target_arch = "wasm32"))]
pub fn real_now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Toast API handle, shared through context.
#[derive(Clone, Copy)]
pub struct ToasterContext {
    pub queue: RwSignal<ToastQueue>,
    default_duration_ms: RwSignal<u64>,
    timers: StoredValue<Vec<(ToastId, TimerHandle)>, LocalStorage>,
}

impl std::fmt::Debug for ToasterContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToasterContext").field("queue", &self.queue).finish()
    }
}

impl ToasterContext {
    pub fn new() -> Self {
        Self {
            queue: RwSignal::new(ToastQueue::default()),
            default_duration_ms: RwSignal::new(DEFAULT_DURATION_MS),
            timers: StoredValue::new_local(Vec::new()),
        }
    }

    /// Shows a toast of `kind` and returns its id.
    pub fn toast(&self, kind: ToastKind, title: impl Into<String>, options: ToastOptions) -> ToastId {
        let duration = match options.duration_ms {
            Some(0) => None,
            Some(d) => Some(d),
            None => Some(self.default_duration_ms.get_untracked()),
        };
        let mut id = 0;
        let now = real_now_ms();
        self.queue
            .update(|q| id = q.add_at(kind, title, options.description, duration, now));
        if let Some(ms) = duration {
            self.schedule(id, ms);
        }
        id
    }

    /// Advances the toaster clock to `now_ms` and dismisses expired toasts.
    /// Target-independent; the browser timer calls this, and native/SSR code
    /// (or tests) may call it with any clock. Returns the dismissed ids.
    pub fn tick(&self, now_ms: u64) -> Vec<ToastId> {
        let mut expired = Vec::new();
        let any = self
            .queue
            .with_untracked(|q| q.next_expiry().is_some_and(|e| e <= now_ms.max(q.now_ms())));
        if any {
            self.queue.update(|q| expired = q.tick(now_ms));
        } else {
            self.queue.update_untracked(|q| {
                q.tick(now_ms);
            });
        }
        for id in &expired {
            self.clear_timer(*id);
        }
        expired
    }

    pub fn message(&self, title: impl Into<String>) -> ToastId {
        self.toast(ToastKind::Message, title, ToastOptions::default())
    }
    pub fn success(&self, title: impl Into<String>) -> ToastId {
        self.toast(ToastKind::Success, title, ToastOptions::default())
    }
    pub fn error(&self, title: impl Into<String>) -> ToastId {
        self.toast(ToastKind::Error, title, ToastOptions::default())
    }
    pub fn info(&self, title: impl Into<String>) -> ToastId {
        self.toast(ToastKind::Info, title, ToastOptions::default())
    }
    pub fn warning(&self, title: impl Into<String>) -> ToastId {
        self.toast(ToastKind::Warning, title, ToastOptions::default())
    }

    /// Dismisses a toast (and cancels its timer). Returns whether it existed.
    pub fn dismiss(&self, id: ToastId) -> bool {
        self.clear_timer(id);
        let mut existed = false;
        self.queue.update(|q| existed = q.dismiss(id));
        existed
    }

    /// Dismisses every toast.
    pub fn dismiss_all(&self) {
        let mut ids = Vec::new();
        self.queue.update(|q| ids = q.dismiss_all());
        for id in ids {
            self.clear_timer(id);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn schedule(&self, id: ToastId, ms: u64) {
        let this = *self;
        if let Ok(handle) = set_timeout_with_handle(
            move || {
                this.timers.update_value(|t| t.retain(|(i, _)| *i != id));
                this.tick(real_now_ms());
            },
            std::time::Duration::from_millis(ms),
        ) {
            self.timers.update_value(|t| t.push((id, handle)));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn schedule(&self, _id: ToastId, _ms: u64) {}

    fn clear_timer(&self, id: ToastId) {
        self.timers.update_value(|t| {
            #[cfg(target_arch = "wasm32")]
            for (_, h) in t.iter().filter(|(i, _)| *i == id) {
                h.clear();
            }
            t.retain(|(i, _)| *i != id);
        });
    }

    fn clear_all_timers(&self) {
        self.timers.update_value(|t| {
            #[cfg(target_arch = "wasm32")]
            for (_, h) in t.iter() {
                h.clear();
            }
            t.clear();
        });
    }
}

impl Default for ToasterContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates and provides a `ToasterContext` (idempotent).
pub fn provide_sonner() -> ToasterContext {
    if let Some(ctx) = use_context::<ToasterContext>() {
        return ctx;
    }
    let ctx = ToasterContext::new();
    provide_context(ctx);
    ctx
}

/// Returns the nearest toast API.
///
/// # Panics
/// Panics if neither `provide_sonner` nor a parent `Toaster` provided it.
pub fn use_sonner() -> ToasterContext {
    use_context::<ToasterContext>().expect("use_sonner requires provide_sonner() or an ancestor <Toaster/>")
}

fn icon_path(kind: ToastKind) -> Option<&'static str> {
    match kind {
        ToastKind::Message => None,
        ToastKind::Success => Some("M22 11.08V12a10 10 0 1 1-5.93-9.14M22 4 12 14.01l-3-3"),
        ToastKind::Error => Some("M12 22c5.52 0 10-4.48 10-10S17.52 2 12 2 2 6.48 2 12s4.48 10 10 10zM15 9l-6 6M9 9l6 6"),
        ToastKind::Info => Some("M12 22c5.52 0 10-4.48 10-10S17.52 2 12 2 2 6.48 2 12s4.48 10 10 10zM12 16v-4M12 8h.01"),
        ToastKind::Warning => Some("m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3ZM12 9v4M12 17h.01"),
    }
}

#[component]
pub fn Toaster(
    #[prop(into, optional)] position: MaybeProp<ToasterPosition>,
    /// Maximum simultaneously visible toasts (default 3).
    #[prop(into, optional)]
    max_visible: MaybeProp<usize>,
    /// Default auto-dismiss duration in ms (default 4000).
    #[prop(into, optional)]
    duration_ms: MaybeProp<u64>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let ctx = provide_sonner();
    Effect::new(move |_| {
        if let Some(m) = max_visible.get() {
            ctx.queue.update(|q| q.set_max_visible(m));
        }
        if let Some(d) = duration_ms.get() {
            ctx.default_duration_ms.set(d);
        }
    });
    on_cleanup(move || ctx.clear_all_timers());

    let position = move || position.get().unwrap_or_default();

    view! {
        <section aria-label="Notifications" aria-live="polite" data-slot="sonner">
            <ol
                data-sonner-toaster="true"
                data-position=move || position().as_str()
                class=move || format!("{} {} {}", TOASTER_CLASS, position().class(), class.get().unwrap_or_default())
                id=move || id.get()
                style=move || format!("{} {}", TOASTER_STYLE, style.get().unwrap_or_default())
            >
                <For each=move || ctx.queue.get().visible() key=|t| t.id let:toast>
                    {
                        let tid = toast.id;
                        let icon = icon_path(toast.kind);
                        view! {
                            <li
                                data-slot="sonner-toast"
                                data-sonner-toast="true"
                                data-type=toast.kind.as_str()
                                role="status"
                                class=TOAST_CLASS
                            >
                                {icon.map(|d| view! {
                                    <div data-icon="" class=TOAST_ICON_CLASS>
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                            <path d=d></path>
                                        </svg>
                                    </div>
                                })}
                                <div data-content="" class=TOAST_CONTENT_CLASS>
                                    <div data-title="" class=TOAST_TITLE_CLASS>{toast.title.clone()}</div>
                                    {toast.description.clone().map(|d| view! {
                                        <div data-description="" class=TOAST_DESCRIPTION_CLASS>{d}</div>
                                    })}
                                </div>
                                <button type="button" aria-label="Close toast" data-close-button="" class=TOAST_CLOSE_CLASS
                                    on:click=move |_| { ctx.dismiss(tid); }>
                                    "×"
                                </button>
                            </li>
                        }
                    }
                </For>
            </ol>
        </section>
    }
}
