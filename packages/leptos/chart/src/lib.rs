//! Leptos port of the shadcn/ui (v4) Chart component.
//!
//! `ChartContainer` turns a [`ChartConfig`] into `--color-<key>` CSS
//! variables; `BarChart` / `LineChart` render plain SVG (no JS deps) using the
//! pure helpers in [`scale`].

use leptos::prelude::*;

mod config;
pub mod scale;
pub use config::*;
pub use scale::{bar_rects, line_path, line_points, linear_scale, nice_max, ticks, BandScale, BarRect};

#[cfg(test)]
mod tests;

pub const CHART_CONTAINER_CLASS: &str = "relative flex aspect-video justify-center text-xs [&_.chart-axis-tick]:fill-muted-foreground [&_.chart-grid_line]:stroke-border/50 [&_.chart-axis-line]:stroke-border";
pub const CHART_TOOLTIP_CLASS: &str = "border-border/50 bg-background grid min-w-[8rem] items-start gap-1.5 rounded-lg border px-2.5 py-1.5 text-xs shadow-xl";
pub const CHART_TOOLTIP_LABEL_CLASS: &str = "font-medium";
pub const CHART_TOOLTIP_ITEM_CLASS: &str = "flex w-full flex-wrap items-center gap-2";
pub const CHART_TOOLTIP_VALUE_CLASS: &str = "text-foreground font-mono font-medium tabular-nums";
pub const CHART_INDICATOR_DOT_CLASS: &str = "h-2.5 w-2.5 shrink-0 rounded-[2px]";
pub const CHART_INDICATOR_LINE_CLASS: &str = "h-2.5 w-1 shrink-0 rounded-[2px]";
pub const CHART_LEGEND_CLASS: &str = "flex items-center justify-center gap-4 pt-3";
pub const CHART_LEGEND_ITEM_CLASS: &str = "[&>svg]:text-muted-foreground flex items-center gap-1.5";
pub const CHART_LEGEND_SWATCH_CLASS: &str = "h-2 w-2 shrink-0 rounded-[2px]";

pub const CHART_WIDTH: f64 = 600.0;
pub const CHART_HEIGHT: f64 = 300.0;
const MARGIN_LEFT: f64 = 40.0;
const MARGIN_RIGHT: f64 = 8.0;
const MARGIN_TOP: f64 = 8.0;
const MARGIN_BOTTOM: f64 = 24.0;
const TICK_COUNT: usize = 4;

/// One category (x value) with a value per series key.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartRow {
    pub label: String,
    pub values: Vec<(String, f64)>,
}

impl ChartRow {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), values: Vec::new() }
    }

    pub fn value(mut self, key: impl Into<String>, v: f64) -> Self {
        self.values.push((key.into(), v));
        self
    }

    pub fn get(&self, key: &str) -> f64 {
        self.values.iter().find(|(k, _)| k == key).map(|(_, v)| *v).unwrap_or(0.0)
    }
}

/// Values laid out as `[series][category]` in config key order.
pub fn series_matrix(config: &ChartConfig, data: &[ChartRow]) -> Vec<Vec<f64>> {
    config
        .keys()
        .map(|k| data.iter().map(|r| r.get(k)).collect())
        .collect()
}

/// Nice y-axis maximum across all series.
pub fn y_max(matrix: &[Vec<f64>]) -> f64 {
    nice_max(matrix.iter().flatten().copied().fold(0.0, f64::max))
}

/// Tooltip indicator style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartIndicator {
    #[default]
    Dot,
    Line,
}

/// Config provided by the nearest `ChartContainer`.
#[derive(Debug, Clone, Copy)]
pub struct ChartContext(pub Signal<ChartConfig>);

pub fn use_chart() -> ChartContext {
    use_context::<ChartContext>().expect("use_chart must be used within a ChartContainer")
}

