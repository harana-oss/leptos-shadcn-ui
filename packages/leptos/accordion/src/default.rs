use leptos::prelude::*;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AccordionType {
    #[default]
    Single,
    Multiple,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AccordionOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Context newtypes: each boolean signal gets its own type so that
/// `provide_context` calls do not overwrite each other.
#[derive(Debug, Clone, Copy)]
pub struct AccordionCollapsible(pub Signal<bool>);
#[derive(Debug, Clone, Copy)]
pub struct AccordionDisabled(pub Signal<bool>);
#[derive(Debug, Clone, Copy)]
pub struct AccordionItemDisabled(pub Signal<bool>);
#[derive(Debug, Clone, Copy)]
pub struct AccordionItemExpanded(pub Signal<bool>);
#[derive(Debug, Clone, PartialEq)]
pub struct AccordionItemValue(pub String);

/// Pure toggle logic, used by `AccordionTrigger`.
pub fn compute_toggle(
    accordion_type: &AccordionType,
    collapsible: bool,
    disabled: bool,
    current: &[String],
    item: &str,
) -> Vec<String> {
    let mut v = current.to_vec();
    if disabled {
        return v;
    }
    match accordion_type {
        AccordionType::Single => {
            if v.iter().any(|x| x == item) {
                if collapsible {
                    v.clear();
                }
            } else {
                v.clear();
                v.push(item.to_string());
            }
        }
        AccordionType::Multiple => {
            if let Some(i) = v.iter().position(|x| x == item) {
                v.remove(i);
            } else {
                v.push(item.to_string());
            }
        }
    }
    v
}

#[component]
pub fn Accordion(
    #[prop(into, optional)] r#type: Signal<AccordionType>,
    #[prop(into, optional)] orientation: Signal<AccordionOrientation>,
    #[prop(into, optional)] collapsible: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] value: RwSignal<Vec<String>>,
    #[prop(into, optional)] default_value: Vec<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Initialize value with default if empty
    Effect::new(move |_| {
        if value.get().is_empty() && !default_value.is_empty() {
            value.set(default_value.clone());
        }
    });

    provide_context(r#type);
    provide_context(orientation);
    provide_context(AccordionCollapsible(collapsible));
    provide_context(AccordionDisabled(disabled));
    provide_context(value);
    provide_context(on_value_change);

    let computed_class =
        Signal::derive(move || format!("w-full {}", class.get().unwrap_or_default()));

    view! {
        <div
            class=move || computed_class.get()
            data-orientation=move || match orientation.get() {
                AccordionOrientation::Vertical => "vertical",
                AccordionOrientation::Horizontal => "horizontal",
            }
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let accordion_disabled = expect_context::<AccordionDisabled>().0;
    let accordion_value = expect_context::<RwSignal<Vec<String>>>();

    let is_disabled = Signal::derive(move || disabled.get() || accordion_disabled.get());
    let value_clone = value.clone();
    let is_expanded = Signal::derive(move || accordion_value.get().contains(&value_clone));

    provide_context(AccordionItemValue(value.clone()));
    provide_context(AccordionItemDisabled(is_disabled));
    provide_context(AccordionItemExpanded(is_expanded));

    let computed_class =
        Signal::derive(move || format!("border-b {}", class.get().unwrap_or_default()));

    view! {
        <div
            class=move || computed_class.get()
            data-state=move || if is_expanded.get() { "open" } else { "closed" }
            data-disabled=move || is_disabled.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: Option<Callback<AccordionTriggerChildProps>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let item_value = expect_context::<AccordionItemValue>().0;
    let is_disabled = expect_context::<AccordionItemDisabled>().0;
    let is_expanded = expect_context::<AccordionItemExpanded>().0;
    let accordion_type = expect_context::<Signal<AccordionType>>();
    let accordion_value = expect_context::<RwSignal<Vec<String>>>();
    let on_value_change = expect_context::<Option<Callback<Vec<String>>>>();
    let collapsible = expect_context::<AccordionCollapsible>().0;

    // Toggle logic extracted so it can be used by both click and keydown without moving closures
    let toggle = {
        let item_value = item_value.clone();
        move || {
            if is_disabled.get() {
                return;
            }

            let current_value = compute_toggle(
                &accordion_type.get(),
                collapsible.get(),
                false,
                &accordion_value.get(),
                &item_value,
            );

            accordion_value.set(current_value.clone());
            if let Some(callback) = &on_value_change {
                callback.run(current_value);
            }
        }
    };

    let handle_click = {
        let toggle = toggle.clone();
        move |_: MouseEvent| {
            toggle();
        }
    };

    let handle_keydown = {
        let toggle = toggle.clone();
        move |e: KeyboardEvent| match e.key().as_str() {
            "Enter" | " " => {
                e.prevent_default();
                toggle();
            }
            _ => {}
        }
    };

    let computed_class = Signal::derive(move || {
        format!(
            "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180 {}",
            class.get().unwrap_or_default()
        )
    });

    if let Some(as_child) = as_child {
        let child_props = AccordionTriggerChildProps {
            class: computed_class.get(),
            onclick: Some(Callback::new(move |_| {
                if is_disabled.get() {
                    return;
                }

                let mut current_value = accordion_value.get();

                match accordion_type.get() {
                    AccordionType::Single => {
                        if current_value.contains(&item_value) {
                            if collapsible.get() {
                                current_value.clear();
                            }
                        } else {
                            current_value.clear();
                            current_value.push(item_value.clone());
                        }
                    }
                    AccordionType::Multiple => {
                        if let Some(index) = current_value.iter().position(|v| v == &item_value) {
                            current_value.remove(index);
                        } else {
                            current_value.push(item_value.clone());
                        }
                    }
                }

                accordion_value.set(current_value.clone());
                if let Some(callback) = &on_value_change {
                    callback.run(current_value);
                }
            })),
            onkeydown: Some(Callback::new(handle_keydown)),
            disabled: is_disabled.get(),
            expanded: is_expanded.get(),
        };
        as_child.run(child_props);
        ().into_any()
    } else {
        view! {
            <button
                class=move || computed_class.get()
                data-state=move || if is_expanded.get() { "open" } else { "closed" }
                disabled=is_disabled
                aria-expanded=move || is_expanded.get()
                on:click=handle_click
                on:keydown=handle_keydown
            >
                {children.map(|c| c())}
                <svg
                    class="h-4 w-4 shrink-0 transition-transform duration-200"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="m6 9 6 6 6-6"/>
                </svg>
            </button>
        }
        .into_any()
    }
}

#[derive(Debug, Clone)]
pub struct AccordionTriggerChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
    pub onkeydown: Option<Callback<KeyboardEvent>>,
    pub disabled: bool,
    pub expanded: bool,
}

#[component]
pub fn AccordionContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] force_mount: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let is_expanded = expect_context::<AccordionItemExpanded>().0;

    let computed_class = Signal::derive(move || {
        format!(
            "overflow-hidden text-sm transition-all data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down {}",
            class.get().unwrap_or_default()
        )
    });

    let _should_render = Signal::derive(move || force_mount.get() || is_expanded.get());

    view! {
        <div
            class=move || computed_class.get()
            data-state=move || if is_expanded.get() { "open" } else { "closed" }
        >
            <div class="pb-4 pt-0">
                {children.map(|c| c())}
            </div>
        </div>
    }
}
#[cfg(test)]
mod context_type_regression_tests {
    use super::*;

    #[test]
    fn context_signals_are_independent() {
        let owner = Owner::new();
        owner.with(|| {
            provide_context(AccordionCollapsible(Signal::derive(|| true)));
            provide_context(AccordionDisabled(Signal::derive(|| false)));
            provide_context(AccordionItemDisabled(Signal::derive(|| false)));
            provide_context(AccordionItemExpanded(Signal::derive(|| true)));
            assert!(expect_context::<AccordionCollapsible>().0.get_untracked());
            assert!(!expect_context::<AccordionDisabled>().0.get_untracked());
            assert!(!expect_context::<AccordionItemDisabled>().0.get_untracked());
            assert!(expect_context::<AccordionItemExpanded>().0.get_untracked());
        });
    }

    #[test]
    fn collapsible_with_enabled_item_collapses() {
        let cur = vec!["a".to_string()];
        assert!(compute_toggle(&AccordionType::Single, true, false, &cur, "a").is_empty());
        assert_eq!(
            compute_toggle(&AccordionType::Single, false, false, &cur, "a"),
            cur
        );
    }

    #[test]
    fn disabled_blocks_toggle_regardless_of_collapsible() {
        let cur = vec!["a".to_string()];
        assert_eq!(
            compute_toggle(&AccordionType::Single, true, true, &cur, "a"),
            cur
        );
        assert!(compute_toggle(&AccordionType::Multiple, false, true, &[], "b").is_empty());
    }

    #[test]
    fn single_and_multiple_open() {
        let cur = vec!["a".to_string()];
        assert_eq!(
            compute_toggle(&AccordionType::Single, false, false, &cur, "b"),
            vec!["b"]
        );
        assert_eq!(
            compute_toggle(&AccordionType::Multiple, false, false, &cur, "b"),
            vec!["a", "b"]
        );
    }
}
