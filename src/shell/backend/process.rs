//! The process transport — one child per exchange, `design.md` §5.4.
//!
//! This module reads a backend's bytes and computes over their lengths against
//! two bounds, which is exactly the arithmetic I9 wants guarded, so it carries
//! the module-level lint the crate-wide table deliberately does not (D53, R-46).
#![deny(clippy::arithmetic_side_effects)]

use std::process::Stdio;
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::time::error::Elapsed;

use crate::semantics::error::ProtocolError;
use crate::semantics::protocol::canonical::Request;
use crate::shell::backend::transport::{Backend, Captured, Exchange};
use crate::shell::error::{BackendError, CleanupFailure};

/// Exceeding this **fails** the exchange: the host cannot act on a response it
/// refused to finish reading (R-43).
const STDOUT_LIMIT: usize = 8 * 1024 * 1024;
/// Exceeding this does **not** fail the exchange. Stderr is diagnostic, so the
/// host keeps the first portion, flags it, and goes on reading to EOF — a
/// chatty backend must not be able to block on a full pipe (D34, R-43).
const STDERR_LIMIT: usize = 256 * 1024;
/// One budget for the whole of disposal: kill, reap, and finishing the drain.
/// Bounded because `wait` on a pathological child can block indefinitely, and a
/// host that blocks is the host going down (brief §13, F-53, D48).
const CLEANUP_LIMIT: Duration = Duration::from_millis(500);
/// How much room a read is offered at a time. Not a bound on anything.
const READ_CHUNK: usize = 4096;

/// A backend invoked as a fresh process per exchange.
///
/// It holds its own command and timeout rather than a `Config`: `exchange`
/// takes only `&mut self` and the request, so the timeout is already the
/// transport's. Loading, and rejecting an empty command or a zero timeout, is
/// configuration's job and happens before one of these is built.
#[derive(Debug)]
pub struct ProcessBackend {
  command: Vec<String>,
  timeout: Duration,
}

impl ProcessBackend {
  pub fn new(command: Vec<String>, timeout: Duration) -> Self {
    Self { command, timeout }
  }
}

impl Backend for ProcessBackend {
  async fn exchange(&mut self, request: &Request) -> Exchange {
    // Before the spawn, so a failure here has no child to clean up after.
    // Unreachable in practice — a `Request` is host-authored and its fields
    // serialize infallibly — but `unwrap` is not available to say so, and the
    // honest name for "these bytes are not a JSON document" is this one. It is
    // not the response-parsing claim that moved to PHASE-07: nothing here
    // parses what a backend wrote.
    let payload = match serde_json::to_vec(request) {
      Ok(payload) => payload,
      Err(error) => return Exchange::failed(BackendError::Protocol(ProtocolError::Json(error))),
    };
    // Configuration rejects `command = []` at load, so this is the belt to that
    // brace — and `split_first` is how the program is separated from its
    // arguments without indexing, which the lint table forbids.
    let Some((program, arguments)) = self.command.split_first() else {
      return Exchange::failed(BackendError::Spawn(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "backend command is empty",
      )));
    };
    let mut command = Command::new(program);
    command
      .args(arguments)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      // The backstop for the paths where no code of ours runs at all —
      // cancellation, a panic unwinding past us. Named as one (I13, R-48).
      .kill_on_drop(true);

    // No `?` past this point: once a child exists, every return must dispose of
    // it, and `?` would quietly hand that job to `kill_on_drop` (F-41, I13).
    // The one return that skips cleanup is the spawn failure itself, where
    // nothing was spawned.
    let mut child = match command.spawn() {
      Ok(child) => child,
      Err(error) => return Exchange::failed(BackendError::Spawn(error)),
    };
    let (Some(stdin), Some(stdout), Some(stderr)) =
      (child.stdin.take(), child.stdout.take(), child.stderr.take())
    else {
      return cleanup_only(&mut child, BackendError::PipeMissing).await;
    };

    let mut seen = Captured::default();
    let (result, cleanup) = {
      // A sub-future of *this* task, not a `tokio::spawn` (D44, F-49): it
      // borrows the caller's buffer, so there is no `Arc<Mutex<…>>`, and if the
      // whole exchange is dropped it goes with it, where a spawned task would
      // have been detached and left running.
      let drain = drain_capped(stderr, STDERR_LIMIT, &mut seen);
      tokio::pin!(drain);

      // Both make progress for the whole window. The `if !drained` guard is
      // load-bearing: `select!` must not poll a future that has completed.
      let mut drained = false;
      let raced = {
        // `body` holds `&mut child`, so it lives in an inner scope: the borrow
        // has to be released before the cleanup budget can take it again.
        let body = body(stdin, stdout, &mut child, &payload);
        tokio::pin!(body);

        tokio::time::timeout(self.timeout, async {
          loop {
            tokio::select! {
              raced = &mut body => break raced,
              () = &mut drain, if !drained => { drained = true; }
            }
          }
        })
        .await
      }; // `body` dropped here, releasing `&mut child`

      // A non-zero status discards the body it came with (D15, R-40). The
      // status is read before the bytes are trusted, so no parsed response
      // outlives the exit code that disclaimed it.
      let result = match raced {
        Ok(Ok((bytes, status))) if status.success() => Ok(bytes),
        Ok(Ok((_, status))) => Err(BackendError::ExitStatus {
          code: status.code(),
        }),
        Ok(Err(error)) => Err(error),
        Err(_) => Err(BackendError::Timeout {
          after: self.timeout,
        }),
      };

      let cleanup = tokio::time::timeout(CLEANUP_LIMIT, async {
        dispose(&mut child).await?;
        if !drained {
          (&mut drain).await;
        }
        Ok::<(), CleanupFailure>(())
      })
      .await;

      (result, observed(cleanup))
    }; // `drain` dropped here, releasing the borrow of `seen`

