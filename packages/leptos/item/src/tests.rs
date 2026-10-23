use crate::default::*;
use leptos::prelude::*;

#[test]
fn variants() {
    assert_eq!(ItemVariant::default(), ItemVariant::Default);
    assert_eq!(ItemVariant::Default.class(), "bg-transparent");
    assert_eq!(ItemVariant::Outline.class(), "border-border");
    assert_eq!(ItemVariant::Muted.class(), "bg-muted/50");
    assert_eq!(ItemVariant::Muted.as_str(), "muted");
}

#[test]
fn sizes() {
    assert_eq!(ItemSize::default(), ItemSize::Default);
    assert_eq!(ItemSize::Default.class(), "p-4 gap-4");
    assert_eq!(ItemSize::Sm.class(), "py-3 px-4 gap-2.5");
    assert_eq!(ItemSize::Sm.as_str(), "sm");
}

#[test]
fn composed_class() {
    let c = item_class(ItemVariant::Outline, ItemSize::Sm, Some("x".into()));
    assert!(c.starts_with(ITEM_CLASS));
    assert!(c.contains("border-border"));
    assert!(c.contains("py-3"));
    assert!(c.ends_with(" x"));
    assert!(item_media_class(ItemMediaVariant::Icon, None).contains("bg-muted"));
}

#[test]
fn renders_html() {
    let html = view! {
        <ItemGroup>
            <Item variant=ItemVariant::Outline size=ItemSize::Sm>
                <ItemHeader>"h"</ItemHeader>
                <ItemMedia variant=ItemMediaVariant::Icon>"m"</ItemMedia>
                <ItemContent><ItemTitle>"t"</ItemTitle><ItemDescription>"d"</ItemDescription></ItemContent>
                <ItemActions>"a"</ItemActions>
                <ItemFooter>"f"</ItemFooter>
            </Item>
            <ItemSeparator/>
        </ItemGroup>
    }.to_html();
    for s in ["item-group", "item", "item-header", "item-media", "item-content", "item-title", "item-description", "item-actions", "item-footer", "item-separator"] {
        assert!(html.contains(&format!("data-slot=\"{s}\"")), "{s}");
    }
    assert!(html.contains(r#"data-variant="outline""#));
    assert!(html.contains(r#"data-size="sm""#));
}
