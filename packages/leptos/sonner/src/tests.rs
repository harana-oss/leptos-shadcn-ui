use crate::*;

#[test]
fn add_returns_increasing_ids() {
    let mut q = ToastQueue::default();
    let a = q.add(ToastKind::Success, "a", None, Some(1000));
    let b = q.add(ToastKind::Error, "b", Some("desc".into()), None);
    assert!(b > a);
    assert_eq!(q.len(), 2);
    assert_eq!(q.get(b).unwrap().description.as_deref(), Some("desc"));
    assert_eq!(q.get(a).unwrap().kind, ToastKind::Success);
}

#[test]
fn dismiss_removes_only_target() {
    let mut q = ToastQueue::default();
    let a = q.add(ToastKind::Info, "a", None, None);
    let b = q.add(ToastKind::Info, "b", None, None);
    assert!(q.dismiss(a));
    assert!(!q.dismiss(a));
    assert_eq!(q.len(), 1);
    assert!(q.get(b).is_some());
    assert_eq!(q.dismiss_all(), vec![b]);
    assert!(q.is_empty());
}

#[test]
fn visible_is_newest_first_and_capped() {
    let mut q = ToastQueue::new(2);
    let ids: Vec<_> = (0..4)
        .map(|i| q.add(ToastKind::Message, format!("t{i}"), None, None))
        .collect();
    let vis: Vec<_> = q.visible().into_iter().map(|t| t.id).collect();
    assert_eq!(vis, vec![ids[3], ids[2]]);
    q.dismiss(ids[3]);
    let vis: Vec<_> = q.visible().into_iter().map(|t| t.id).collect();
    assert_eq!(vis, vec![ids[2], ids[1]]);
    q.set_max_visible(10);
    assert_eq!(q.visible().len(), 3);
}

#[test]
fn ids_not_reused_after_dismiss() {
    let mut q = ToastQueue::default();
    let a = q.add(ToastKind::Message, "a", None, None);
    q.dismiss(a);
    let b = q.add(ToastKind::Message, "b", None, None);
    assert_ne!(a, b);
}

#[test]
fn kind_and_position_strings() {
    assert_eq!(ToastKind::default().as_str(), "default");
    assert_eq!(ToastKind::Warning.as_str(), "warning");
    assert_eq!(ToasterPosition::default().as_str(), "bottom-right");
    assert_eq!(ToasterPosition::TopCenter.as_str(), "top-center");
    assert!(ToasterPosition::TopLeft.class().contains("left-4"));
    assert!(ToasterPosition::BottomCenter.class().contains("bottom-4"));
}

#[test]
fn context_api_returns_ids_and_dismisses() {
    let owner = Owner::new();
    owner.with(|| {
        let ctx = ToasterContext::new();
        let s = ctx.success("ok");
        let e = ctx.error("bad");
        let i = ctx.info("fyi");
        let w = ctx.warning("careful");
        let m = ctx.message("hi");
        assert_eq!(ctx.queue.get_untracked().len(), 5);
        assert_eq!(
            ctx.queue.get_untracked().get(w).unwrap().duration_ms,
            Some(DEFAULT_DURATION_MS)
        );
        assert!(ctx.dismiss(s));
        assert!(!ctx.dismiss(s));
        let sticky = ctx.toast(
            ToastKind::Info,
            "sticky",
            ToastOptions { description: None, duration_ms: Some(0) },
        );
        assert_eq!(ctx.queue.get_untracked().get(sticky).unwrap().duration_ms, None);
        let _ = (e, i, m);
        ctx.dismiss_all();
        assert!(ctx.queue.get_untracked().is_empty());
    });
}

#[test]
fn tick_prunes_only_expired_toasts() {
    let mut q = ToastQueue::default();
    let a = q.add_at(ToastKind::Info, "a", None, Some(1000), 0);
    let b = q.add_at(ToastKind::Info, "b", None, Some(3000), 500);
    let sticky = q.add_at(ToastKind::Info, "s", None, None, 500);
    assert_eq!(q.get(a).unwrap().expires_at_ms, Some(1000));
    assert_eq!(q.get(b).unwrap().expires_at_ms, Some(3500));
    assert_eq!(q.next_expiry(), Some(1000));
    assert!(q.tick(999).is_empty());
    assert_eq!(q.tick(1000), vec![a]);
    assert_eq!(q.next_expiry(), Some(3500));
    assert!(q.tick(3499).is_empty());
    assert_eq!(q.tick(10_000), vec![b]);
    assert_eq!(q.len(), 1);
    assert!(q.get(sticky).is_some());
    assert_eq!(q.next_expiry(), None);
}

#[test]
fn tick_clock_is_monotonic_and_add_uses_it() {
    let mut q = ToastQueue::default();
    q.tick(5000);
    assert!(q.tick(100).is_empty());
    assert_eq!(q.now_ms(), 5000);
    let id = q.add(ToastKind::Message, "x", None, Some(200));
    assert_eq!(q.get(id).unwrap().expires_at_ms, Some(5200));
    // A stale add_at timestamp cannot move the clock backwards.
    let late = q.add_at(ToastKind::Message, "y", None, Some(200), 10);
    assert_eq!(q.get(late).unwrap().expires_at_ms, Some(5200));
    assert_eq!(q.tick(5200), vec![id, late]);
}

#[test]
fn context_tick_auto_dismisses_with_mock_clock() {
    let owner = Owner::new();
    owner.with(|| {
        let ctx = ToasterContext::new();
        let quick = ctx.toast(
            ToastKind::Success,
            "quick",
            ToastOptions { description: None, duration_ms: Some(100) },
        );
        let default = ctx.info("default");
        let sticky = ctx.toast(
            ToastKind::Info,
            "sticky",
            ToastOptions { description: None, duration_ms: Some(0) },
        );
        assert!(ctx.tick(99).is_empty());
        assert_eq!(ctx.tick(100), vec![quick]);
        assert_eq!(ctx.tick(DEFAULT_DURATION_MS), vec![default]);
        let q = ctx.queue.get_untracked();
        assert_eq!(q.len(), 1);
        assert!(q.get(sticky).is_some());
    });
}
