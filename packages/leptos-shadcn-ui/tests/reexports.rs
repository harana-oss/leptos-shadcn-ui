//! Asserts that every enabled feature exposes its components at the crate root and in the prelude.
#![allow(unused_imports)]

#[cfg(feature = "accordion")]
#[test]
fn accordion_symbols_resolve() {
    {
        use leptos_shadcn_ui::Accordion as _Root;
        use leptos_shadcn_ui::prelude::Accordion as _Pre;
    }
    {
        use leptos_shadcn_ui::AccordionContent as _Root;
        use leptos_shadcn_ui::prelude::AccordionContent as _Pre;
    }
    {
        use leptos_shadcn_ui::AccordionItem as _Root;
        use leptos_shadcn_ui::prelude::AccordionItem as _Pre;
    }
    {
        use leptos_shadcn_ui::AccordionOrientation as _Root;
        use leptos_shadcn_ui::prelude::AccordionOrientation as _Pre;
    }
    {
        use leptos_shadcn_ui::AccordionTrigger as _Root;
        use leptos_shadcn_ui::prelude::AccordionTrigger as _Pre;
    }
}

#[cfg(feature = "alert")]
#[test]
fn alert_symbols_resolve() {
    {
        use leptos_shadcn_ui::Alert as _Root;
        use leptos_shadcn_ui::prelude::Alert as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDescription as _Root;
        use leptos_shadcn_ui::prelude::AlertDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertTitle as _Root;
        use leptos_shadcn_ui::prelude::AlertTitle as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertVariant as _Root;
        use leptos_shadcn_ui::prelude::AlertVariant as _Pre;
    }
}

#[cfg(feature = "alert-dialog")]
#[test]
fn alert_dialog_symbols_resolve() {
    {
        use leptos_shadcn_ui::AlertDialog as _Root;
        use leptos_shadcn_ui::prelude::AlertDialog as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogAction as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogAction as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogCancel as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogCancel as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogContent as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogContent as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogDescription as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogFooter as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogFooter as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogHeader as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogHeader as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogOverlay as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogOverlay as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogTitle as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogTitle as _Pre;
    }
    {
        use leptos_shadcn_ui::AlertDialogTrigger as _Root;
        use leptos_shadcn_ui::prelude::AlertDialogTrigger as _Pre;
    }
}

#[cfg(feature = "aspect-ratio")]
#[test]
fn aspect_ratio_symbols_resolve() {
    {
        use leptos_shadcn_ui::AspectRatio as _Root;
        use leptos_shadcn_ui::prelude::AspectRatio as _Pre;
    }
}

#[cfg(feature = "avatar")]
#[test]
fn avatar_symbols_resolve() {
    {
        use leptos_shadcn_ui::Avatar as _Root;
        use leptos_shadcn_ui::prelude::Avatar as _Pre;
    }
    {
        use leptos_shadcn_ui::AvatarFallback as _Root;
        use leptos_shadcn_ui::prelude::AvatarFallback as _Pre;
    }
    {
        use leptos_shadcn_ui::AvatarGroup as _Root;
        use leptos_shadcn_ui::prelude::AvatarGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::AvatarImage as _Root;
        use leptos_shadcn_ui::prelude::AvatarImage as _Pre;
    }
}

#[cfg(feature = "badge")]
#[test]
fn badge_symbols_resolve() {
    {
        use leptos_shadcn_ui::Badge as _Root;
        use leptos_shadcn_ui::prelude::Badge as _Pre;
    }
    {
        use leptos_shadcn_ui::BadgeVariant as _Root;
        use leptos_shadcn_ui::prelude::BadgeVariant as _Pre;
    }
}

#[cfg(feature = "breadcrumb")]
#[test]
fn breadcrumb_symbols_resolve() {
    {
        use leptos_shadcn_ui::Breadcrumb as _Root;
        use leptos_shadcn_ui::prelude::Breadcrumb as _Pre;
    }
    {
        use leptos_shadcn_ui::BreadcrumbEllipsis as _Root;
        use leptos_shadcn_ui::prelude::BreadcrumbEllipsis as _Pre;
    }
    {
        use leptos_shadcn_ui::BreadcrumbItem as _Root;
        use leptos_shadcn_ui::prelude::BreadcrumbItem as _Pre;
    }
    {
        use leptos_shadcn_ui::BreadcrumbLink as _Root;
        use leptos_shadcn_ui::prelude::BreadcrumbLink as _Pre;
    }
    {
        use leptos_shadcn_ui::BreadcrumbList as _Root;
        use leptos_shadcn_ui::prelude::BreadcrumbList as _Pre;
    }
    {
        use leptos_shadcn_ui::BreadcrumbPage as _Root;
        use leptos_shadcn_ui::prelude::BreadcrumbPage as _Pre;
    }
    {
        use leptos_shadcn_ui::BreadcrumbSeparator as _Root;
        use leptos_shadcn_ui::prelude::BreadcrumbSeparator as _Pre;
    }
}

#[cfg(feature = "button")]
#[test]
fn button_symbols_resolve() {
    {
        use leptos_shadcn_ui::Button as _Root;
        use leptos_shadcn_ui::prelude::Button as _Pre;
    }
    {
        use leptos_shadcn_ui::ButtonResponsiveSize as _Root;
        use leptos_shadcn_ui::prelude::ButtonResponsiveSize as _Pre;
    }
    {
        use leptos_shadcn_ui::ButtonSize as _Root;
        use leptos_shadcn_ui::prelude::ButtonSize as _Pre;
    }
    {
        use leptos_shadcn_ui::ButtonVariant as _Root;
        use leptos_shadcn_ui::prelude::ButtonVariant as _Pre;
    }
}

