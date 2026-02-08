# 24-Hour Clock

A circular 24-hour clock visualization showing timezone overlaps between Dallas, Connecticut, and London.

Demo WASM app: https://gustavmaskowitz.github.io/24hourclock/

## Features

- **24-hour circular clock** with three concentric rings for different timezones
- **Dynamic ring assignment** - configure which timezone appears on each ring
- **Working hours visualization** - green segments highlight 09:00-18:00 business hours for each timezone
- **Current time indicator** - pink accent line and border show the current moment across all timezones
- **4 visual themes** - Minimalist, Bold, Professional, and Playful styles
- **Light/Dark mode** - toggle between light and dark color schemes
- **Meeting management** - track meetings and see overlap windows

## Running Locally

Built with **Rust + WebAssembly** using the [Leptos](https://leptos.dev/) framework and [Trunk](https://trunkrs.dev/) build tool.

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- WASM target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`

### Dev server

```bash
trunk serve
```

Opens at http://localhost:5173

### Production build

```bash
trunk build --release
```

Output goes to `dist/`.
