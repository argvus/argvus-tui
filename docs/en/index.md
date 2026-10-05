---
title: ARGVUS TUI
description: Terminal User Interface framework for ARGVUS
---

# ARGVUS TUI

ARGVUS TUI is a Terminal User Interface framework that provides consistent, keyboard-first UI components for text-based ARGVUS applications and tools.

## Features

- **Keyboard-First Navigation**: Designed for efficient keyboard-driven interaction
- **Consistent Styling**: Unified visual appearance across TUI applications
- **Theme Support**: Adapts to ARGVUS color schemes
- **Accessibility**: Built with accessible text interfaces in mind

## Components

The TUI framework includes:
- Text input fields
- Selection menus and lists
- Progress indicators
- Dialog boxes
- Status displays
- Help and documentation viewers

### Single menu list (`menu`)

Every page is one vertical list of typed rows (`RowKind`): **Info** (label
and value, never focused), **Action**, **Submenu** (`›` marker), **Toggle**
(`[x]`/`[ ]`), **Choice** (`●` on the current option), **Value** (Enter
edits; `←/→` adjust when it has a step), **Destructive** (always goes
through the confirmation) and **Separator**. Each row carries its own icon
from the `icons` catalog; Info rows have no decorative icon.

The cursor (`MenuState`) only rests on selectable rows: Info rows,
separators and disabled rows are skipped by `↑/↓`, `j/k`, `PgUp/PgDn` and
`Home/End`, and the cursor stops at both ends instead of wrapping. A page
with no selectable row only scrolls. With icons turned off, the icon column
disappears and labels stay aligned.

### Confirmation (`confirm`)

One component confirms destructive or privileged actions: a title, the
message and two rows, Confirm and Cancel. The focus starts on Cancel.
`Enter` runs the focused row, `y` confirms and `n`/`Esc` cancel. It can
show a countdown (for example, to revert a display configuration).

### Contextual footer (`hints`)

The help footer is built from the selected row's kind and the page's
capabilities (search, refresh, back, quit), showing only the keys that
apply to the current row. Texts come from the `control-center` catalog of
`argvus-i18n` (`control_center.hint.*` keys).

## Usage

ARGVUS TUI is used in:
- System administration tools
- Installation and setup utilities
- Status and monitoring displays
- Configuration interfaces

## For Developers

To use ARGVUS TUI in your application:

1. Add `argvus-tui` as a dependency
2. Import TUI components into your project
3. Use the provided API for UI elements
4. Follow ARGVUS UI guidelines

For detailed API documentation, see the repository.