#[cfg(feature = "button-group")]
#[test]
fn button_group_symbols_resolve() {
    {
        use leptos_shadcn_ui::ButtonGroup as _Root;
        use leptos_shadcn_ui::prelude::ButtonGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::ButtonGroupOrientation as _Root;
        use leptos_shadcn_ui::prelude::ButtonGroupOrientation as _Pre;
    }
    {
        use leptos_shadcn_ui::ButtonGroupSeparator as _Root;
        use leptos_shadcn_ui::prelude::ButtonGroupSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::ButtonGroupText as _Root;
        use leptos_shadcn_ui::prelude::ButtonGroupText as _Pre;
    }
}

#[cfg(feature = "calendar")]
#[test]
fn calendar_symbols_resolve() {
    {
        use leptos_shadcn_ui::Calendar as _Root;
        use leptos_shadcn_ui::prelude::Calendar as _Pre;
    }
}

#[cfg(feature = "card")]
#[test]
fn card_symbols_resolve() {
    {
        use leptos_shadcn_ui::Card as _Root;
        use leptos_shadcn_ui::prelude::Card as _Pre;
    }
    {
        use leptos_shadcn_ui::CardContent as _Root;
        use leptos_shadcn_ui::prelude::CardContent as _Pre;
    }
    {
        use leptos_shadcn_ui::CardDescription as _Root;
        use leptos_shadcn_ui::prelude::CardDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::CardFooter as _Root;
        use leptos_shadcn_ui::prelude::CardFooter as _Pre;
    }
    {
        use leptos_shadcn_ui::CardHeader as _Root;
        use leptos_shadcn_ui::prelude::CardHeader as _Pre;
    }
    {
        use leptos_shadcn_ui::CardTitle as _Root;
        use leptos_shadcn_ui::prelude::CardTitle as _Pre;
    }
    {
        use leptos_shadcn_ui::CardVariant as _Root;
        use leptos_shadcn_ui::prelude::CardVariant as _Pre;
    }
    {
        use leptos_shadcn_ui::InteractiveCard as _Root;
        use leptos_shadcn_ui::prelude::InteractiveCard as _Pre;
    }
}

#[cfg(feature = "carousel")]
#[test]
fn carousel_symbols_resolve() {
    {
        use leptos_shadcn_ui::Carousel as _Root;
        use leptos_shadcn_ui::prelude::Carousel as _Pre;
    }
    {
        use leptos_shadcn_ui::CarouselContent as _Root;
        use leptos_shadcn_ui::prelude::CarouselContent as _Pre;
    }
    {
        use leptos_shadcn_ui::CarouselItem as _Root;
        use leptos_shadcn_ui::prelude::CarouselItem as _Pre;
    }
    {
        use leptos_shadcn_ui::CarouselNext as _Root;
        use leptos_shadcn_ui::prelude::CarouselNext as _Pre;
    }
    {
        use leptos_shadcn_ui::CarouselOrientation as _Root;
        use leptos_shadcn_ui::prelude::CarouselOrientation as _Pre;
    }
    {
        use leptos_shadcn_ui::CarouselPrevious as _Root;
        use leptos_shadcn_ui::prelude::CarouselPrevious as _Pre;
    }
}

#[cfg(feature = "chart")]
#[test]
fn chart_symbols_resolve() {
    {
        use leptos_shadcn_ui::BarChart as _Root;
        use leptos_shadcn_ui::prelude::BarChart as _Pre;
    }
    {
        use leptos_shadcn_ui::ChartContainer as _Root;
        use leptos_shadcn_ui::prelude::ChartContainer as _Pre;
    }
    {
        use leptos_shadcn_ui::ChartLegendContent as _Root;
        use leptos_shadcn_ui::prelude::ChartLegendContent as _Pre;
    }
    {
        use leptos_shadcn_ui::ChartTooltipContent as _Root;
        use leptos_shadcn_ui::prelude::ChartTooltipContent as _Pre;
    }
    {
        use leptos_shadcn_ui::LineChart as _Root;
        use leptos_shadcn_ui::prelude::LineChart as _Pre;
    }
}

#[cfg(feature = "checkbox")]
#[test]
fn checkbox_symbols_resolve() {
    {
        use leptos_shadcn_ui::Checkbox as _Root;
        use leptos_shadcn_ui::prelude::Checkbox as _Pre;
    }
}

#[cfg(feature = "collapsible")]
#[test]
fn collapsible_symbols_resolve() {
    {
        use leptos_shadcn_ui::Collapsible as _Root;
        use leptos_shadcn_ui::prelude::Collapsible as _Pre;
    }
    {
        use leptos_shadcn_ui::CollapsibleContent as _Root;
        use leptos_shadcn_ui::prelude::CollapsibleContent as _Pre;
    }
    {
        use leptos_shadcn_ui::CollapsibleTrigger as _Root;
        use leptos_shadcn_ui::prelude::CollapsibleTrigger as _Pre;
    }
}

