use crust::{Crust, Pane, Input};
use crust::style;
use std::path::PathBuf;

const FONT_SIZES: [u32; 9] = [10, 13, 15, 18, 20, 22, 24, 28, 32];
const DEFAULT_FONT: u32 = 13;

const THEME_NAMES: [&str; 6] = ["default", "tokyonight", "gruvbox", "nord", "dracula", "solarized"];
// (bg, fg, cursor)
const THEMES: [(&str, &str, &str); 6] = [
    ("#000000", "#ffffff", "#f7768e"),
    ("#1a1b26", "#c0caf5", "#f7768e"),
    ("#282828", "#ebdbb2", "#fb4934"),
    ("#2e3440", "#d8dee9", "#88c0d0"),
    ("#282a36", "#f8f8f2", "#ff79c6"),
    ("#002b36", "#839496", "#cb4b16"),
];

const DEFAULT_BG_CYCLE: &str = "#000000,#001a33,#002200,#200033,#330011,#332200,#003333";

#[derive(Clone)]
enum ItemKind {
    HexColor(String),                       // bg / fg / cursor
    FontSize(usize),                        // index into FONT_SIZES
    Number(u32, u32, u32),                  // value, min, max
    BgCycle(Vec<String>),                   // list of hex strings
    Toggle(bool, &'static str, &'static str), // (on, on_str, off_str)
    Keybind(String),                        // free-form like "alt+plus"
    Theme,
}

#[derive(Clone)]
struct Item {
    label: String,
    key: &'static str,
    kind: ItemKind,
}

struct Category {
    name: String,
    items: Vec<Item>,
}

struct App {
    top: Pane,
    left: Pane,
    right: Pane,
    status: Pane,
    categories: Vec<Category>,
    cat_index: usize,
    item_index: usize,
    theme_idx: usize,
    dirty: bool,
    config_path: PathBuf,
}

impl App {
    fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        let config_path = PathBuf::from(&home).join(".glassrc");
        let (cols, rows) = Crust::terminal_size();
        let split = 22u16;
        let lw = split - 1;
        let rx = split + 3;
        let rw = cols.saturating_sub(rx).saturating_sub(1);

