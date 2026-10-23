//! Leptos port of shadcn/ui slider

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{RangeSlider, Slider, SliderRoot, SliderSize, SliderVariant};
pub use new_york::{
    RangeSlider as RangeSliderNewYork, Slider as SliderNewYork, SliderRoot as SliderRootNewYork,
    SliderSize as SliderSizeNewYork, SliderVariant as SliderVariantNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