#[cfg(feature = "combobox")]
#[test]
fn combobox_symbols_resolve() {
    {
        use leptos_shadcn_ui::Combobox as _Root;
        use leptos_shadcn_ui::prelude::Combobox as _Pre;
    }
}

#[cfg(feature = "command")]
#[test]
fn command_symbols_resolve() {
    {
        use leptos_shadcn_ui::Command as _Root;
        use leptos_shadcn_ui::prelude::Command as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandEmpty as _Root;
        use leptos_shadcn_ui::prelude::CommandEmpty as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandGroup as _Root;
        use leptos_shadcn_ui::prelude::CommandGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandGroupHeading as _Root;
        use leptos_shadcn_ui::prelude::CommandGroupHeading as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandInput as _Root;
        use leptos_shadcn_ui::prelude::CommandInput as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandItem as _Root;
        use leptos_shadcn_ui::prelude::CommandItem as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandList as _Root;
        use leptos_shadcn_ui::prelude::CommandList as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandSeparator as _Root;
        use leptos_shadcn_ui::prelude::CommandSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::CommandShortcut as _Root;
        use leptos_shadcn_ui::prelude::CommandShortcut as _Pre;
    }
}

#[cfg(feature = "context-menu")]
#[test]
fn context_menu_symbols_resolve() {
    {
        use leptos_shadcn_ui::ContextMenu as _Root;
        use leptos_shadcn_ui::prelude::ContextMenu as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuCheckboxItem as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuCheckboxItem as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuContent as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuContent as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuItem as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuItem as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuLabel as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuLabel as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuRadioGroup as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuRadioGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuRadioItem as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuRadioItem as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuSeparator as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuShortcut as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuShortcut as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuSub as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuSub as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuSubContent as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuSubContent as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuSubTrigger as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuSubTrigger as _Pre;
    }
    {
        use leptos_shadcn_ui::ContextMenuTrigger as _Root;
        use leptos_shadcn_ui::prelude::ContextMenuTrigger as _Pre;
    }
}

#[cfg(feature = "date-picker")]
#[test]
fn date_picker_symbols_resolve() {
    {
        use leptos_shadcn_ui::DatePicker as _Root;
        use leptos_shadcn_ui::prelude::DatePicker as _Pre;
    }
    {
        use leptos_shadcn_ui::DatePickerWithRange as _Root;
        use leptos_shadcn_ui::prelude::DatePickerWithRange as _Pre;
    }
}

#[cfg(feature = "dialog")]
#[test]
fn dialog_symbols_resolve() {
    {
        use leptos_shadcn_ui::Dialog as _Root;
        use leptos_shadcn_ui::prelude::Dialog as _Pre;
    }
    {
        use leptos_shadcn_ui::DialogClose as _Root;
        use leptos_shadcn_ui::prelude::DialogClose as _Pre;
    }
    {
        use leptos_shadcn_ui::DialogContent as _Root;
        use leptos_shadcn_ui::prelude::DialogContent as _Pre;
    }
    {
        use leptos_shadcn_ui::DialogDescription as _Root;
        use leptos_shadcn_ui::prelude::DialogDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::DialogFooter as _Root;
        use leptos_shadcn_ui::prelude::DialogFooter as _Pre;
    }
    {
        use leptos_shadcn_ui::DialogHeader as _Root;
        use leptos_shadcn_ui::prelude::DialogHeader as _Pre;
    }
    {
        use leptos_shadcn_ui::DialogTitle as _Root;
        use leptos_shadcn_ui::prelude::DialogTitle as _Pre;
    }
    {
        use leptos_shadcn_ui::DialogTrigger as _Root;
        use leptos_shadcn_ui::prelude::DialogTrigger as _Pre;
    }
}

#[cfg(feature = "drawer")]
#[test]
fn drawer_symbols_resolve() {
    {
        use leptos_shadcn_ui::Drawer as _Root;
        use leptos_shadcn_ui::prelude::Drawer as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerClose as _Root;
        use leptos_shadcn_ui::prelude::DrawerClose as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerContent as _Root;
        use leptos_shadcn_ui::prelude::DrawerContent as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerDescription as _Root;
        use leptos_shadcn_ui::prelude::DrawerDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerDirection as _Root;
        use leptos_shadcn_ui::prelude::DrawerDirection as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerFooter as _Root;
        use leptos_shadcn_ui::prelude::DrawerFooter as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerHeader as _Root;
        use leptos_shadcn_ui::prelude::DrawerHeader as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerNestedRoot as _Root;
        use leptos_shadcn_ui::prelude::DrawerNestedRoot as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerOverlay as _Root;
        use leptos_shadcn_ui::prelude::DrawerOverlay as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerPortal as _Root;
        use leptos_shadcn_ui::prelude::DrawerPortal as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerTitle as _Root;
        use leptos_shadcn_ui::prelude::DrawerTitle as _Pre;
    }
    {
        use leptos_shadcn_ui::DrawerTrigger as _Root;
        use leptos_shadcn_ui::prelude::DrawerTrigger as _Pre;
    }
}