        let mut app = App {
            top: Pane::new(1, 1, cols, 1, 0, 236),
            left: Pane::new(2, 3, lw, rows.saturating_sub(4), 255, 0),
            right: Pane::new(rx, 3, rw, rows.saturating_sub(4), 252, 0),
            status: Pane::new(1, rows, cols, 1, 252, 236),
            categories: Vec::new(),
            cat_index: 0,
            item_index: 0,
            theme_idx: 0,
            dirty: false,
            config_path,
        };
        app.left.border = true;
        app.right.border = true;
        app.build_categories();
        app.load_config();
        app
    }

    fn build_categories(&mut self) {
        self.categories = vec![
            Category { name: "Theme".into(), items: vec![
                Item { label: "Theme preset".into(), key: "_theme", kind: ItemKind::Theme },
            ]},
            Category { name: "Colors".into(), items: vec![
                Item { label: "Background".into(),  key: "bg",     kind: ItemKind::HexColor("#000000".into()) },
                Item { label: "Foreground".into(),  key: "fg",     kind: ItemKind::HexColor("#ffffff".into()) },
                Item { label: "Cursor".into(),      key: "cursor", kind: ItemKind::HexColor("#f7768e".into()) },
            ]},
            Category { name: "Font".into(), items: vec![
                Item { label: "Font size".into(), key: "font_size",
                       kind: ItemKind::FontSize(FONT_SIZES.iter().position(|&s| s == DEFAULT_FONT).unwrap()) },
                Item { label: "Font weight".into(), key: "font_weight",
                       kind: ItemKind::Toggle(false, "bold", "regular") },
            ]},
            Category { name: "Window".into(), items: vec![
                Item { label: "Opacity (%)".into(),     key: "opacity",      kind: ItemKind::Number(100, 0, 100) },
                Item { label: "Cursor blink (ms)".into(), key: "cursor_blink", kind: ItemKind::Number(0, 0, 2000) },
                Item { label: "Unfocused dim (%)".into(), key: "unfocused_dim", kind: ItemKind::Number(0, 0, 100) },
            ]},
            Category { name: "BG Cycle".into(), items: vec![
                Item { label: "Cycle list".into(), key: "bg_cycle",
                       kind: ItemKind::BgCycle(DEFAULT_BG_CYCLE.split(',').map(|s| s.trim().to_string()).collect()) },
            ]},
            Category { name: "Keybindings".into(), items: vec![
                Item { label: "Font size +".into(), key: "key.font_inc",
                       kind: ItemKind::Keybind("alt+plus".into()) },
                Item { label: "Font size -".into(), key: "key.font_dec",
                       kind: ItemKind::Keybind("alt+minus".into()) },
                Item { label: "Font reset".into(), key: "key.font_reset",
                       kind: ItemKind::Keybind("alt+underscore".into()) },
                Item { label: "BG cycle".into(),   key: "key.bg_cycle",
                       kind: ItemKind::Keybind("alt+b".into()) },
                Item { label: "Opacity toggle".into(), key: "key.opacity",
                       kind: ItemKind::Keybind("alt+t".into()) },
            ]},
        ];
    }

    fn load_config(&mut self) {
        let content = match std::fs::read_to_string(&self.config_path) {
            Ok(c) => c, Err(_) => return,
        };
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            let Some((key, val)) = line.split_once('=') else { continue };
            let key = key.trim();
            let val = val.trim();
            for cat in &mut self.categories {
                for item in &mut cat.items {
                    if item.key != key { continue; }
                    match &mut item.kind {
                        ItemKind::HexColor(s) => *s = val.to_string(),
                        ItemKind::FontSize(idx) => {
                            if let Ok(n) = val.parse::<u32>() {
                                if let Some(i) = FONT_SIZES.iter().position(|&s| s == n) { *idx = i; }
                            }
                        }
                        ItemKind::Number(v, _, _) => {
                            if let Ok(n) = val.parse::<u32>() { *v = n; }
                        }
                        ItemKind::BgCycle(v) => {
                            *v = val.split(',').map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty()).collect();
                        }
                        ItemKind::Toggle(on, on_str, _) => { *on = val == *on_str; }
                        ItemKind::Keybind(s) => { *s = val.to_string(); }
                        _ => {}
                    }
                }
            }
        }
    }

    fn save_config(&self) {
        let mut out = String::from("# glass config — managed by glassconf\n");
        for cat in &self.categories {
            for item in &cat.items {
                match &item.kind {
                    ItemKind::HexColor(s)  => out += &format!("{} = {}\n", item.key, s),
                    ItemKind::FontSize(i)  => out += &format!("{} = {}\n", item.key, FONT_SIZES[*i]),
                    ItemKind::Number(v, _, _) => out += &format!("{} = {}\n", item.key, v),
                    ItemKind::BgCycle(v)   => {
                        if !v.is_empty() { out += &format!("{} = {}\n", item.key, v.join(",")); }
                    }
                    ItemKind::Toggle(on, on_str, off_str) => {
                        out += &format!("{} = {}\n", item.key, if *on { on_str } else { off_str });
                    }
                    ItemKind::Keybind(s) => {
                        out += &format!("{} = {}\n", item.key, s);
                    }
                    ItemKind::Theme => {}
                }
            }
        }
        atomic_write(&self.config_path, out.as_bytes());
    }

    // --- helpers ----------------------------------------------------

    fn current_color(&self, key: &str) -> Option<&str> {
        for cat in &self.categories {
            for item in &cat.items {
                if item.key == key {
                    if let ItemKind::HexColor(s) = &item.kind { return Some(s); }
                }
            }
        }
        None
    }

    fn set_color(&mut self, key: &str, val: &str) {
        for cat in &mut self.categories {
            for item in &mut cat.items {
                if item.key == key {
                    if let ItemKind::HexColor(s) = &mut item.kind { *s = val.into(); }
                }
            }
        }
    }

    // Render a hex color "#rrggbb" as a foreground 24-bit ANSI string applied to text.
    fn fg24(text: &str, hex: &str) -> String {
        if let Some((r, g, b)) = parse_hex(hex) {
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        } else { text.to_string() }
    }
    fn bg24(text: &str, hex: &str) -> String {
        if let Some((r, g, b)) = parse_hex(hex) {
            format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        } else { text.to_string() }
    }

    // --- render -----------------------------------------------------

    fn render(&mut self) {
        let dirty_mark = if self.dirty { " [modified]" } else { "" };
        let bg = self.current_color("bg").unwrap_or("#000000").to_string();
        let fg = self.current_color("fg").unwrap_or("#ffffff").to_string();
        let cursor = self.current_color("cursor").unwrap_or("#f7768e").to_string();
        let preview = format!(" {}{}{}",
            Self::bg24(&Self::fg24(" geir@juba: ~/ ", &fg), &bg),
            Self::bg24(&Self::fg24(">", &cursor), &bg),
            Self::bg24(" ", &bg));
        self.top.say(&format!(" glassconf{}    preview:{}", dirty_mark, preview));

        let mut lines = Vec::new();
        for (i, cat) in self.categories.iter().enumerate() {
            if i == self.cat_index {
                lines.push(style::reverse(&format!(" {} ", cat.name)));
            } else {
                lines.push(format!(" {} ", cat.name));
            }
        }
        self.left.set_text(&lines.join("\n"));
        self.left.ix = 0;
        self.left.full_refresh();

        self.render_items();

        let cat_len = self.categories.get(self.cat_index).map(|c| c.items.len()).unwrap_or(0);
        self.status.say(&format!(
            " {}/{}  j/k:item  J/K:category  h/l:change  Enter:edit  W/s:save  q:quit",
            self.item_index + 1, cat_len));
    }

    fn render_items(&mut self) {
        let Some(cat) = self.categories.get(self.cat_index) else { return };
        let mut lines = Vec::new();
        lines.push(style::fg(&style::bold(&cat.name), 81));
        lines.push(style::fg(&"\u{2500}".repeat(40), 245));
        lines.push(String::new());

        for (i, item) in cat.items.iter().enumerate() {
            let selected = i == self.item_index;
            let label = format!("{:<20}", item.label);
            let label = if selected { style::underline(&label) } else { label };
            let al = if selected { "\u{25C0} " } else { "  " };
            let ar = if selected { " \u{25B6}" } else { "  " };

            let val_str = match &item.kind {
                ItemKind::HexColor(hex) => {
                    let swatch = Self::bg24("    ", hex);
                    format!("{} {}", swatch, hex)
                }
                ItemKind::FontSize(idx) => format!("{}", FONT_SIZES[*idx]),
                ItemKind::Number(v, _, _) => format!("{}", v),
                ItemKind::BgCycle(v) => {
                    let mut s = String::new();
                    for hex in v { s += &Self::bg24("  ", hex); }
                    if v.is_empty() { s = style::fg("(empty)", 245); }
                    format!("{} ({} colors)", s, v.len())
                }
                ItemKind::Toggle(on, on_str, off_str) => {
                    if *on { style::fg(on_str, 82) } else { style::fg(off_str, 245) }
                }
                ItemKind::Keybind(s) => {
                    if s.is_empty() { style::fg("(disabled)", 245) } else { style::fg(s, 81) }
                }
                ItemKind::Theme => style::fg(THEME_NAMES[self.theme_idx], 220),
            };
            lines.push(format!("  {}{}{}{}", label, al, val_str, ar));
        }

        // Live preview block at the bottom
        let bg = self.current_color("bg").unwrap_or("#000000").to_string();
        let fg = self.current_color("fg").unwrap_or("#ffffff").to_string();
        let cursor = self.current_color("cursor").unwrap_or("#f7768e").to_string();
        lines.push(String::new());
        lines.push(style::fg("Live preview:", 245));
        let l1 = format!("{}{}{}",
            Self::bg24(&Self::fg24(" geir@juba: ~/projects ", &fg), &bg),
            Self::bg24(&Self::fg24(">", &cursor), &bg),
            Self::bg24(" ", &bg));
        let l2 = Self::bg24(&format!("{:<60}", " ls -la | grep glass"), &bg);
        let l3 = Self::bg24(&format!("{:<60}", " "), &bg);
        lines.push(l1);
        lines.push(l2);
        lines.push(l3);

        self.right.set_text(&lines.join("\n"));
        self.right.ix = 0;
        self.right.full_refresh();
    }

    // --- navigation -------------------------------------------------

    fn move_down(&mut self) {
        let len = self.categories.get(self.cat_index).map(|c| c.items.len()).unwrap_or(0);
        if self.item_index + 1 < len { self.item_index += 1; }
    }
    fn move_up(&mut self) {
        if self.item_index > 0 { self.item_index -= 1; }
    }
    fn next_category(&mut self) {
        if self.cat_index + 1 < self.categories.len() {
            self.cat_index += 1; self.item_index = 0;
        }
    }
    fn prev_category(&mut self) {
        if self.cat_index > 0 { self.cat_index -= 1; self.item_index = 0; }
    }

    fn next_value(&mut self) {
        let mut new_theme: Option<usize> = None;
        if let Some(cat) = self.categories.get_mut(self.cat_index) {
            if let Some(item) = cat.items.get_mut(self.item_index) {
                match &mut item.kind {
                    ItemKind::FontSize(idx) => {
                        if *idx + 1 < FONT_SIZES.len() { *idx += 1; self.dirty = true; }
                    }
                    ItemKind::Number(v, _, max) => {
                        if *v < *max { *v += 1; self.dirty = true; }
                    }
                    ItemKind::Theme => {
                        self.theme_idx = (self.theme_idx + 1) % THEME_NAMES.len();
                        new_theme = Some(self.theme_idx);
                        self.dirty = true;
                    }
                    ItemKind::Toggle(on, _, _) => { *on = !*on; self.dirty = true; }
                    _ => {}
                }
            }
        }
        if let Some(t) = new_theme { self.apply_theme(t); }
    }

    fn prev_value(&mut self) {
        let mut new_theme: Option<usize> = None;
        if let Some(cat) = self.categories.get_mut(self.cat_index) {
            if let Some(item) = cat.items.get_mut(self.item_index) {
                match &mut item.kind {
                    ItemKind::FontSize(idx) => {
                        if *idx > 0 { *idx -= 1; self.dirty = true; }
                    }
                    ItemKind::Number(v, min, _) => {
                        if *v > *min { *v -= 1; self.dirty = true; }
                    }
                    ItemKind::Theme => {
                        self.theme_idx = (self.theme_idx + THEME_NAMES.len() - 1) % THEME_NAMES.len();
                        new_theme = Some(self.theme_idx);
                        self.dirty = true;
                    }
                    ItemKind::Toggle(on, _, _) => { *on = !*on; self.dirty = true; }
                    _ => {}
                }
            }
        }
        if let Some(t) = new_theme { self.apply_theme(t); }
    }

    fn apply_theme(&mut self, t: usize) {
        let (bg, fg, cursor) = THEMES[t];
        self.set_color("bg", bg);
        self.set_color("fg", fg);
        self.set_color("cursor", cursor);
    }

    fn edit_value(&mut self) {
        let key = match self.categories.get(self.cat_index)
            .and_then(|c| c.items.get(self.item_index))
        {
            Some(it) => it.key,
            None => return,
        };

        // Snapshot current value as initial input
        let (kind_label, initial) = {
            let item = &self.categories[self.cat_index].items[self.item_index];
            let init = match &item.kind {
                ItemKind::HexColor(s) => s.clone(),
                ItemKind::FontSize(i) => FONT_SIZES[*i].to_string(),
                ItemKind::Number(v, _, _) => v.to_string(),
                ItemKind::BgCycle(v) => v.join(","),
                ItemKind::Keybind(s) => s.clone(),
                ItemKind::Theme | ItemKind::Toggle(_, _, _) => return self.next_value(),
            };
            (item.label.clone(), init)
        };

        let orig_bg = self.status.bg;
        self.status.bg = 18;
        let new_val = self.status.ask(&format!("{}: ", kind_label), &initial);
        self.status.bg = orig_bg;
        let new_val = new_val.trim().to_string();
        if new_val.is_empty() { return; }

        let item = &mut self.categories[self.cat_index].items[self.item_index];
        match &mut item.kind {
            ItemKind::HexColor(s) => {
                if normalize_hex(&new_val).is_some() {
                    *s = normalize_hex(&new_val).unwrap();
                    self.dirty = true;
                }
            }
            ItemKind::FontSize(idx) => {
                if let Ok(n) = new_val.parse::<u32>() {
                    if let Some(i) = FONT_SIZES.iter().position(|&s| s == n) {
                        *idx = i; self.dirty = true;
                    }
                }
            }
            ItemKind::Number(v, min, max) => {
                if let Ok(n) = new_val.parse::<u32>() {
                    if n >= *min && n <= *max { *v = n; self.dirty = true; }
                }
            }
            ItemKind::BgCycle(v) => {
                let parsed: Vec<String> = new_val.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| normalize_hex(s).is_some())
                    .map(|s| normalize_hex(&s).unwrap())
                    .collect();
                if !parsed.is_empty() { *v = parsed; self.dirty = true; }
            }
            ItemKind::Keybind(s) => { *s = new_val.to_lowercase(); self.dirty = true; }
            ItemKind::Theme | ItemKind::Toggle(_, _, _) => {}
        }
        let _ = key;
    }
}

