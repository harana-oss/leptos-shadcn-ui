use crate::default::*;
use leptos::prelude::*;

#[test]
fn variants() {
    assert_eq!(EmptyMediaVariant::default(), EmptyMediaVariant::Default);
    assert_eq!(EmptyMediaVariant::Default.class(), "bg-transparent");
    assert!(EmptyMediaVariant::Icon.class().contains("bg-muted"));
    assert_eq!(EmptyMediaVariant::Icon.as_str(), "icon");
    assert!(empty_media_class(EmptyMediaVariant::Icon, Some("z".into())).ends_with("size-6 z"));
}

#[test]
fn tokens() {
    assert!(EMPTY_CLASS.contains("border-dashed"));
    assert!(EMPTY_DESCRIPTION_CLASS.contains("text-muted-foreground"));
}

#[test]
fn renders_html() {
    let html = view! {
        <Empty><EmptyHeader><EmptyMedia variant=EmptyMediaVariant::Icon>"i"</EmptyMedia>
        <EmptyTitle>"No data"</EmptyTitle><EmptyDescription>"d"</EmptyDescription></EmptyHeader>
        <EmptyContent>"c"</EmptyContent></Empty>
    }.to_html();
    for s in ["empty", "empty-header", "empty-icon", "empty-title", "empty-description", "empty-content"] {
        assert!(html.contains(&format!("data-slot=\"{s}\"")), "{s}");
    }
    assert!(html.contains(r#"data-variant="icon""#));
}