#[cfg(feature = "dropdown-menu")]
#[test]
fn dropdown_menu_symbols_resolve() {
    {
        use leptos_shadcn_ui::DropdownMenu as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenu as _Pre;
    }
    {
        use leptos_shadcn_ui::DropdownMenuContent as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenuContent as _Pre;
    }
    {
        use leptos_shadcn_ui::DropdownMenuGroup as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenuGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::DropdownMenuItem as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenuItem as _Pre;
    }
    {
        use leptos_shadcn_ui::DropdownMenuLabel as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenuLabel as _Pre;
    }
    {
        use leptos_shadcn_ui::DropdownMenuSeparator as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenuSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::DropdownMenuShortcut as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenuShortcut as _Pre;
    }
    {
        use leptos_shadcn_ui::DropdownMenuTrigger as _Root;
        use leptos_shadcn_ui::prelude::DropdownMenuTrigger as _Pre;
    }
}

#[cfg(feature = "empty")]
#[test]
fn empty_symbols_resolve() {
    {
        use leptos_shadcn_ui::Empty as _Root;
        use leptos_shadcn_ui::prelude::Empty as _Pre;
    }
    {
        use leptos_shadcn_ui::EmptyContent as _Root;
        use leptos_shadcn_ui::prelude::EmptyContent as _Pre;
    }
    {
        use leptos_shadcn_ui::EmptyDescription as _Root;
        use leptos_shadcn_ui::prelude::EmptyDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::EmptyHeader as _Root;
        use leptos_shadcn_ui::prelude::EmptyHeader as _Pre;
    }
    {
        use leptos_shadcn_ui::EmptyMedia as _Root;
        use leptos_shadcn_ui::prelude::EmptyMedia as _Pre;
    }
    {
        use leptos_shadcn_ui::EmptyMediaVariant as _Root;
        use leptos_shadcn_ui::prelude::EmptyMediaVariant as _Pre;
    }
    {
        use leptos_shadcn_ui::EmptyTitle as _Root;
        use leptos_shadcn_ui::prelude::EmptyTitle as _Pre;
    }
}

#[cfg(feature = "field")]
#[test]
fn field_symbols_resolve() {
    {
        use leptos_shadcn_ui::Field as _Root;
        use leptos_shadcn_ui::prelude::Field as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldContent as _Root;
        use leptos_shadcn_ui::prelude::FieldContent as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldDescription as _Root;
        use leptos_shadcn_ui::prelude::FieldDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldError as _Root;
        use leptos_shadcn_ui::prelude::FieldError as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldGroup as _Root;
        use leptos_shadcn_ui::prelude::FieldGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldLabel as _Root;
        use leptos_shadcn_ui::prelude::FieldLabel as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldLegend as _Root;
        use leptos_shadcn_ui::prelude::FieldLegend as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldOrientation as _Root;
        use leptos_shadcn_ui::prelude::FieldOrientation as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldSeparator as _Root;
        use leptos_shadcn_ui::prelude::FieldSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldSet as _Root;
        use leptos_shadcn_ui::prelude::FieldSet as _Pre;
    }
    {
        use leptos_shadcn_ui::FieldTitle as _Root;
        use leptos_shadcn_ui::prelude::FieldTitle as _Pre;
    }
}

#[cfg(feature = "form")]
#[test]
fn form_symbols_resolve() {
    {
        use leptos_shadcn_ui::Form as _Root;
        use leptos_shadcn_ui::prelude::Form as _Pre;
    }
    {
        use leptos_shadcn_ui::FormControl as _Root;
        use leptos_shadcn_ui::prelude::FormControl as _Pre;
    }
    {
        use leptos_shadcn_ui::FormDescription as _Root;
        use leptos_shadcn_ui::prelude::FormDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::FormField as _Root;
        use leptos_shadcn_ui::prelude::FormField as _Pre;
    }
    {
        use leptos_shadcn_ui::FormItem as _Root;
        use leptos_shadcn_ui::prelude::FormItem as _Pre;
    }
    {
        use leptos_shadcn_ui::FormLabel as _Root;
        use leptos_shadcn_ui::prelude::FormLabel as _Pre;
    }
    {
        use leptos_shadcn_ui::FormMessage as _Root;
        use leptos_shadcn_ui::prelude::FormMessage as _Pre;
    }
}

#[cfg(feature = "hover-card")]
#[test]
fn hover_card_symbols_resolve() {
    {
        use leptos_shadcn_ui::HoverCard as _Root;
        use leptos_shadcn_ui::prelude::HoverCard as _Pre;
    }
    {
        use leptos_shadcn_ui::HoverCardContent as _Root;
        use leptos_shadcn_ui::prelude::HoverCardContent as _Pre;
    }
    {
        use leptos_shadcn_ui::HoverCardTrigger as _Root;
        use leptos_shadcn_ui::prelude::HoverCardTrigger as _Pre;
    }
}

#[cfg(feature = "input")]
#[test]
fn input_symbols_resolve() {
    {
        use leptos_shadcn_ui::Input as _Root;
        use leptos_shadcn_ui::prelude::Input as _Pre;
    }
}

#[cfg(feature = "input-group")]
#[test]
fn input_group_symbols_resolve() {
    {
        use leptos_shadcn_ui::InputGroup as _Root;
        use leptos_shadcn_ui::prelude::InputGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::InputGroupAddon as _Root;
        use leptos_shadcn_ui::prelude::InputGroupAddon as _Pre;
    }
    {
        use leptos_shadcn_ui::InputGroupButton as _Root;
        use leptos_shadcn_ui::prelude::InputGroupButton as _Pre;
    }
    {
        use leptos_shadcn_ui::InputGroupInput as _Root;
        use leptos_shadcn_ui::prelude::InputGroupInput as _Pre;
    }
    {
        use leptos_shadcn_ui::InputGroupText as _Root;
        use leptos_shadcn_ui::prelude::InputGroupText as _Pre;
    }
    {
        use leptos_shadcn_ui::InputGroupTextarea as _Root;
        use leptos_shadcn_ui::prelude::InputGroupTextarea as _Pre;
    }
}

