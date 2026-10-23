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
    /// Logical-clock time (ms) at which this toast expires; `None` = sticky.
    pub expires_at_ms: Option<u64>,
}

/// Ordered toast queue. Toasts are stored oldest-first; `visible()` yields
/// newest-first, capped at `max_visible`.
#[derive(Debug, Clone, PartialEq)]
pub struct ToastQueue {
    toasts: Vec<Toast>,
    next_id: ToastId,
    max_visible: usize,
    /// Current logical clock (ms). Advanced only by [`ToastQueue::tick`].
    now_ms: u64,
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
            now_ms: 0,
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
        let now = self.now_ms;
        self.add_at(kind, title, description, duration_ms, now)
    }

    /// Adds a toast created at logical time `now_ms` (also advances the clock).
    pub fn add_at(
        &mut self,
        kind: ToastKind,
        title: impl Into<String>,
        description: Option<String>,
        duration_ms: Option<u64>,
        now_ms: u64,
    ) -> ToastId {
        self.now_ms = self.now_ms.max(now_ms);
        let id = self.next_id;
        self.next_id += 1;
        self.toasts.push(Toast {
            id,
            kind,
            title: title.into(),
            description,
            duration_ms,
            expires_at_ms: duration_ms.map(|d| self.now_ms.saturating_add(d)),
        });
        id
    }

    /// Current logical clock (ms).
    pub fn now_ms(&self) -> u64 {
        self.now_ms
    }

    /// Advances the clock to `now_ms` (never backwards) and removes every
    /// toast whose `expires_at_ms <= now`. Returns the removed ids.
    /// Pure and target-independent: drive it from any timer or a test clock.
    pub fn tick(&mut self, now_ms: u64) -> Vec<ToastId> {
        self.now_ms = self.now_ms.max(now_ms);
        let now = self.now_ms;
        let mut expired = Vec::new();
        self.toasts.retain(|t| match t.expires_at_ms {
            Some(e) if e <= now => {
                expired.push(t.id);
                false
            }
            _ => true,
        });
        expired
    }

    /// Earliest pending expiry, if any toast is timed.
    pub fn next_expiry(&self) -> Option<u64> {
        self.toasts.iter().filter_map(|t| t.expires_at_ms).min()
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
