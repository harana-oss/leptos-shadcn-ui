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
