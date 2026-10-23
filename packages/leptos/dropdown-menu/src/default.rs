use leptos::prelude::*;
use leptos_style::Style;

const DROPDOWN_MENU_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
const DROPDOWN_MENU_TRIGGER_CLASS: &str = "inline-flex items-center justify-center outline-none disabled:pointer-events-none disabled:opacity-50";
const DROPDOWN_MENU_CONTENT_CLASS: &str = "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md";
const DROPDOWN_MENU_ITEM_CLASS: &str = "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
const DROPDOWN_MENU_LABEL_CLASS: &str = "px-2 py-1.5 text-sm font-semibold";
const DROPDOWN_MENU_SEPARATOR_CLASS: &str = "-mx-1 my-1 h-px bg-muted";
const DROPDOWN_MENU_SHORTCUT_CLASS: &str = "ml-auto text-xs tracking-widest opacity-60";

/// Shared dropdown state, provided via context by `DropdownMenu`.
#[derive(Debug, Clone, Copy)]
pub struct DropdownMenuContext {
    pub open: RwSignal<bool>,
    pub value: RwSignal<String>,
    pub disabled: Signal<bool>,
    pub on_select: MaybeProp<Callback<String>>,
}

impl DropdownMenuContext {
    fn current() -> Self {
        use_context::<Self>().unwrap_or_else(|| Self {
            open: RwSignal::new(false),
            value: RwSignal::new(String::new()),
            disabled: Signal::from(false),
            on_select: MaybeProp::default(),
        })
    }

    fn data_state(&self) -> &'static str {
        if self.open.get() { "open" } else { "closed" }
    }
}

