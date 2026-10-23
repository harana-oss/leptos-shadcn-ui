//! Leptos port of the shadcn/ui (v4) Sidebar component.
//!
//! `SidebarProvider` owns the open/collapsed state and registers the
//! `ctrl/cmd + b` shortcut through `window_event_listener`; the listener is
//! removed in `on_cleanup`, so nothing leaks after unmount.

use leptos::ev;
use leptos::prelude::*;

mod classes;
pub use classes::*;

#[cfg(test)]
mod tests;

/// Keyboard key (with ctrl/cmd) that toggles the sidebar.
pub const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";
pub const SIDEBAR_WIDTH: &str = "16rem";
pub const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
pub const SIDEBAR_WIDTH_ICON: &str = "3rem";
/// Viewport width (px) at and above which the layout is considered desktop.
pub const MOBILE_BREAKPOINT: u32 = 768;

/// Media query matching mobile viewports (`max-width: 767px`).
pub fn mobile_media_query() -> String {
    format!("(max-width: {}px)", MOBILE_BREAKPOINT - 1)
}

/// Whether a viewport of `width` CSS pixels counts as mobile.
pub fn is_mobile_width(width: f64) -> bool {
    width < MOBILE_BREAKPOINT as f64
}

/// Resolves the effective mobile flag: an explicit override wins over detection.
pub fn resolve_is_mobile(override_value: Option<bool>, detected: bool) -> bool {
    override_value.unwrap_or(detected)
}

/// Reactive mobile detection, mirroring shadcn's `useIsMobile`.
///
/// In the browser this subscribes to `matchMedia("(max-width: 767px)")`
/// change events and removes the listener on cleanup. During SSR and on
/// native targets it stays `false`.
pub fn use_is_mobile() -> Signal<bool> {
    let is_mobile = RwSignal::new(false);
    #[cfg(target_arch = "wasm32")]
    {
        // Effects only run on the client, so SSR never touches `window`.
        Effect::new(move |prev: Option<()>| {
            if prev.is_none() {
                attach_media_listener(is_mobile);
            }
        });
    }
    is_mobile.into()
}

#[cfg(target_arch = "wasm32")]
fn attach_media_listener(is_mobile: RwSignal<bool>) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::{closure::Closure, JsCast};

    let Some(window) = web_sys::window() else { return };
    let Ok(Some(mql)) = window.match_media(&mobile_media_query()) else {
        return;
    };
    is_mobile.set(mql.matches());

    let closure = Closure::<dyn Fn(web_sys::MediaQueryListEvent)>::new(
        move |e: web_sys::MediaQueryListEvent| is_mobile.set(e.matches()),
    );
    let _ = mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());

    let guard = SendWrapper::new((mql, closure));
    on_cleanup(move || {
        let (mql, closure) = guard.take();
        let _ = mql
            .remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
    });
}

/// Which edge the sidebar is attached to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

impl SidebarSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        }
    }
}

/// Visual variant of the sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        }
    }
}

/// How the sidebar collapses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

impl SidebarCollapsible {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarCollapsible::Offcanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
            SidebarCollapsible::None => "none",
        }
    }
}

/// Variant of `SidebarMenuButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

/// Size of `SidebarMenuButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl SidebarMenuButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuButtonSize::Default => "default",
            SidebarMenuButtonSize::Sm => "sm",
            SidebarMenuButtonSize::Lg => "lg",
        }
    }
}

/// `data-state` value for the given open flag.
pub fn sidebar_state(open: bool) -> &'static str {
    if open {
        "expanded"
    } else {
        "collapsed"
    }
}

/// Returns true when a keydown should toggle the sidebar.
pub fn is_toggle_shortcut(key: &str, ctrl: bool, meta: bool) -> bool {
    (ctrl || meta) && key.eq_ignore_ascii_case(SIDEBAR_KEYBOARD_SHORTCUT)
}

/// Shared sidebar state provided by `SidebarProvider`.
#[derive(Debug, Clone, Copy)]
pub struct SidebarContext {
    pub open: RwSignal<bool>,
    pub open_mobile: RwSignal<bool>,
    pub is_mobile: RwSignal<bool>,
    on_open_change: Option<Callback<bool>>,
}

