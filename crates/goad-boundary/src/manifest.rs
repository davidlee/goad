//! The manifest allowlist: which dependency names a stratum may carry (§5.6).
//!
//! An allowlist, not a denylist — a denylist of runtime, renderer and
//! filesystem crate names requires classifying an unbounded universe and lets
//! a new dependency through by not being on it. The allowlist fails closed.

use std::path::Path;

use crate::scan::Breach;

/// Every table named `dependencies`, `dev-dependencies` or
/// `build-dependencies`, at any depth — so `[target.'cfg(unix)'.dependencies]`
/// cannot slip by.
const DEPENDENCY_TABLES: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];

/// `Ok(n)` = dependency entries inspected, across every dependency table the
/// manifest carries.
///
/// # Errors
///
/// Lists every unpermitted entry, not the first. A renamed entry —
/// `clock = { package = "tokio" }` — is checked by its `package` value as well
/// as its key. Zero entries across all tables is a `Breach::Vacuous`, the same
/// guard `Scan::run` already carries.
pub fn unpermitted(manifest: &Path, text: &str, permitted: &[&str]) -> Result<usize, Vec<Breach>> {
  let table: toml::Table = toml::from_str(text).map_err(|error| {
    vec![Breach::Unreadable {
      path: manifest.to_owned(),
      error: error.to_string(),
    }]
  })?;

  let mut dependency_tables = Vec::new();
  collect_dependency_tables(&table, &mut dependency_tables);

  let mut inspected = 0usize;
  let mut breaches = Vec::new();
  for (key, value) in dependency_tables.iter().flat_map(|deps| deps.iter()) {
    inspected += 1;
    let package = value
      .as_table()
      .and_then(|attributes| attributes.get("package"))
      .and_then(toml::Value::as_str);
    let name = package.unwrap_or(key.as_str());
    if !permitted.contains(&name) {
      breaches.push(Breach::Token {
        path: manifest.to_owned(),
        line: 0, // not tracked: the parser this reads through carries no span.
        token: name.to_owned().into(),
      });
    }
  }

  if inspected == 0 {
    breaches.push(Breach::Vacuous {
      root: manifest.to_owned(),
    });
  }

  if breaches.is_empty() {
    Ok(inspected)
  } else {
    Err(breaches)
  }
}

/// Recurses through every nested table looking for one of `DEPENDENCY_TABLES`
/// by name, at any depth — which is how `[target.'cfg(unix)'.dependencies]` is
/// found beside a plain `[dependencies]`.
fn collect_dependency_tables<'a>(table: &'a toml::Table, out: &mut Vec<&'a toml::Table>) {
  for (key, value) in table {
    if let toml::Value::Table(nested) = value {
      if DEPENDENCY_TABLES.contains(&key.as_str()) {
        out.push(nested);
      }
      collect_dependency_tables(nested, out);
    }
  }
}
