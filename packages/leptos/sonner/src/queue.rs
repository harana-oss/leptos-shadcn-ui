//! Pure, framework-agnostic toast queue used by the `Toaster`.

/// Identifier returned by every toast call.
pub type ToastId = u64;

/// Kind of toast; drives the icon and `data-type` attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastKind {
    #[default]
    Message,
    Success,
    Error,
    Info,
    Warning,
}

impl ToastKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToastKind::Message => "default",
            ToastKind::Success => "success",
            ToastKind::Error => "error",
            ToastKind::Info => "info",
            ToastKind::Warning => "warning",
        }
    }
}

/// A single toast entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub id: ToastId,
    pub kind: ToastKind,
    pub title: String,
    pub description: Option<String>,
    /// Auto-dismiss after this many milliseconds; `None` keeps it open.
    pub duration_ms: Option<u64>,
}

/// Ordered toast queue. Toasts are stored oldest-first; `visible()` yields
/// newest-first, capped at `max_visible`.
#[derive(Debug, Clone, PartialEq)]
pub struct ToastQueue {
    toasts: Vec<Toast>,
    next_id: ToastId,
    max_visible: usize,
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_VISIBLE)
    }
}

pub const DEFAULT_MAX_VISIBLE: usize = 3;
pub const DEFAULT_DURATION_MS: u64 = 4000;

impl ToastQueue {
    pub fn new(max_visible: usize) -> Self {
        Self {
            toasts: Vec::new(),
            next_id: 1,
            max_visible,
        }
    }

    pub fn max_visible(&self) -> usize {
        self.max_visible
    }

    pub fn set_max_visible(&mut self, max: usize) {
        self.max_visible = max;
    }

    /// Adds a toast and returns its id.
    pub fn add(
        &mut self,
        kind: ToastKind,
        title: impl Into<String>,
        description: Option<String>,
        duration_ms: Option<u64>,
    ) -> ToastId {
        let id = self.next_id;
        self.next_id += 1;
        self.toasts.push(Toast {
            id,
            kind,
            title: title.into(),
            description,
            duration_ms,
        });
        id
    }

    /// Removes a toast; returns whether it existed.
    pub fn dismiss(&mut self, id: ToastId) -> bool {
        let before = self.toasts.len();
        self.toasts.retain(|t| t.id != id);
        before != self.toasts.len()
    }

    /// Removes all toasts and returns their ids.
    pub fn dismiss_all(&mut self) -> Vec<ToastId> {
        self.toasts.drain(..).map(|t| t.id).collect()
    }

    pub fn get(&self, id: ToastId) -> Option<&Toast> {
        self.toasts.iter().find(|t| t.id == id)
    }

    pub fn len(&self) -> usize {
        self.toasts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    /// Newest-first, at most `max_visible` toasts.
    pub fn visible(&self) -> Vec<Toast> {
        self.toasts
            .iter()
            .rev()
            .take(self.max_visible)
            .cloned()
            .collect()
    }
}
