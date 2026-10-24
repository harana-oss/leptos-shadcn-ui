//! Leptos port of shadcn/ui card

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, CardVariant,
    InteractiveCard,
};
pub use new_york::{
    Card as CardNewYork, CardContent as CardContentNewYork,
    CardDescription as CardDescriptionNewYork, CardFooter as CardFooterNewYork,
    CardHeader as CardHeaderNewYork, CardTitle as CardTitleNewYork,
};
pub use signal_managed::{
    EnhancedCard, SignalManagedCard, SignalManagedCardContent, SignalManagedCardDescription,
    SignalManagedCardFooter, SignalManagedCardHeader, SignalManagedCardState,
    SignalManagedCardTitle,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

mod implementation_tests;

mod new_york_tests;
