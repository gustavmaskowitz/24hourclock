# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust + Leptos + WebAssembly application that displays a circular 24-hour clock visualization showing timezone overlaps between Dallas, Connecticut, and London. The clock helps identify optimal meeting times when all three timezones are in working hours (9:00-18:00 local time).

## Tech Stack

- **Language**: Rust (compiled to WebAssembly)
- **Web Framework**: Leptos 0.7 (client-side rendering, fine-grained reactivity)
- **Build Tool**: Trunk
- **Deployment**: GitHub Pages

## Common Commands

```bash
# Start development server (runs on http://localhost:5173)
trunk serve

# Build for production
trunk build --release

# Build for GitHub Pages specifically
trunk build --release --public-url /24hourclock/

# Check for compile errors without building
cargo check --target wasm32-unknown-unknown

# Run unit tests
cargo test
```

## Application Architecture

### Entry Point
- `src/main.rs` - App initialization, mounts Leptos `App` component to body
- `index.html` - Base HTML template (processed by Trunk)

### State Management (src/app.rs)
- Root component holding all application state as Leptos signals
- `signal()` for meetings, selected slot, current time, ring assignments, theme, mode
- `Signal::derive()` for computed theme colors
- `gloo_timers::Interval` for 60-second time updates

### Components (src/components/)
- `clock.rs` - Main SVG clock component, renders all 3 rings
- `clock_segment.rs` - Individual hour segment within a ring
- `now_highlight.rs` - Current time indicator (outlines + radial line)
- `center_display.rs` - Center circle showing current time in all 3 timezones
- `controls.rs` - Header bar (theme/mode toggle) + ring selector dropdowns
- `info_panels.rs` - Overlap summary, meetings-outside-overlap warnings
- `slot_detail.rs` - Selected time slot detail panel with meeting CRUD

### Business Logic (src/modules/)
- `types.rs` - Timezone enum, Meeting struct, RingAssignments, SVG constants
- `timezone.rs` - UTC conversion, work hour checks, overlap detection
- `geometry.rs` - Polar-to-Cartesian math, SVG arc path generation
- `themes.rs` - 4 themes x 2 modes = 8 color schemes (ThemeColors struct)

### Key Architectural Patterns

**Leptos 0.7 Signals** (not React hooks):
- `signal(value)` creates `(ReadSignal, WriteSignal)` pair
- `Signal::derive(|| ...)` for computed values
- `Effect::new(|| ...)` for side effects
- `.get()` / `.set()` for access

**SVG Rendering**:
- viewBox="0 0 400 400", center at (200, 200)
- Three concentric rings: outer (192/156), middle (152/116), inner (112/70)
- Segments colored green (#22c55e) when timezone is in work hours
- All rings reference the outer ring's timezone for hour positioning

**Theme System**:
- 4 themes (Minimalist, Bold, Professional, Playful) x 2 modes (Light, Dark)
- ~40 color properties per theme applied via inline `style` attributes
- `ThemeColors` struct with static instances for zero-allocation lookups

**Ring Swap Logic**:
- When selecting a timezone already assigned to another ring, they swap
- Prevents duplicate timezone assignments

## Development Notes

- Leptos 0.7 uses `use leptos::prelude::*;` (not `use leptos::*;`)
- Views are statically typed; use `.into_any()` for conditional branches
- SVG attributes use kebab-case in view! macro (e.g., `stroke-width`)
- Browser APIs accessed via `js-sys` and `web-sys` crates
- Dev build WASM ~2.8MB, release build ~216KB (with opt-level="z" and LTO)
