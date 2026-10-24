//! Leptos port of shadcn/ui carousel

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    Carousel, CarouselApi, CarouselContent, CarouselItem, CarouselNext, CarouselOrientation,
    CarouselPrevious,
};

pub use new_york::{
    Carousel as CarouselNewYork, CarouselApi as CarouselApiNewYork,
    CarouselContent as CarouselContentNewYork, CarouselItem as CarouselItemNewYork,
    CarouselNext as CarouselNextNewYork, CarouselOrientation as CarouselOrientationNewYork,
    CarouselPrevious as CarouselPreviousNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
