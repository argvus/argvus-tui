# ARGVUS TUI

[![License: GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue.svg)](LICENSE)

Shared Rust libraries for the ARGVUS terminal user interfaces. This workspace
does not provide a standalone application: it supplies the common terminal
lifecycle, Ratatui widgets, page primitives, icon catalog, image rendering,
text helpers, and theme resolution used by ARGVUS applications.

The libraries are intended to keep the visual language and terminal behavior
consistent across projects such as:

- [`argvus-greeter`](https://github.com/argvus/argvus-greeter)
- [`argvus-tui-terminal`](https://github.com/argvus/argvus-tui-terminal)
- [`argvus-control-center`](https://github.com/argvus/argvus-control-center)

## Workspace crates

### `argvus-tui`

The shared TUI toolkit built on [Ratatui](https://ratatui.rs/) and
[Crossterm](https://github.com/crossterm-rs/crossterm). It provides:

- `terminal`: raw mode, alternate-screen, mouse and bracketed-paste lifecycle;
- `chrome`: headers, footers, help dialogs, and small-terminal messages;
- `page`: shared page shells, lists, read-only content, status areas, and
  keyboard selection handling;
- `components` and `buttons`: status messages, confirmation dialogs, and
  semantic buttons;
- `icons`: the shared Material Design Icons catalog for Nerd Fonts;
- `image`: PNG, JPEG, WebP, and SVG loading with terminal graphics protocols
  and a text fallback;
- `text`: display-width, truncation, and ellipsis helpers.

The recommended minimum terminal size exposed by the crate is `60x18`.

### `argvus-theme`

The shared semantic theme layer for TUI consumers. It loads ARGVUS CSS theme
resources, resolves palette references, composites alpha colors, and exposes a
`Theme` suitable for Ratatui styles. When resources or user selection are
missing or invalid, it falls back to the safe `argvus-dark` theme.

Theme selection can be supplied explicitly for pre-authentication surfaces:

```rust
let theme = argvus_theme::Theme::load_for_theme_name("silver-dark");
```

The regular loader reads the active theme from the ARGVUS configuration and
looks for resources in the installed ARGVUS tree or the development source
tree. `ARGVUS_CONTROL_CENTER_RESOURCE_DIR` can be used to point it at a custom
resource directory, while `ARGVUS_SYSTEM_CONFIG` changes the installed config
root.

## Usage

Add the crates as path dependencies while developing the ARGVUS workspace, or
use the published package source when it is available:

```toml
[dependencies]
argvus-tui = { path = "../argvus-tui/crates/argvus-tui" }
argvus-theme = { path = "../argvus-tui/crates/argvus-theme" }
```

A minimal terminal setup is:

```rust
use argvus_theme::Theme;
use argvus_tui::terminal::{TerminalGuard, install_panic_hook};

install_panic_hook();
let mut terminal = TerminalGuard::new()?;
let theme = Theme::load();

terminal.terminal_mut().draw(|frame| {
    // Compose application-specific routes and shared ARGVUS widgets here.
    let _ = (frame, &theme);
})?;
```

Applications own their domain state, routes, event loop, and localized user
content. These crates own reusable terminal and presentation behavior; they do
not contain application-specific backends or settings workflows.

## Development

Requirements are pinned by [`rust-toolchain.toml`](rust-toolchain.toml). Common
workspace commands are:

```sh
cargo build --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
make check
```

See [`DEVELOPMENT.md`](DEVELOPMENT.md) for repository conventions and the
release workflow.

## License

Copyright © ARGVUS. Distributed under the
[GNU General Public License, version 3.0 only](LICENSE).