#[cfg(feature = "input-otp")]
#[test]
fn input_otp_symbols_resolve() {
    {
        use leptos_shadcn_ui::InputOtp as _Root;
        use leptos_shadcn_ui::prelude::InputOtp as _Pre;
    }
}

#[cfg(feature = "item")]
#[test]
fn item_symbols_resolve() {
    {
        use leptos_shadcn_ui::Item as _Root;
        use leptos_shadcn_ui::prelude::Item as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemActions as _Root;
        use leptos_shadcn_ui::prelude::ItemActions as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemContent as _Root;
        use leptos_shadcn_ui::prelude::ItemContent as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemDescription as _Root;
        use leptos_shadcn_ui::prelude::ItemDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemFooter as _Root;
        use leptos_shadcn_ui::prelude::ItemFooter as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemGroup as _Root;
        use leptos_shadcn_ui::prelude::ItemGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemHeader as _Root;
        use leptos_shadcn_ui::prelude::ItemHeader as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemMedia as _Root;
        use leptos_shadcn_ui::prelude::ItemMedia as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemMediaVariant as _Root;
        use leptos_shadcn_ui::prelude::ItemMediaVariant as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemSeparator as _Root;
        use leptos_shadcn_ui::prelude::ItemSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemSize as _Root;
        use leptos_shadcn_ui::prelude::ItemSize as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemTitle as _Root;
        use leptos_shadcn_ui::prelude::ItemTitle as _Pre;
    }
    {
        use leptos_shadcn_ui::ItemVariant as _Root;
        use leptos_shadcn_ui::prelude::ItemVariant as _Pre;
    }
}

#[cfg(feature = "kbd")]
#[test]
fn kbd_symbols_resolve() {
    {
        use leptos_shadcn_ui::Kbd as _Root;
        use leptos_shadcn_ui::prelude::Kbd as _Pre;
    }
    {
        use leptos_shadcn_ui::KbdGroup as _Root;
        use leptos_shadcn_ui::prelude::KbdGroup as _Pre;
    }
}

#[cfg(feature = "label")]
#[test]
fn label_symbols_resolve() {
    {
        use leptos_shadcn_ui::Label as _Root;
        use leptos_shadcn_ui::prelude::Label as _Pre;
    }
}

#[cfg(feature = "menubar")]
#[test]
fn menubar_symbols_resolve() {
    {
        use leptos_shadcn_ui::Menubar as _Root;
        use leptos_shadcn_ui::prelude::Menubar as _Pre;
    }
    {
        use leptos_shadcn_ui::MenubarContent as _Root;
        use leptos_shadcn_ui::prelude::MenubarContent as _Pre;
    }
    {
        use leptos_shadcn_ui::MenubarItem as _Root;
        use leptos_shadcn_ui::prelude::MenubarItem as _Pre;
    }
    {
        use leptos_shadcn_ui::MenubarMenu as _Root;
        use leptos_shadcn_ui::prelude::MenubarMenu as _Pre;
    }
    {
        use leptos_shadcn_ui::MenubarTrigger as _Root;
        use leptos_shadcn_ui::prelude::MenubarTrigger as _Pre;
    }
}

#[cfg(feature = "native-select")]
#[test]
fn native_select_symbols_resolve() {
    {
        use leptos_shadcn_ui::NativeSelect as _Root;
        use leptos_shadcn_ui::prelude::NativeSelect as _Pre;
    }
    {
        use leptos_shadcn_ui::NativeSelectOptGroup as _Root;
        use leptos_shadcn_ui::prelude::NativeSelectOptGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::NativeSelectOption as _Root;
        use leptos_shadcn_ui::prelude::NativeSelectOption as _Pre;
    }
}

#[cfg(feature = "navigation-menu")]
#[test]
fn navigation_menu_symbols_resolve() {
    {
        use leptos_shadcn_ui::NavigationMenu as _Root;
        use leptos_shadcn_ui::prelude::NavigationMenu as _Pre;
    }
    {
        use leptos_shadcn_ui::NavigationMenuContent as _Root;
        use leptos_shadcn_ui::prelude::NavigationMenuContent as _Pre;
    }
    {
        use leptos_shadcn_ui::NavigationMenuItem as _Root;
        use leptos_shadcn_ui::prelude::NavigationMenuItem as _Pre;
    }
    {
        use leptos_shadcn_ui::NavigationMenuLink as _Root;
        use leptos_shadcn_ui::prelude::NavigationMenuLink as _Pre;
    }
    {
        use leptos_shadcn_ui::NavigationMenuList as _Root;
        use leptos_shadcn_ui::prelude::NavigationMenuList as _Pre;
    }
    {
        use leptos_shadcn_ui::NavigationMenuTrigger as _Root;
        use leptos_shadcn_ui::prelude::NavigationMenuTrigger as _Pre;
    }
}

