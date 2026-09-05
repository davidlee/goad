//! Every `workspace.members` entry, read from the root manifest — who the
//! vocabulary scan applies to (`crate::scan`), enumerated rather than
//! hand-listed so a new member cannot arrive unscanned (D13, D17).

use std::path::{Path, PathBuf};

use crate::scan::Breach;

/// Every `workspace.members` entry, as a directory relative to the workspace
/// root, in manifest order.
///
/// # Errors
///
/// Fails on a glob entry — it hides from a reader exactly what this crate
/// exists to make visible — on a manifest that cannot be read or parsed, and
/// on an empty list: a workspace with no members is the vacuity guard one
/// level up. Lists every glob entry found, not the first.
pub fn members(root_manifest: &Path) -> Result<Vec<PathBuf>, Vec<Breach>> {
  let text = std::fs::read_to_string(root_manifest).map_err(|error| {
    vec![Breach::Unreadable {
      path: root_manifest.to_owned(),
      error: error.to_string(),
    }]
  })?;
  let table = text.parse::<toml::Table>().map_err(|error| {
    vec![Breach::Unreadable {
      path: root_manifest.to_owned(),
      error: error.to_string(),
    }]
  })?;

  let entries: Vec<&str> = table
    .get("workspace")
    .and_then(toml::Value::as_table)
    .and_then(|workspace| workspace.get("members"))
    .and_then(toml::Value::as_array)
    .into_iter()
    .flatten()
    .filter_map(toml::Value::as_str)
    .collect();

  let mut paths = Vec::new();
  let mut breaches = Vec::new();
  for entry in entries {
    if entry.contains('*') {
      breaches.push(Breach::GlobMember {
        entry: entry.to_owned(),
      });
    } else {
      paths.push(PathBuf::from(entry));
    }
  }

  if breaches.is_empty() && paths.is_empty() {
    breaches.push(Breach::Vacuous {
      root: root_manifest.to_owned(),
    });
  }

  if breaches.is_empty() {
    Ok(paths)
  } else {
    Err(breaches)
  }
}
