//! Regression tests for cloud-shuttle/leptos-shadcn-ui#5: component classes
//! render in HTML but are missing from the compiled CSS because Tailwind never
//! scans the component crates.

use leptos_shadcn_tailwind::{discover_sources, render_sources_css};
use std::fs;
use std::path::Path;

fn read_rs_files(dir: &Path, out: &mut String) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            read_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push_str(&fs::read_to_string(&path).unwrap());
        }
    }
}

#[test]
fn discovers_button_crate_sources() {
    let sources = discover_sources().expect("cargo metadata");
    let button = sources
        .iter()
        .find(|s| s.name == "leptos-shadcn-button")
        .expect("button crate is in the dependency graph");
    assert!(button.src_dir.is_dir(), "{} must exist", button.src_dir.display());
}

#[test]
fn scanned_sources_contain_outline_button_classes() {
    // The exact variant from the issue report.
    let sources = discover_sources().unwrap();
    let mut all = String::new();
    for s in &sources {
        read_rs_files(&s.src_dir, &mut all);
    }
    for class in [
        "border-input",
        "bg-background",
        "hover:bg-accent",
        "hover:text-accent-foreground",
    ] {
        assert!(all.contains(class), "class {class} must be visible to Tailwind");
    }
}

#[test]
fn every_component_crate_is_covered() {
    let sources = discover_sources().unwrap();
    let css = render_sources_css(&sources);
    for name in [
        "leptos-shadcn-button",
        "leptos-shadcn-input",
        "leptos-shadcn-card",
        "leptos-shadcn-context-menu",
    ] {
        let s = sources
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("{name} missing"));
        let path = s.src_dir.to_string_lossy().replace('\\', "/");
        assert!(css.contains(&format!("@source \"{path}\";")), "{name} not in generated css");
    }
}

#[test]
fn write_is_idempotent() {
    let dir = std::env::temp_dir().join(format!("shadcn-tw-{}", std::process::id()));
    let file = dir.join("shadcn-sources.css");
    let written = leptos_shadcn_tailwind::write_sources_css(&file).unwrap();
    let first = fs::read_to_string(&written).unwrap();
    let mtime = fs::metadata(&written).unwrap().modified().unwrap();
    leptos_shadcn_tailwind::write_sources_css(&file).unwrap();
    assert_eq!(first, fs::read_to_string(&written).unwrap());
    assert_eq!(mtime, fs::metadata(&written).unwrap().modified().unwrap());
    fs::remove_dir_all(dir).unwrap();
}