#[cfg(feature = "pagination")]
#[test]
fn pagination_symbols_resolve() {
    {
        use leptos_shadcn_ui::Pagination as _Root;
        use leptos_shadcn_ui::prelude::Pagination as _Pre;
    }
    {
        use leptos_shadcn_ui::PaginationContent as _Root;
        use leptos_shadcn_ui::prelude::PaginationContent as _Pre;
    }
    {
        use leptos_shadcn_ui::PaginationEllipsis as _Root;
        use leptos_shadcn_ui::prelude::PaginationEllipsis as _Pre;
    }
    {
        use leptos_shadcn_ui::PaginationItem as _Root;
        use leptos_shadcn_ui::prelude::PaginationItem as _Pre;
    }
    {
        use leptos_shadcn_ui::PaginationLink as _Root;
        use leptos_shadcn_ui::prelude::PaginationLink as _Pre;
    }
    {
        use leptos_shadcn_ui::PaginationNext as _Root;
        use leptos_shadcn_ui::prelude::PaginationNext as _Pre;
    }
    {
        use leptos_shadcn_ui::PaginationPrevious as _Root;
        use leptos_shadcn_ui::prelude::PaginationPrevious as _Pre;
    }
}

#[cfg(feature = "popover")]
#[test]
fn popover_symbols_resolve() {
    {
        use leptos_shadcn_ui::Popover as _Root;
        use leptos_shadcn_ui::prelude::Popover as _Pre;
    }
    {
        use leptos_shadcn_ui::PopoverContent as _Root;
        use leptos_shadcn_ui::prelude::PopoverContent as _Pre;
    }
    {
        use leptos_shadcn_ui::PopoverTrigger as _Root;
        use leptos_shadcn_ui::prelude::PopoverTrigger as _Pre;
    }
}

#[cfg(feature = "progress")]
#[test]
fn progress_symbols_resolve() {
    {
        use leptos_shadcn_ui::Progress as _Root;
        use leptos_shadcn_ui::prelude::Progress as _Pre;
    }
    {
        use leptos_shadcn_ui::ProgressIndicator as _Root;
        use leptos_shadcn_ui::prelude::ProgressIndicator as _Pre;
    }
    {
        use leptos_shadcn_ui::ProgressLabel as _Root;
        use leptos_shadcn_ui::prelude::ProgressLabel as _Pre;
    }
    {
        use leptos_shadcn_ui::ProgressRoot as _Root;
        use leptos_shadcn_ui::prelude::ProgressRoot as _Pre;
    }
    {
        use leptos_shadcn_ui::ProgressVariant as _Root;
        use leptos_shadcn_ui::prelude::ProgressVariant as _Pre;
    }
}

#[cfg(feature = "radio-group")]
#[test]
fn radio_group_symbols_resolve() {
    {
        use leptos_shadcn_ui::RadioGroup as _Root;
        use leptos_shadcn_ui::prelude::RadioGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::RadioGroupItem as _Root;
        use leptos_shadcn_ui::prelude::RadioGroupItem as _Pre;
    }
}

#[cfg(feature = "resizable")]
#[test]
fn resizable_symbols_resolve() {
    {
        use leptos_shadcn_ui::ResizableHandle as _Root;
        use leptos_shadcn_ui::prelude::ResizableHandle as _Pre;
    }
    {
        use leptos_shadcn_ui::ResizablePanel as _Root;
        use leptos_shadcn_ui::prelude::ResizablePanel as _Pre;
    }
    {
        use leptos_shadcn_ui::ResizablePanelGroup as _Root;
        use leptos_shadcn_ui::prelude::ResizablePanelGroup as _Pre;
    }
}

#[cfg(feature = "scroll-area")]
#[test]
fn scroll_area_symbols_resolve() {
    {
        use leptos_shadcn_ui::ScrollArea as _Root;
        use leptos_shadcn_ui::prelude::ScrollArea as _Pre;
    }
}

#[cfg(feature = "select")]
#[test]
fn select_symbols_resolve() {
    {
        use leptos_shadcn_ui::Select as _Root;
        use leptos_shadcn_ui::prelude::Select as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectContent as _Root;
        use leptos_shadcn_ui::prelude::SelectContent as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectGroup as _Root;
        use leptos_shadcn_ui::prelude::SelectGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectItem as _Root;
        use leptos_shadcn_ui::prelude::SelectItem as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectLabel as _Root;
        use leptos_shadcn_ui::prelude::SelectLabel as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectScrollDownButton as _Root;
        use leptos_shadcn_ui::prelude::SelectScrollDownButton as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectScrollUpButton as _Root;
        use leptos_shadcn_ui::prelude::SelectScrollUpButton as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectSeparator as _Root;
        use leptos_shadcn_ui::prelude::SelectSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectTrigger as _Root;
        use leptos_shadcn_ui::prelude::SelectTrigger as _Pre;
    }
    {
        use leptos_shadcn_ui::SelectValue as _Root;
        use leptos_shadcn_ui::prelude::SelectValue as _Pre;
    }
}

#[cfg(feature = "separator")]
#[test]
fn separator_symbols_resolve() {
    {
        use leptos_shadcn_ui::Separator as _Root;
        use leptos_shadcn_ui::prelude::Separator as _Pre;
    }
}