// Atomic file replace: write PATH.tmp, rename PATH→PATH.bak, rename
// PATH.tmp→PATH. Guarantees the target file is never empty/truncated
// even if killed mid-save; PATH.bak holds the previous good copy.
fn atomic_write(path: &std::path::Path, data: &[u8]) {
    use std::ffi::OsString;
    let mut tmp: OsString = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let mut bak: OsString = path.as_os_str().to_owned();
    bak.push(".bak");
    if std::fs::write(&tmp, data).is_err() { return; }
    let _ = std::fs::rename(path, &bak);
    let _ = std::fs::rename(&tmp, path);
}

fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 { return None; }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    Some((r, g, b))
}
fn normalize_hex(hex: &str) -> Option<String> {
    parse_hex(hex).map(|(r, g, b)| format!("#{:02x}{:02x}{:02x}", r, g, b))
}

fn main() {
    Crust::init();
    let mut app = App::new();
    app.left.border_refresh();
    app.right.border_refresh();
    app.render();

    loop {
        let Some(key) = Input::getchr(None) else { continue };
        match key.as_str() {
            "q" | "ESC" => {
                if app.dirty {
                    app.status.say(&style::fg(" Save changes? (y/n)", 220));
                    if let Some(k) = Input::getchr(None) {
                        if k == "y" || k == "Y" { app.save_config(); }
                    }
                }
                break;
            }
            "j" | "DOWN" => { app.move_down(); app.render(); }
            "k" | "UP" => { app.move_up(); app.render(); }
            "J" | "PgDOWN" => { app.next_category(); app.render(); }
            "K" | "PgUP" => { app.prev_category(); app.render(); }
            "l" | "RIGHT" | "TAB" => { app.next_value(); app.render(); }
            "h" | "LEFT" | "S-TAB" => { app.prev_value(); app.render(); }
            "ENTER" => { app.edit_value(); app.render(); }
            "W" | "s" => {
                app.save_config();
                app.dirty = false;
                app.status.say(&style::fg(" Config saved", 82));
                std::thread::sleep(std::time::Duration::from_millis(500));
                app.render();
            }
            "RESIZE" => {
                let (cols, rows) = Crust::terminal_size();
                let split = 22u16;
                let lw = split - 1;
                let rx = split + 3;
                let rw = cols.saturating_sub(rx).saturating_sub(1);
                app.top = Pane::new(1, 1, cols, 1, 0, 236);
                app.left = Pane::new(2, 3, lw, rows.saturating_sub(4), 255, 0);
                app.right = Pane::new(rx, 3, rw, rows.saturating_sub(4), 252, 0);
                app.status = Pane::new(1, rows, cols, 1, 252, 236);
                app.left.border = true;
                app.right.border = true;
                Crust::clear_screen();
                app.left.border_refresh();
                app.right.border_refresh();
                app.render();
            }
            _ => {}
        }
    }

    Crust::cleanup();
}
