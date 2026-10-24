//! Leptos port of shadcn/ui accordion

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    Accordion, AccordionContent, AccordionItem, AccordionOrientation, AccordionTrigger,
    AccordionType,
};

pub use new_york::{
    Accordion as AccordionNewYork, AccordionContent as AccordionContentNewYork,
    AccordionItem as AccordionItemNewYork, AccordionOrientation as AccordionOrientationNewYork,
    AccordionTrigger as AccordionTriggerNewYork, AccordionType as AccordionTypeNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
// Leptos effects spawn on the async executor; native tests need one.
#[cfg(test)]
#[ctor::ctor]
unsafe fn init_test_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}