#[cfg(feature = "sheet")]
#[test]
fn sheet_symbols_resolve() {
    {
        use leptos_shadcn_ui::Sheet as _Root;
        use leptos_shadcn_ui::prelude::Sheet as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetClose as _Root;
        use leptos_shadcn_ui::prelude::SheetClose as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetContent as _Root;
        use leptos_shadcn_ui::prelude::SheetContent as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetDescription as _Root;
        use leptos_shadcn_ui::prelude::SheetDescription as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetFooter as _Root;
        use leptos_shadcn_ui::prelude::SheetFooter as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetHeader as _Root;
        use leptos_shadcn_ui::prelude::SheetHeader as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetSide as _Root;
        use leptos_shadcn_ui::prelude::SheetSide as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetTitle as _Root;
        use leptos_shadcn_ui::prelude::SheetTitle as _Pre;
    }
    {
        use leptos_shadcn_ui::SheetTrigger as _Root;
        use leptos_shadcn_ui::prelude::SheetTrigger as _Pre;
    }
}

#[cfg(feature = "sidebar")]
#[test]
fn sidebar_symbols_resolve() {
    {
        use leptos_shadcn_ui::Sidebar as _Root;
        use leptos_shadcn_ui::prelude::Sidebar as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarGroupAction as _Root;
        use leptos_shadcn_ui::prelude::SidebarGroupAction as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarInset as _Root;
        use leptos_shadcn_ui::prelude::SidebarInset as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenu as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenu as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuAction as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuAction as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuButton as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuButton as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuButtonSize as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuButtonSize as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuButtonVariant as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuButtonVariant as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuItem as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuItem as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuSkeleton as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuSkeleton as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuSub as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuSub as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuSubButton as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuSubButton as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarMenuSubItem as _Root;
        use leptos_shadcn_ui::prelude::SidebarMenuSubItem as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarProvider as _Root;
        use leptos_shadcn_ui::prelude::SidebarProvider as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarRail as _Root;
        use leptos_shadcn_ui::prelude::SidebarRail as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarSeparator as _Root;
        use leptos_shadcn_ui::prelude::SidebarSeparator as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarSide as _Root;
        use leptos_shadcn_ui::prelude::SidebarSide as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarTrigger as _Root;
        use leptos_shadcn_ui::prelude::SidebarTrigger as _Pre;
    }
    {
        use leptos_shadcn_ui::SidebarVariant as _Root;
        use leptos_shadcn_ui::prelude::SidebarVariant as _Pre;
    }
}

#[cfg(feature = "skeleton")]
#[test]
fn skeleton_symbols_resolve() {
    {
        use leptos_shadcn_ui::Skeleton as _Root;
        use leptos_shadcn_ui::prelude::Skeleton as _Pre;
    }
    {
        use leptos_shadcn_ui::SkeletonAvatar as _Root;
        use leptos_shadcn_ui::prelude::SkeletonAvatar as _Pre;
    }
    {
        use leptos_shadcn_ui::SkeletonCard as _Root;
        use leptos_shadcn_ui::prelude::SkeletonCard as _Pre;
    }
    {
        use leptos_shadcn_ui::SkeletonSize as _Root;
        use leptos_shadcn_ui::prelude::SkeletonSize as _Pre;
    }
    {
        use leptos_shadcn_ui::SkeletonText as _Root;
        use leptos_shadcn_ui::prelude::SkeletonText as _Pre;
    }
    {
        use leptos_shadcn_ui::SkeletonVariant as _Root;
        use leptos_shadcn_ui::prelude::SkeletonVariant as _Pre;
    }
}

#[cfg(feature = "slider")]
#[test]
fn slider_symbols_resolve() {
    {
        use leptos_shadcn_ui::RangeSlider as _Root;
        use leptos_shadcn_ui::prelude::RangeSlider as _Pre;
    }
    {
        use leptos_shadcn_ui::Slider as _Root;
        use leptos_shadcn_ui::prelude::Slider as _Pre;
    }
    {
        use leptos_shadcn_ui::SliderRoot as _Root;
        use leptos_shadcn_ui::prelude::SliderRoot as _Pre;
    }
    {
        use leptos_shadcn_ui::SliderSize as _Root;
        use leptos_shadcn_ui::prelude::SliderSize as _Pre;
    }
    {
        use leptos_shadcn_ui::SliderVariant as _Root;
        use leptos_shadcn_ui::prelude::SliderVariant as _Pre;
    }
}

#[cfg(feature = "sonner")]
#[test]
fn sonner_symbols_resolve() {
    {
        use leptos_shadcn_ui::Toaster as _Root;
        use leptos_shadcn_ui::prelude::Toaster as _Pre;
    }
}

#[cfg(feature = "spinner")]
#[test]
fn spinner_symbols_resolve() {
    {
        use leptos_shadcn_ui::Spinner as _Root;
        use leptos_shadcn_ui::prelude::Spinner as _Pre;
    }
}

#[cfg(feature = "switch")]
#[test]
fn switch_symbols_resolve() {
    {
        use leptos_shadcn_ui::Switch as _Root;
        use leptos_shadcn_ui::prelude::Switch as _Pre;
    }
    {
        use leptos_shadcn_ui::SwitchLabel as _Root;
        use leptos_shadcn_ui::prelude::SwitchLabel as _Pre;
    }
    {
        use leptos_shadcn_ui::SwitchRoot as _Root;
        use leptos_shadcn_ui::prelude::SwitchRoot as _Pre;
    }
    {
        use leptos_shadcn_ui::SwitchSize as _Root;
        use leptos_shadcn_ui::prelude::SwitchSize as _Pre;
    }
    {
        use leptos_shadcn_ui::SwitchThumb as _Root;
        use leptos_shadcn_ui::prelude::SwitchThumb as _Pre;
    }
    {
        use leptos_shadcn_ui::SwitchVariant as _Root;
        use leptos_shadcn_ui::prelude::SwitchVariant as _Pre;
    }
}