/// Dropdown menu root. Holds the open/value state shared by its parts.
///
/// For backwards compatibility it still accepts `variant`, `size` and
/// `on_click` (fired on any click inside the root).
#[component]
pub fn DropdownMenu(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: MaybeProp<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    /// Optional externally controlled open state.
    #[prop(optional)] open: Option<RwSignal<bool>>,
    /// Optional externally controlled selected value.
    #[prop(optional)] value: Option<RwSignal<String>>,
    /// Called with an item's `value` when it is selected.
    #[prop(into, optional)] on_select: MaybeProp<Callback<String>>,
    /// Called whenever the open state changes.
    #[prop(into, optional)] on_open_change: MaybeProp<Callback<bool>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = open.unwrap_or_else(|| RwSignal::new(false));
    let value = value.unwrap_or_else(|| RwSignal::new(String::new()));
    if let Some(cb) = on_open_change.get_untracked() {
        Effect::new(move |prev: Option<bool>| {
            let current = open.get();
            if prev.is_some_and(|p| p != current) {
                cb.run(current);
            }
            current
        });
    }
    let ctx = DropdownMenuContext { open, value, disabled, on_select };
    provide_context(ctx);

    let handle_click = move |_| {
        if let Some(callback) = on_click.get() {
            callback.run(());
        }
    };

    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().unwrap_or_default().as_str() {
            "default" => "bg-primary text-primary-foreground hover:bg-primary/90",
            "destructive" => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            "outline" => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            "secondary" => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            "ghost" => "hover:bg-accent hover:text-accent-foreground",
            "link" => "text-primary underline-offset-4 hover:underline",
            _ => "bg-primary text-primary-foreground hover:bg-primary/90",
        };

        let size_class = match size.get().unwrap_or_default().as_str() {
            "default" => "h-10 px-4 py-2",
            "sm" => "h-9 rounded-md px-3",
            "lg" => "h-11 rounded-md px-8",
            "icon" => "h-10 w-10",
            _ => "h-10 px-4 py-2",
        };

        format!("relative {} {} {} {}", DROPDOWN_MENU_CLASS, variant_class, size_class, class.get().unwrap_or_default())
    });

    view! {
        // Root is a `div` (not a button) so it can host the trigger button
        // without invalid nested-button markup.
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            aria-disabled=move || disabled.get().to_string()
            data-disabled=move || disabled.get().then_some("")
            data-state=move || ctx.data_state()
            on:click=handle_click
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Button that toggles the enclosing `DropdownMenu`.
#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_focus: MaybeProp<Callback<()>>,
    #[prop(into, optional)] on_blur: MaybeProp<Callback<()>>,
    #[prop(into, optional)] on_mouseenter: MaybeProp<Callback<()>>,
    #[prop(into, optional)] on_mouseleave: MaybeProp<Callback<()>>,
    /// Extra click handler; runs before the menu is toggled.
    #[prop(into, optional)] on_click: MaybeProp<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = DropdownMenuContext::current();
    let is_disabled = move || disabled.get() || ctx.disabled.get();
    let computed_class = Signal::derive(move || {
        format!("{} {}", DROPDOWN_MENU_TRIGGER_CLASS, class.get().unwrap_or_default())
    });
    view! {
        <button
            type="button"
            class=computed_class
            id=move || id.get()
            style=move || style.get()
            disabled=is_disabled
            aria-haspopup="menu"
            aria-expanded=move || ctx.open.get().to_string()
            data-state=move || ctx.data_state()
            data-disabled=move || is_disabled().then_some("")
            on:click=move |_| {
                if !is_disabled() {
                    if let Some(cb) = on_click.get_untracked() {
                        cb.run(());
                    }
                    ctx.open.update(|o| *o = !*o);
                }
            }
            on:focus=move |_| {
                if let Some(cb) = on_focus.get_untracked() {
                    cb.run(());
                }
            }
            on:blur=move |_| {
                if let Some(cb) = on_blur.get_untracked() {
                    cb.run(());
                }
            }
            on:mouseenter=move |_| {
                if let Some(cb) = on_mouseenter.get_untracked() {
                    cb.run(());
                }
            }
            on:mouseleave=move |_| {
                if let Some(cb) = on_mouseleave.get_untracked() {
                    cb.run(());
                }
            }
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Menu content; rendered only while the enclosing `DropdownMenu` is open.
#[component]
pub fn DropdownMenuContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = DropdownMenuContext::current();
    let computed_class = Signal::derive(move || {
        format!("{} {}", DROPDOWN_MENU_CONTENT_CLASS, class.get().unwrap_or_default())
    });
    let children = StoredValue::new(children);
    view! {
        <Show when=move || ctx.open.get()>
            <div
                role="menu"
                aria-orientation="vertical"
                class=computed_class
                id=move || id.get()
                style=move || style.get()
                data-state=move || ctx.data_state()
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </div>
        </Show>
    }
}

/// Selectable item. Clicking runs `on_click`, records `value` as the
/// selected value, calls the root's `on_select`, then closes the menu.
#[component]
pub fn DropdownMenuItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_click: MaybeProp<Callback<()>>,
    #[prop(into, optional)] on_keydown: MaybeProp<Callback<leptos::ev::KeyboardEvent>>,
    #[prop(into, optional)] on_mouseenter: MaybeProp<Callback<()>>,
    #[prop(into, optional)] on_mouseleave: MaybeProp<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = DropdownMenuContext::current();
    let computed_class = Signal::derive(move || {
        format!("{} {}", DROPDOWN_MENU_ITEM_CLASS, class.get().unwrap_or_default())
    });
    let select = move || {
        if disabled.get_untracked() {
            return;
        }
        if let Some(cb) = on_click.get_untracked() {
            cb.run(());
        }
        if let Some(v) = value.get_untracked() {
            ctx.value.set(v.clone());
            if let Some(cb) = ctx.on_select.get_untracked() {
                cb.run(v);
            }
        }
        ctx.open.set(false);
    };
    view! {
        <div
            role="menuitem"
            tabindex="-1"
            class=computed_class
            id=move || id.get()
            style=move || style.get()
            aria-disabled=move || disabled.get().to_string()
            data-disabled=move || disabled.get().then_some("")
            data-value=move || value.get()
            on:click=move |_| select()
            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                if let Some(cb) = on_keydown.get_untracked() {
                    cb.run(ev.clone());
                }
                if ev.key() == "Enter" || ev.key() == " " {
                    select();
                } else if ev.key() == "Escape" {
                    ctx.open.set(false);
                }
            }
            on:mouseenter=move |_| {
                if let Some(cb) = on_mouseenter.get_untracked() {
                    cb.run(());
                }
            }
            on:mouseleave=move |_| {
                if let Some(cb) = on_mouseleave.get_untracked() {
                    cb.run(());
                }
            }
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Non-interactive heading inside `DropdownMenuContent`.
#[component]
pub fn DropdownMenuLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", DROPDOWN_MENU_LABEL_CLASS, class.get().unwrap_or_default())
    });
    view! { <div class=computed_class id=move || id.get()>{children.map(|c| c())}</div> }
}

/// Visual divider between groups of items.
#[component]
pub fn DropdownMenuSeparator(#[prop(into, optional)] class: MaybeProp<String>) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", DROPDOWN_MENU_SEPARATOR_CLASS, class.get().unwrap_or_default())
    });
    view! { <div role="separator" aria-orientation="horizontal" class=computed_class></div> }
}

/// Groups related items.
#[component]
pub fn DropdownMenuGroup(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! { <div role="group" class=move || class.get()>{children.map(|c| c())}</div> }
}

/// Keyboard shortcut hint shown at the end of an item.
#[component]
pub fn DropdownMenuShortcut(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", DROPDOWN_MENU_SHORTCUT_CLASS, class.get().unwrap_or_default())
    });
    view! { <span class=computed_class>{children.map(|c| c())}</span> }
}
