//! The fake listener, the spawn helper, and the nine cases.
use std::io::{BufRead as _, Write as _};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::thread::JoinHandle;

use goad_semantics::protocol::canonical::Event;
use goad_shell::ingress::envelope;

/// A socket of this case's own, unlinked first. No lock file to clear beside
/// it: nothing in this tier calls `ingress::bind`, which is what writes one.
fn socket_path(case: &str) -> PathBuf {
  let path = std::env::temp_dir().join(format!("goad-emit-{case}-{}.sock", std::process::id()));
  match std::fs::remove_file(&path) {
    Ok(()) | Err(_) => (),
  }
  path
}

fn cleanup(path: &Path) {
  match std::fs::remove_file(path) {
    Ok(()) | Err(_) => (),
  }
}

/// One connection, one line read, one canned reply, then close — the shape
/// `tests/integration/ingress.rs:1296` uses one tier up. Bound *before* the
/// thread is spawned, so the socket exists by the time this returns and the
/// client cannot lose a race with it.
///
/// The handle yields the line the client sent, which is VT-4's subject.
fn fake_listener(path: &Path, reply: &'static str) -> JoinHandle<String> {
  let listener = UnixListener::bind(path).expect("the fake listener must bind");
  std::thread::spawn(move || {
    let (mut stream, _peer) = listener.accept().expect("one connection must arrive");
    let mut line = String::new();
    std::io::BufReader::new(&stream)
      .read_line(&mut line)
      .expect("the binary must send one line");
    stream
      .write_all(reply.as_bytes())
      .expect("the canned reply must be written");
    line
  })
}

/// The binary itself, not `cargo run`: `CARGO_BIN_EXE_goad-emit` is set for a
/// test target in the binary's own package, and cargo has already built it.
fn emit(arguments: &[&str]) -> Output {
  std::process::Command::new(env!("CARGO_BIN_EXE_goad-emit"))
    .args(arguments)
    .output()
    .expect("the built binary must be runnable")
}

fn code_of(output: &Output) -> i32 {
  output
    .status
    .code()
    .expect("the binary must exit rather than be signalled")
}

fn stderr_of(output: &Output) -> String {
  String::from_utf8(output.stderr.clone()).expect("emit writes UTF-8")
}

fn stdout_of(output: &Output) -> String {
  String::from_utf8(output.stdout.clone()).expect("emit writes UTF-8")
}