#[cfg(feature = "table")]
#[test]
fn table_symbols_resolve() {
    {
        use leptos_shadcn_ui::Table as _Root;
        use leptos_shadcn_ui::prelude::Table as _Pre;
    }
}

#[cfg(feature = "tabs")]
#[test]
fn tabs_symbols_resolve() {
    {
        use leptos_shadcn_ui::Tabs as _Root;
        use leptos_shadcn_ui::prelude::Tabs as _Pre;
    }
    {
        use leptos_shadcn_ui::TabsContent as _Root;
        use leptos_shadcn_ui::prelude::TabsContent as _Pre;
    }
    {
        use leptos_shadcn_ui::TabsList as _Root;
        use leptos_shadcn_ui::prelude::TabsList as _Pre;
    }
    {
        use leptos_shadcn_ui::TabsTrigger as _Root;
        use leptos_shadcn_ui::prelude::TabsTrigger as _Pre;
    }
}

#[cfg(feature = "textarea")]
#[test]
fn textarea_symbols_resolve() {
    {
        use leptos_shadcn_ui::Textarea as _Root;
        use leptos_shadcn_ui::prelude::Textarea as _Pre;
    }
}

#[cfg(feature = "toast")]
#[test]
fn toast_symbols_resolve() {
    {
        use leptos_shadcn_ui::Toast as _Root;
        use leptos_shadcn_ui::prelude::Toast as _Pre;
    }
}

#[cfg(feature = "toggle")]
#[test]
fn toggle_symbols_resolve() {
    {
        use leptos_shadcn_ui::Toggle as _Root;
        use leptos_shadcn_ui::prelude::Toggle as _Pre;
    }
}

#[cfg(feature = "toggle-group")]
#[test]
fn toggle_group_symbols_resolve() {
    {
        use leptos_shadcn_ui::ToggleGroup as _Root;
        use leptos_shadcn_ui::prelude::ToggleGroup as _Pre;
    }
    {
        use leptos_shadcn_ui::ToggleGroupItem as _Root;
        use leptos_shadcn_ui::prelude::ToggleGroupItem as _Pre;
    }
    {
        use leptos_shadcn_ui::ToggleGroupSize as _Root;
        use leptos_shadcn_ui::prelude::ToggleGroupSize as _Pre;
    }
    {
        use leptos_shadcn_ui::ToggleGroupVariant as _Root;
        use leptos_shadcn_ui::prelude::ToggleGroupVariant as _Pre;
    }
}

#[cfg(feature = "tooltip")]
#[test]
fn tooltip_symbols_resolve() {
    {
        use leptos_shadcn_ui::Tooltip as _Root;
        use leptos_shadcn_ui::prelude::Tooltip as _Pre;
    }
    {
        use leptos_shadcn_ui::TooltipContent as _Root;
        use leptos_shadcn_ui::prelude::TooltipContent as _Pre;
    }
    {
        use leptos_shadcn_ui::TooltipProvider as _Root;
        use leptos_shadcn_ui::prelude::TooltipProvider as _Pre;
    }
    {
        use leptos_shadcn_ui::TooltipSide as _Root;
        use leptos_shadcn_ui::prelude::TooltipSide as _Pre;
    }
    {
        use leptos_shadcn_ui::TooltipTrigger as _Root;
        use leptos_shadcn_ui::prelude::TooltipTrigger as _Pre;
    }
    {
        use leptos_shadcn_ui::TooltipVariant as _Root;
        use leptos_shadcn_ui::prelude::TooltipVariant as _Pre;
    }
}

#[cfg(feature = "typography")]
#[test]
fn typography_symbols_resolve() {
    {
        use leptos_shadcn_ui::TypographyBlockquote as _Root;
        use leptos_shadcn_ui::prelude::TypographyBlockquote as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyH1 as _Root;
        use leptos_shadcn_ui::prelude::TypographyH1 as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyH2 as _Root;
        use leptos_shadcn_ui::prelude::TypographyH2 as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyH3 as _Root;
        use leptos_shadcn_ui::prelude::TypographyH3 as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyH4 as _Root;
        use leptos_shadcn_ui::prelude::TypographyH4 as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyInlineCode as _Root;
        use leptos_shadcn_ui::prelude::TypographyInlineCode as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyLarge as _Root;
        use leptos_shadcn_ui::prelude::TypographyLarge as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyLead as _Root;
        use leptos_shadcn_ui::prelude::TypographyLead as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyList as _Root;
        use leptos_shadcn_ui::prelude::TypographyList as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyMuted as _Root;
        use leptos_shadcn_ui::prelude::TypographyMuted as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographyP as _Root;
        use leptos_shadcn_ui::prelude::TypographyP as _Pre;
    }
    {
        use leptos_shadcn_ui::TypographySmall as _Root;
        use leptos_shadcn_ui::prelude::TypographySmall as _Pre;
    }
}
#[cfg(feature = "error-boundary")]
#[test]
fn error_boundary_symbols_resolve() {
    use leptos_shadcn_ui::ErrorBoundary as _Root;
    use leptos_shadcn_ui::prelude::ErrorBoundary as _Pre;
}
