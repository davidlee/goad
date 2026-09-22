//! Every line the binary can write, as a `String` with no sink.
//!
//! Pure, for the reason `diagnostics.rs`'s own `*_line` functions are: a line
//! is asserted with a literal, and the one place a stream is touched is
//! `main`. Nothing here reads an environment, a clock, a file or a socket.
//!
//! **Success renders nothing** (005/D-7). A cron job that prints on success
//! trains its owner to ignore its output, so the acceptance path has no line
//! at all — which is why [`refused_line`] answers an acceptance with `""`.

use std::path::Path;

use goad_shell::ingress::client::{Answered, SendFault};

use crate::StartupFault;
use crate::args::UsageError;

/// One `const`, no trailing newline — `report::line_to`'s `writeln!` supplies
/// the one, and two sources of that newline would be a fact stated twice.
///
/// Two sentences here are load-bearing rather than decorative: the wait has no
/// deadline (005/D-10, `SPEC-003` §6.4), and discovery covers the host's
/// **default** configuration path only, so a host started on an explicit
/// configuration is reached with `--socket` (F-6).
///
/// The discovery paragraph is prose *about*
/// [`goad_shell::config::default_path`], which is where the rule is
/// executable and where its own doc table states it; this copy is not the
/// authority and a change to that function is what must drive a change here
/// (F-8). It states the `HOME`-empty row for that reason: without it the
/// paragraph reads as though `$HOME/.config/goad/config.toml` is always the
/// fallback, which the function has never said.
pub(crate) const USAGE: &str = "usage: goad-emit --source S --kind K [--data JSON] [--socket PATH]
       goad-emit -h | --help
       goad-emit --version

Writes one event to a running goad host: exit 0 if the host accepted it, 1 if
the host refused it, and 2 if no usable answer could be had.

  --source S    who observed the event. \"host\" is reserved to the host itself.
  --kind K      what happened, in the backend's own vocabulary.
  --data JSON   carried to the backend untouched; defaults to null.
  --socket PATH the socket to write to, consulting no configuration at all.

Without --socket the path is read from the ingress section of
$XDG_CONFIG_HOME/goad/config.toml, or of $HOME/.config/goad/config.toml when
XDG_CONFIG_HOME is unset, empty or not absolute. When HOME is unset or empty
as well, there is no default path at all and emit says so rather than guessing
one: pass --socket. That is the host's default configuration path and the only
one emit looks at: a host started on an explicit configuration file is reached
with --socket.

The host answers when it has judged the event, and takes as long as that takes;
emit sets no deadline of its own. Wrap it if you need one:

  timeout 5 goad-emit --source cron --kind woke";

/// A refusal, as the caller's one line of stderr: the reason **token** first,
/// so a wrapper can branch on it, then `retry_after_ms` exactly when the host
/// sent it, then `detail`.
///
/// `detail` is *interpolated* and never read (`SPEC-003/R-14`). There is no
/// arm on it here and no test can hold its absence — review does.
///
/// An acceptance renders the empty string. Success is silent (005/D-7), so
/// `main` never asks; this is that "nothing", spelled so it cannot become a
/// line by accident.
#[must_use]
pub(crate) fn refused_line(answered: &Answered) -> String {
  match answered {
    Answered::Accepted => String::new(),
    Answered::Refused {
      reason,
      retry_after_ms,
      detail,
    } => {
      let advice = retry_after_ms
        .map(|milliseconds| format!(" retry_after_ms={milliseconds}"))
        .unwrap_or_default();
      let detail = detail
        .as_ref()
        .map(|detail| format!(" ({detail})"))
        .unwrap_or_default();
      format!("goad-emit: refused: {reason}{advice}{detail}")
    }
  }
}

