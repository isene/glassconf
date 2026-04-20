# glassconf - TUI Config for glass

<img src="img/glassconf.svg" align="left" width="150" height="150">

![Version](https://img.shields.io/badge/version-0.1.0-blue) ![Rust](https://img.shields.io/badge/language-Rust-f74c00) ![License](https://img.shields.io/badge/license-Unlicense-green) ![Platform](https://img.shields.io/badge/platform-Linux-blue) ![Dependencies](https://img.shields.io/badge/dependencies-crust-blue)

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

## Part of the CHasm Suite

| Tool | Purpose |
|------|---------|
| [bare](https://github.com/isene/bare) | Shell (assembly) |
| [show](https://github.com/isene/show) | File viewer (assembly) |
| [glass](https://github.com/isene/glass) | Terminal emulator (assembly) |
| **glassconf** | **Config TUI (Rust)** |

## License

[Unlicense](https://unlicense.org/) (public domain).
