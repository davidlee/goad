//! The socket's lifecycle, the accepted path, the two read budgets and the
//! closed refusal vocabulary — `plan.md` PHASE-03 and PHASE-08, split at the
//! listener's own read/reply seam (`plan.md` PL-10). Every case here binds a
//! real socket, in its own directory under `std::env::temp_dir()`
//! (`config.rs:226`'s precedent — `tempfile` is not on the manifest
//! allowlist), against a **fake judge**: a task that takes every `Arrival`
//! the listener hands it and answers according to a script, recording what it
//! saw. `serve` itself is untouched until PHASE-04; no *production* code here
//! constructs `engaged` or `too_soon` — `Verdict::Refuse` lets a case script
//! either, which is how PHASE-08's VT-9 drives them without `serve`.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use goad_semantics::protocol::canonical::Event;
use goad_shell::ingress::envelope::EnvelopeFault;
use goad_shell::ingress::{
  BindFault, ENVELOPE_DEADLINE, ENVELOPE_LIMIT, Ingress, Refusal, UnavailableCause, bind, lock_path,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

/// `design.md`'s own example, `SPEC-003` §6.2 — one line, so a case can
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

/// Removes the socket **and the lock file beside it**. The host itself never
/// unlinks either (`SPEC-003/R-5`); a case that leaves both behind litters
/// `temp_dir()` with two files per run instead of one.
fn cleanup(path: &Path) {
  match std::fs::remove_file(path) {
    Ok(()) | Err(_) => (),
  }
  match std::fs::remove_file(lock_path(path)) {
    Ok(()) | Err(_) => (),
  }
}

// ---------------------------------------------------------------------------
// The fake judge
// ---------------------------------------------------------------------------

/// What the fake judge does with one arrival, for an envelope it reached as
/// an `Event` — a shape refusal the listener already decided is always
/// relayed as-is (below), whatever the script says for that slot.
enum Verdict {
  Accept,
  Drop,
  /// Answers with a scripted [`Refusal`] instead — how PHASE-08's cases
  /// exercise `engaged`, `too_soon` and a loop-side `unavailable` without
  /// `serve`, which is the only thing that constructs them for real
  /// (`plan.md` PHASE-08/EX-5).
  Refuse(Refusal),
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
          Verdict::Refuse(refusal) => answer.refused(&refusal),
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

fn detail(reply: &str) -> String {
  match parsed(reply)["detail"].as_str() {
    Some(detail) => detail.to_owned(),
    None => panic!("a refusal must carry detail: {reply}"),
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

/// A socket **a forked child still holds** is not a socket a live host holds,
/// and R-3 says it must be reclaimed. This is the case that could not be
/// written while liveness was a `connect` (`review-code.md` F-18).
///
/// The child holds a duplicate of the listening descriptor as its stdin. That
/// is exactly what `fork` hands a child between `fork` and `exec` — and `dup2`
/// onto fd 0 clears `CLOEXEC`, so the window that is microseconds wide in a
/// real spawn is the child's whole life here, and the case is deterministic
/// rather than a race. Nothing in *this* process holds the listener any more
/// and no host ever held it, yet `connect` still succeeds against the path:
/// the socket object outlives its owner's descriptor. The old probe read that
/// as *in use by a live host* and refused to start against a path nobody held.
#[tokio::test]
async fn a_socket_a_forked_child_still_holds_is_reclaimed_and_the_new_listener_serves() {
  let path = socket_path("vt1b-fork-window");
  let stale = match std::os::unix::net::UnixListener::bind(&path) {
    Ok(listener) => listener,
    Err(error) => panic!("could not bind the stale listener: {error}"),
  };

  // The child inherits the listener as its stdin; dropping `command` closes
  // this process's last descriptor for it, the way the owner's `close` does.
  let mut command = std::process::Command::new("sleep");
  command
    .arg("30")
    .stdin(std::process::Stdio::from(std::os::fd::OwnedFd::from(stale)))
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null());
  let mut child = match command.spawn() {
    Ok(child) => child,
    Err(error) => panic!("could not spawn the child holding the descriptor: {error}"),
  };
  drop(command);

  assert!(
    std::os::unix::net::UnixStream::connect(&path).is_ok(),
    "the case says nothing unless the child keeps the socket connectable"
  );

  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("a socket no host holds must be reclaimed: {error}"),
  };
  accept_everything(ingress);

  let reply = send_with_newline(&path, GOOD).await;
  assert!(
    accepted(&reply),
    "the reclaimed listener must serve: {reply}"
  );
  assert!(
    lock_path(&path).exists(),
    "the lock this host holds must sit beside the socket"
  );

  match child.kill() {
    Ok(()) | Err(_) => (),
  }
  match child.wait() {
    Ok(_) | Err(_) => (),
  }
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

/// **The lock is the signal, not the file** (`SPEC-003/R-3`, R-5). A socket
/// removed underneath a live host does not hand its path to a second one: the
/// first still holds the lock, so the second is `InUse` even though it finds
/// the path empty.
///
/// This is the half of `review-code.md` F-18 that the old probe could not see
/// at all — with nothing at the path there is nothing to `connect` to, so a
/// second host bound happily beside a first, which is the bind race §6.1 used
/// to record as a standing limit.
#[tokio::test]
async fn a_live_host_keeps_its_path_after_the_socket_file_is_removed() {
  let path = socket_path("vt2b-lock-outlives-the-file");
  let first = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("the first bind must succeed: {error}"),
  };
  accept_everything(first);
  match std::fs::remove_file(&path) {
    Ok(()) => (),
    Err(error) => panic!("could not remove the socket file: {error}"),
  }

  let error = match bind(&path) {
    Err(error) => error,
    Ok(_second) => panic!("a live host's path must not be taken while it holds the lock"),
  };
  assert_eq!(error.path, path);
  assert!(
    matches!(error.fault, BindFault::InUse),
    "expected InUse, got: {}",
    error.fault
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

/// A **symlink** at the path is refused as a symlink, whatever it points at —
/// `SPEC-003/R-4`, and the case R-3's letter and the implementation used to
/// disagree about (`review-code.md` F-11).
///
/// The link points at a socket a **live** host holds, which is the whole
/// point: by R-3 read alone that is *a socket a live host holds* and would be
/// `InUse`. `reclaim` does not follow the link — following one at a path the
/// host is about to `chmod` would put the owner-only mode on a file the
/// configuration never named — so it is `NotASocket` naming the symlink. A
/// link to nothing would exercise the same arm while agreeing with every
/// reading of R-3, and so would pass whether the rule existed or not.
#[tokio::test]
async fn a_symlink_to_a_live_socket_is_refused_unfollowed_and_the_target_keeps_serving() {
  let target = socket_path("vt3b-symlink-target");
  let link = socket_path("vt3b-symlink");
  let live = match bind(&target) {
    Ok(ingress) => ingress,
    Err(error) => panic!("binding the target must succeed: {error}"),
  };
  accept_everything(live);
  match std::os::unix::fs::symlink(&target, &link) {
    Ok(()) => (),
    Err(error) => panic!("could not create the symlink: {error}"),
  }

  let error = match bind(&link) {
    Err(error) => error,
    Ok(_ingress) => panic!("a symlink must be refused rather than followed"),
  };
  assert_eq!(error.path, link);
  assert!(
    matches!(error.fault, BindFault::NotASocket { found: "a symlink" }),
    "expected NotASocket naming a symlink rather than InUse, got: {}",
    error.fault
  );

  let reply = send_with_newline(&target, GOOD).await;
  assert!(
    accepted(&reply),
    "the host holding the link's target must be untouched: {reply}"
  );

  cleanup(&link);
  cleanup(&target);
}

/// A **symlink at the lock's own path** is refused unfollowed, for R-4's
/// reason applied to the other path the host creates (`review-code.md` F-22).
///
/// The link is **dangling**, which is the arm that does damage: `open(2)`
/// follows a final symlink unless `O_NOFOLLOW`, and the lock is opened with
/// `create(true)` and `.mode(0600)`, so following one creates an owner-only
/// file at a location the configuration never named. The case asserts the
/// refusal *and* that nothing appeared at the link's target, which is the half
/// a fault assertion alone would not catch.
#[tokio::test]
async fn a_symlink_at_the_lock_path_is_refused_and_nothing_is_created_through_it() {
  let path = socket_path("vt3c-lock-symlink");
  let target = std::env::temp_dir().join(format!(
    "goad-ingress-vt3c-lock-target-{}.txt",
    std::process::id()
  ));
  match std::fs::remove_file(&target) {
    Ok(()) | Err(_) => (),
  }
  match std::os::unix::fs::symlink(&target, lock_path(&path)) {
    Ok(()) => (),
    Err(error) => panic!("could not create the symlink at the lock path: {error}"),
  }

  let error = match bind(&path) {
    Err(error) => error,
    Ok(_ingress) => panic!("a symlink at the lock path must be refused rather than followed"),
  };
  assert_eq!(error.path, path);
  assert!(
    matches!(error.fault, BindFault::LockNotAFile { found: "a symlink" }),
    "expected LockNotAFile naming a symlink, got: {}",
    error.fault
  );
  assert!(
    !target.exists(),
    "the host must not create a file through a link the configuration did not name"
  );

  cleanup(&path);
  match std::fs::remove_file(&target) {
    Ok(()) | Err(_) => (),
  }
}

// ---------------------------------------------------------------------------
// R-5 — neither file is unlinked on the way out (`review-code.md` F-24)
// ---------------------------------------------------------------------------

/// **R-5, from the drop path.** The host does not unlink the socket when it
/// stops, and does not unlink the lock file either. That absence is what
/// makes R-3's reclaim path the one every ordinary restart takes.
///
/// Teardown cannot catch a regression in it: every `cleanup` helper in all
/// three test files ignores its errors, so a host that removed either file at
/// exit would be invisible to all of them. Other cases *do* notice,
/// incidentally — `a_connection_accepted_after_the_judge_is_gone_…` below
/// drops its own `Ingress` and then connects to the path — but they notice
/// while testing something else and say nothing about R-5, and before this
/// case no case in the workspace held R-5 at all. (An earlier draft of this
/// comment said nothing in the suite would notice; measurement disproved it —
/// `review-code.md` F-24, F-27.)
///
/// **What this holds is narrower than R-5's sentence, and deliberately so.**
/// Process exit is not `Drop`, so no test in this workspace can assert what a
/// killed host leaves behind. The drop path is the only path a regression
/// could reach — `main` drops `Served.ingress` — which makes it the right
/// bound to state rather than the "cannot be asserted" the spec's Verification
/// row used to claim (`review-code.md` F-24). An `impl Drop for Ingress` that
/// removed either file, or a `remove_file` on the way out of `serve`, turns
/// this red.
#[tokio::test]
async fn dropping_the_ingress_unlinks_neither_the_socket_nor_the_lock() {
  let path = socket_path("r5-drop");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("binding a fresh path must succeed: {error}"),
  };
  assert!(
    std::fs::symlink_metadata(&path).is_ok(),
    "the case says nothing unless the bind created the socket it is about"
  );

  drop(ingress);

  assert!(
    std::fs::symlink_metadata(&path).is_ok(),
    "the socket must outlive the listener: {}",
    path.display()
  );
  assert!(
    std::fs::symlink_metadata(lock_path(&path)).is_ok(),
    "the lock file must outlive the listener too: {}",
    lock_path(&path).display()
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-4 — a path that cannot be created
// ---------------------------------------------------------------------------

/// Holds one half of R-4: the refusal **names the path**. The other half —
/// that it names what was found — is its sibling below, deliberately a second
/// case (`review-code.md` F-21).
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

/// Holds R-4's other half: the refusal names **what was found**, and what was
/// found is an unusable location rather than an undetermined liveness
/// (`review-code.md` F-21).
///
/// Its own case rather than an assertion added to the one above, because the
/// two answer different questions — *is the person pointed at the right path*
/// and *is the person told the right kind of thing went wrong* — and the
/// history here is one of them moving while the other stayed true. `reclaim`
/// now takes the lock before it binds, so a directory that does not exist is
/// met at the lock file's `open`; the variant must not move with the step. A
/// missing directory is the commonest misconfiguration there is, and it is
/// deterministic, where the sibling's unwritable directory does not apply to
/// root.
#[tokio::test]
async fn a_missing_directory_is_refused_as_an_unusable_location_not_as_an_unknown_liveness() {
  let path = std::env::temp_dir()
    .join(format!("goad-ingress-vt4b-absent-{}", std::process::id()))
    .join("sub.sock");

  let error = match bind(&path) {
    Err(error) => error,
    Ok(_ingress) => panic!("a path under a directory that does not exist must be refused"),
  };
  assert!(
    matches!(error.fault, BindFault::Unbindable(_)),
    "a missing directory is R-4's unusable location, not R-3's undetermined liveness, got: {}",
    error.fault
  );
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
// The reply's own framing — `SPEC-003` §6.3, `review-code.md` F-1
// ---------------------------------------------------------------------------

/// §6.3 opens *"One JSON object, **newline-terminated**, then the host
/// closes."* The assertion is on the **raw bytes**, not on `parsed(&reply)`:
/// `serde_json::from_str` accepts the document with or without the terminator,
/// so an assertion routed through it would be the same blind instrument in a
/// new place — which is why every reader in both tiers missed this
/// (`review-code.md` F-1). Both replies the host can write are checked, because
/// accepted and refused are two calls to `reply`.
#[tokio::test]
async fn every_reply_is_newline_terminated_before_the_close() {
  let path = socket_path("reply-framing");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  accept_everything(ingress);

  for envelope in [GOOD, "not json"] {
    let reply = send_with_newline(&path, envelope).await;
    assert!(
      reply.ends_with('\n'),
      "the reply to {envelope:?} must carry its own terminator rather than relying on the \
       close: {reply:?}"
    );
  }

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

/// R-8 admits exactly one unanswered close — *the host process itself is
/// gone*. A connection accepted after the judge has been dropped but while the
/// process is still unwinding is not that case, and it used to close with no
/// reply at all (`review-code.md` F-8): the `Arrival` came back inside the
/// `SendError` and was dropped with its `Answer`, so nothing was written.
///
/// The judge here is dropped outright rather than scripted, which is exactly
/// the state `main.rs`'s `spawn_local` block leaves behind while the accept
/// task keeps accepting.
#[tokio::test]
async fn a_connection_accepted_after_the_judge_is_gone_is_answered_unavailable() {
  let path = socket_path("judge-gone");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  drop(ingress); // the receiver is gone; the accept task is not

  let reply = send_with_newline(&path, GOOD).await;
  assert!(!accepted(&reply), "expected a refusal: {reply}");
  assert_eq!(
    reason(&reply),
    "unavailable",
    "the same reason the adjacent dropped-`Answer` case gives: {reply}"
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-6b — unavailable's two wire causes are distinguished by detail
// ---------------------------------------------------------------------------

/// `unavailable` carries one reason token for two causes of the moment
/// (`SPEC-003` §6.3): the host is stopping (VT-6, above — a dropped
/// `Answer`), or the clock cannot be read (`design.md` §5.4 step 4, scripted
/// here exactly as `judge`'s own doc comment says a loop-side `unavailable`
/// is exercised without `serve`). Both must read `unavailable` on the wire,
/// and `detail` must say which — the point of naming the cause at all.
#[tokio::test]
async fn unavailable_s_two_causes_carry_different_detail() {
  let clock_path = socket_path("vt6b-clock");
  let clock_ingress = match bind(&clock_path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  let _clock_judge = judge(
    clock_ingress,
    vec![Verdict::Refuse(Refusal::Unavailable(
      UnavailableCause::ClockUnreadable,
    ))],
  );
  let clock_reply = send_with_newline(&clock_path, GOOD).await;
  assert_eq!(reason(&clock_reply), "unavailable");
  cleanup(&clock_path);

  let shutdown_path = socket_path("vt6b-shutdown");
  let shutdown_ingress = match bind(&shutdown_path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  let _shutdown_judge = judge(shutdown_ingress, vec![Verdict::Drop]);
  let shutdown_reply = send_with_newline(&shutdown_path, GOOD).await;
  assert_eq!(reason(&shutdown_reply), "unavailable");
  cleanup(&shutdown_path);

  assert_ne!(
    detail(&clock_reply),
    detail(&shutdown_reply),
    "the clock cause and the shutdown cause must not share a detail: \
     {clock_reply} vs {shutdown_reply}"
  );
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

// ---------------------------------------------------------------------------
// PHASE-08
// ---------------------------------------------------------------------------
//
// The refusal vocabulary: `reserved_source` as its own wire reason, the
// closed eight-token reason set, and `retry_after_ms` (`plan.md` PHASE-08).
// No case below waits on a bound — each is decided by the fake judge's
// scripted answer or by the shape of the bytes written, per PHASE-08's own
// Verification preamble.

// ---------------------------------------------------------------------------
// VT-7 — the three shape reasons this phase owns, read off the wire
// ---------------------------------------------------------------------------

#[tokio::test]
async fn the_three_shape_reasons_this_phase_owns_are_read_off_the_wire() {
  let path = socket_path("vt7-shape-reasons");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  // `Verdict::Drop` for all three: if a shape refusal ever let the script's
  // own verdict decide the answer, a dropped `Answer` would read back as
  // `unavailable` instead of the shape's own reason. It never does — shape
  // takes precedence over state (`design.md` §5.4 step 1) — so scripting the
  // one verdict that would expose a leak is a stronger check than `Accept`.
  let seen = judge(ingress, vec![Verdict::Drop, Verdict::Drop, Verdict::Drop]);

  let malformed = send_with_newline(&path, "not json").await;
  assert_eq!(reason(&malformed), "malformed", "{malformed}");

  let not_an_object = send_with_newline(&path, "[1, 2, 3]").await;
  assert_eq!(
    reason(&not_an_object),
    "invalid_envelope",
    "{not_an_object}"
  );

  let reserved = send_with_newline(
    &path,
    &GOOD.replace(r#""source":"reddit-watcher""#, r#""source":"host""#),
  )
  .await;
  assert_eq!(reason(&reserved), "reserved_source", "{reserved}");

  let guard = seen
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
  assert_eq!(guard.len(), 3, "all three arrivals must reach the judge");
  assert!(
    guard.iter().all(|entry| matches!(entry, Seen::Refused(_))),
    "a shape refusal must never arrive at the judge as an Event"
  );

  cleanup(&path);
}

// ---------------------------------------------------------------------------
// VT-8 — the reason token set is closed at eight
// ---------------------------------------------------------------------------

/// The eight tokens `SPEC-003` §6.3 closes the wire's reason set at, as a
/// literal a client's parser could be written from.
const EXPECTED: [&str; 8] = [
  "malformed",
  "invalid_envelope",
  "reserved_source",
  "too_large",
  "timed_out",
  "engaged",
  "too_soon",
  "unavailable",
];

/// The wire's reason set, and **which of R-14's four directions each half
/// holds** — R-14's Verification row was weakened to match this comment rather
/// than this comment written to flatter the row (`review-code.md` F-5, user's
/// call 2026-09-09).
///
/// **Held by assertion:** a token *renamed*, a token *removed* from
/// `Refusal::reason()`'s match, and the eight-way mapping being correct.
///
/// **Held by the compiler, and then by review:** a token *added*. The `match`
/// below is the source of the set compared — every token in `reasons` comes
/// off one of its arms — and it has no `_` arm at either level, over
/// `Refusal`'s variants or over `UnavailableCause`'s. So a ninth variant, or a
/// fifth cause, fails to compile **in this file**, and the suite stays red
/// until someone edits it. Measured rather than assumed: a ninth variant was
/// added and reverted, giving (1) `error[E0004]: non-exhaustive patterns:
/// &Refusal::Ninth not covered` here; (2) green once only the arm was added;
/// (3) failing — on the length link, then on the set — once the arm *and* a
/// witness were added.
///
/// **Which set is being closed.** The **eight wire tokens**, and only those.
/// `UnavailableCause`'s four causes share one token, so the or-pattern in the
/// `unavailable` arm grows when a cause is added and `EXPECTED` does not —
/// that is R-14's own arrangement (§6.3: *"a fourth cause of one of them, not a
/// ninth token"*), not a gap in this case. What the exhaustive cause pattern
/// buys is that a fifth cause has to be *looked at* here, where the decision
/// about whether it earns a token belongs.
///
/// **(2) is the residue, and it is why R-14 no longer claims otherwise.** Rust
/// cannot force the witness list below to cover a newly added variant — that
/// needs a derive macro or an enumeration crate, and neither is on this
/// manifest. So the compile gate is forced, and whether the assertion then also
/// fails depends on that author adding a witness beside the arm the compiler
/// has just made them write, one line away. The shape that would close it —
/// `Refusal::reason()` returning a closed `Reason` *type*, so a ninth variant
/// cannot mint a token at all — is a follow-up in `slice-004.md`, not this
/// slice's to take.
#[test]
fn the_reason_token_set_is_closed_at_eight() {
  /// One `Refusal` per token. A witness belongs here for every arm of the
  /// match below; the match is what turns it into the token under test.
  fn witnesses() -> [Refusal; 8] {
    [
      Refusal::Unavailable(UnavailableCause::Stopping),
      Refusal::Malformed,
      Refusal::InvalidEnvelope(EnvelopeFault::NotAnObject { found: "array" }),
      Refusal::InvalidEnvelope(EnvelopeFault::ReservedSource),
      Refusal::TooLarge {
        limit: ENVELOPE_LIMIT,
      },
      Refusal::TimedOut {
        after: ENVELOPE_DEADLINE,
      },
      Refusal::Engaged,
      Refusal::TooSoon {
        retry_after: Duration::from_millis(1),
      },
    ]
  }

  /// The vocabulary, arm by arm. **No `_` arm, at either level** — this is the
  /// instrument, and the tokens it returns are the set compared below.
  fn token(refusal: &Refusal) -> &'static str {
    match refusal {
      Refusal::Unavailable(
        UnavailableCause::Stopping
        | UnavailableCause::ClockUnreadable
        | UnavailableCause::IngressStopped
        | UnavailableCause::Unreadable(_),
      ) => "unavailable",
      Refusal::Malformed => "malformed",
      Refusal::InvalidEnvelope(EnvelopeFault::ReservedSource) => "reserved_source",
      Refusal::InvalidEnvelope(_) => "invalid_envelope",
      Refusal::TooLarge { .. } => "too_large",
      Refusal::TimedOut { .. } => "timed_out",
      Refusal::Engaged => "engaged",
      Refusal::TooSoon { .. } => "too_soon",
    }
  }

  let witnesses = witnesses();
  assert_eq!(
    witnesses.len(),
    EXPECTED.len(),
    "one witness per token, or the set below is built from fewer arms than there are tokens"
  );

  let reasons: std::collections::BTreeSet<&str> = witnesses.iter().map(token).collect();
  let expected: std::collections::BTreeSet<&str> = EXPECTED.into_iter().collect();
  assert_eq!(
    reasons, expected,
    "the wire's reason set must be exactly these eight tokens, not a subset or a superset"
  );

  // And production reads the same vocabulary the match above states, arm for
  // arm: a token renamed in `Refusal::reason()` alone fails here.
  for witness in &witnesses {
    assert_eq!(
      witness.reason(),
      token(witness),
      "`Refusal::reason()` and this case's own match must agree"
    );
  }
}

// ---------------------------------------------------------------------------
// VT-9 — retry_after_ms: present only on too_soon, and rounded up
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_too_soon_reply_carries_retry_after_ms_rounded_up() {
  let path = socket_path("vt9-rounds-up");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  // 1_400_300ns past 1400ms: a truncated remainder would read 1400ms, which a
  // writer that waited exactly that long would still find itself inside the
  // spacing (`SPEC-003/R-14`'s own argument for rounding up rather than
  // truncating).
  let retry_after = Duration::from_micros(1_400_300);
  let _seen = judge(
    ingress,
    vec![Verdict::Refuse(Refusal::TooSoon { retry_after })],
  );

  let reply = send_with_newline(&path, GOOD).await;
  assert_eq!(reason(&reply), "too_soon");
  assert_eq!(
    parsed(&reply)["retry_after_ms"],
    serde_json::json!(1401),
    "1400.3ms must round up to 1401, not truncate to 1400: {reply}"
  );

  cleanup(&path);
}

#[tokio::test]
async fn retry_after_ms_is_absent_from_every_reason_but_too_soon() {
  let path = socket_path("vt9-absent-elsewhere");
  let ingress = match bind(&path) {
    Ok(ingress) => ingress,
    Err(error) => panic!("bind failed: {error}"),
  };
  // Every reason but `too_soon`, each scripted directly so no case here waits
  // on a bound (not even `too_large`'s or `timed_out`'s own real trigger) —
  // the fact under test is the wire's field, not how the reason was reached.
  let refusals = [
    Refusal::Unavailable(UnavailableCause::Stopping),
    Refusal::Malformed,
    Refusal::InvalidEnvelope(EnvelopeFault::NotAnObject { found: "array" }),
    Refusal::InvalidEnvelope(EnvelopeFault::ReservedSource),
    Refusal::TooLarge {
      limit: ENVELOPE_LIMIT,
    },
    Refusal::TimedOut {
      after: ENVELOPE_DEADLINE,
    },
    Refusal::Engaged,
  ];
  let expected_reasons: Vec<&str> = refusals.iter().map(Refusal::reason).collect();
  let script = refusals.into_iter().map(Verdict::Refuse).collect();
  let _seen = judge(ingress, script);

  for expected in expected_reasons {
    let reply = send_with_newline(&path, GOOD).await;
    assert_eq!(reason(&reply), expected);
    assert!(
      parsed(&reply).get("retry_after_ms").is_none(),
      "{expected} must not carry retry_after_ms: {reply}"
    );
  }

  cleanup(&path);
}