/// PHASE-03/EX-6, D-4: `--help` is the one place the usage block lives, it
/// goes to **stdout** so it can be paged, and it is exit 0. Held at this tier
/// because `render::USAGE` being right says nothing about `main` reaching it.
#[test]
fn help_prints_the_usage_block_on_stdout_and_exits_0() {
  let output = emit(&["--help"]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  let stdout = stdout_of(&output);
  assert!(stdout.starts_with("usage: goad-emit"), "{stdout}");
  assert!(stdout.contains("--socket"), "{stdout}");
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}

/// PHASE-03/EX-6: `--version` is the package version on stdout, exit 0 —
/// nothing else, so a caller can read it as a value.
#[test]
fn version_prints_the_package_version_on_stdout_and_exits_0() {
  let output = emit(&["--version"]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  assert_eq!(stdout_of(&output).trim_end(), env!("CARGO_PKG_VERSION"));
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}

/// PHASE-04/VT-1, AC-1. Exit 0, and **nothing on either stream**: a cron job
/// that prints on success trains its owner to ignore it (005/D-7). Both are
/// asserted, because stderr is the outlet `main` uses for every other
/// outcome and is where a regression would land (F-10).
#[test]
fn an_accepted_envelope_exits_0_and_says_nothing() {
  let path = socket_path("accepted");
  let listener = fake_listener(&path, r#"{"protocol":1,"accepted":true}"#);

  let output = emit(&[
    "--socket",
    &path.display().to_string(),
    "--source",
    "w",
    "--kind",
    "k",
  ]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  assert!(output.stdout.is_empty(), "{}", stdout_of(&output));
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
  listener.join().expect("the listener thread must finish");
  cleanup(&path);
}

/// PHASE-04/VT-2, AC-2. Exit 1 is "the host refused it", and the reason
/// **token** is what a wrapper branches on, so that is what is asserted —
/// not the sentence around it.
#[test]
fn a_refusal_exits_1_with_the_reason_token_on_stderr() {
  let path = socket_path("refused");
  let listener = fake_listener(
    &path,
    r#"{"protocol":1,"accepted":false,"reason":"engaged"}"#,
  );

  let output = emit(&[
    "--socket",
    &path.display().to_string(),
    "--source",
    "w",
    "--kind",
    "k",
  ]);

  assert_eq!(code_of(&output), 1, "{}", stderr_of(&output));
  let stderr = stderr_of(&output);
  assert!(stderr.contains("engaged"), "{stderr}");
  assert!(!stderr.contains("retry_after_ms"), "{stderr}");
  listener.join().expect("the listener thread must finish");
  cleanup(&path);
}

/// PHASE-04/VT-2, AC-2. `retry_after_ms` is advice, reported and never obeyed
/// — emit does not sleep on it.
#[test]
fn a_too_soon_refusal_also_shows_retry_after_ms() {
  let path = socket_path("too-soon");
  let listener = fake_listener(
    &path,
    r#"{"protocol":1,"accepted":false,"reason":"too_soon","retry_after_ms":1401}"#,
  );

  let output = emit(&[
    "--socket",
    &path.display().to_string(),
    "--source",
    "w",
    "--kind",
    "k",
  ]);

  assert_eq!(code_of(&output), 1, "{}", stderr_of(&output));
  let stderr = stderr_of(&output);
  assert!(stderr.contains("too_soon"), "{stderr}");
  assert!(stderr.contains("1401"), "{stderr}");
  listener.join().expect("the listener thread must finish");
  cleanup(&path);
}

/// PHASE-04/VT-3, AC-3, first of three. Exit 2 never means "the host refused
/// it", and the line names **where emit looked**: unreachable says nothing
/// about a host without the path.
#[test]
fn a_path_with_nothing_listening_exits_2_and_names_the_path() {
  let path = socket_path("unreachable");

  let output = emit(&[
    "--socket",
    &path.display().to_string(),
    "--source",
    "w",
    "--kind",
    "k",
  ]);

  assert_eq!(code_of(&output), 2, "{}", stderr_of(&output));
  let stderr = stderr_of(&output);
  assert!(stderr.contains(&path.display().to_string()), "{stderr}");
}

/// PHASE-04/VT-3, AC-3. A usage error costs no connection: parsing settles
/// before anything reads a configuration, a clock or a socket — which is why
/// this case can omit `--socket` without touching the machine's real
/// configuration.
#[test]
fn a_usage_error_exits_2_before_anything_is_opened() {
  let output = emit(&["--source", "w"]);

  assert_eq!(code_of(&output), 2, "{}", stderr_of(&output));
  let stderr = stderr_of(&output);
  assert!(stderr.contains("--kind"), "{stderr}");
  assert!(!stderr.contains("usage:"), "{stderr}");
}

/// PHASE-04/VT-3, AC-3. A refusal with no reason token is the **host's**
/// breach of §6.3, and it is exit 2 rather than exit 1 because exit 1
/// promises a token the caller can branch on and there is none to give.
#[test]
fn a_reply_that_breaches_6_3_exits_2_rather_than_1() {
  let path = socket_path("non-conforming");
  let listener = fake_listener(&path, r#"{"protocol":1,"accepted":false}"#);

  let output = emit(&[
    "--socket",
    &path.display().to_string(),
    "--source",
    "w",
    "--kind",
    "k",
  ]);

  assert_eq!(code_of(&output), 2, "{}", stderr_of(&output));
  let stderr = stderr_of(&output);
  assert!(stderr.contains("6.3"), "{stderr}");
  listener.join().expect("the listener thread must finish");
  cleanup(&path);
}

/// PHASE-04/VT-4, **AC-6**: the bytes a real invocation of the built binary
/// puts on a real socket normalize to an `Event` carrying the `source`, the
/// `kind` and the `data` as sent.
///
/// The subject of the assertion is the **normalized `Event`**, never the raw
/// bytes: a case that matched the JSON text would assert emit's serializer
/// against itself, and the thing worth knowing is that the host's own
/// `envelope::normalize` — the real one, not a copy — reads what emit wrote.
/// The timestamp is asserted only as *present and accepted*, because its
/// value is the moment of invocation and `normalize` refusing it is what R-22
/// compliance means here.
#[test]
fn the_bytes_on_the_socket_normalize_to_the_event_that_was_sent() {
  let path = socket_path("normalizes");
  let listener = fake_listener(&path, r#"{"protocol":1,"accepted":true}"#);

  let output = emit(&[
    "--socket",
    &path.display().to_string(),
    "--source",
    "w",
    "--kind",
    "k",
    "--data",
    r#"{"n":4}"#,
  ]);
  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));

  let sent = listener.join().expect("the listener thread must finish");
  let event: Event = envelope::normalize(sent.as_bytes())
    .unwrap_or_else(|fault| panic!("what emit sent must be an admissible envelope: {fault}"));
  assert_eq!(event.source, "w");
  assert_eq!(event.kind, "k");
  assert_eq!(event.data, serde_json::json!({"n": 4}));
  cleanup(&path);
}
