use crate::*;

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn linear_scale_maps_and_inverts() {
    assert!(approx(linear_scale(5.0, (0.0, 10.0), (0.0, 100.0)), 50.0));
    assert!(approx(linear_scale(0.0, (0.0, 10.0), (200.0, 0.0)), 200.0));
    assert!(approx(linear_scale(10.0, (0.0, 10.0), (200.0, 0.0)), 0.0));
    assert!(approx(linear_scale(3.0, (2.0, 2.0), (7.0, 9.0)), 7.0));
}

#[test]
fn nice_max_rounds_up() {
    assert_eq!(nice_max(0.0), 1.0);
    assert_eq!(nice_max(7.0), 10.0);
    assert_eq!(nice_max(186.0), 200.0);
    assert_eq!(nice_max(240.0), 250.0);
    assert_eq!(nice_max(305.0), 500.0);
    assert_eq!(nice_max(1000.0), 1000.0);
}

#[test]
fn ticks_evenly_spaced() {
    assert_eq!(ticks(100.0, 4), vec![0.0, 25.0, 50.0, 75.0, 100.0]);
    assert_eq!(ticks(10.0, 0), vec![0.0, 10.0]);
}

#[test]
fn band_scale_layout() {
    let b = BandScale::new(4, 400.0, 0.2);
    assert!(approx(b.step, 100.0));
    assert!(approx(b.bandwidth, 80.0));
    assert!(approx(b.x(1), 110.0));
    assert!(approx(b.center(2), 250.0));
    assert_eq!(BandScale::new(0, 100.0, 0.1).step, 0.0);
}

#[test]
fn bar_rects_grouped() {
    let rects = bar_rects(&[vec![50.0, 100.0], vec![25.0, 0.0]], 200.0, 100.0, 100.0, 0.0);
    assert_eq!(rects.len(), 4);
    let r = &rects[0];
    assert!(
        approx(r.x, 0.0) && approx(r.width, 50.0) && approx(r.height, 50.0) && approx(r.y, 50.0)
    );
    let r = &rects[1];
    assert!(approx(r.x, 100.0) && approx(r.height, 100.0));
    let r = &rects[2];
    assert!(approx(r.x, 50.0) && approx(r.height, 25.0));
    assert!(approx(rects[3].height, 0.0));
}

#[test]
fn line_path_math() {
    let pts = line_points(&[0.0, 50.0, 100.0], 300.0, 100.0, 100.0);
    assert_eq!(pts, vec![(50.0, 100.0), (150.0, 50.0), (250.0, 0.0)]);
    assert_eq!(line_path(&pts), "M50,100 L150,50 L250,0");
    assert_eq!(line_path(&[(1.234, 2.5)]), "M1.23,2.5");
    assert_eq!(line_path(&[]), "");
}

#[test]
fn css_var_generation() {
    let cfg = ChartConfig::new()
        .series("desktop", "Desktop", "var(--chart-1)")
        .series("mobile", "Mobile", "#2563eb");
    assert_eq!(cfg.css_vars(), "--color-desktop: var(--chart-1); --color-mobile: #2563eb;");
    assert_eq!(cfg.get("mobile").unwrap().label, "Mobile");
    assert_eq!(color_var("desktop"), "var(--color-desktop)");
}

#[test]
fn css_var_sanitizes_and_replaces() {
    let mut cfg = ChartConfig::new().series("a b", "A", "red; x: y");
    assert_eq!(cfg.css_vars(), "--color-a-b: red x: y;");
    cfg.insert("a b", "A2", "blue");
    assert_eq!(cfg.len(), 1);
    assert_eq!(cfg.get("a b").unwrap().color, "blue");
}

#[test]
fn matrix_and_ymax() {
    let cfg = ChartConfig::new()
        .series("d", "D", "red")
        .series("m", "M", "blue");
    let data = vec![
        ChartRow::new("Jan").value("d", 186.0).value("m", 80.0),
        ChartRow::new("Feb").value("d", 305.0),
    ];
    let m = series_matrix(&cfg, &data);
    assert_eq!(m, vec![vec![186.0, 305.0], vec![80.0, 0.0]]);
    assert_eq!(y_max(&m), 500.0);
}

#[test]
fn default_size_is_backward_compatible() {
    assert_eq!(resolve_size(None, None), (600.0, 300.0));
    assert_eq!(view_box(600.0, 300.0), "0 0 600 300");
    assert_eq!(plot_size(600.0, 300.0), (552.0, 268.0));
}

#[test]
fn custom_size_viewbox_and_plot_area() {
    let (w, h) = resolve_size(Some(800.0), Some(400.0));
    assert_eq!(view_box(w, h), "0 0 800 400");
    assert_eq!(plot_size(w, h), (752.0, 368.0));
    // Invalid values fall back to defaults; tiny sizes clamp to zero.
    assert_eq!(resolve_size(Some(-1.0), Some(f64::NAN)), (600.0, 300.0));
    assert_eq!(plot_size(10.0, 10.0), (0.0, 0.0));
}

#[test]
fn custom_size_threads_into_coords() {
    let (pw, ph) = plot_size(800.0, 400.0);
    let pts = line_points(&[0.0, 100.0], pw, ph, 100.0);
    assert!(approx(pts[0].1, ph));
    assert!(approx(pts[1].1, 0.0));
    assert!(pts[1].0 > plot_size(600.0, 300.0).0);
    let bars = bar_rects(&[vec![100.0]], pw, ph, 100.0, 0.0);
    assert!(approx(bars[0].height, ph));
    assert!(approx(bars[0].y, 0.0));
    let band = BandScale::new(4, pw, 0.0);
    assert!(approx(band.step, pw / 4.0));
}
