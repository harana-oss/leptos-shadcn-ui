use leptos::prelude::*;
use leptos_style::Style;

const NAVIGATION_MENU_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn NavigationMenu(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    /// Optional externally controlled value of the currently open item
    /// (empty string when no item is open).
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = NavigationMenuContext {
        active: value.unwrap_or_else(|| RwSignal::new(String::new())),
    };
    provide_context(ctx);

    let handle_click = {
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
            "outline" => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
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

        format!(
            "{} {} {} {}",
            NAVIGATION_MENU_CLASS,
            variant_class,
            size_class,
            class.get().unwrap_or_default()
        )
    });

    view! {
        // Root is a `nav` (not a button) so it can host trigger buttons
        // without invalid nested-button markup.
        <nav
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            aria-label="Main"
            aria-disabled=move || disabled.get().to_string()
            data-disabled=move || disabled.get().then_some("")
            on:click=handle_click
        >
            {children.map(|c| c())}
        </nav>
    }
}

const NAVIGATION_MENU_LIST_CLASS: &str =
    "group flex flex-1 list-none items-center justify-center space-x-1";
const NAVIGATION_MENU_TRIGGER_CLASS: &str = "group inline-flex h-10 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50";
const NAVIGATION_MENU_CONTENT_CLASS: &str = "left-0 top-0 w-full md:absolute md:w-auto";
const NAVIGATION_MENU_LINK_CLASS: &str = "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground";

/// Root state: the value of the currently open item ("" when none).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuContext {
    pub active: RwSignal<String>,
}

/// Per-item state: the item's unique value within the menu.
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationMenuItemContext {
    pub value: StoredValue<String>,
}

fn nav_ctx() -> NavigationMenuContext {
    use_context::<NavigationMenuContext>().unwrap_or_else(|| NavigationMenuContext {
        active: RwSignal::new(String::new()),
    })
}

fn item_is_open(ctx: NavigationMenuContext, item: Option<NavigationMenuItemContext>) -> bool {
    match item {
        Some(item) => {
            let value = item.value.get_value();
            ctx.active.with(|a| !a.is_empty() && *a == value)
        }
        None => false,
    }
}

/// Horizontal list of `NavigationMenuItem`s.
#[component]
pub fn NavigationMenuList(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", NAVIGATION_MENU_LIST_CLASS, class.get().unwrap_or_default())
    });
    view! { <ul class=computed_class>{children.map(|c| c())}</ul> }
}

/// A single entry in the list. Scopes its trigger/content pair.
#[component]
pub fn NavigationMenuItem(
    /// Unique value for this item; auto-generated when omitted.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    let value = value
        .get_untracked()
        .unwrap_or_else(|| format!("nav-item-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
    provide_context(NavigationMenuItemContext {
        value: StoredValue::new(value),
    });
    view! { <li class=move || class.get()>{children.map(|c| c())}</li> }
}

/// Button that opens/closes its item's content; opening one item closes the others.
#[component]
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = nav_ctx();
    let item = use_context::<NavigationMenuItemContext>();
    let item_for_open = item.clone();
    let is_open = move || item_is_open(ctx, item_for_open.clone());
    let is_open_for_state = is_open.clone();
    let is_open_for_aria = is_open.clone();
    let computed_class = Signal::derive(move || {
        format!("{} {}", NAVIGATION_MENU_TRIGGER_CLASS, class.get().unwrap_or_default())
    });
    view! {
        <button
            type="button"
            class=computed_class
            id=move || id.get()
            style=move || style.get()
            disabled=move || disabled.get()
            aria-expanded=move || is_open_for_aria().to_string()
            data-state=move || if is_open_for_state() { "open" } else { "closed" }
            data-disabled=move || disabled.get().then_some("")
            on:click=move |_| {
                if disabled.get_untracked() {
                    return;
                }
                if let Some(item) = &item {
                    let value = item.value.get_value();
                    ctx.active.update(|a| {
                        *a = if *a == value { String::new() } else { value };
                    });
                }
            }
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Content panel; rendered only while its item is open.
#[component]
pub fn NavigationMenuContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = nav_ctx();
    let item = use_context::<NavigationMenuItemContext>();
    let computed_class = Signal::derive(move || {
        format!("{} {}", NAVIGATION_MENU_CONTENT_CLASS, class.get().unwrap_or_default())
    });
    let children = StoredValue::new(children);
    view! {
        <Show when=move || item_is_open(ctx, item.clone())>
            <div
                class=computed_class
                id=move || id.get()
                style=move || style.get()
                data-state="open"
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </div>
        </Show>
    }
}

/// Navigation link; closes any open item when clicked.
#[component]
pub fn NavigationMenuLink(
    #[prop(into, optional)] href: MaybeProp<String>,
    #[prop(into, optional)] active: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = nav_ctx();
    let computed_class = Signal::derive(move || {
        format!("{} {}", NAVIGATION_MENU_LINK_CLASS, class.get().unwrap_or_default())
    });
    view! {
        <a
            href=move || href.get()
            class=computed_class
            data-active=move || active.get().then_some("")
            aria-current=move || active.get().then_some("page")
            on:click=move |_| {
                if let Some(cb) = on_select {
                    cb.run(());
                }
                ctx.active.set(String::new());
            }
        >
            {children.map(|c| c())}
        </a>
    }
}
