//! Leptos port of shadcn/ui skeleton

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    Skeleton, SkeletonAvatar, SkeletonCard, SkeletonSize, SkeletonText, SkeletonVariant,
};
pub use new_york::{
    Skeleton as SkeletonNewYork, SkeletonAvatar as SkeletonAvatarNewYork,
    SkeletonCard as SkeletonCardNewYork, SkeletonSize as SkeletonSizeNewYork,
    SkeletonText as SkeletonTextNewYork, SkeletonVariant as SkeletonVariantNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
