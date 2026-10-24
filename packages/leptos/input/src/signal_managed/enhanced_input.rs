//! Enhanced Signal-Managed Input Component
//!
//! This module contains the enhanced signal-managed input component with advanced features.

use super::types::{INPUT_CLASS, INPUT_ERROR_CLASS, SignalManagedInputState};
use crate::validation::{InputValidator, ValidationResult};
use leptos::wasm_bindgen::JsCast;
use leptos::{ev::Event, prelude::*};
use leptos_shadcn_signal_management::*;
use leptos_style::Style;

/// Enhanced Input component with advanced signal management
#[component]
pub fn EnhancedInput(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] input_type: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] validator: Option<InputValidator>,
    #[prop(into, optional)] validation_error: MaybeProp<String>,
    #[prop(into, optional)] show_validation: Signal<bool>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let input_state = ArcRwSignal::new(SignalManagedInputState {
        value: value.get().unwrap_or_default(),
        placeholder: placeholder.get().unwrap_or_default(),
        disabled: disabled.get(),
        input_type: input_type.get().unwrap_or_else(|| "text".to_string()),
        validation_result: ValidationResult::new(),
        is_validating: false,
        has_error: false,
        focus_count: 0,
    });

    // Create computed class using ArcMemo
    let input_state_for_class = input_state.clone();
    let input_class = ArcMemo::new(move |_| {
        let state = input_state_for_class.get();
        let external_error = validation_error.get().is_some_and(|e| !e.is_empty());
        let base_class = if state.has_error || external_error {
            format!("{} {}", INPUT_CLASS, INPUT_ERROR_CLASS)
        } else {
            INPUT_CLASS.to_string()
        };
        format!("{} {}", base_class, class.get().unwrap_or_default())
    });

    // Create performance metrics
    let input_state_for_metrics = input_state.clone();
    let performance_metrics = ArcMemo::new(move |_| {
        let state = input_state_for_metrics.get();
        format!(
            "Focus: {}, Validating: {}, Error: {}",
            state.focus_count, state.is_validating, state.has_error
        )
    });

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(input_state.clone());
    theme_manager.track_memo(input_class.clone());
    theme_manager.track_memo(performance_metrics.clone());

    // Create memory manager for monitoring
    let memory_manager = SignalMemoryManager::new();

    // Create batched updater for performance
    let _batched_updater = BatchedSignalUpdater::new();

    // Create event handler with performance monitoring
    let handle_input = {
        let input_state = input_state.clone();
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                let input_value = input.value();
                callback.run(input_value.clone());

                // Update state atomically
                input_state.update(|state| {
                    state.value = input_value.clone();
                    state.focus_count += 1;
                });

                // Real-time validation
                if let Some(validator) = &validator {
                    input_state.update(|state| {
                        state.is_validating = true;
                    });

                    let result = validator.validate(&input_value);

                    input_state.update(|state| {
                        state.validation_result = result.clone();
                        state.is_validating = false;
                        state.has_error = !result.is_valid;
                    });
                }

                // Check memory pressure and perform cleanup if needed
                if let Some(MemoryPressureLevel::High | MemoryPressureLevel::Critical) =
                    memory_manager.detect_memory_pressure()
                {
                    memory_manager.perform_automatic_cleanup();
                }
            }
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    let input_state_for_type = input_state.clone();
    let input_state_for_value = input_state.clone();
    let input_state_for_placeholder = input_state.clone();
    let input_state_for_disabled = input_state.clone();
    let input_state_for_validation_display = input_state.clone();

    view! {
        <div class="enhanced-input-container">
            <input
                type=move || input_state_for_type.get().input_type
                value=move || input_state_for_value.get().value
                placeholder=move || input_state_for_placeholder.get().placeholder
                disabled=move || input_state_for_disabled.get().disabled
                class=move || input_class.get()
                id=move || id.get().unwrap_or_default()
                style=move || style.get().to_string()
                on:input=handle_input
            />

            // Validation display
            // An externally supplied `validation_error` takes precedence over the validator's message
            {move || {
                let state = input_state_for_validation_display.get();
                let error_msg = validation_error.get().filter(|e| !e.is_empty()).or_else(|| {
                    state
                        .has_error
                        .then(|| state.validation_result.get_error_message("input").unwrap_or_default().to_string())
                });
                match (show_validation.get(), error_msg) {
                    (true, Some(error_msg)) => view! {
                        <div class="validation-error text-sm text-destructive mt-1">
                            {error_msg}
                        </div>
                    }
                    .into_any(),
                    _ => ().into_any(),
                }
            }}

            // Performance monitoring (only in development)
            #[cfg(debug_assertions)]
            <div class="performance-monitor text-xs text-muted-foreground mt-1">
                {move || performance_metrics.get()}
            </div>
        </div>
    }
}