#[component]
pub fn ChartContainer(
    #[prop(into)] config: Signal<ChartConfig>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    provide_context(ChartContext(config));
    view! {
        <div
            data-slot="chart"
            data-chart=move || id.get()
            id=move || id.get()
            class=move || format!("{} {}", CHART_CONTAINER_CLASS, class.get().unwrap_or_default())
            style=move || format!("{} {}", config.get().css_vars(), style.get().unwrap_or_default())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ChartTooltipContent(
    #[prop(into, optional)] label: MaybeProp<String>,
    /// `(series key, value)` pairs.
    #[prop(into)]
    items: Signal<Vec<(String, f64)>>,
    #[prop(into, optional)] indicator: MaybeProp<ChartIndicator>,
    #[prop(into, optional)] hide_label: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let cfg = use_context::<ChartContext>();
    let indicator_class = move || match indicator.get().unwrap_or_default() {
        ChartIndicator::Dot => CHART_INDICATOR_DOT_CLASS,
        ChartIndicator::Line => CHART_INDICATOR_LINE_CLASS,
    };
    view! {
        <div
            data-slot="chart-tooltip"
            role="tooltip"
            class=move || format!("{} {}", CHART_TOOLTIP_CLASS, class.get().unwrap_or_default())
            id=move || id.get()
            style=move || style.get()
        >
            <Show when=move || !hide_label.get().unwrap_or(false) && label.get().is_some()>
                <div class=CHART_TOOLTIP_LABEL_CLASS>{move || label.get()}</div>
            </Show>
            <div class="grid gap-1.5">
                {move || items.get().into_iter().map(|(key, value)| {
                    let name = cfg
                        .and_then(|c| c.0.get().get(&key).map(|s| s.label.clone()))
                        .unwrap_or_else(|| key.clone());
                    let color = color_var(&key);
                    view! {
                        <div class=CHART_TOOLTIP_ITEM_CLASS data-key=key.clone()>
                            <div class=indicator_class style=format!("background-color: {color}")></div>
                            <div class="flex flex-1 items-center justify-between gap-2 leading-none">
                                <span class="text-muted-foreground">{name}</span>
                                <span class=CHART_TOOLTIP_VALUE_CLASS>{value.to_string()}</span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn ChartLegendContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let cfg = use_chart();
    view! {
        <div
            data-slot="chart-legend"
            class=move || format!("{} {}", CHART_LEGEND_CLASS, class.get().unwrap_or_default())
            id=move || id.get()
            style=move || style.get()
        >
            {move || cfg.0.get().entries().iter().map(|(key, s)| {
                let color = color_var(key);
                view! {
                    <div class=CHART_LEGEND_ITEM_CLASS data-key=key.clone()>
                        <div class=CHART_LEGEND_SWATCH_CLASS style=format!("background-color: {color}")></div>
                        <span>{s.label.clone()}</span>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

fn plot_size() -> (f64, f64) {
    (CHART_WIDTH - MARGIN_LEFT - MARGIN_RIGHT, CHART_HEIGHT - MARGIN_TOP - MARGIN_BOTTOM)
}

fn axes(data: Vec<ChartRow>, ymax: f64) -> impl IntoView {
    let (w, h) = plot_size();
    let band = BandScale::new(data.len(), w, 0.0);
    view! {
        <g class="chart-grid">
            {ticks(ymax, TICK_COUNT).into_iter().map(|t| {
                let y = linear_scale(t, (0.0, ymax), (h, 0.0));
                view! {
                    <line x1="0" x2=w y1=y y2=y stroke="currentColor" stroke-opacity="0.1" stroke-dasharray="3 3"></line>
                    <text class="chart-axis-tick" x="-6" y=y dy="0.32em" text-anchor="end" fill="currentColor">{t.to_string()}</text>
                }
            }).collect_view()}
        </g>
        <g class="chart-axis">
            {data.into_iter().enumerate().map(|(i, r)| view! {
                <text class="chart-axis-tick" x=band.center(i) y=h + 16.0 text-anchor="middle" fill="currentColor">{r.label}</text>
            }).collect_view()}
        </g>
    }
}

fn tooltip_items(config: &ChartConfig, row: &ChartRow) -> Vec<(String, f64)> {
    config.keys().map(|k| (k.to_string(), row.get(k))).collect()
}

#[component]
pub fn BarChart(
    #[prop(into)] data: Signal<Vec<ChartRow>>,
    #[prop(into, optional)] show_tooltip: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let cfg = use_chart().0;
    let hovered = RwSignal::new(None::<usize>);
    let (w, h) = plot_size();
    let keys = move || cfg.get().keys().map(String::from).collect::<Vec<_>>();
    view! {
        <svg
            data-slot="chart-bar"
            viewBox=format!("0 0 {CHART_WIDTH} {CHART_HEIGHT}")
            class=move || format!("h-full w-full overflow-visible {}", class.get().unwrap_or_default())
            id=move || id.get()
            style=move || style.get()
            on:mouseleave=move |_| hovered.set(None)
        >
            <g transform=format!("translate({MARGIN_LEFT},{MARGIN_TOP})")>
                {move || {
                    let config = cfg.get();
                    let rows = data.get();
                    let m = series_matrix(&config, &rows);
                    let ymax = y_max(&m);
                    let keys = keys();
                    let band = BandScale::new(rows.len(), w, 0.0);
                    view! {
                        {axes(rows.clone(), ymax)}
                        {(0..rows.len()).map(|i| view! {
                            <rect x=band.x(i) y="0" width=band.step height=h fill="currentColor"
                                fill-opacity=move || if hovered.get() == Some(i) { "0.05" } else { "0" }
                                on:mouseenter=move |_| hovered.set(Some(i))></rect>
                        }).collect_view()}
                        {bar_rects(&m, w, h, ymax, 0.2).into_iter().map(|b| view! {
                            <rect data-series=keys[b.series].clone() x=b.x y=b.y width=b.width height=b.height rx="4"
                                fill=color_var(&keys[b.series]) pointer-events="none"></rect>
                        }).collect_view()}
                    }
                }}
            </g>
        </svg>
        <Show when=move || show_tooltip.get().unwrap_or(true) && hovered.get().is_some()>
            {move || hovered.get().and_then(|i| data.get().get(i).cloned()).map(|row| {
                let n = data.get().len().max(1);
                let left = (MARGIN_LEFT + BandScale::new(n, w, 0.0).center(hovered.get().unwrap_or(0))) / CHART_WIDTH * 100.0;
                let items = tooltip_items(&cfg.get(), &row);
                view! {
                    <div class="pointer-events-none absolute top-2" style=format!("left: {left:.2}%")>
                        <ChartTooltipContent label=row.label.clone() items=items />
                    </div>
                }
            })}
        </Show>
    }
}

#[component]
pub fn LineChart(
    #[prop(into)] data: Signal<Vec<ChartRow>>,
    #[prop(into, optional)] show_dots: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let cfg = use_chart().0;
    let (w, h) = plot_size();
    view! {
        <svg
            data-slot="chart-line"
            viewBox=format!("0 0 {CHART_WIDTH} {CHART_HEIGHT}")
            class=move || format!("h-full w-full overflow-visible {}", class.get().unwrap_or_default())
            id=move || id.get()
            style=move || style.get()
        >
            <g transform=format!("translate({MARGIN_LEFT},{MARGIN_TOP})")>
                {move || {
                    let config = cfg.get();
                    let rows = data.get();
                    let m = series_matrix(&config, &rows);
                    let ymax = y_max(&m);
                    let dots = show_dots.get().unwrap_or(false);
                    view! {
                        {axes(rows, ymax)}
                        {config.keys().zip(m.iter()).map(|(key, vals)| {
                            let pts = line_points(vals, w, h, ymax);
                            let color = color_var(key);
                            view! {
                                <path data-series=key.to_string() d=line_path(&pts) fill="none" stroke=color.clone()
                                    stroke-width="2" stroke-linejoin="round" stroke-linecap="round"></path>
                                {dots.then(|| pts.iter().map(|(x, y)| view! {
                                    <circle cx=*x cy=*y r="3" fill=color.clone()></circle>
                                }).collect_view())}
                            }
                        }).collect_view()}
                    }
                }}
            </g>
        </svg>
    }
}