/// No usable answer, and which of the six it was. The path is named in every
/// one of them: *unreachable* means nothing about the host without saying
/// where emit looked (AC-3).
///
/// The six divide by **which side was wrong**, which is the obligation
/// `SendFault`'s own doc carries: the first two are the transport or the
/// caller's path, and the last four are the host — one that said nothing, one
/// that never stopped talking, one whose bytes were not a document, and one
/// whose document was not a verdict.
#[must_use]
pub(crate) fn fault_line(fault: &SendFault, path: &Path) -> String {
  let path = path.display();
  match fault {
    SendFault::Unreachable(error) => {
      format!("goad-emit: nothing is listening at {path}: {error}")
    }
    SendFault::Faulted(error) => {
      format!("goad-emit: the connection to {path} faulted: {error}")
    }
    SendFault::NoReply => {
      format!("goad-emit: {path} closed without answering, which SPEC-003/R-8 forbids")
    }
    SendFault::Oversized { limit } => {
      format!("goad-emit: the reply from {path} did not end within {limit} bytes")
    }
    SendFault::Unreadable(error) => {
      format!("goad-emit: the reply from {path} is not one JSON object: {error}")
    }
    SendFault::NonConforming(breach) => {
      format!("goad-emit: the reply from {path} breaches SPEC-003 6.3: {breach}")
    }
  }
}

/// A usage error, naming the flag at fault and **not reprinting the usage
/// block** — `crates/goad`'s principle 4: a caller who mistyped one flag does
/// not need the whole page again, and `--help` is where the page lives.
#[must_use]
pub(crate) fn usage_error_line(error: &UsageError) -> String {
  match error {
    UsageError::Missing(flag) => format!("goad-emit: {flag} is required"),
    UsageError::Empty(flag) => format!("goad-emit: {flag} was given an empty value"),
    UsageError::NoValue(flag) => format!("goad-emit: {flag} needs a value after it"),
    UsageError::Repeated(flag) => format!("goad-emit: {flag} was given more than once"),
    UsageError::NotUtf8(flag) => format!("goad-emit: {flag} was given a value that is not UTF-8"),
    UsageError::NotJson { raw, fault } => {
      format!("goad-emit: --data is not JSON: {fault}, in {raw}")
    }
    UsageError::Unknown(flag) => {
      format!("goad-emit: unknown flag {flag}; run `goad-emit --help` for usage")
    }
    UsageError::Positional(argument) => {
      format!("goad-emit: unexpected argument {argument}; emit takes flags only")
    }
  }
}

/// The envelope never left, and why. Each of the four configuration faults
/// names the path, because the remedy is a thing done *to that file* — and
/// the fifth names the clock.
#[must_use]
pub(crate) fn startup_error_line(error: &StartupFault) -> String {
  match error {
    StartupFault::NoPath => "goad-emit: neither XDG_CONFIG_HOME nor HOME names a directory, so \
                             there is no configuration path to read; pass --socket"
      .to_owned(),
    StartupFault::Unreadable { path, fault } => {
      format!("goad-emit: {} could not be read: {fault}", path.display())
    }
    StartupFault::Unparseable { path, fault } => {
      format!("goad-emit: {}: {fault}", path.display())
    }
    StartupFault::NoIngress { path } => format!(
      "goad-emit: {} has no [ingress] section, so that host is not listening",
      path.display()
    ),
    StartupFault::ClockUnreadable(fault) => {
      format!("goad-emit: the clock could not be read, so the event has no timestamp: {fault}")
    }
  }
}

