//! Memory management utilities for signal lifecycle

use leptos::prelude::*;
use leptos::reactive::graph::{AnySource, ToAnySource};
use std::collections::{HashMap, HashSet};

use crate::error::SignalManagementError;

/// Current time in milliseconds since the Unix epoch.
///
/// Uses `js_sys::Date::now()` on wasm32 and `SystemTime` elsewhere, since
/// wasm-bindgen imports panic when called on native targets.
pub(crate) fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }
}

/// Memory usage statistics for signal management
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryStats {
    /// Number of active signals
    pub active_signals: usize,
    /// Number of active memos
    pub active_memos: usize,
    /// Estimated memory usage in bytes
    pub estimated_memory_bytes: usize,
    /// Number of tracked signal groups
    pub tracked_groups: usize,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            active_signals: 0,
            active_memos: 0,
            estimated_memory_bytes: 0,
            tracked_groups: 0,
        }
    }
}

/// Name of the group used by `add_signal` / `add_memo` when no group is active
pub const DEFAULT_GROUP_NAME: &str = "default";

/// Memory manager for tracking and managing signal memory usage
#[derive(Debug, Clone)]
pub struct SignalMemoryManager {
    /// Tracked signal groups
    pub tracked_groups: ArcRwSignal<HashMap<String, SignalGroup>>,
    /// Memory statistics
    pub stats: ArcRwSignal<MemoryStats>,
    /// Maximum memory usage threshold
    pub max_memory_bytes: usize,
    /// Memory limit for pressure detection
    pub memory_limit: usize,
    /// Adaptive management enabled flag
    pub adaptive_management: bool,
    /// Group targeted by `add_signal` / `add_memo`: the most recently created group
    current_group: ArcRwSignal<Option<String>>,
}

/// A type-erased, strongly held reference to a tracked signal or memo.
///
/// Equality is the identity of the underlying reactive node, so the same
/// signal added twice is tracked once. Holding a strong reference keeps the
/// node alive while tracked, so its identity cannot be reused by a new signal.
#[derive(Clone)]
pub struct TrackedItem {
    source: AnySource,
    _keep_alive: std::sync::Arc<dyn std::any::Any + Send + Sync>,
}

impl TrackedItem {
    fn new<S>(item: S) -> Self
    where
        S: ToAnySource + Send + Sync + 'static,
    {
        Self {
            source: item.to_any_source(),
            _keep_alive: std::sync::Arc::new(item),
        }
    }
}

impl PartialEq for TrackedItem {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for TrackedItem {}

impl std::fmt::Debug for TrackedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrackedItem").field("source", &self.source).finish()
    }
}

/// A group of related signals that can be managed together
#[derive(Debug, Clone)]
pub struct SignalGroup {
    /// Group name
    pub name: String,
    /// Signals in this group (deduplicated by identity)
    pub signals: Vec<TrackedItem>,
    /// Memos in this group (deduplicated by identity)
    pub memos: Vec<TrackedItem>,
    /// Created timestamp
    pub created_at: f64,
    /// Identity index of `signals`, for O(1) deduplication
    signal_index: HashSet<AnySource>,
    /// Identity index of `memos`, for O(1) deduplication
    memo_index: HashSet<AnySource>,
}

/// Insert `item` unless an item with the same identity is already tracked.
fn insert_unique(list: &mut Vec<TrackedItem>, index: &mut HashSet<AnySource>, item: TrackedItem) {
    // `list` is public and may have been modified directly; resync if so.
    if index.len() != list.len() {
        *index = list.iter().map(|i| i.source.clone()).collect();
    }
    if index.insert(item.source.clone()) {
        list.push(item);
    }
}

fn remove_by_source(list: &mut Vec<TrackedItem>, index: &mut HashSet<AnySource>, source: &AnySource) -> bool {
    index.remove(source);
    let before = list.len();
    list.retain(|item| &item.source != source);
    before != list.len()
}