    Exchange {
      result,
      stderr: seen,
      cleanup,
    }
  }
}

/// The exchange proper, up to and including the exit status.
///
/// It ends at **exit**, not at EOF on stdout (F-59): a host that stops at EOF
/// has already committed to a response the exit code may disclaim. Its own
/// scope, and not a block inside `exchange`, so that every `?` an exchange
/// needs is one that returns *here* — where there is nothing to clean up —
/// rather than one sitting in the region F-41 is about.
async fn body(
  mut stdin: ChildStdin,
  stdout: ChildStdout,
  child: &mut Child,
  payload: &[u8],
) -> Result<(Vec<u8>, std::process::ExitStatus), BackendError> {
  stdin.write_all(payload).await.map_err(BackendError::Io)?;
  // Taken by value and dropped here rather than at the end of the exchange.
  // The close is load-bearing: a backend that reads to EOF — the obvious way to
  // write one — hangs forever if the host holds stdin open, and the symptom is
  // a timeout on every call that looks like a slow backend (R-37).
  drop(stdin);
  let bytes = read_capped(stdout, STDOUT_LIMIT).await?;
  let status = child.wait().await.map_err(BackendError::Io)?;
  Ok((bytes, status))
}

/// Kill and reap, without waiting on either unboundedly — the caller owns the
/// budget both of its callers share.
async fn dispose(child: &mut Child) -> Result<(), CleanupFailure> {
  child.start_kill().map_err(CleanupFailure::Io)?;
  child.wait().await.map_err(CleanupFailure::Io)?;
  Ok(())
}

/// What the host managed to observe, which is all `cleanup` ever claims.
fn observed(outcome: Result<Result<(), CleanupFailure>, Elapsed>) -> Option<CleanupFailure> {
  match outcome {
    Ok(Ok(())) => None,
    Ok(Err(failure)) => Some(failure),
    Err(_) => Some(CleanupFailure::TimedOut {
      after: CLEANUP_LIMIT,
    }),
  }
}

/// A child exists but the exchange cannot proceed.
///
/// A function rather than a second `Exchange::failed` because the whole point
/// of I13 is that a child, once spawned, is disposed of on *every* returning
/// path — so this runs the same bounded disposal and reports it on the same
/// channel.
async fn cleanup_only(child: &mut Child, error: BackendError) -> Exchange {
  let cleanup = tokio::time::timeout(CLEANUP_LIMIT, dispose(child)).await;
  Exchange {
    result: Err(error),
    stderr: Captured::default(),
    cleanup: observed(cleanup),
  }
}

/// Read to EOF, or fail on exceeding `limit`.
///
/// Stops reading the moment the bound is passed and **closes the stream**:
/// there is nothing to be gained by finishing a response that is already
/// refused, and leaving the pipe open only lets the flood continue (R-43).
///
/// The reader is taken **by value**, and that is the mechanism rather than a
/// convenience — dropping the handle here is what closes the pipe at the bound
/// instead of at the end of the exchange. Measured: with a borrow, a flooding
/// backend observes the close 1.8 ms *after* the call returns; with ownership,
/// 500 ms *before* it, on a case whose disposal stalls. `transport_shape.rs`
/// asserts the signature for that reason.
async fn read_capped(
  mut reader: impl AsyncRead + Unpin + Send,
  limit: usize,
) -> Result<Vec<u8>, BackendError> {
  let mut out = Vec::new();
  loop {
    out.reserve(READ_CHUNK);
    if reader.read_buf(&mut out).await.map_err(BackendError::Io)? == 0 {
      return Ok(out);
    }
    if out.len() > limit {
      return Err(BackendError::OutputTooLarge { limit });
    }
  }
}

/// Read to EOF, keeping at most `limit` of it.
///
/// The asymmetry with `read_capped` is the whole of D34: reaching the bound
/// stops *storing*, never stops *reading*. Collapsing the two into one function
/// with a flag makes the stderr flood deadlock rather than fail, which is the
/// worst available symptom. Errors are indistinguishable from EOF here on
/// purpose — a diagnostic stream that cannot be read is not a reason to fail an
/// exchange that otherwise worked.
async fn drain_capped(
  mut reader: impl AsyncRead + Unpin + Send,
  limit: usize,
  into: &mut Captured,
) {
  let mut chunk = Vec::with_capacity(READ_CHUNK);
  loop {
    chunk.clear();
    match reader.read_buf(&mut chunk).await {
      Ok(0) | Err(_) => return,
      Ok(_) => {
        let room = limit.saturating_sub(into.bytes.len());
        if chunk.len() > room {
          chunk.truncate(room);
          into.truncated = true;
        }
        into.bytes.extend_from_slice(&chunk);
      }
    }
  }
}
