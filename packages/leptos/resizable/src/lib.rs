//! Leptos port of shadcn/ui resizable

pub mod default;
pub mod new_york;
pub mod resizable;
mod shared;
pub mod signal_managed;

pub use default::{ResizableHandle, ResizablePanel, ResizablePanelGroup};
pub use new_york::{
    ResizableHandle as ResizableHandleNewYork, ResizablePanel as ResizablePanelNewYork,
    ResizablePanelGroup as ResizablePanelGroupNewYork,
};
pub use resizable::{ResizableConfig, ResizableState, ResizeDirection};

mod tests;

mod resizable_tests;

// Signal-managed exports
pub use signal_managed::*;
