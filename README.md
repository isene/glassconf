# glassconf - TUI Config for glass

<img src="img/glassconf.svg" align="left" width="150" height="150">

![Version](https://img.shields.io/badge/version-0.1.1-blue) ![Rust](https://img.shields.io/badge/language-Rust-f74c00) ![License](https://img.shields.io/badge/license-Unlicense-green) ![Platform](https://img.shields.io/badge/platform-Linux-blue) ![Dependencies](https://img.shields.io/badge/dependencies-crust-blue)

TUI configuration tool for the [glass](https://github.com/isene/glass) terminal emulator, built on [crust](https://github.com/isene/crust). Live-preview pane shows what the prompt will look like as you tweak colors.

<br clear="left"/>

## Features

- Edit `bg`, `fg`, `cursor` as 24-bit hex (`#rrggbb`) with swatch preview
- Step `font_size` through glass's nine presets (10/13/15/18/20/22/24/28/32)
- Adjust `opacity` (0..100) and `cursor_blink` (ms)
- Manage the `bg_cycle` color list (Alt+b cycle in glass)
- Six built-in themes (default, tokyonight, gruvbox, nord, dracula, solarized) — selecting one rewrites bg/fg/cursor
- Reads and writes `~/.glassrc` directly

## Controls

| Key | Action |
|-----|--------|
| j / k       | Move within the current category |
| J / K       | Switch categories |
| h / l       | Cycle through values (font size, theme, numbers) |
| Enter       | Edit the current value (free-text input) |
| W / s       | Save to `~/.glassrc` |
| q / ESC     | Quit (prompts to save when modified) |

## Build

```bash
cargo build --release
```

The binary lands at `target/release/glassconf`.

## Part of the [CHasm](https://github.com/isene/chasm) Suite

| Tool | Purpose |
|------|---------|
| [bare](https://github.com/isene/bare)         | Shell (assembly) |
| [glass](https://github.com/isene/glass)       | Terminal emulator (assembly) |
| [tile](https://github.com/isene/tile)         | Window manager + strip status bar (assembly) |
| [show](https://github.com/isene/show)         | File viewer (assembly) |
| [chasm-bits](https://github.com/isene/chasm-bits) | Asmite helpers fed into strip (assembly) |
| [bareconf](https://github.com/isene/bareconf) | Config TUI for bare (Rust) |
| **glassconf**                                 | **Config TUI for glass (Rust)** |
| [tileconf](https://github.com/isene/tileconf) | Config TUI for tile (Rust) |
| [stripconf](https://github.com/isene/stripconf) | Config TUI for strip (Rust) |

## License

[Unlicense](https://unlicense.org/) (public domain).
