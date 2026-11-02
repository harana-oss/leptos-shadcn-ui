# API Docs Drift Audit

Audit of `docs/components/api/*.md` against `packages/leptos/<name>/src/default.rs` (and `src/default_components/`), done for the 20 most-used pages. Run `build/scripts/check-docs-api.sh` to check page/crate mapping and crate names.

## All pages

| Page | Issue | Fix |
|------|-------|-----|
| all 47 pages | Crate names `shadcn-ui-leptos-*` / `shadcn_ui_leptos_*` | Renamed to `leptos-shadcn-*` / `leptos_shadcn_*` |
| all 47 pages | Install snippet pointed at crates.io `"0.7"` | Switched to the harana-oss git dependency at tag `v0.9.1` (same as root README) and added a note that crates.io `0.9.0` is the legacy cloud-shuttle line |
| several pages | `yourusername` placeholder links | Replaced with `harana-oss` (all of `docs/`) |

## Per-page audit

| Page | Issue | Fix |
|------|-------|-----|
| button | Example used `ButtonVariant::Primary` (real variants: Default, Destructive, Outline, Secondary, Ghost, Link) | Example now uses `ButtonVariant::Default` |
| button | `responsive_size` and `touch_friendly` props were missing | Added both to the props table and documented `ButtonResponsiveSize { SmMd, MdLg, XsSm }` |
| tabs | Root callback documented as `on_change` | Renamed to `on_value_change` in the table, example and TS block |
| tabs | Documented an `id` prop on `Tabs` (none exists); `default_value` was missing | Prop tables regenerated from source; note added that `Tabs` has no `id` |
| tabs | `TabsTrigger` `disabled` is not in source yet | Kept in the docs on purpose (being added in parallel work) |
| select | `on_change`, `placeholder` on `Select`, `position` on `SelectContent`, `disabled` on `SelectTrigger`/`SelectItem` don't exist | Tables regenerated: `Select` has `open`/`on_open_change`/`value`/`on_value_change`/`default_value`/`disabled`/`required`/`name`; added the Group, Label, Separator and scroll-button parts. Example now uses `on_value_change` |
| popover | `open` documented as `Signal<bool>` required; `side`/`align` on `PopoverContent` don't exist; `disabled` on `PopoverTrigger` was missing | Tables regenerated (`open: Option<RwSignal<bool>>`); example now uses `RwSignal` |
| tooltip | `TooltipProvider` missing; `delay_duration` type was wrong (`u64`, actually `Signal<u32>`); `side` type was `Side` (actually `TooltipSide`) | Tables regenerated, `TooltipSide` enum added; example wrapped in `TooltipProvider` |
| accordion | `value: Signal<String>`, `on_change`, `multiple` don't exist | Tables regenerated (`type: AccordionType`, `orientation`, `value: RwSignal<Vec<String>>`, `on_value_change`); enums added; example fixed |
| form | Only `Form` was documented; `on_submit` took `FormEvent` (actually `FormData`); example used `ButtonVariant::Primary` | Added FormField/FormItem/FormLabel/FormControl/FormMessage/FormDescription tables; example fixed |
| toast | Documented `ToastProvider`, `ToastTitle`, `ToastDescription`, `ToastClose` and `title`/`description`/`duration`/`on_close` props, none of which exist; wrong `ToastVariant` | Tables regenerated: `Toast` (`variant: String`) plus the Sonner API (`SonnerProvider`, `SonnerViewport`, `SonnerToast`, `toast::*`, `ToastBuilder`); real `ToastVariant` and `ToastPosition` enums; example rewritten |
| table | Documented `TableHeader`/`Body`/`Footer`/`Row`/`Head`/`Cell`/`Caption`, none of which the crate exports | Documented the single `Table` plus the `DataTable` API; example uses plain `<thead>`/`<tr>`/`<td>` |
| drawer | `open: Signal<bool>`, a `Direction` enum (actually `DrawerDirection`, default `Bottom`); most parts missing | Tables regenerated for all 11 parts; enum fixed; example uses `RwSignal` |
| label | Documented a `for` prop (none exists) | Removed; examples use `attr:for` (also in input.md) |
| textarea | Documented a `rows` prop (none exists); `input_type` was missing | Table regenerated; example uses `attr:rows` |
| badge | `BadgeVariant::Primary` / `Success` don't exist (real: Default, Secondary, Destructive, Outline); `id`/`style` were missing | Enum and table regenerated; examples removed |
| alert | `AlertVariant::Info` doesn't exist (real: Default, Destructive, Success, Warning); `id`/`style` were missing | Enum and table regenerated; Info example removed |
| input | `Label for=` in examples | Changed to `attr:for` |
| card | No drift found | - |
| dialog | No drift found | - |
| checkbox | No drift found | - |
| switch | No drift found | - |
| sheet | The page documents the full compound API, but source has only `Sheet` | Left unchanged on purpose: the compound Sheet API is being added in parallel work to match this page |

## New stub pages

Generated from source for crates that had no page: analytics, button-group, chart, date-picker, empty, field, input-group, item, kbd, native-select, sidebar, sonner, spinner, toggle-group, typography, utils.

## Known remaining drift (not fixed)

- The "TypeScript API" sections are leftovers from a JS template. They don't describe the Rust API and should be removed or regenerated.
- The 27 pages outside the top 20 weren't audited prop by prop.