impl SidebarContext {
    pub fn new(default_open: bool) -> Self {
        Self {
            open: RwSignal::new(default_open),
            open_mobile: RwSignal::new(false),
            is_mobile: RwSignal::new(false),
            on_open_change: None,
        }
    }

    /// `"expanded"` or `"collapsed"`.
    pub fn state(&self) -> &'static str {
        sidebar_state(self.open.get())
    }

    pub fn set_open(&self, value: bool) {
        self.open.set(value);
        if let Some(cb) = self.on_open_change {
            cb.run(value);
        }
    }

    /// Toggles the mobile sheet on mobile, otherwise the desktop sidebar.
    pub fn toggle(&self) {
        if self.is_mobile.get_untracked() {
            self.open_mobile.update(|o| *o = !*o);
        } else {
            self.set_open(!self.open.get_untracked());
        }
    }
}

/// Access the nearest `SidebarProvider` context.
///
/// # Panics
/// Panics when used outside of a `SidebarProvider`.
pub fn use_sidebar() -> SidebarContext {
    use_context::<SidebarContext>().expect("use_sidebar must be used within a SidebarProvider")
}

fn join(parts: &[&str]) -> String {
    parts
        .iter()
        .filter(|p| !p.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
}

#[component]
pub fn SidebarProvider(
    #[prop(optional, default = true)] default_open: bool,
    /// Optional externally controlled open signal.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    /// Optional override for mobile detection. When unset, the provider
    /// detects mobile viewports automatically via [`use_is_mobile`].
    #[prop(into, optional)]
    is_mobile: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let mut ctx = SidebarContext::new(default_open);
    if let Some(open) = open {
        ctx.open = open;
    }
    ctx.on_open_change = on_open_change;
    let detected = use_is_mobile();
    Effect::new(move |_| {
        ctx.is_mobile.set(resolve_is_mobile(is_mobile.get(), detected.get()));
    });
    provide_context(ctx);

    let handle = window_event_listener(ev::keydown, move |e: ev::KeyboardEvent| {
        if is_toggle_shortcut(&e.key(), e.ctrl_key(), e.meta_key()) {
            e.prevent_default();
            ctx.toggle();
        }
    });
    on_cleanup(move || handle.remove());

    let computed_class = move || join(&[SIDEBAR_WRAPPER_CLASS, &class.get().unwrap_or_default()]);
    let computed_style = move || {
        format!(
            "--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}; {}",
            style.get().unwrap_or_default()
        )
    };

    view! {
        <div data-slot="sidebar-wrapper" class=computed_class id=move || id.get() style=computed_style>
            {children()}
        </div>
    }
}

#[component]
pub fn Sidebar(
    #[prop(into, optional)] side: MaybeProp<SidebarSide>,
    #[prop(into, optional)] variant: MaybeProp<SidebarVariant>,
    #[prop(into, optional)] collapsible: MaybeProp<SidebarCollapsible>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_sidebar();
    let side = move || side.get().unwrap_or_default();
    let variant = move || variant.get().unwrap_or_default();
    let collapsible = move || collapsible.get().unwrap_or_default();
    let user_class = move || class.get().unwrap_or_default();
    let children = StoredValue::new(children);

    move || {
        if collapsible() == SidebarCollapsible::None {
            view! {
                <div
                    data-slot="sidebar"
                    class=move || join(&[SIDEBAR_STATIC_CLASS, &user_class()])
                    id=move || id.get()
                    style=move || style.get()
                >
                    {children.with_value(|c| c())}
                </div>
            }
            .into_any()
        } else if ctx.is_mobile.get() {
            view! {
                <div data-slot="sidebar-mobile" data-state=move || if ctx.open_mobile.get() { "open" } else { "closed" }>
                    <Show when=move || ctx.open_mobile.get()>
                        <div class=SIDEBAR_MOBILE_OVERLAY_CLASS on:click=move |_| ctx.open_mobile.set(false)></div>
                        <div
                            data-slot="sidebar"
                            data-sidebar="sidebar"
                            data-mobile="true"
                            data-side=move || side().as_str()
                            class=move || join(&[SIDEBAR_MOBILE_CLASS, sidebar_mobile_side_class(side()), &user_class()])
                            id=move || id.get()
                            style=move || format!("--sidebar-width: {SIDEBAR_WIDTH_MOBILE}; {}", style.get().unwrap_or_default())
                        >
                            <div class="flex h-full w-full flex-col">{children.with_value(|c| c())}</div>
                        </div>
                    </Show>
                </div>
            }
            .into_any()
        } else {
            view! {
                <div
                    data-slot="sidebar"
                    class=SIDEBAR_PEER_CLASS
                    data-state=move || ctx.state()
                    data-collapsible=move || if ctx.open.get() { "" } else { collapsible().as_str() }
                    data-variant=move || variant().as_str()
                    data-side=move || side().as_str()
                >
                    <div data-slot="sidebar-gap" class=move || sidebar_gap_class(variant())></div>
                    <div
                        data-slot="sidebar-container"
                        class=move || join(&[&sidebar_container_class(side(), variant()), &user_class()])
                        id=move || id.get()
                        style=move || style.get()
                    >
                        <div data-sidebar="sidebar" data-slot="sidebar-inner" class=SIDEBAR_INNER_CLASS>
                            {children.with_value(|c| c())}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
    }
}

#[component]
pub fn SidebarTrigger(
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_sidebar();
    view! {
        <button
            type="button"
            data-sidebar="trigger"
            data-slot="sidebar-trigger"
            class=move || join(&[SIDEBAR_TRIGGER_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
            on:click=move |e| {
                if let Some(cb) = on_click {
                    cb.run(e);
                }
                ctx.toggle();
            }
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <rect width="18" height="18" x="3" y="3" rx="2"></rect>
                <path d="M9 3v18"></path>
            </svg>
            <span class="sr-only">"Toggle Sidebar"</span>
        </button>
    }
}

#[component]
pub fn SidebarRail(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_sidebar();
    view! {
        <button
            type="button"
            data-sidebar="rail"
            data-slot="sidebar-rail"
            aria-label="Toggle Sidebar"
            tabindex="-1"
            title="Toggle Sidebar"
            class=move || join(&[SIDEBAR_RAIL_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
            on:click=move |_| ctx.toggle()
        ></button>
    }
}

macro_rules! slot_div {
    ($(#[$m:meta])* $name:ident, $slot:literal, $sidebar:literal, $class:ident) => {
        $(#[$m])*
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: MaybeProp<String>,
            #[prop(into, optional)] id: MaybeProp<String>,
            #[prop(into, optional)] style: MaybeProp<String>,
            #[prop(optional)] children: Option<Children>,
        ) -> impl IntoView {
            view! {
                <div
                    data-slot=$slot
                    data-sidebar=$sidebar
                    class=move || join(&[$class, &class.get().unwrap_or_default()])
                    id=move || id.get()
                    style=move || style.get()
                >
                    {children.map(|c| c())}
                </div>
            }
        }
    };
}

slot_div!(SidebarHeader, "sidebar-header", "header", SIDEBAR_HEADER_CLASS);
slot_div!(SidebarFooter, "sidebar-footer", "footer", SIDEBAR_FOOTER_CLASS);
slot_div!(SidebarContent, "sidebar-content", "content", SIDEBAR_CONTENT_CLASS);
slot_div!(SidebarGroup, "sidebar-group", "group", SIDEBAR_GROUP_CLASS);
slot_div!(SidebarGroupLabel, "sidebar-group-label", "group-label", SIDEBAR_GROUP_LABEL_CLASS);
slot_div!(SidebarGroupContent, "sidebar-group-content", "group-content", SIDEBAR_GROUP_CONTENT_CLASS);

#[component]
pub fn SidebarInset(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <main
            data-slot="sidebar-inset"
            class=move || join(&[SIDEBAR_INSET_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </main>
    }
}

#[component]
pub fn SidebarSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            role="separator"
            aria-orientation="horizontal"
            data-orientation="horizontal"
            data-slot="sidebar-separator"
            data-sidebar="separator"
            class=move || join(&[SIDEBAR_SEPARATOR_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
        ></div>
    }
}

#[component]
pub fn SidebarGroupAction(
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-slot="sidebar-group-action"
            data-sidebar="group-action"
            class=move || join(&[SIDEBAR_GROUP_ACTION_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
            on:click=move |e| if let Some(cb) = on_click { cb.run(e) }
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn SidebarMenu(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu"
            data-sidebar="menu"
            class=move || join(&[SIDEBAR_MENU_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </ul>
    }
}

#[component]
pub fn SidebarMenuItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-item"
            data-sidebar="menu-item"
            class=move || join(&[SIDEBAR_MENU_ITEM_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </li>
    }
}

#[component]
pub fn SidebarMenuButton(
    #[prop(into, optional)] variant: MaybeProp<SidebarMenuButtonVariant>,
    #[prop(into, optional)] size: MaybeProp<SidebarMenuButtonSize>,
    #[prop(into, optional)] is_active: MaybeProp<bool>,
    /// Native tooltip shown when the sidebar is collapsed.
    #[prop(into, optional)]
    tooltip: MaybeProp<String>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_context::<SidebarContext>();
    let size = move || size.get().unwrap_or_default();
    let title = move || {
        let collapsed = ctx.map(|c| !c.open.get() && !c.is_mobile.get()).unwrap_or(false);
        if collapsed {
            tooltip.get()
        } else {
            None
        }
    };
    view! {
        <button
            type="button"
            data-slot="sidebar-menu-button"
            data-sidebar="menu-button"
            data-size=move || size().as_str()
            data-active=move || is_active.get().unwrap_or(false).to_string()
            title=title
            class=move || join(&[
                &sidebar_menu_button_class(variant.get().unwrap_or_default(), size()),
                &class.get().unwrap_or_default(),
            ])
            id=move || id.get()
            style=move || style.get()
            on:click=move |e| if let Some(cb) = on_click { cb.run(e) }
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn SidebarMenuAction(
    #[prop(into, optional)] show_on_hover: MaybeProp<bool>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-slot="sidebar-menu-action"
            data-sidebar="menu-action"
            class=move || join(&[
                &sidebar_menu_action_class(show_on_hover.get().unwrap_or(false)),
                &class.get().unwrap_or_default(),
            ])
            id=move || id.get()
            style=move || style.get()
            on:click=move |e| if let Some(cb) = on_click { cb.run(e) }
        >
            {children.map(|c| c())}
        </button>
    }
}

slot_div!(SidebarMenuBadge, "sidebar-menu-badge", "menu-badge", SIDEBAR_MENU_BADGE_CLASS);

#[component]
pub fn SidebarMenuSkeleton(
    #[prop(into, optional)] show_icon: MaybeProp<bool>,
    /// CSS width of the text bar (default `70%`).
    #[prop(into, optional)]
    width: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-menu-skeleton"
            data-sidebar="menu-skeleton"
            class=move || join(&[SIDEBAR_MENU_SKELETON_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
        >
            <Show when=move || show_icon.get().unwrap_or(false)>
                <div data-sidebar="menu-skeleton-icon" class=join(&[SKELETON_CLASS, "size-4 rounded-md"])></div>
            </Show>
            <div
                data-sidebar="menu-skeleton-text"
                class=join(&[SKELETON_CLASS, "h-4 max-w-(--skeleton-width) flex-1"])
                style=move || format!("--skeleton-width: {}", width.get().unwrap_or_else(|| "70%".into()))
            ></div>
        </div>
    }
}

#[component]
pub fn SidebarMenuSub(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu-sub"
            data-sidebar="menu-sub"
            class=move || join(&[SIDEBAR_MENU_SUB_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </ul>
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-sub-item"
            data-sidebar="menu-sub-item"
            class=move || join(&[SIDEBAR_MENU_SUB_ITEM_CLASS, &class.get().unwrap_or_default()])
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </li>
    }
}

#[component]
pub fn SidebarMenuSubButton(
    #[prop(into, optional)] href: MaybeProp<String>,
    /// `sm` or `md` (default `md`).
    #[prop(into, optional)]
    size: MaybeProp<String>,
    #[prop(into, optional)] is_active: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let size = move || size.get().unwrap_or_else(|| "md".into());
    view! {
        <a
            href=move || href.get()
            data-slot="sidebar-menu-sub-button"
            data-sidebar="menu-sub-button"
            data-size=size
            data-active=move || is_active.get().unwrap_or(false).to_string()
            class=move || join(&[
                SIDEBAR_MENU_SUB_BUTTON_CLASS,
                if size() == "sm" { "text-xs" } else { "text-sm" },
                &class.get().unwrap_or_default(),
            ])
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </a>
    }
}
