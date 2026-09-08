//! The socket's lifecycle, the accepted path, and the two read budgets —
//! `plan.md` PHASE-03. Every case here binds a real socket, in its own
//! directory under `std::env::temp_dir()` (`config.rs:226`'s precedent —
//! `tempfile` is not on the manifest allowlist), against a **fake judge**: a
//! task that takes every `Arrival` the listener hands it and answers
//! according to a script, recording what it saw. `serve` itself is untouched
//! until PHASE-04; nothing here constructs `engaged` or `too_soon`, and
//! `reserved_source` still reads off the wire as `invalid_envelope` until
//! PHASE-08/EX-13.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use goad_semantics::protocol::canonical::Event;
use goad_shell::ingress::{BindFault, ENVELOPE_DEADLINE, ENVELOPE_LIMIT, Ingress, bind};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

/// `design.md`'s own example, `draft-spec.md` §6.2 — one line, so a case can
/// substitute a value with `str::replace` the way `envelope.rs`'s own tests do.
const GOOD: &str = r#"{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00","data":{"count_last_hour":4}}"#;

/// A unique path per case, under `std::env::temp_dir()`.
fn socket_path(case: &str) -> PathBuf {
  let path = std::env::temp_dir().join(format!("goad-ingress-{case}-{}.sock", std::process::id()));
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

// ---------------------------------------------------------------------------
// The fake judge
// ---------------------------------------------------------------------------

/// What the fake judge does with one arrival, for an envelope it reached as
/// an `Event` — a shape refusal the listener already decided is always
/// relayed as-is (below), so there is no `Refuse` verdict of this phase's own:
/// nothing here constructs `engaged` or `too_soon`, which is `PHASE-08`'s.
enum Verdict {
  Accept,
  Drop,
}

/// What the judge recorded about one arrival. Cheap to keep — `Event` clones,
/// and a refusal is recorded by its wire reason, since `use_debug` is denied
/// crate-wide and `Refusal` carries an `io::Error` that is not `Clone`.
enum Seen {
  Event(Event),
  Refused(&'static str),
}

/// Spawns the fake judge: answers the arrivals in `script`, in order,
/// recording each one's outcome before answering; anything after the script
/// is exhausted — a liveness control a case sends afterward, or a stray
/// connection (VT-2's own reclaim probe lands on the *first* listener as one
/// of these) — is answered `accepted` so it never blocks the listener.
/// Returns the `Arc` the script's own recordings land in.
fn judge(ingress: Ingress, script: Vec<Verdict>) -> Arc<Mutex<Vec<Seen>>> {
  let seen = Arc::new(Mutex::new(Vec::new()));
  let recorded = Arc::clone(&seen);
  tokio::spawn(async move {
    let mut ingress = ingress;
    for verdict in script {
      let Some(arrival) = ingress.arrival().await else {
        return;
      };
      let (result, answer) = arrival.into_parts();
      let entry = match &result {
        Ok(event) => Seen::Event(event.clone()),
        Err(refusal) => Seen::Refused(refusal.reason()),
      };
      match recorded.lock() {
        Ok(mut guard) => guard.push(entry),
        Err(_poisoned) => (),
      }
      // Shape before state (`design.md` §5.4 step 1): a refusal the listener
      // already decided is relayed as-is, whatever the script says for this
      // slot — the fixture must not overrule a shape refusal into `accepted`,
      // which is exactly what a real judge would never do either. The script
      // governs only what happens to a well-formed `Event`.
      match result {
        Err(refusal) => answer.refused(&refusal),
        Ok(_event) => match verdict {
          Verdict::Accept => answer.accepted(),
          Verdict::Drop => drop(answer),
        },
      }
    }
    while let Some(arrival) = ingress.arrival().await {
      let (result, answer) = arrival.into_parts();
      match result {
        Err(refusal) => answer.refused(&refusal),
        Ok(_event) => answer.accepted(),
      }
    }
  });
  seen
}

/// A judge that only ever accepts, for cases whose point is the listener's
/// own behaviour rather than what a judge decides.
fn accept_everything(ingress: Ingress) {
  let _seen = judge(
    ingress,
    vec![Verdict::Accept, Verdict::Accept, Verdict::Accept],
  );
}

// ---------------------------------------------------------------------------
// A watcher's side of one connection
// ---------------------------------------------------------------------------

async fn read_reply(mut stream: UnixStream) -> String {
  let mut reply = String::new();
  match stream.read_to_string(&mut reply).await {
    Ok(_bytes) => (),
    Err(error) => panic!("reading the reply failed: {error}"),
  }
  reply
}

/// One envelope, terminated by a newline — `SPEC-003/R-6`'s first framing.
async fn send_with_newline(path: &Path, envelope: &str) -> String {
  let mut stream = match UnixStream::connect(path).await {
    Ok(stream) => stream,
    Err(error) => panic!("connect failed: {error}"),
  };
  let mut bytes = envelope.as_bytes().to_vec();
  bytes.push(b'\n');
  match stream.write_all(&bytes).await {
    Ok(()) => (),
    Err(error) => panic!("write failed: {error}"),
  }
  read_reply(stream).await
}

/// One envelope, terminated by closing the write side — `SPEC-003/R-6`'s
/// second framing (A-2's `socat`, half-closing rather than sending `\n`).
async fn send_half_closed(path: &Path, envelope: &str) -> String {
  let mut stream = match UnixStream::connect(path).await {
    Ok(stream) => stream,
    Err(error) => panic!("connect failed: {error}"),
  };
  match stream.write_all(envelope.as_bytes()).await {
    Ok(()) => (),
    Err(error) => panic!("write failed: {error}"),
  }
  match stream.shutdown().await {
    Ok(()) => (),
    Err(error) => panic!("half-close failed: {error}"),
  }
  read_reply(stream).await
}

fn parsed(reply: &str) -> serde_json::Value {
  match serde_json::from_str(reply) {
    Ok(value) => value,
    Err(error) => panic!("the reply itself is not valid JSON: {error}: {reply}"),
  }
}

fn accepted(reply: &str) -> bool {
  parsed(reply)["accepted"] == serde_json::json!(true)
}

fn reason(reply: &str) -> String {
  match parsed(reply)["reason"].as_str() {
    Some(reason) => reason.to_owned(),
    None => panic!("a refusal must name a reason: {reply}"),
  }
}

/// The design's own example, grown to exactly `total_len` bytes by padding
/// `data`'s string — `harness.rs::padded_evaluate`'s own trick, over the
/// envelope wire form rather than a `Request`.
fn padded_envelope(total_len: usize) -> String {
  let template = |padding: &str| {
    format!(
      r#"{{"source":"s","kind":"k","timestamp":"2026-08-22T17:10:00+10:00","data":"{padding}"}}"#
    )
  };
  let base_len = template("").len();
  assert!(
    total_len >= base_len,
    "target length {total_len} is smaller than the empty template ({base_len})"
  );
  template(&"a".repeat(total_len - base_len))
}

// ---------------------------------------------------------------------------
// VT-1 — reclaim
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves() {
  let path = socket_path("vt1-reclaim");
  {
    let stale = match std::os::unix::net::UnixListener::bind(&path) {
      Ok(listener) => listener,
      Err(error) => panic!("could not bind the stale listener: {error}"),
    };
    drop(stale); // the socket file remains; nothing answers on it any more
  }
  assert!(
    path.exists(),
    "the stale socket file must still be there to reclaim"
  );

  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("a stale socket must be reclaimed: {error}"),
  };
  accept_everything(ingress);

  let reply = send_with_newline(&path, GOOD).await;
  assert!(
    accepted(&reply),
    "the reclaimed listener must serve: {reply}"
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-2 — a live socket
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_live_socket_refuses_a_second_bind_and_keeps_serving() {
  let path = socket_path("vt2-live");
  let first = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("the first bind must succeed: {error}"),
  };
  accept_everything(first);

  let error = match bind(&path) {
    Err(error) => error,
    Ok(_second) => panic!("a live socket must refuse a second bind"),
  };
  assert_eq!(error.path, path);
  assert!(
    matches!(error.fault, BindFault::InUse),
    "expected InUse, got: {}",
    error.fault
  );

  let reply = send_with_newline(&path, GOOD).await;
  assert!(
    accepted(&reply),
    "the first listener must still be serving: {reply}"
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-3 — a regular file at the path
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_regular_file_at_the_path_is_refused_naming_what_was_found() {
  let path = socket_path("vt3-regular-file");
  match std::fs::write(&path, b"not a socket") {
    Ok(()) => (),
    Err(error) => panic!("could not create the regular file: {error}"),
  }

  let error = match bind(&path) {
    Err(error) => error,
    Ok(_ingress) => panic!("a regular file must be refused"),
  };
  assert_eq!(error.path, path);
  assert!(
    matches!(
      error.fault,
      BindFault::NotASocket {
        found: "a regular file"
      }
    ),
    "expected NotASocket naming a regular file, got: {}",
    error.fault
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-4 — a path that cannot be created
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_directory_with_no_write_permission_is_refused_naming_the_path() {
  use std::os::unix::fs::PermissionsExt;

  let dir = std::env::temp_dir().join(format!("goad-ingress-vt4-dir-{}", std::process::id()));
  match std::fs::remove_dir_all(&dir) {
    Ok(()) | Err(_) => (),
  }
  match std::fs::create_dir(&dir) {
    Ok(()) => (),
    Err(error) => panic!("could not create the directory: {error}"),
  }
  match std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o500)) {
    Ok(()) => (),
    Err(error) => panic!("could not remove write permission: {error}"),
  }
  let path = dir.join("sub.sock");

  let error = match bind(&path) {
    Err(error) => error,
    Ok(_ingress) => panic!(
      "a directory with no write permission must be refused — unless this ran as root, where \
       permission checks do not apply"
    ),
  };
  assert_eq!(error.path, path);

  match std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)) {
    Ok(()) | Err(_) => (),
  }
  match std::fs::remove_dir_all(&dir) {
    Ok(()) | Err(_) => (),
  }
}

// ---------------------------------------------------------------------------
// VT-5 — framing: newline, EOF, and a second envelope never read
// ---------------------------------------------------------------------------

#[tokio::test]
async fn an_envelope_terminated_by_a_newline_is_accepted() {
  let path = socket_path("vt5-newline");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  accept_everything(ingress);

  let reply = send_with_newline(&path, GOOD).await;
  assert!(accepted(&reply), "expected accepted: {reply}");

  cleanup(&path);
}

#[tokio::test]
async fn an_envelope_terminated_by_closing_the_write_side_is_accepted() {
  let path = socket_path("vt5-eof");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  accept_everything(ingress);

  let reply = send_half_closed(&path, GOOD).await;
  assert!(accepted(&reply), "expected accepted: {reply}");

  cleanup(&path);
}

#[tokio::test]
async fn a_second_envelope_on_the_same_connection_is_never_read() {
  let path = socket_path("vt5-second-envelope");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  let seen = judge(ingress, vec![Verdict::Accept]);

  let raw = format!("{GOOD}\nsecond envelope\n");
  let mut stream = match UnixStream::connect(&path).await {
    Ok(stream) => stream,
    Err(error) => panic!("connect failed: {error}"),
  };
  match stream.write_all(raw.as_bytes()).await {
    Ok(()) => (),
    Err(error) => panic!("write failed: {error}"),
  }
  let reply = read_reply(stream).await;

  assert!(
    accepted(&reply),
    "expected exactly one accepted reply: {reply}"
  );
  let guard = seen
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
  assert_eq!(guard.len(), 1, "the judge must see exactly one arrival");

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-6 — a dropped Answer
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_dropped_answer_yields_unavailable_then_a_close() {
  let path = socket_path("vt6-drop");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  let _seen = judge(ingress, vec![Verdict::Drop]);

  let reply = send_with_newline(&path, GOOD).await;
  assert!(!accepted(&reply), "expected a refusal: {reply}");
  assert_eq!(reason(&reply), "unavailable");

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-10 — the mode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn the_socket_is_owner_only_after_bind() {
  use std::os::unix::fs::MetadataExt;

  let path = socket_path("vt10-mode");
  let _ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };

  let metadata = match std::fs::metadata(&path) {
    Ok(metadata) => metadata,
    Err(error) => panic!("could not stat the bound socket: {error}"),
  };
  assert_eq!(
    metadata.mode() & 0o777,
    0o600,
    "the host must set the mode itself, not inherit the umask"
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-11 — a malformed envelope reaches no Event; the listener stays up
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_malformed_envelope_reaches_no_event_and_the_listener_stays_up() {
  let path = socket_path("vt11-malformed");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  let seen = judge(ingress, vec![Verdict::Accept, Verdict::Accept]);

  let bad = send_with_newline(&path, "not json").await;
  assert!(!accepted(&bad), "malformed bytes must be refused: {bad}");
  assert_eq!(reason(&bad), "malformed");

  let good = send_with_newline(&path, GOOD).await;
  assert!(
    accepted(&good),
    "a well-formed envelope on a new connection right after must still be accepted: {good}"
  );

  let guard = seen
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
  assert_eq!(guard.len(), 2, "both arrivals must have reached the judge");
  assert!(
    matches!(guard.first(), Some(Seen::Refused("malformed"))),
    "the malformed bytes must reach the judge as a refusal, not an Event"
  );
  assert!(
    matches!(guard.get(1), Some(Seen::Event(_))),
    "the well-formed envelope must reach the judge as an Event"
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-12 — the positive control
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_well_formed_envelope_reaches_the_judge_as_the_event_it_wrote() {
  let path = socket_path("vt12-positive");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  let seen = judge(ingress, vec![Verdict::Accept]);

  let reply = send_with_newline(&path, GOOD).await;
  assert!(accepted(&reply), "expected accepted: {reply}");

  let guard = seen
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
  match guard.first() {
    Some(Seen::Event(event)) => {
      assert_eq!(event.source, "reddit-watcher");
      assert_eq!(event.kind, "reddit-opened");
      assert_eq!(event.data, serde_json::json!({"count_last_hour": 4}));
      let expected: jiff::Timestamp = match "2026-08-22T17:10:00+10:00".parse() {
        Ok(instant) => instant,
        Err(error) => panic!("could not parse the design's own example: {error}"),
      };
      assert_eq!(event.timestamp.instant(), expected);
    }
    _other => panic!("expected an Event, found a refusal"),
  }

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-13 — the byte budget
// ---------------------------------------------------------------------------

#[tokio::test]
async fn more_than_the_byte_limit_is_refused_too_large_and_the_limit_itself_is_accepted() {
  let path = socket_path("vt13-too-large");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  accept_everything(ingress);

  let too_big = padded_envelope(ENVELOPE_LIMIT + 1);
  let over_reply = send_with_newline(&path, &too_big).await;
  assert!(!accepted(&over_reply), "expected too_large: {over_reply}");
  assert_eq!(reason(&over_reply), "too_large");

  let at_the_limit = padded_envelope(ENVELOPE_LIMIT);
  let boundary_reply = send_with_newline(&path, &at_the_limit).await;
  assert!(
    accepted(&boundary_reply),
    "an envelope at the limit must be accepted (the liveness control): {boundary_reply}"
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-14 — the time budget
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_connection_that_writes_nothing_times_out_and_the_listener_serves_next() {
  let path = socket_path("vt14-timeout");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  accept_everything(ingress);

  let started = Instant::now();
  let stream = match UnixStream::connect(&path).await {
    Ok(stream) => stream,
    Err(error) => panic!("connect failed: {error}"),
  };
  // Nothing is written at all; the deadline alone must produce a reply.
  let reply = read_reply(stream).await;
  let elapsed = started.elapsed();

  assert!(!accepted(&reply), "expected timed_out: {reply}");
  assert_eq!(reason(&reply), "timed_out");

  // VA-2: the margin is measured at the bound that governs — ENVELOPE_DEADLINE
  // itself, not an arbitrary wall clock. The read must not resolve before its
  // own deadline, and must not overshoot it by an order of magnitude either
  // (S-3): a connect-to-reply round trip lands inside both.
  assert!(
    elapsed >= ENVELOPE_DEADLINE,
    "the read resolved before its own deadline: {elapsed:?} < {ENVELOPE_DEADLINE:?}"
  );
  assert!(
    elapsed < ENVELOPE_DEADLINE * 10,
    "the read overshot its deadline by more than 10x (S-3): {elapsed:?}"
  );

  let next = send_with_newline(&path, GOOD).await;
  assert!(
    accepted(&next),
    "the listener must serve the next connection normally: {next}"
  );

  cleanup(&path);
}