/// The `--version` answer: the package version, and the revision the build
/// stamped — when it stamped one.
///
/// **No placeholder for the unstamped case**, and no `"goad-emit: "` prefix:
/// this is a direct answer on stdout, not a report about a fault. The shape
/// and the reasoning are `crates/goad`'s `diagnostics::version_line`; the two
/// are separate because stratum 3 has two binaries and no shared crate, and
/// each reads the `GOAD_REVISION` of its own compilation (design.md §5.2(g)).
///
/// **Set-but-empty is unset**, and the test is here rather than at the call
/// site for the reason the sibling states: one function decides the line, so
/// the unit tier can reach the rule and the two crates cannot drift apart
/// unnoticed (`review-code.md` F-2).
#[must_use]
pub(crate) fn version_line(revision: Option<&str>) -> String {
  match revision.filter(|revision| !revision.is_empty()) {
    Some(revision) => format!("{} ({revision})", env!("CARGO_PKG_VERSION")),
    None => env!("CARGO_PKG_VERSION").to_owned(),
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use goad_shell::clock::ClockError;
  use goad_shell::error::ConfigError;
  use goad_shell::ingress::client::{Answered, SendFault};

  use super::{fault_line, refused_line, startup_error_line, usage_error_line, version_line};
  use crate::StartupFault;
  use crate::args::UsageError;

  /// 006/PHASE-03/VT-1, AC-5's rendering half. The **stamped** branch is
  /// unreachable anywhere else under `cargo test`: nothing in the gate sets
  /// `GOAD_REVISION`, so `tests/binary/`'s own `--version` case can only ever
  /// see the bare form. This case is the real assertion for it, not a proxy
  /// for one (`docs/memory/tests-asserting-proxies.md`).
  #[test]
  fn a_stamped_build_names_its_revision_beside_the_version() {
    assert_eq!(version_line(Some("08528b5")), "0.1.0 (08528b5)");
  }

  /// No placeholder. A build that stamped no revision says only what is
  /// known — the `cargo install` path is exactly this branch, and it is not a
  /// fault to be reported.
  #[test]
  fn an_unstamped_build_says_only_the_version() {
    assert_eq!(version_line(None), "0.1.0");
  }

  /// **Set-but-empty is unset** (`review-code.md` F-2). A tarball consumer of
  /// the flake stamps `GOAD_REVISION=""`, which `option_env!` hands on as
  /// `Some("")`; without the filter this binary prints `0.1.0 ()`.
  #[test]
  fn a_build_stamped_with_an_empty_revision_is_an_unstamped_build() {
    assert_eq!(version_line(Some("")), "0.1.0");
  }

  fn refusal(reason: &str, retry_after_ms: Option<u64>, detail: Option<&str>) -> Answered {
    Answered::Refused {
      reason: reason.to_owned(),
      retry_after_ms,
      detail: detail.map(ToOwned::to_owned),
    }
  }

  /// PHASE-03/VT-4, AC-2.
  #[test]
  fn a_refusal_shows_its_reason_token() {
    let line = refused_line(&refusal("engaged", None, None));
    assert!(line.contains("engaged"), "{line}");
    assert!(!line.contains("retry_after_ms"), "{line}");
  }

  #[test]
  fn retry_after_ms_is_shown_exactly_when_it_is_there() {
    let line = refused_line(&refusal("too_soon", Some(1401), None));
    assert!(line.contains("too_soon"), "{line}");
    assert!(line.contains("retry_after_ms=1401"), "{line}");
  }

  /// `detail` is interpolated, never branched on (`SPEC-003/R-14`). What no
  /// test can hold is the *absence* of a branch; review holds that, and this
  /// case holds only that the text arrives.
  #[test]
  fn detail_is_carried_into_the_line() {
    let line = refused_line(&refusal(
      "invalid_envelope",
      None,
      Some("key `kind` is missing"),
    ));
    assert!(line.contains("key `kind` is missing"), "{line}");
  }

  /// D-7: success is silent, so an acceptance renders nothing. `main` never
  /// asks — this is what "nothing" is, spelled out.
  #[test]
  fn an_acceptance_renders_nothing() {
    assert_eq!(refused_line(&Answered::Accepted), "");
  }

  /// PHASE-03/VT-2, AC-3: every usage error names what was wrong with it, and
  /// none of them reprints the usage block (`crates/goad`'s principle 4).
  ///
  /// The third column is what makes this a test of the **mapping** rather
  /// than of the set (F-7): naming the flag is a property every arm has by
  /// construction, so a case asserting only that stays green when two arms
  /// are swapped and a caller is told `--kind was given an empty value` when
  /// `--kind` was never given at all.
  #[test]
  fn every_usage_error_names_the_flag_and_reprints_nothing() {
    let cases: [(UsageError, &str, &str); 8] = [
      (UsageError::Missing("--source"), "--source", "is required"),
      (UsageError::Empty("--kind"), "--kind", "an empty value"),
      (
        UsageError::NoValue("--data"),
        "--data",
        "needs a value after it",
      ),
      (
        UsageError::Repeated("--socket"),
        "--socket",
        "more than once",
      ),
      (
        UsageError::NotUtf8("--source"),
        "--source",
        "a value that is not UTF-8",
      ),
      (
        UsageError::NotJson {
          raw: "{oops".to_owned(),
          fault: serde_json::from_str::<serde_json::Value>("{oops").expect_err("this is not JSON"),
        },
        "--data",
        "is not JSON",
      ),
      (
        UsageError::Unknown("--socket=x".to_owned()),
        "--socket=x",
        "unknown flag",
      ),
      (
        UsageError::Positional("stray".to_owned()),
        "stray",
        "unexpected argument",
      ),
    ];
    for (error, named, said) in cases {
      let line = usage_error_line(&error);
      assert!(line.contains(named), "{line} must name {named}");
      assert!(line.contains(said), "{line} must say {said}");
      assert!(
        !line.contains("usage:"),
        "{line} must not reprint the usage block"
      );
    }
  }

  /// PHASE-03/VT-6, AC-4: each way the configuration road ends names the path
  /// and the fault. The fifth — the clock — names neither, because neither is
  /// what went wrong.
  ///
  /// Each of the three file faults is paired with a phrase only its own arm
  /// says (F-7). Naming the file is common to all three, so a case asserting
  /// only that survives swapping `Unreadable` and `Unparseable` — after which
  /// a user whose configuration is absent is told it is unparseable.
  #[test]
  fn every_startup_fault_names_the_path_and_what_was_wrong() {
    let path = Path::new("/home/someone/.config/goad/config.toml");
    let naming_the_file = [
      (
        StartupFault::Unreadable {
          path: path.to_owned(),
          fault: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        "could not be read",
      ),
      (
        StartupFault::Unparseable {
          path: path.to_owned(),
          fault: ConfigError::EmptyCommand,
        },
        "names no program",
      ),
      (
        StartupFault::NoIngress {
          path: path.to_owned(),
        },
        "has no [ingress] section",
      ),
    ];
    for (fault, said) in naming_the_file {
      let line = startup_error_line(&fault);
      assert!(line.contains("config.toml"), "{line}");
      assert!(line.contains(said), "{line} must say {said}");
    }

    let nowhere = startup_error_line(&StartupFault::NoPath);
    assert!(nowhere.contains("XDG_CONFIG_HOME"), "{nowhere}");
    assert!(nowhere.contains("HOME"), "{nowhere}");
    assert!(nowhere.contains("--socket"), "{nowhere}");

    let clock = startup_error_line(&StartupFault::ClockUnreadable(ClockError::BeforeEpoch));
    assert!(clock.contains("clock"), "{clock}");
    assert!(clock.contains("timestamp"), "{clock}");
  }

  /// AC-3: a fault names which of them happened *and* the path involved.
  ///
  /// All six, each paired with the phrase its own arm says (F-7). Naming the
  /// path and reading differently from the other five are properties the set
  /// has by construction: a case holding only those stays green when
  /// `Unreachable` and `Faulted` are swapped, and a caller whose host is not
  /// running is then told the connection faulted. The distinctness check
  /// stays, because two arms could say each other's phrase *as well as* their
  /// own and the per-arm pins would not see it.
  #[test]
  fn every_send_fault_names_the_path_and_which_fault_it_was() {
    let path = Path::new("/run/goad.sock");
    let faults = [
      (
        SendFault::Unreachable(std::io::Error::from(std::io::ErrorKind::NotFound)),
        "nothing is listening at",
      ),
      (
        SendFault::Faulted(std::io::Error::from(std::io::ErrorKind::ConnectionReset)),
        "faulted",
      ),
      (SendFault::NoReply, "closed without answering"),
      (
        SendFault::Oversized { limit: 65_536 },
        "did not end within 65536 bytes",
      ),
      (
        SendFault::Unreadable(
          serde_json::from_str::<serde_json::Value>("not json").expect_err("this is not JSON"),
        ),
        "is not one JSON object",
      ),
      (
        SendFault::NonConforming("the reply carries no `accepted` field"),
        "breaches SPEC-003 6.3",
      ),
    ];
    let mut lines = Vec::new();
    for (fault, said) in faults {
      let line = fault_line(&fault, path);
      assert!(line.contains("/run/goad.sock"), "{line}");
      assert!(line.contains(said), "{line} must say {said}");
      lines.push(line);
    }
    let mut distinct = lines.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(
      distinct.len(),
      lines.len(),
      "two faults read the same: {lines:?}"
    );
  }
}
