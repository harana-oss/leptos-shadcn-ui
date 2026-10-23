use leptos::prelude::*;
use leptos_style::Style;

const MENUBAR_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn Menubar(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let handle_click = {
        let on_click = on_click.clone();
        move |_| {
            if let Some(callback) = &on_click {
                callback.run(());
            }
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
        
        format!("{} {} {} {}", MENUBAR_CLASS, variant_class, size_class, class.get().unwrap_or_default())
    });

    view! {
        // Root is a `div[role=menubar]` (not a button) so that it can host
        // `MenubarTrigger` buttons without invalid nested-button markup.
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            role="menubar"
            aria-disabled=move || disabled.get().to_string()
            data-disabled=move || disabled.get().then_some("")
            on:click=handle_click
        >
            {children.map(|c| c())}
        </div>
    }
}

const MENUBAR_TRIGGER_CLASS: &str = "flex cursor-default select-none items-center rounded-sm px-3 py-1.5 text-sm font-medium outline-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground disabled:pointer-events-none disabled:opacity-50";
const MENUBAR_CONTENT_CLASS: &str = "z-50 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md";
const MENUBAR_ITEM_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

/// Shared open state for a single `MenubarMenu`, provided via context.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarMenuContext {
    pub open: RwSignal<bool>,
}

impl MenubarMenuContext {
    fn current() -> Self {
        use_context::<Self>().unwrap_or_else(|| Self { open: RwSignal::new(false) })
    }

    fn data_state(&self) -> &'static str {
        if self.open.get() { "open" } else { "closed" }
    }
}

/// A single menu within a `Menubar`. Provides open state to its trigger and content.
#[component]
pub fn MenubarMenu(
    /// Optional externally controlled open state.
    #[prop(into, optional)] open: Option<RwSignal<bool>>,
    /// Called whenever the open state changes.
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let open = open.unwrap_or_else(|| RwSignal::new(false));
    if let Some(cb) = on_open_change {
        Effect::new(move |prev: Option<bool>| {
            let current = open.get();
            if prev.is_some_and(|p| p != current) {
                cb.run(current);
            }
            current
        });
    }
    let ctx = MenubarMenuContext { open };
    provide_context(ctx);
    view! {
        <div class="relative" data-state=move || ctx.data_state()>
            {children()}
        </div>
    }
}

/// Button that toggles the enclosing `MenubarMenu`.
#[component]
pub fn MenubarTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = MenubarMenuContext::current();
    let computed_class = Signal::derive(move || {
        format!("{} {}", MENUBAR_TRIGGER_CLASS, class.get().unwrap_or_default())
    });
    view! {
        <button
            type="button"
            role="menuitem"
            class=computed_class
            id=move || id.get()
            style=move || style.get()
            disabled=move || disabled.get()
            aria-haspopup="menu"
            aria-expanded=move || ctx.open.get().to_string()
            data-state=move || ctx.data_state()
            data-disabled=move || disabled.get().then_some("")
            on:click=move |_| {
                if !disabled.get_untracked() {
                    ctx.open.update(|o| *o = !*o);
                }
            }
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Menu content; rendered only while the enclosing `MenubarMenu` is open.
#[component]
pub fn MenubarContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = MenubarMenuContext::current();
    let computed_class = Signal::derive(move || {
        format!("{} {}", MENUBAR_CONTENT_CLASS, class.get().unwrap_or_default())
    });
    let children = StoredValue::new(children);
    view! {
        <Show when=move || ctx.open.get()>
            <div
                role="menu"
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

/// Selectable item inside `MenubarContent`. Clicking runs `on_click` and closes the menu.
#[component]
pub fn MenubarItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_context::<MenubarMenuContext>();
    let computed_class = Signal::derive(move || {
        format!("{} {}", MENUBAR_ITEM_CLASS, class.get().unwrap_or_default())
    });
    view! {
        <div
            role="menuitem"
            tabindex="-1"
            class=computed_class
            id=move || id.get()
            style=move || style.get()
            aria-disabled=move || disabled.get().to_string()
            data-disabled=move || disabled.get().then_some("")
            on:click=move |_| {
                if disabled.get_untracked() {
                    return;
                }
                if let Some(cb) = on_click {
                    cb.run(());
                }
                if let Some(ctx) = ctx {
                    ctx.open.set(false);
                }
            }
        >
            {children.map(|c| c())}
        </div>
    }
}
