//! Implements resource loading in crate `argvus theme`. This separation keeps external effects from contaminating models, routes, or rendering.
//!
//! External tool dependencies remain in backend layers;
//! the UI consumes normalized models and results.
use std::fs;
use std::path::{Path, PathBuf};

use super::parser::{self, Rgba};

/// Defines the constant `DEFAULT_THEME`. Its explicit shape preserves the contract consumed by the rest of the workspace and keeps the intent visible as the module evolves.
pub const DEFAULT_THEME: &str = "argvus-dark-aether";

/// Official ARGVUS TUI theme identifiers accepted by pre-authentication
/// surfaces and shared theme consumers.
pub const OFFICIAL_THEMES: &[&str] = &[
  "argvus-dark-aether",
  "argvus-dark-aether-float",
  "argvus-dark-silver",
  "argvus-dark-silver-float",
  "argvus-dark-slate",
  "argvus-dark-slate-float",
  "argvus-dark-universe",
  "argvus-dark-universe-float",
  "argvus-light-veil",
  "argvus-light-veil-float",
];

/// Represents `Loader`. Its explicit shape preserves the contract consumed by the rest of the workspace and keeps the intent visible as the module evolves.
pub struct Loader {
  resource_dir: PathBuf,
  active_file: PathBuf,
  cache_file: PathBuf,
  active_name_override: Option<String>,
  accent_override: Option<Rgba>,
}

impl Loader {
  /// Constructs `new` with this module's expected initial state. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
  pub fn new() -> Self {
    let explicit = std::env::var_os("ARGVUS_CONTROL_CENTER_RESOURCE_DIR").map(PathBuf::from);
    let installed_root = std::env::var_os("ARGVUS_SYSTEM_CONFIG")
      .map(PathBuf::from)
      .unwrap_or_else(|| PathBuf::from("/usr/share/argvus"));
    let installed = installed_root.join("appearance/config/tui");
    let legacy_installed = installed_root.join("control-center/config");
    let development = development_resources_dir();
    let resource_dir = explicit.unwrap_or_else(|| {
      if installed.is_dir() {
        installed
      } else if legacy_installed.is_dir() {
        legacy_installed
      } else {
        development
      }
    });
    let cache_home = std::env::var_os("XDG_CACHE_HOME")
      .map(PathBuf::from)
      .unwrap_or_else(|| home().join(".cache"));
    Self {
      resource_dir,
      active_file: argvus_config_home().join(".active-theme"),
      cache_file: cache_home.join("argvus-control-center/theme.css"),
      active_name_override: None,
      accent_override: None,
    }
  }

  /// Creates a loader whose active theme comes from a validated external
  /// selection instead of the current process user's private config.
  ///
  /// This is used by pre-authentication surfaces, which run as the `greeter`
  /// account and cannot safely read a target user's `$HOME` before login.
  pub fn with_active_name(mut self, name: &str) -> Self {
    self.active_name_override = Some(normalize_theme_name(name).to_string());
    self
  }

  /// Applies a validated opaque accent supplied by a pre-authentication
  /// projection. Invalid values are ignored so callers retain the theme
  /// palette as the safe fallback.
  pub fn with_accent_override(mut self, accent: &str) -> Self {
    let accent = accent.trim();
    if accent.len() == 7
      && accent.starts_with('#')
      && accent[1..]
        .chars()
        .all(|character| character.is_ascii_hexdigit())
    {
      self.accent_override = parser::parse_color(accent);
    }
    self
  }

  pub fn accent_override(&self) -> Option<Rgba> {
    self.accent_override
  }

  /// Executes the `active_name` step in this module. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
  pub fn active_name(&self) -> String {
    if let Some(name) = &self.active_name_override {
      return name.clone();
    }
    fs::read_to_string(&self.active_file)
      .ok()
      .map(|value| value.trim().to_string())
      .filter(|value| !value.is_empty())
      .unwrap_or_else(|| DEFAULT_THEME.to_string())
  }

  /// Executes the `sources` step in this module. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
  pub fn sources(&self) -> Vec<PathBuf> {
    let active = self.active_name();
    vec![
      self.resource_dir.join("style.css"),
      self.resource_dir.join("theme.css"),
      self
        .resource_dir
        .join("themes")
        .join(format!("{active}.css")),
      self.cache_file.clone(),
    ]
  }
}

/// Returns whether `name` is an official ARGVUS theme identifier.
pub fn is_official_theme(name: &str) -> bool {
  OFFICIAL_THEMES.contains(&name)
}

