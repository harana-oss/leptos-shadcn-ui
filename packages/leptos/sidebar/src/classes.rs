//! Tailwind v4 class constants for the sidebar (mirrors shadcn/ui v4).

use crate::{SidebarMenuButtonSize, SidebarMenuButtonVariant, SidebarSide, SidebarVariant};

pub const SIDEBAR_WRAPPER_CLASS: &str =
    "group/sidebar-wrapper has-data-[variant=inset]:bg-sidebar flex min-h-svh w-full";
pub const SIDEBAR_STATIC_CLASS: &str =
    "bg-sidebar text-sidebar-foreground flex h-full w-(--sidebar-width) flex-col";
pub const SIDEBAR_PEER_CLASS: &str = "group peer text-sidebar-foreground hidden md:block";
pub const SIDEBAR_GAP_CLASS: &str = "relative w-(--sidebar-width) bg-transparent transition-[width] duration-200 ease-linear group-data-[collapsible=offcanvas]:w-0 group-data-[side=right]:rotate-180";
pub const SIDEBAR_CONTAINER_CLASS: &str = "fixed inset-y-0 z-10 hidden h-svh w-(--sidebar-width) transition-[left,right,width] duration-200 ease-linear md:flex";
pub const SIDEBAR_INNER_CLASS: &str = "bg-sidebar group-data-[variant=floating]:border-sidebar-border flex h-full w-full flex-col group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:shadow-sm";
pub const SIDEBAR_MOBILE_OVERLAY_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const SIDEBAR_MOBILE_CLASS: &str = "bg-sidebar text-sidebar-foreground fixed inset-y-0 z-50 flex h-full w-(--sidebar-width) flex-col p-0 shadow-lg";
pub const SIDEBAR_TRIGGER_CLASS: &str = "inline-flex size-7 shrink-0 items-center justify-center rounded-md text-sm font-medium outline-none transition-all hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4";
pub const SIDEBAR_RAIL_CLASS: &str = "hover:after:bg-sidebar-border absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear group-data-[side=left]:-right-4 group-data-[side=right]:left-0 after:absolute after:inset-y-0 after:left-1/2 after:w-[2px] sm:flex in-data-[side=left]:cursor-w-resize in-data-[side=right]:cursor-e-resize [[data-side=left][data-state=collapsed]_&]:cursor-e-resize [[data-side=right][data-state=collapsed]_&]:cursor-w-resize hover:group-data-[collapsible=offcanvas]:bg-sidebar group-data-[collapsible=offcanvas]:translate-x-0 group-data-[collapsible=offcanvas]:after:left-full [[data-side=left][data-collapsible=offcanvas]_&]:-right-2 [[data-side=right][data-collapsible=offcanvas]_&]:-left-2";
pub const SIDEBAR_INSET_CLASS: &str = "bg-background relative flex w-full flex-1 flex-col md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow-sm md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2";
pub const SIDEBAR_HEADER_CLASS: &str = "flex flex-col gap-2 p-2";
pub const SIDEBAR_FOOTER_CLASS: &str = "flex flex-col gap-2 p-2";
pub const SIDEBAR_SEPARATOR_CLASS: &str = "bg-sidebar-border mx-2 h-px w-auto shrink-0";
pub const SIDEBAR_CONTENT_CLASS: &str = "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden";
pub const SIDEBAR_GROUP_CLASS: &str = "relative flex w-full min-w-0 flex-col p-2";
pub const SIDEBAR_GROUP_LABEL_CLASS: &str = "text-sidebar-foreground/70 ring-sidebar-ring flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0";
pub const SIDEBAR_GROUP_ACTION_CLASS: &str = "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground absolute top-3.5 right-3 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 after:absolute after:-inset-2 md:after:hidden group-data-[collapsible=icon]:hidden";
pub const SIDEBAR_GROUP_CONTENT_CLASS: &str = "w-full text-sm";
pub const SIDEBAR_MENU_CLASS: &str = "flex w-full min-w-0 flex-col gap-1";
pub const SIDEBAR_MENU_ITEM_CLASS: &str = "group/menu-item relative";
pub const SIDEBAR_MENU_BUTTON_CLASS: &str = "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-hidden ring-sidebar-ring transition-[width,height,padding] hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 active:bg-sidebar-accent active:text-sidebar-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidebar=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium data-[active=true]:text-sidebar-accent-foreground data-[state=open]:hover:bg-sidebar-accent data-[state=open]:hover:text-sidebar-accent-foreground group-data-[collapsible=icon]:size-8! group-data-[collapsible=icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0";
pub const SIDEBAR_MENU_BUTTON_VARIANT_DEFAULT: &str =
    "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground";
