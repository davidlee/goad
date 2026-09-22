//! `--version`, against the built binary. The spawn helpers are
//! `crate::process`'s — see there for why they are not in this file.
use crate::process::{code_of, goad, stderr_of, stdout_of};

/// 006/PHASE-03/VT-2, AC-4: the version on **stdout**, so a caller can read it
/// as a value, and exit 0.
///
/// Until 006 this invocation was read as a configuration path: `goad` opened a
/// file called `--version`, failed, and exited 2 with a line on stderr. Both
/// halves of that regression are pinned here — the code, and stderr being
/// empty.
#[test]
fn version_prints_the_package_version_on_stdout_and_exits_0() {
  let output = goad(&["--version"]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  assert_eq!(stdout_of(&output).trim_end(), env!("CARGO_PKG_VERSION"));
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}
