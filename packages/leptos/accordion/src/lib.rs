//! Leptos port of shadcn/ui accordion

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Accordion, AccordionItem, AccordionTrigger, AccordionContent,
    AccordionType, AccordionOrientation,
};

pub use new_york::{
    Accordion as AccordionNewYork,
    AccordionItem as AccordionItemNewYork,
    AccordionTrigger as AccordionTriggerNewYork,
    AccordionContent as AccordionContentNewYork,
    AccordionType as AccordionTypeNewYork,
    AccordionOrientation as AccordionOrientationNewYork,
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