impl SignalGroup {
    /// Create a new signal group
    pub fn new(name: String) -> Self {
        Self::with_timestamp(name, now_ms())
    }

    /// Add a signal to this group. Adding the same signal twice is a no-op.
    pub fn add_signal<T: Send + Sync + 'static>(&mut self, signal: ArcRwSignal<T>) -> ArcRwSignal<T> {
        insert_unique(&mut self.signals, &mut self.signal_index, TrackedItem::new(signal.clone()));
        signal
    }

    /// Add a memo to this group. Adding the same memo twice is a no-op.
    pub fn add_memo<T: Send + Sync + 'static>(&mut self, memo: ArcMemo<T>) -> ArcMemo<T> {
        insert_unique(&mut self.memos, &mut self.memo_index, TrackedItem::new(memo.clone()));
        memo
    }

    /// Remove a specific signal (by identity). Returns true if it was tracked.
    pub fn remove_tracked_signal<T: Send + Sync + 'static>(&mut self, signal: &ArcRwSignal<T>) -> bool {
        remove_by_source(&mut self.signals, &mut self.signal_index, &signal.to_any_source())
    }

    /// Remove a specific memo (by identity). Returns true if it was tracked.
    pub fn remove_tracked_memo<T: Send + Sync + 'static>(&mut self, memo: &ArcMemo<T>) -> bool {
        remove_by_source(&mut self.memos, &mut self.memo_index, &memo.to_any_source())
    }

    /// Get the number of signals in this group
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Get the number of memos in this group
    pub fn memo_count(&self) -> usize {
        self.memos.len()
    }

    /// Get the total count of tracked items
    pub fn total_count(&self) -> usize {
        self.signal_count() + self.memo_count()
    }

    /// Check if this group is empty
    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }

    /// Clear all signals and memos from this group
    pub fn clear(&mut self) {
        self.signals.clear();
        self.memos.clear();
        self.signal_index.clear();
        self.memo_index.clear();
    }

    /// Remove a signal from this group
    pub fn remove_signal(&mut self, index: usize) -> Option<()> {
        if index < self.signals.len() {
            let removed = self.signals.remove(index);
            self.signal_index.remove(&removed.source);
            Some(())
        } else {
            None
        }
    }

    /// Remove a memo from this group
    pub fn remove_memo(&mut self, index: usize) -> Option<()> {
        if index < self.memos.len() {
            let removed = self.memos.remove(index);
            self.memo_index.remove(&removed.source);
            Some(())
        } else {
            None
        }
    }

    /// Create a group with timestamp
    pub fn with_timestamp(name: String, timestamp: f64) -> Self {
        Self {
            name,
            signals: Vec::new(),
            memos: Vec::new(),
            created_at: timestamp,
            signal_index: HashSet::new(),
            memo_index: HashSet::new(),
        }
    }
}

impl SignalMemoryManager {
    fn build(max_memory_bytes: usize, memory_limit: usize, adaptive_management: bool) -> Self {
        Self {
            tracked_groups: ArcRwSignal::new(HashMap::new()),
            stats: ArcRwSignal::new(MemoryStats::default()),
            max_memory_bytes,
            memory_limit,
            adaptive_management,
            current_group: ArcRwSignal::new(None),
        }
    }

    /// Create a new memory manager
    pub fn new() -> Self {
        Self::build(10 * 1024 * 1024, 10 * 1024 * 1024, false)
    }

    /// Create a new memory manager with custom memory limit
    pub fn with_memory_limit(max_memory_bytes: usize) -> Self {
        Self::build(max_memory_bytes, max_memory_bytes, false)
    }

    /// Create a new memory manager with adaptive management enabled
    pub fn with_adaptive_management() -> Self {
        Self::build(10 * 1024 * 1024, 10 * 1024 * 1024, true)
    }

    /// Create a manager with custom limits
    pub fn with_limits(max_memory_bytes: usize, memory_limit: usize) -> Self {
        Self::build(max_memory_bytes, memory_limit, false)
    }

