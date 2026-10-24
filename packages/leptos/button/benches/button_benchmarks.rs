//! Button Component Performance Benchmarks
//!
//! TDD Approach: These benchmarks define the performance requirements
//! and will guide the implementation of comprehensive performance testing.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use leptos::prelude::*;
use leptos_shadcn_button::default::{Button, ButtonSize, ButtonVariant};

/// Runs `f` inside a fresh reactive owner so signals and views have a scope.
fn with_owner<T>(f: impl FnOnce() -> T) -> T {
    Owner::new().with(f)
}

fn benchmark_button_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_creation");

    let variants = [
        ButtonVariant::Default,
        ButtonVariant::Destructive,
        ButtonVariant::Outline,
        ButtonVariant::Secondary,
        ButtonVariant::Ghost,
        ButtonVariant::Link,
    ];

    for variant in variants {
        group.bench_with_input(
            BenchmarkId::new("variant", format!("{:?}", variant)),
            &variant,
            |b, variant| {
                b.iter(|| {
                    with_owner(|| {
                        let variant = variant.clone();
                        let button = view! {
                            <Button
                                variant=variant
                                size=ButtonSize::Default
                                class="benchmark-button"
                                id="benchmark-button"
                            >
                                "Benchmark Button"
                            </Button>
                        };
                        black_box(button);
                    })
                });
            },
        );
    }

    group.finish();
}

fn benchmark_button_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_rendering");

    let sizes = [
        ButtonSize::Sm,
        ButtonSize::Default,
        ButtonSize::Lg,
        ButtonSize::Icon,
    ];

    for size in sizes {
        group.bench_with_input(
            BenchmarkId::new("size", format!("{:?}", size)),
            &size,
            |b, size| {
                b.iter(|| {
                    with_owner(|| {
                        let size = size.clone();
                        let view = view! {
                            <Button variant=ButtonVariant::Default size=size class="benchmark-button">
                                "Benchmark Button"
                            </Button>
                        }
                        .into_any();
                        black_box(view);
                    })
                });
            },
        );
    }

    group.finish();
}

fn benchmark_button_state_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_state_changes");

    group.bench_function("disabled_toggle", |b| {
        with_owner(|| {
            let disabled_signal = RwSignal::new(false);
            let _button = view! {
                <Button disabled=disabled_signal>"Benchmark Button"</Button>
            };

            b.iter(|| {
                disabled_signal.set(!disabled_signal.get());
                black_box(disabled_signal.get());
            });
        })
    });

    group.bench_function("class_changes", |b| {
        with_owner(|| {
            let class_signal = RwSignal::new("benchmark-button".to_string());
            let _button = view! {
                <Button class=class_signal>"Benchmark Button"</Button>
            };
            let mut counter = 0u32;

            b.iter(|| {
                counter = counter.wrapping_add(1);
                class_signal.set(format!("benchmark-button-{}", counter));
                black_box(class_signal.get());
            });
        })
    });

    group.finish();
}

fn benchmark_button_click_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_click_handling");

    group.bench_function("click_callback", |b| {
        with_owner(|| {
            let click_count = RwSignal::new(0u64);
            let callback = Callback::new(move |_| {
                click_count.update(|count| *count += 1);
            });
            let _button = view! {
                <Button on_click=callback>"Benchmark Button"</Button>
            };

            b.iter(|| {
                callback.run(());
                black_box(click_count.get());
            });
        })
    });

    group.bench_function("rapid_clicks", |b| {
        with_owner(|| {
            let click_count = RwSignal::new(0u64);
            let callback = Callback::new(move |_| {
                click_count.update(|count| *count += 1);
            });
            let _button = view! {
                <Button on_click=callback>"Benchmark Button"</Button>
            };

            b.iter(|| {
                for _ in 0..100 {
                    callback.run(());
                }
                black_box(click_count.get());
            });
        })
    });

    group.finish();
}

fn benchmark_button_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_memory_usage");

    group.bench_function("multiple_buttons", |b| {
        b.iter(|| {
            with_owner(|| {
                let buttons = (0..1000)
                    .map(|i| {
                        view! {
                            <Button
                                class=format!("benchmark-button-{}", i)
                                id=format!("benchmark-button-{}", i)
                            >
                                {format!("Button {}", i)}
                            </Button>
                        }
                    })
                    .collect::<Vec<_>>();
                black_box(buttons);
            })
        });
    });

    group.bench_function("complex_children", |b| {
        b.iter(|| {
            with_owner(|| {
                let button = view! {
                    <Button class="benchmark-button">
                        <div>
                            <span>"Complex Button Content"</span>
                            <div>
                                <span>"Nested Content"</span>
                                <span>"More Nested Content"</span>
                            </div>
                        </div>
                    </Button>
                };
                black_box(button);
            })
        });
    });

    group.finish();
}

fn benchmark_button_accessibility(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_accessibility");

    group.bench_function("accessibility_attributes", |b| {
        b.iter(|| {
            with_owner(|| {
                let button = view! {
                    <Button aria_label="Accessible Button" aria_describedby="button-help">
                        "Accessible Button"
                    </Button>
                };
                black_box(button);
            })
        });
    });

    group.bench_function("keyboard_navigation", |b| {
        with_owner(|| {
            let ids = (0..100)
                .map(|i| format!("benchmark-button-{}", i))
                .collect::<Vec<_>>();
            let _buttons = ids
                .iter()
                .cloned()
                .map(|id| view! { <Button id=id>"Button"</Button> })
                .collect::<Vec<_>>();

            b.iter(|| {
                // Simulate tab navigation through buttons
                for i in 0..100 {
                    black_box(&ids[i % ids.len()]);
                }
            });
        })
    });

    group.finish();
}

criterion_group!(
    name = button_benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(2))
        .noise_threshold(0.05);
    targets =
        benchmark_button_creation,
        benchmark_button_rendering,
        benchmark_button_state_changes,
        benchmark_button_click_handling,
        benchmark_button_memory_usage,
        benchmark_button_accessibility
);

criterion_main!(button_benches);
