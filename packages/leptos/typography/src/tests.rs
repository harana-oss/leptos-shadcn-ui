use crate::default::*;
use leptos::prelude::*;

#[test]
fn classes() {
    assert!(TYPOGRAPHY_H1_CLASS.contains("text-4xl"));
    assert!(TYPOGRAPHY_H2_CLASS.contains("border-b"));
    assert!(TYPOGRAPHY_INLINE_CODE_CLASS.contains("bg-muted"));
    assert!(TYPOGRAPHY_LEAD_CLASS.contains("text-muted-foreground"));
    assert!(TYPOGRAPHY_MUTED_CLASS.contains("text-muted-foreground"));
}

#[test]
fn renders_html() {
    let html = view! {
        <div>
            <TypographyH1>"a"</TypographyH1><TypographyH2>"b"</TypographyH2>
            <TypographyH3>"c"</TypographyH3><TypographyH4>"d"</TypographyH4>
            <TypographyP>"e"</TypographyP><TypographyBlockquote>"f"</TypographyBlockquote>
            <TypographyList><li>"g"</li></TypographyList><TypographyInlineCode>"h"</TypographyInlineCode>
            <TypographyLead>"i"</TypographyLead><TypographyLarge>"j"</TypographyLarge>
            <TypographySmall>"k"</TypographySmall><TypographyMuted class="extra">"l"</TypographyMuted>
        </div>
    }.to_html();
    for t in [
        "<h1",
        "<h2",
        "<h3",
        "<h4",
        "<blockquote",
        "<ul",
        "<code",
        "<small",
    ] {
        assert!(html.contains(t), "{t}");
    }
    assert!(html.contains(r#"data-slot="typography-inline-code""#));
    assert!(html.contains("text-sm extra"));
}
