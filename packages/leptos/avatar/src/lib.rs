//! Leptos port of shadcn/ui avatar

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Avatar, AvatarFallback, AvatarGroup, AvatarImage};
pub use new_york::{
    Avatar as AvatarNewYork, AvatarFallback as AvatarFallbackNewYork,
    AvatarGroup as AvatarGroupNewYork, AvatarImage as AvatarImageNewYork,
};

mod tests;

// Signal-managed exports
pub use signal_managed::*;
