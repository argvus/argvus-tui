//! Root module of crate `argvus theme`. It exposes public boundaries and composes internal responsibilities without duplicating domain rules.
//!
//! External tool dependencies remain in backend layers;
//! the UI consumes normalized models and results.
use ratatui::style::Color;

pub mod fallback;
pub mod loader;
pub mod parser;
pub mod resolver;

#[derive(Debug, Clone)]
/// Represents `Theme`. Its explicit shape preserves the contract consumed by the rest of the workspace and keeps the intent visible as the module evolves.
pub struct Theme {
  pub name: String,
  pub is_light: bool,
  pub background: Color,
  pub foreground: Color,
  pub accent: Color,
  pub tab_active: Color,
  pub tab_inactive: Color,
  pub selected_background: Color,
  pub selected_foreground: Color,
  pub border: Color,
  pub border_active: Color,
  pub muted: Color,
  pub link: Color,
  pub success: Color,
  pub warning: Color,
  pub error: Color,
  pub surface: Color,
  pub accent_alpha: Color,
}

impl Theme {
  /// Retrieves data for `load` without mixing collection with TUI rendering. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
  pub fn load() -> Self {
    resolver::resolve(&loader::Loader::new())
  }

  /// Loads a theme selected by an external account or pre-authentication
  /// source without changing process-wide XDG environment variables.
  pub fn load_for_theme_name(name: &str) -> Self {
    resolver::resolve(&loader::Loader::new().with_active_name(name))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  /// Retrieves data for `load_always_produces_a_readable_theme` without mixing collection with TUI rendering. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
  fn load_always_produces_a_readable_theme() {
    let theme = Theme::load();
    assert!(!theme.name.is_empty());
    assert_ne!(theme.background, theme.foreground);
  }

  #[test]
  fn load_for_theme_name_uses_external_selection() {
    let theme = Theme::load_for_theme_name("argvus-dark-silver");
    assert_eq!(theme.name, "argvus-dark-silver");
  }

  #[test]
  fn load_for_theme_name_falls_back_for_unknown_selection() {
    let theme = Theme::load_for_theme_name("../../outside");
    assert_eq!(theme.name, loader::DEFAULT_THEME);
  }
}