/// Normalizes untrusted or missing theme state to the safe ARGVUS default.
pub fn normalize_theme_name(name: &str) -> &str {
  let name = name.trim();
  if is_official_theme(name) {
    name
  } else {
    DEFAULT_THEME
  }
}

impl Default for Loader {
  /// Executes the `default` step in this module. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
  fn default() -> Self {
    Self::new()
  }
}

/// Executes the `argvus_config_home` step in this module. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
fn argvus_config_home() -> PathBuf {
  std::env::var_os("ARGVUS_CONFIG_HOME")
    .map(PathBuf::from)
    .unwrap_or_else(|| {
      std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
    })
    .join("argvus")
}

/// Executes the `home` step in this module. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
fn home() -> PathBuf {
  std::env::var_os("HOME")
    .map(PathBuf::from)
    .unwrap_or_else(|| PathBuf::from("/tmp"))
}

/// Executes the `development_resources_dir` step in this module. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
fn development_resources_dir() -> PathBuf {
  let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let sources = [
    manifest_dir.join("../../../argvus-appearance/src/usr/share/argvus/appearance/config/tui"),
    manifest_dir.join("../../src/usr/share/argvus/control-center/config"),
  ];
  if let Some(source) = sources.into_iter().find(|candidate| candidate.is_dir()) {
    return source;
  }
  std::env::current_dir()
    .ok()
    .and_then(|dir| {
      [
        dir.join("../argvus-appearance/src/usr/share/argvus/appearance/config/tui"),
        dir.join("src/usr/share/argvus/control-center/config"),
        dir.join("../../src/usr/share/argvus/control-center/config"),
        dir.join("resources"),
        dir.join("argvus-control-center/resources"),
        dir.join("../../resources"),
      ]
      .into_iter()
      .find(|candidate| candidate.is_dir())
    })
    .unwrap_or_else(|| PathBuf::from("resources"))
}

/// Retrieves data for `read_recursive` without mixing collection with TUI rendering. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
pub fn read_recursive(path: &Path, stack: &mut Vec<PathBuf>) -> Option<String> {
  let contents = fs::read_to_string(path).ok()?;
  let mut output = String::new();
  for line in contents.lines() {
    let trimmed = line.trim();
    let import = trimmed
      .strip_prefix("@import url(")
      .and_then(|value| value.strip_suffix(");").or_else(|| value.strip_suffix(')')))
      .map(|value| value.trim().trim_matches(['\'', '"']))
      .filter(|value| !value.is_empty())
      .map(|value| path.parent().unwrap_or(Path::new(".")).join(value));
    if let Some(import) = import {
      if !stack.contains(&import) {
        stack.push(import.clone());
        if let Some(imported) = read_recursive(&import, stack) {
          output.push_str(&imported);
        }
        stack.pop();
      }
    } else {
      output.push_str(line);
      output.push('\n');
    }
  }
  Some(output)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  /// Executes the `every_official_theme_exposes_the_shared_palette` step in this module. The behavior is encapsulated here so callers depend on a clear domain decision instead of duplicating system or UI details.
  fn every_official_theme_exposes_the_shared_palette() {
    let themes = development_resources_dir().join("themes");
    let entries = fs::read_dir(themes).expect("official theme directory");
    let mut count = 0;
    for entry in entries.flatten() {
      if entry.path().extension().and_then(|value| value.to_str()) != Some("css") {
        continue;
      }
      let css = read_recursive(&entry.path(), &mut Vec::new()).expect("read official theme");
      let definitions = crate::parser::extract_define_colors(&css);
      assert!(
        definitions.iter().any(|(name, _)| name == "argvus_bg"),
        "{} has no background",
        entry.path().display()
      );
      assert!(
        definitions.iter().any(|(name, _)| name == "argvus_accent"),
        "{} has no accent",
        entry.path().display()
      );
      count += 1;
    }
    assert!(count >= 10, "expected all packaged ARGVUS themes");
  }

  #[test]
  fn official_theme_validation_covers_the_declared_families() {
    for name in [
      "argvus-dark-aether",
      "argvus-dark-silver",
      "argvus-dark-universe",
      "argvus-light-veil",
    ] {
      assert!(is_official_theme(name));
      assert_eq!(normalize_theme_name(name), name);
    }
    assert_eq!(normalize_theme_name("invalid"), DEFAULT_THEME);
  }
}
