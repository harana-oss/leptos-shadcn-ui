//! Pure scale / geometry helpers used by the SVG charts.

/// Maps `value` from `domain` to `range` linearly. A degenerate domain maps
/// everything to `range.0`.
pub fn linear_scale(value: f64, domain: (f64, f64), range: (f64, f64)) -> f64 {
    let (d0, d1) = domain;
    let (r0, r1) = range;
    if (d1 - d0).abs() < f64::EPSILON {
        return r0;
    }
    r0 + (value - d0) / (d1 - d0) * (r1 - r0)
}

/// Rounds `max` up to a "nice" number (1, 2, 2.5, 5 or 10 times a power of ten).
pub fn nice_max(max: f64) -> f64 {
    if max <= 0.0 || !max.is_finite() {
        return 1.0;
    }
    let exp = max.log10().floor();
    let base = 10f64.powf(exp);
    let f = max / base;
    let nice = if f <= 1.0 {
        1.0
    } else if f <= 2.0 {
        2.0
    } else if f <= 2.5 {
        2.5
    } else if f <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * base
}

/// `count + 1` evenly spaced ticks from 0 to `max`.
pub fn ticks(max: f64, count: usize) -> Vec<f64> {
    let count = count.max(1);
    (0..=count).map(|i| max * i as f64 / count as f64).collect()
}

/// Band layout for `n` categories over `width` with fractional `padding`
/// (0..1) between bands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandScale {
    pub step: f64,
    pub bandwidth: f64,
    pub offset: f64,
}

impl BandScale {
    pub fn new(n: usize, width: f64, padding: f64) -> Self {
        if n == 0 {
            return Self {
                step: 0.0,
                bandwidth: 0.0,
                offset: 0.0,
            };
        }
        let padding = padding.clamp(0.0, 0.99);
        let step = width / n as f64;
        let bandwidth = step * (1.0 - padding);
        Self {
            step,
            bandwidth,
            offset: (step - bandwidth) / 2.0,
        }
    }

    /// Left x of band `i`.
    pub fn x(&self, i: usize) -> f64 {
        i as f64 * self.step + self.offset
    }

    /// Center x of band `i`.
    pub fn center(&self, i: usize) -> f64 {
        i as f64 * self.step + self.step / 2.0
    }
}

/// A computed bar rectangle.
#[derive(Debug, Clone, PartialEq)]
pub struct BarRect {
    pub series: usize,
    pub index: usize,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Grouped bar rectangles for `values[series][category]` inside `width x height`.
pub fn bar_rects(
    values: &[Vec<f64>],
    width: f64,
    height: f64,
    y_max: f64,
    padding: f64,
) -> Vec<BarRect> {
    let n = values.iter().map(|s| s.len()).max().unwrap_or(0);
    let band = BandScale::new(n, width, padding);
    let series_count = values.len().max(1);
    let bar_w = band.bandwidth / series_count as f64;
    let mut out = Vec::new();
    for (s, row) in values.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            let v = v.max(0.0);
            let y = linear_scale(v, (0.0, y_max), (height, 0.0));
            out.push(BarRect {
                series: s,
                index: i,
                x: band.x(i) + bar_w * s as f64,
                y,
                width: bar_w,
                height: height - y,
            });
        }
    }
    out
}

/// Points for a line series spread over band centers.
pub fn line_points(values: &[f64], width: f64, height: f64, y_max: f64) -> Vec<(f64, f64)> {
    let band = BandScale::new(values.len(), width, 0.0);
    values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (
                band.center(i),
                linear_scale(*v, (0.0, y_max), (height, 0.0)),
            )
        })
        .collect()
}

fn fmt(v: f64) -> String {
    let r = (v * 100.0).round() / 100.0;
    if r == r.trunc() {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

/// SVG path `d` attribute for a polyline through `points`.
pub fn line_path(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .enumerate()
        .map(|(i, (x, y))| format!("{}{},{}", if i == 0 { "M" } else { "L" }, fmt(*x), fmt(*y)))
        .collect::<Vec<_>>()
        .join(" ")
}
