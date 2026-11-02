# Carousel Component API

A rotating content display with navigation controls.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-carousel = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_carousel::Carousel;
```

---

## Import

```rust
use leptos_shadcn_carousel::{
    Carousel,
    CarouselContent,
    CarouselItem,
    CarouselPrevious,
    CarouselNext,
    CarouselDots
};
```

---

## Component API

### Carousel

Root container managing carousel state.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `current` | `Signal<usize>` | **Required** | Current slide index |
| `on_change` | `Option<Callback<usize>>` | `None` | Change handler |
| `auto_play` | `Signal<bool>` | `false` | Auto-rotate slides |
| `interval` | `MaybeProp<u64>` | `5000` | Auto-play interval (ms) |

---

## Usage Examples

### Basic Carousel

```rust
use leptos::prelude::*;
use leptos_shadcn_carousel::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (current, set_current) = signal(0);

    view! {
        <Carousel
            current=current.into()
            on_change=Some(Callback::new(move |i| set_current.set(i)))
        >
            <CarouselContent>
                <CarouselItem>
                    <div class="flex aspect-square items-center justify-center bg-muted">
                        "Slide 1"
                    </div>
                </CarouselItem>
                <CarouselItem>
                    <div class="flex aspect-square items-center justify-center bg-muted">
                        "Slide 2"
                    </div>
                </CarouselItem>
                <CarouselItem>
                    <div class="flex aspect-square items-center justify-center bg-muted">
                        "Slide 3"
                    </div>
                </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
            <CarouselDots />
        </Carousel>
    }
}
```

---

## CSS Classes

```css
.carousel {
    relative
}

.carousel-content {
    flex overflow-hidden
}

.carousel-item {
    min-w-0 shrink-0 grow-0 basis-full
}

.carousel-previous,
.carousel-next {
    absolute h-8 w-8 rounded-full border bg-background
    top-1/2 -translate-y-1/2
}

.carousel-previous {
    left-4
}

.carousel-next {
    right-4
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Arrow Left** | Previous slide |
| **Arrow Right** | Next slide |

---

## TypeScript API

```typescript
interface CarouselProps {
  current: number;
  onChange?: (index: number) => void;
  autoPlay?: boolean;
  interval?: number;
}

export const Carousel: React.FC<CarouselProps>;
export const CarouselContent: React.FC<ComponentProps>;
export const CarouselItem: React.FC<ComponentProps>;
export const CarouselPrevious: React.FC<ButtonProps>;
export const CarouselNext: React.FC<ButtonProps>;
```

---

*Source: [packages/leptos/carousel/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/carousel/src/default.rs)*
