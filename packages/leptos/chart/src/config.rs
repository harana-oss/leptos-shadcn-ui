//! Chart configuration and CSS variable generation.

/// Label and color for one series.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSeriesConfig {
    pub label: String,
    pub color: String,
}

/// Ordered map of series key -> label + color.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChartConfig {
    entries: Vec<(String, ChartSeriesConfig)>,
}

impl ChartConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: adds (or replaces) a series.
    pub fn series(mut self, key: impl Into<String>, label: impl Into<String>, color: impl Into<String>) -> Self {
        self.insert(key, label, color);
        self
    }

    pub fn insert(&mut self, key: impl Into<String>, label: impl Into<String>, color: impl Into<String>) {
        let key = key.into();
        let cfg = ChartSeriesConfig { label: label.into(), color: color.into() };
        if let Some(e) = self.entries.iter_mut().find(|(k, _)| *k == key) {
            e.1 = cfg;
        } else {
            self.entries.push((key, cfg));
        }
    }

    pub fn get(&self, key: &str) -> Option<&ChartSeriesConfig> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(k, _)| k.as_str())
    }

    pub fn entries(&self) -> &[(String, ChartSeriesConfig)] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inline CSS declaring `--color-<key>: <color>;` for every series.
    pub fn css_vars(&self) -> String {
        self.entries
            .iter()
            .map(|(k, v)| format!("--color-{}: {};", sanitize_key(k), v.color.replace([';', '{', '}'], "")))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Keeps only characters valid in a CSS custom property name.
pub fn sanitize_key(key: &str) -> String {
    key.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect()
}

/// `var(--color-<key>)` reference for a series key.
pub fn color_var(key: &str) -> String {
    format!("var(--color-{})", sanitize_key(key))
}