    /// Create a new signal group (or reuse an existing one with this name).
    ///
    /// The group becomes the active group: subsequent `add_signal` /
    /// `add_memo` calls add to it until another group is created or it is removed.
    pub fn create_group(&self, name: String) -> Result<String, SignalManagementError> {
        self.tracked_groups.update(|groups| {
            groups
                .entry(name.clone())
                .or_insert_with(|| SignalGroup::new(name.clone()));
        });
        self.current_group.set(Some(name.clone()));

        self.update_stats();
        Ok(name)
    }

    /// Name of the group `add_signal` / `add_memo` currently target
    pub fn active_group_name(&self) -> String {
        let current = self.current_group.get_untracked();
        match current {
            Some(name) if self.tracked_groups.with_untracked(|g| g.contains_key(&name)) => name,
            _ => DEFAULT_GROUP_NAME.to_string(),
        }
    }

    fn forget_current_if(&self, removed: impl Fn(&str) -> bool) {
        let clear = self
            .current_group
            .with_untracked(|c| c.as_deref().map(|n| removed(n)).unwrap_or(false));
        if clear {
            self.current_group.set(None);
        }
    }

    /// Add a signal to a group
    pub fn add_signal_to_group<T: Send + Sync + 'static>(
        &self,
        group_name: &str,
        signal: ArcRwSignal<T>,
    ) -> Result<ArcRwSignal<T>, SignalManagementError> {
        let signal_clone = signal.clone();
        self.tracked_groups.update(|groups| {
            groups
                .entry(group_name.to_string())
                .or_insert_with(|| SignalGroup::new(group_name.to_string()))
                .add_signal(signal);
        });

        self.update_stats();
        Ok(signal_clone)
    }

    /// Add a memo to a group
    pub fn add_memo_to_group<T: Send + Sync + 'static>(
        &self,
        group_name: &str,
        memo: ArcMemo<T>,
    ) -> Result<ArcMemo<T>, SignalManagementError> {
        let memo_clone = memo.clone();
        self.tracked_groups.update(|groups| {
            groups
                .entry(group_name.to_string())
                .or_insert_with(|| SignalGroup::new(group_name.to_string()))
                .add_memo(memo);
        });

        self.update_stats();
        Ok(memo_clone)
    }

    /// Remove a signal group
    pub fn remove_group(&self, group_name: &str) -> Result<(), SignalManagementError> {
        self.tracked_groups.update(|groups| {
            groups.remove(group_name);
        });
        self.forget_current_if(|n| n == group_name);

        self.update_stats();
        Ok(())
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        self.stats.get()
    }

    /// Get the number of tracked groups
    pub fn group_count(&self) -> usize {
        self.tracked_groups.with(|groups| groups.len())
    }

    /// Get the maximum memory limit
    pub fn max_memory_bytes(&self) -> usize {
        self.max_memory_bytes
    }

    /// Check if memory usage is within limits
    pub fn is_memory_within_limits(&self) -> bool {
        self.stats.with(|s| s.estimated_memory_bytes) <= self.max_memory_bytes
    }

    /// Get memory usage percentage
    pub fn memory_usage_percentage(&self) -> f64 {
        let used = self.stats.with(|s| s.estimated_memory_bytes);
        (used as f64 / self.max_memory_bytes as f64) * 100.0
    }

    /// Cleanup empty groups
    pub fn cleanup_empty_groups(&self) -> Result<usize, SignalManagementError> {
        let mut removed_count = 0;

        self.tracked_groups.update(|groups| {
            groups.retain(|_, group| {
                if group.is_empty() {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
        });
        self.forget_removed_current();

        self.update_stats();
        Ok(removed_count)
    }

    pub(crate) fn forget_removed_current(&self) {
        let groups = &self.tracked_groups;
        self.forget_current_if(|n| !groups.with_untracked(|g| g.contains_key(n)));
    }

    /// Force cleanup of all groups
    pub fn force_cleanup_all(&self) -> Result<(), SignalManagementError> {
        self.tracked_groups.set(HashMap::new());
        self.current_group.set(None);
        self.update_stats();
        Ok(())
    }

    /// Update memory statistics
    pub(crate) fn update_stats(&self) {
        let mut stats = self.tracked_groups.with_untracked(|groups| {
            let mut stats = MemoryStats::default();
            for group in groups.values() {
                stats.active_signals += group.signal_count();
                stats.active_memos += group.memo_count();
            }
            stats.tracked_groups = groups.len();
            stats
        });
        stats.estimated_memory_bytes = self.estimate_memory_usage(&stats);

        self.stats.set(stats);
    }

    /// Estimate memory usage based on statistics
    fn estimate_memory_usage(&self, stats: &MemoryStats) -> usize {
        // Rough estimation: each signal/memo uses approximately 1KB
        let base_usage = (stats.active_signals + stats.active_memos) * 1024;

        // Add overhead for tracking
        let overhead = stats.tracked_groups * 512;

        base_usage + overhead
    }

    /// Get total number of signals across all groups
    pub fn total_signals(&self) -> usize {
        self.tracked_groups.with(|groups| {
            groups.values().map(|group| group.signal_count()).sum()
        })
    }

    /// Get total number of memos across all groups
    pub fn total_memos(&self) -> usize {
        self.tracked_groups.with(|groups| {
            groups.values().map(|group| group.memo_count()).sum()
        })
    }

    /// Get memory usage in KB
    pub fn memory_usage_kb(&self) -> f64 {
        self.stats.with(|s| s.estimated_memory_bytes) as f64 / 1024.0
    }

    /// Add a signal to the active group (the most recently created group,
    /// or the default group if none is active)
    pub fn add_signal<T: Send + Sync + 'static>(&self, signal: ArcRwSignal<T>) -> Result<ArcRwSignal<T>, SignalManagementError> {
        let group = self.active_group_name();
        self.add_signal_to_group(&group, signal)
    }

    /// Add a memo to the active group (the most recently created group,
    /// or the default group if none is active)
    pub fn add_memo<T: Send + Sync + 'static>(&self, memo: ArcMemo<T>) -> Result<ArcMemo<T>, SignalManagementError> {
        let group = self.active_group_name();
        self.add_memo_to_group(&group, memo)
    }

    /// Remove a signal from tracking in every group that holds it
    pub fn remove_signal<T: Send + Sync + 'static>(&self, signal: &ArcRwSignal<T>) -> Result<(), SignalManagementError> {
        self.tracked_groups.update(|groups| {
            for group in groups.values_mut() {
                group.remove_tracked_signal(signal);
            }
        });
        self.update_stats();
        Ok(())
    }

    /// Remove a memo from tracking in every group that holds it
    pub fn remove_memo<T: Send + Sync + 'static>(&self, memo: &ArcMemo<T>) -> Result<(), SignalManagementError> {
        self.tracked_groups.update(|groups| {
            for group in groups.values_mut() {
                group.remove_tracked_memo(memo);
            }
        });
        self.update_stats();
        Ok(())
    }

    /// Cleanup a specific group
    pub fn cleanup_group(&self, group_name: &str) -> Result<(), SignalManagementError> {
        self.remove_group(group_name)
    }

    /// Cleanup all groups
    pub fn cleanup_all(&self) -> Result<(), SignalManagementError> {
        self.force_cleanup_all()
    }

    /// Adaptive cleanup for low priority (empty) groups
    pub fn cleanup_low_priority_groups(&self) -> Result<(), SignalManagementError> {
        self.cleanup_empty_groups().map(|_| ())
    }

    /// Evict the oldest groups until estimated usage is within `memory_limit`.
    /// Returns the number of groups evicted.
    pub(crate) fn evict_until_within_limit(&self) -> usize {
        let mut evicted = 0;
        while self.stats.with_untracked(|s| s.estimated_memory_bytes) > self.memory_limit {
            let oldest = self.tracked_groups.with_untracked(|groups| {
                groups
                    .values()
                    .min_by(|a, b| {
                        a.created_at
                            .partial_cmp(&b.created_at)
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then_with(|| a.name.cmp(&b.name))
                    })
                    .map(|g| g.name.clone())
            });
            match oldest {
                Some(name) => {
                    let _ = self.remove_group(&name);
                    evicted += 1;
                }
                None => break,
            }
        }
        evicted
    }

    /// Adaptive cleanup based on memory pressure.
    ///
    /// When estimated usage exceeds `memory_limit`, empty groups are removed
    /// first, then the oldest groups are evicted until usage is within the limit.
    pub fn adaptive_cleanup(&self) -> Result<(), SignalManagementError> {
        if self.stats.with_untracked(|s| s.estimated_memory_bytes) <= self.memory_limit {
            return Ok(());
        }
        self.cleanup_low_priority_groups()?;
        self.evict_until_within_limit();
        Ok(())
    }

    /// Update memory statistics
    pub fn update_memory_stats(&self) -> Result<(), SignalManagementError> {
        self.update_stats();
        Ok(())
    }

    /// Get memory statistics
    pub fn get_memory_stats(&self) -> MemoryStats {
        self.stats.get()
    }

    /// Get a specific group by name
    pub fn get_group(&self, group_name: &str) -> Option<SignalGroup> {
        self.tracked_groups.with(|groups| {
            groups.get(group_name).cloned()
        })
    }
}