pub const SIDEBAR_MENU_BUTTON_VARIANT_OUTLINE: &str = "bg-background shadow-[0_0_0_1px_var(--sidebar-border)] hover:bg-sidebar-accent hover:text-sidebar-accent-foreground hover:shadow-[0_0_0_1px_var(--sidebar-accent)]";
pub const SIDEBAR_MENU_BUTTON_SIZE_DEFAULT: &str = "h-8 text-sm";
pub const SIDEBAR_MENU_BUTTON_SIZE_SM: &str = "h-7 text-xs";
pub const SIDEBAR_MENU_BUTTON_SIZE_LG: &str = "h-12 text-sm group-data-[collapsible=icon]:p-0!";
pub const SIDEBAR_MENU_ACTION_CLASS: &str = "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground peer-hover/menu-button:text-sidebar-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 after:absolute after:-inset-2 md:after:hidden peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 group-data-[collapsible=icon]:hidden";
pub const SIDEBAR_MENU_ACTION_HOVER_CLASS: &str = "peer-data-[active=true]/menu-button:text-sidebar-accent-foreground group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 md:opacity-0";
pub const SIDEBAR_MENU_BADGE_CLASS: &str = "text-sidebar-foreground pointer-events-none absolute right-1 flex h-5 min-w-5 items-center justify-center rounded-md px-1 text-xs font-medium tabular-nums select-none peer-hover/menu-button:text-sidebar-accent-foreground peer-data-[active=true]/menu-button:text-sidebar-accent-foreground peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 group-data-[collapsible=icon]:hidden";
pub const SIDEBAR_MENU_SKELETON_CLASS: &str = "flex h-8 items-center gap-2 rounded-md px-2";
pub const SKELETON_CLASS: &str = "bg-accent animate-pulse rounded-md";
pub const SIDEBAR_MENU_SUB_CLASS: &str = "border-sidebar-border mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l px-2.5 py-0.5 group-data-[collapsible=icon]:hidden";
pub const SIDEBAR_MENU_SUB_ITEM_CLASS: &str = "group/menu-sub-item relative";
pub const SIDEBAR_MENU_SUB_BUTTON_CLASS: &str = "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground active:bg-sidebar-accent active:text-sidebar-accent-foreground [&>svg]:text-sidebar-accent-foreground flex h-7 min-w-0 -translate-x-px items-center gap-2 overflow-hidden rounded-md px-2 outline-hidden focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0 data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground group-data-[collapsible=icon]:hidden";

pub fn sidebar_gap_class(variant: SidebarVariant) -> String {
    let extra = match variant {
        SidebarVariant::Floating | SidebarVariant::Inset => {
            "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]"
        }
        SidebarVariant::Sidebar => "group-data-[collapsible=icon]:w-(--sidebar-width-icon)",
    };
    format!("{SIDEBAR_GAP_CLASS} {extra}")
}

pub fn sidebar_container_class(side: SidebarSide, variant: SidebarVariant) -> String {
    let side_class = match side {
        SidebarSide::Left => {
            "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]"
        }
        SidebarSide::Right => {
            "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]"
        }
    };
    let variant_class = match variant {
        SidebarVariant::Floating | SidebarVariant::Inset => {
            "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
        }
        SidebarVariant::Sidebar => {
            "group-data-[collapsible=icon]:w-(--sidebar-width-icon) group-data-[side=left]:border-r group-data-[side=right]:border-l"
        }
    };
    format!("{SIDEBAR_CONTAINER_CLASS} {side_class} {variant_class}")
}

pub fn sidebar_mobile_side_class(side: SidebarSide) -> &'static str {
    match side {
        SidebarSide::Left => "left-0 border-r",
        SidebarSide::Right => "right-0 border-l",
    }
}

pub fn sidebar_menu_button_class(
    variant: SidebarMenuButtonVariant,
    size: SidebarMenuButtonSize,
) -> String {
    let v = match variant {
        SidebarMenuButtonVariant::Default => SIDEBAR_MENU_BUTTON_VARIANT_DEFAULT,
        SidebarMenuButtonVariant::Outline => SIDEBAR_MENU_BUTTON_VARIANT_OUTLINE,
    };
    let s = match size {
        SidebarMenuButtonSize::Default => SIDEBAR_MENU_BUTTON_SIZE_DEFAULT,
        SidebarMenuButtonSize::Sm => SIDEBAR_MENU_BUTTON_SIZE_SM,
        SidebarMenuButtonSize::Lg => SIDEBAR_MENU_BUTTON_SIZE_LG,
    };
    format!("{SIDEBAR_MENU_BUTTON_CLASS} {v} {s}")
}

pub fn sidebar_menu_action_class(show_on_hover: bool) -> String {
    if show_on_hover {
        format!("{SIDEBAR_MENU_ACTION_CLASS} {SIDEBAR_MENU_ACTION_HOVER_CLASS}")
    } else {
        SIDEBAR_MENU_ACTION_CLASS.to_string()
    }
}
