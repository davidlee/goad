//! The kit's root and its files, enumerated, never listed.
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

/// `kit/`, from this package's manifest: a test's working directory is the
/// package root, not the workspace's.
pub(crate) fn root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kit")
}

/// Every regular file under `dir`, recursively, sorted. A unix socket is
/// skipped by type: a host run on an example's config leaves one there,
/// wherever the host started, because a relative `ingress.path` resolves
/// beside the config (slice 012 PHASE-06 A-V6), and opening it fails. Anything else (a
/// symlink) is not expected.
pub(crate) fn files(dir: &Path) -> Vec<PathBuf> {
  let mut found = Vec::new();
  for entry in
    fs::read_dir(dir).unwrap_or_else(|e| panic!("{} must be readable: {e}", dir.display()))
  {
    let entry = entry.expect("a directory entry");
    let (path, kind) = (entry.path(), entry.file_type().expect("a file type"));
    if kind.is_dir() {
      found.extend(files(&path));
    } else if kind.is_file() {
      found.push(path);
    } else if !kind.is_socket() {
      panic!(
        "{} is neither a file, a directory nor a socket",
        path.display()
      );
    }
  }
  found.sort();
  found
}

/// A kit file's text. The kit ships text, so one that is not UTF-8 fails.
pub(crate) fn text(file: &Path) -> String {
  fs::read_to_string(file)
    .unwrap_or_else(|e| panic!("{} must be readable UTF-8: {e}", file.display()))
}