impl Default for SignalMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory leak detector for signal management
pub struct MemoryLeakDetector {
    /// Baseline memory stats
    baseline_stats: MemoryStats,
    /// Current memory stats
    current_stats: ArcRwSignal<MemoryStats>,
    /// Memory growth threshold
    growth_threshold: f64,
    /// Leak prevention enabled flag
    pub leak_prevention_enabled: bool,
}

impl MemoryLeakDetector {
    /// Create a new memory leak detector
    pub fn new() -> Self {
        Self {
            baseline_stats: MemoryStats::default(),
            current_stats: ArcRwSignal::new(MemoryStats::default()),
            growth_threshold: 0.1, // 10% growth threshold
            leak_prevention_enabled: false,
        }
    }
    
    /// Create a new memory leak detector with custom threshold
    pub fn with_threshold(growth_threshold: f64) -> Self {
        Self {
            baseline_stats: MemoryStats::default(),
            current_stats: ArcRwSignal::new(MemoryStats::default()),
            growth_threshold,
            leak_prevention_enabled: false,
        }
    }
    
    /// Set baseline memory stats
    pub fn set_baseline(&mut self, stats: MemoryStats) {
        self.baseline_stats = stats;
    }
    
    /// Update current memory stats
    pub fn update_current(&self, stats: MemoryStats) {
        self.current_stats.set(stats);
    }
    
    /// Check for memory leaks
    pub fn check_for_leaks(&self) -> Result<bool, SignalManagementError> {
        let current = self.current_stats.get();
        
        // Check if memory usage has grown significantly
        let memory_growth = if self.baseline_stats.estimated_memory_bytes > 0 {
            (current.estimated_memory_bytes as f64 - self.baseline_stats.estimated_memory_bytes as f64)
                / self.baseline_stats.estimated_memory_bytes as f64
        } else {
            0.0
        };
        
        Ok(memory_growth > self.growth_threshold)
    }
    
    /// Get memory growth percentage
    pub fn memory_growth_percentage(&self) -> f64 {
        let current = self.current_stats.get();
        
        if self.baseline_stats.estimated_memory_bytes > 0 {
            ((current.estimated_memory_bytes as f64 - self.baseline_stats.estimated_memory_bytes as f64)
                / self.baseline_stats.estimated_memory_bytes as f64) * 100.0
        } else {
            0.0
        }
    }
    
    /// Get the growth threshold
    pub fn growth_threshold(&self) -> f64 {
        self.growth_threshold
    }
}

impl Default for MemoryLeakDetector {
    fn default() -> Self {
        Self::new()
    }
}
