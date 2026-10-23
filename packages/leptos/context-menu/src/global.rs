//! Shared open-menu tracking for all context menus on a page.
//!
//! A single document-level listener closes whichever menu is open when the
//! user clicks elsewhere or presses Escape. Opening a menu closes any other
//! open menu, so at most one context menu is visible at a time. The listener
//! is installed once per thread instead of once per trigger, so mounting and
//! unmounting menus does not accumulate listeners.

use leptos::prelude::*;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_MENU_ID: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static OPEN_MENU: RefCell<Option<(u64, RwSignal<bool>)>> = const { RefCell::new(None) };
    static LISTENERS_INSTALLED: Cell<u32> = const { Cell::new(0) };
}

/// Allocates a unique id for a context menu instance.
pub fn next_menu_id() -> u64 {
    NEXT_MENU_ID.fetch_add(1, Ordering::Relaxed)
}

/// Marks the menu `id` as open, closing any other open menu first.
pub fn open_menu(id: u64, open: RwSignal<bool>) {
    ensure_document_listener();
    let previous = OPEN_MENU.with(|m| m.borrow_mut().replace((id, open)));
    if let Some((prev_id, prev_open)) = previous
        && prev_id != id
    {
        prev_open.try_set(false);
    }
    open.try_set(true);
}

/// Closes the menu `id` if it is the open one.
pub fn close_menu(id: u64) {
    let closed = OPEN_MENU.with(|m| {
        let mut m = m.borrow_mut();
        match *m {
            Some((open_id, _)) if open_id == id => m.take(),
            _ => None,
        }
    });
    if let Some((_, open)) = closed {
        open.try_set(false);
    }
}

/// Forgets the menu `id` without touching its signal. Used when the menu
/// unmounts and its signal may already be disposed.
pub fn release_menu(id: u64) {
    OPEN_MENU.with(|m| {
        let mut m = m.borrow_mut();
        if matches!(*m, Some((open_id, _)) if open_id == id) {
            m.take();
        }
    });
}

/// Closes whichever menu is currently open.
pub fn close_all_menus() {
    if let Some((_, open)) = OPEN_MENU.with(|m| m.borrow_mut().take()) {
        open.try_set(false);
    }
}

/// Returns the id of the open menu, if any.
pub fn open_menu_id() -> Option<u64> {
    OPEN_MENU.with(|m| m.borrow().as_ref().map(|(id, _)| *id))
}

/// Number of document listener sets installed on this thread (0 or 1).
pub fn document_listener_count() -> u32 {
    LISTENERS_INSTALLED.with(|c| c.get())
}

fn ensure_document_listener() {
    if LISTENERS_INSTALLED.with(|c| c.get()) > 0 {
        return;
    }
    LISTENERS_INSTALLED.with(|c| c.set(1));
    install_document_listener();
}

#[cfg(target_arch = "wasm32")]
fn install_document_listener() {
    use wasm_bindgen::{closure::Closure, JsCast};

    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };

    // These closures live for the lifetime of the page. They are created once
    // per thread, so leaking them with `forget` is bounded.
    let on_click = Closure::<dyn Fn(web_sys::MouseEvent)>::new(|_e: web_sys::MouseEvent| {
        close_all_menus();
    });
    let on_keydown = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(|e: web_sys::KeyboardEvent| {
        if e.key() == "Escape" {
            close_all_menus();
        }
    });
    let _ = document.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
    let _ = document.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref());
    on_click.forget();
    on_keydown.forget();
}

#[cfg(not(target_arch = "wasm32"))]
fn install_document_listener() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn reset() {
        OPEN_MENU.with(|m| m.borrow_mut().take());
    }

    #[test]
    fn opening_a_menu_closes_the_previous_one() {
        let owner = Owner::new();
        owner.with(|| {
            reset();
            let (a, b) = (RwSignal::new(false), RwSignal::new(false));
            let (id_a, id_b) = (next_menu_id(), next_menu_id());

            open_menu(id_a, a);
            assert!(a.get_untracked());
            assert_eq!(open_menu_id(), Some(id_a));

            open_menu(id_b, b);
            assert!(!a.get_untracked(), "first menu must close when second opens");
            assert!(b.get_untracked());
            assert_eq!(open_menu_id(), Some(id_b));
        });
    }

    #[test]
    fn reopening_same_menu_keeps_it_open() {
        let owner = Owner::new();
        owner.with(|| {
            reset();
            let a = RwSignal::new(false);
            let id = next_menu_id();
            open_menu(id, a);
            open_menu(id, a);
            assert!(a.get_untracked());
        });
    }

    #[test]
    fn close_menu_ignores_other_ids() {
        let owner = Owner::new();
        owner.with(|| {
            reset();
            let a = RwSignal::new(false);
            let (id_a, other) = (next_menu_id(), next_menu_id());
            open_menu(id_a, a);
            close_menu(other);
            assert!(a.get_untracked());
            close_menu(id_a);
            assert!(!a.get_untracked());
            assert_eq!(open_menu_id(), None);
        });
    }

    #[test]
    fn close_all_closes_open_menu() {
        let owner = Owner::new();
        owner.with(|| {
            reset();
            let a = RwSignal::new(false);
            open_menu(next_menu_id(), a);
            close_all_menus();
            assert!(!a.get_untracked());
            assert_eq!(open_menu_id(), None);
        });
    }

    #[test]
    fn document_listener_installed_once() {
        let owner = Owner::new();
        owner.with(|| {
            reset();
            for _ in 0..50 {
                let s = RwSignal::new(false);
                let id = next_menu_id();
                open_menu(id, s);
                close_menu(id);
            }
            assert_eq!(document_listener_count(), 1);
        });
    }

    #[test]
    fn release_tolerates_disposed_signal() {
        let owner = Owner::new();
        let id = next_menu_id();
        owner.with(|| {
            reset();
            let s = RwSignal::new(false);
            open_menu(id, s);
        });
        drop(owner);
        release_menu(id);
        assert_eq!(open_menu_id(), None);
        // Opening another menu after a disposed one must not panic.
        let owner2 = Owner::new();
        owner2.with(|| {
            open_menu(next_menu_id(), RwSignal::new(false));
            close_all_menus();
        });
    }
}
