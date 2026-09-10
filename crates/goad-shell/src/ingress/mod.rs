//! The local socket a user-owned watcher writes an event to — `SPEC-003`.
//!
//! Three parts (`design.md` §5.1): [`bind`] probes, reclaims, binds and moves
//! the socket; the accept task reads one bounded envelope per connection and
//! answers it; [`Ingress::arrival`] is the door a judge — `crates/goad`'s
//! `serve`, not this module — takes each [`Arrival`] through. This module
//! decides only what the *filesystem* and the *bytes on the wire* can get
//! wrong: the framing, the two read budgets, and the one reply a well-formed
//! envelope gets. The wire's reason vocabulary is closed here at eight tokens
//! (`SPEC-003` §6.3, `SPEC-003/R-14`): `Engaged` and `TooSoon` are
//! constructed only by `crates/goad`'s loop, not by this module — this module
//! only carries them to the wire.
#![deny(clippy::arithmetic_side_effects)]

use std::io;
use std::os::unix::fs::{FileTypeExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{mpsc, oneshot};

use envelope::EnvelopeFault;
use goad_semantics::protocol::canonical::Event;

pub mod envelope;

/// Bounds the bytes a **read** may consume before an envelope ends, not the
/// connection: a writer that sends this much and keeps the connection open is
/// `too_large`; one that sends nothing is `timed_out` (`ENVELOPE_DEADLINE`
/// below). An unbounded read from an untrusted writer is the defect
/// `SPEC-001/R-43` names on the other socket (`SPEC-003` §6.4).
pub const ENVELOPE_LIMIT: usize = 64 * 1024;

/// Bounds the **read**, not the connection: the wait for the loop's judgement
/// is unbounded by design (I-2, `SPEC-003` §6.4). A writer that connects
/// and never completes an envelope must not hold ingress.
pub const ENVELOPE_DEADLINE: Duration = Duration::from_millis(500);

/// Owner-only. The host sets this itself, immediately after `bind`, rather
/// than relying on the umask it was started under (`SPEC-003/R-2`). There is
/// no safe API to set the umask instead — it is process-global while `cargo
/// test` runs cases in parallel in one process — so the window between `bind`
/// and this host setting the mode is stated residue (`design.md` §5.5 A-5),
/// not a defect this constant closes.
pub const SOCKET_MODE: u32 = 0o600;

/// Why a path could not be turned into a bound, owner-only listener.
///
/// One struct, not an enum of paths: every message names the path once, and
/// the fault says what was found (`design.md` §5.2).
#[derive(Debug)]
pub struct IngressError {
  pub path: PathBuf,
  pub fault: BindFault,
}

impl std::fmt::Display for IngressError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.path.display(), self.fault)
  }
}

impl std::error::Error for IngressError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self.fault.source()
  }
}

/// What `bind` found wrong, and about which step.
#[derive(Debug)]
pub enum BindFault {
  /// Something other than a socket occupies the path — a regular file, a
  /// directory, or a symlink (followed to neither a socket nor anything
  /// else: a symlink *to* a socket is ambiguous and is refused rather than
  /// followed, `design.md` §5.5).
  NotASocket { found: &'static str },
  /// Something other than a regular file is at the **lock's** own path
  /// ([`lock_path`]) — a symlink above all, which is refused rather than
  /// followed for the reason `SPEC-003/R-4` gives about the socket path:
  /// following one would put the owner-only mode R-2 requires on a file the
  /// configuration did not name. Names the socket's path, like every other
  /// fault, and says the lock beside it is what was wrong.
  LockNotAFile { found: &'static str },
  /// A live host holds the path's lock (`SPEC-003/R-3`) — whatever is or is
  /// not at the path itself. See [`lock_path`].
  InUse,
  /// A path could not be inspected at all — the socket's, or the lock's
  /// beside it ([`lock_path`]). Names the socket's path either way.
  Unprobeable(io::Error),
  /// Whether a live host holds the path could not be determined: the lock
  /// beside it ([`lock_path`]) exists and could not be *asked* — a filesystem
  /// with no advisory locking, most often. Neither answer is safe to assume,
  /// so neither is assumed. A lock file that could not be **opened** is not
  /// this: the location is unusable, which is [`Self::Unbindable`]
  /// (`review-code.md` F-21).
  LivenessUnknown(io::Error),
  /// A stale socket file could not be removed.
  Unlinkable(io::Error),
  /// The location cannot be used — a path component that is not a directory,
  /// a directory that does not exist or the host cannot write, a path longer
  /// than `sun_path` admits, or any other reason the platform refuses it.
  /// Raised by whichever step meets it first: opening the lock file beside
  /// the path, or `UnixListener::bind` itself. The two steps ask the same
  /// question of the same directory, so a person reads one message about the
  /// path they mistyped rather than two about which step noticed.
  Unbindable(io::Error),
  /// The socket was bound but its mode could not be set to owner-only.
  ModeUnsettable(io::Error),
}

impl std::fmt::Display for BindFault {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::NotASocket { found } => write!(f, "not a socket — found {found}"),
      Self::LockNotAFile { found } => write!(
        f,
        "the lock beside it is not a regular file — found {found}"
      ),
      Self::InUse => write!(f, "in use by a live host"),
      Self::Unprobeable(inner) => write!(f, "could not be inspected: {inner}"),
      Self::LivenessUnknown(inner) => write!(
        f,
        "whether a live host holds it could not be determined: {inner}"
      ),
      Self::Unlinkable(inner) => write!(f, "a stale socket could not be removed: {inner}"),
      Self::Unbindable(inner) => write!(f, "could not be bound: {inner}"),
      Self::ModeUnsettable(inner) => write!(f, "its mode could not be set: {inner}"),
    }
  }
}

impl std::error::Error for BindFault {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Unprobeable(inner)
      | Self::LivenessUnknown(inner)
      | Self::Unlinkable(inner)
      | Self::Unbindable(inner)
      | Self::ModeUnsettable(inner) => Some(inner),
      Self::NotASocket { .. } | Self::LockNotAFile { .. } | Self::InUse => None,
    }
  }
}

/// Probe, reclaim, bind, set the mode, spawn the accept task. Synchronous,
/// and called under the runtime guard `main.rs` already holds.
///
/// # Errors
///
/// An [`IngressError`] naming the path and what `bind` found wrong with it.
/// The host must not start without the listener its configuration asked for
/// (`SPEC-003/R-4`).
pub fn bind(path: &Path) -> Result<Ingress, IngressError> {
  let lock = reclaim(path)?;
  let listener =
    UnixListener::bind(path).map_err(|error| fault(path, BindFault::Unbindable(error)))?;
  std::fs::set_permissions(path, std::fs::Permissions::from_mode(SOCKET_MODE))
    .map_err(|error| fault(path, BindFault::ModeUnsettable(error)))?;
  let (arrivals, receiver) = mpsc::channel(1);
  tokio::spawn(accept_loop(listener, arrivals));
  Ok(Ingress {
    arrivals: Some(receiver),
    _lock: Some(lock),
  })
}

fn fault(path: &Path, kind: BindFault) -> IngressError {
  IngressError {
    path: path.to_path_buf(),
    fault: kind,
  }
}

/// Where the lock that says *a live host holds this path* lives: beside the
/// socket, named for it — `goad.sock` next to `goad.sock.lock`. A regular
/// file, so `sun_path`'s length limit does not reach it, and one a person can
/// find by looking (`SPEC-003/R-3`).
#[must_use]
pub fn lock_path(socket: &Path) -> PathBuf {
  let mut name = socket.as_os_str().to_os_string();
  name.push(".lock");
  PathBuf::from(name)
}

/// Takes the exclusive advisory lock on `path`'s sidecar file and hands it
/// back for the host to hold. Owner-only like the socket and for the same
/// reason (`SOCKET_MODE`), and here without A-5's window, because `open(2)`
/// takes the mode rather than needing a `chmod` after it.
///
/// The file is created and never removed: a lock file with nobody holding it
/// is not a claim, and `SPEC-003/R-5` keeps it for the reason it keeps the
/// socket.
///
/// **The two failures here are two different questions** (`review-code.md`
/// F-21). Failing to *open* the file says the location is unusable — the
/// directory is missing, or unwritable — which is R-4's fault and the one
/// `bind` would have raised had it got that far; failing to *lock* an opened
/// file says liveness could not be determined, which is R-3's. Collapsing
/// them points a person at a lock they did not know existed instead of at the
/// directory they mistyped.
///
/// **What is at the lock's own path is settled before it is opened**
/// (`review-code.md` F-22), by the same [`probe`] `reclaim` uses on the socket
/// path and for the reason `SPEC-003/R-4` states there: `open(2)` follows a
/// final symlink, and `.mode(SOCKET_MODE)` is applied on creation, so
/// following one would put the owner-only mode R-2 requires on a file the
/// configuration did not name. One rule, one mechanism, both host-created
/// paths — `bind(2)` has no `O_NOFOLLOW` to offer the socket path, so the
/// probe is the only mechanism that can cover both.
fn hold(path: &Path) -> Result<std::fs::File, IngressError> {
  let beside = lock_path(path);
  let occupant = probe(&beside).map_err(|error| fault(path, BindFault::Unprobeable(error)))?;
  if let Some(metadata) = &occupant
    && !metadata.file_type().is_file()
  {
    return Err(fault(
      path,
      BindFault::LockNotAFile {
        found: describe(metadata.file_type()),
      },
    ));
  }
  let lock = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .mode(SOCKET_MODE)
    .open(&beside)
    .map_err(|error| fault(path, BindFault::Unbindable(error)))?;
  match lock.try_lock() {
    Ok(()) => Ok(lock),
    Err(std::fs::TryLockError::WouldBlock) => Err(fault(path, BindFault::InUse)),
    Err(std::fs::TryLockError::Error(error)) => Err(fault(path, BindFault::LivenessUnknown(error))),
  }
}

/// A path with nothing at it is left alone. A path with a **stale** socket —
/// one no live host holds — is unlinked, so `bind` can take it
/// (`SPEC-003/R-3`). Anything else is a fault naming what was found; `bind`
/// never touches a path it did not itself just clear. Returns the lock the
/// host holds from here until it exits.
///
/// **Liveness is the lock, never a `connect`.** Taking the lock ([`hold`]) is
/// the question and holding it is the answer: a host that gets it knows no
/// live host holds the path, and one that cannot get it knows one does. The
/// order is R-4 before R-3 — what is *at* the path is settled first, so a
/// symlink or a regular file is refused as such without a lock file appearing
/// beside it.
///
/// A `connect` cannot decide this, and the code that thought it could was
/// wrong rather than imprecise (`review-code.md` F-18, measured across three
/// implementations). `connect` succeeding says *a listening socket is bound
/// here*; it does not say *a host holds it*. `fork` duplicates a listening
/// descriptor into the child, so the socket stays bound and connectable after
/// its owner has closed its own descriptor and for as long as any child holds
/// the copy — and this host forks once per exchange. The old probe read that
/// as a live host and refused to start against a path nobody held.
///
/// **Inheritance is not the difference.** A `flock` belongs to the open file
/// description too, so a `fork` duplicates it the same way. What closes the
/// gap is narrower: the lock is **never released while a host lives**, so
/// there is no release for anything to race — the old failure needed a
/// release followed by a probe of the same path, and here a path that reads
/// as reclaimable has never been locked by anyone when `bind` reaches it. It
/// also costs a live host nothing, because nothing connects to it. The
/// residue that follows from the same inheritance — an un-exec'd child of a
/// host that has just died still holds the lock, briefly and self-clearingly
/// — is stated in `SPEC-003` §6.1.
///
/// **Upgrade skew, stated rather than discovered.** A socket left by a host
/// that predates the lock file has no lock beside it, so this reads it as
/// stale and unlinks it: right for a dead old host, wrong for a live one,
/// which would go on serving a socket no path reaches (`SPEC-003` §6.1,
/// *the path after the bind*). The window is the one upgrade that crosses this
/// change, and one restart of the old host closes it.
fn reclaim(path: &Path) -> Result<std::fs::File, IngressError> {
  let occupant = probe(path).map_err(|error| fault(path, BindFault::Unprobeable(error)))?;
  if let Some(metadata) = &occupant
    && !metadata.file_type().is_socket()
  {
    return Err(fault(
      path,
      BindFault::NotASocket {
        found: describe(metadata.file_type()),
      },
    ));
  }
  let lock = hold(path)?;
  if occupant.is_some() {
    std::fs::remove_file(path).map_err(|error| fault(path, BindFault::Unlinkable(error)))?;
  }
  Ok(lock)
}

/// What is at a path, without following a final symlink — `None` for a path
/// with nothing at it. Both paths the host creates are asked this way and for
/// one reason: a link is refused as the link it is rather than followed to
/// whatever it names (`SPEC-003/R-3`, R-4).
fn probe(path: &Path) -> io::Result<Option<std::fs::Metadata>> {
  match std::fs::symlink_metadata(path) {
    Ok(metadata) => Ok(Some(metadata)),
    Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
    Err(error) => Err(error),
  }
}

/// What was found at a path that is not what belongs there, for
/// `BindFault::NotASocket` and `BindFault::LockNotAFile`.
fn describe(file_type: std::fs::FileType) -> &'static str {
  if file_type.is_symlink() {
    "a symlink"
  } else if file_type.is_dir() {
    "a directory"
  } else if file_type.is_file() {
    "a regular file"
  } else if file_type.is_fifo() {
    "a FIFO"
  } else if file_type.is_char_device() {
    "a character device"
  } else if file_type.is_block_device() {
    "a block device"
  } else {
    "something this host does not recognise"
  }
}

/// The handle `serve` holds. Either a bound receiver, or [`Ingress::none()`]'s
/// permanent park — two different states, both represented, so `serve` takes
/// one unconditionally (`design.md` §5.1, AC-7 by construction).
#[derive(Debug)]
pub struct Ingress {
  arrivals: Option<mpsc::Receiver<Arrival>>,
  /// The exclusive lock on the socket path, held for as long as this handle
  /// lives — which is the process, since `serve` owns it. Never read: holding
  /// it *is* the signal a second host reads (`reclaim`), so dropping it early
  /// would tell that host the path is free while this one still serves it.
  /// `None` for [`Ingress::none()`], which holds no path.
  _lock: Option<std::fs::File>,
}

impl Ingress {
  /// The handle a host with no socket holds.
  #[must_use]
  pub fn none() -> Self {
    Self {
      arrivals: None,
      _lock: None,
    }
  }

  /// Cancel-safe. **Never resolves** when nothing is bound — that is
  /// `none()`'s park, and it is a different state from the one below.
  ///
  /// `None` means a **bound** receiver's senders are all gone: the accept
  /// task has ended for a reason other than this receiver being dropped,
  /// which today means it panicked. The receiver is dropped on the way out,
  /// so the arm parks from then on instead of spinning on a closed channel.
  pub async fn arrival(&mut self) -> Option<Arrival> {
    let Some(receiver) = &mut self.arrivals else {
      return std::future::pending().await;
    };
    let arrival = receiver.recv().await;
    if arrival.is_none() {
      self.arrivals = None;
    }
    arrival
  }
}

/// One connection's outcome, handed from the accept task to whatever judges
/// it: an [`Event`] normalized from the bytes it wrote, or the [`Refusal`]
/// that shape earned it — plus the one channel its reply leaves by.
#[derive(Debug)]
pub struct Arrival {
  result: Result<Event, Refusal>,
  answer: Answer,
}

impl Arrival {
  /// Both halves, each consumed exactly once by type.
  pub fn into_parts(self) -> (Result<Event, Refusal>, Answer) {
    (self.result, self.answer)
  }
}

/// The one reply a connection gets. `accepted` and `refused` both consume
/// `self`; a **dropped** `Answer` is the defined `unavailable` path — the
/// accept task sees the channel close and writes it (`design.md` §5.2).
#[derive(Debug)]
pub struct Answer(oneshot::Sender<String>);

impl Answer {
  pub fn accepted(self) {
    match self.0.send(reply(true, None)) {
      Ok(()) | Err(_) => (),
    }
  }

  pub fn refused(self, refusal: &Refusal) {
    match self.0.send(reply(false, Some(refusal))) {
      Ok(()) | Err(_) => (),
    }
  }
}

/// `Refusal::Unavailable`'s cause: which of the four `SPEC-003` §6.3
/// admits produced it — the `unavailable` row's *writer's fix* column,
/// "`detail` says which". One token, four causes, and it stays one token: the
/// wire's reason set is closed at eight whatever this enum grows
/// (`SPEC-003/R-14`).
///
/// [`IngressStopped`](Self::IngressStopped) is the one that never reaches the
/// wire — it answers no envelope, so it has no reply to be carried in. It is a
/// variant anyway, because the diagnostics surface is where it *is* reported
/// and `crates/goad`'s fold reads `reason()` and `Display` off this value like
/// every other refusal, rather than spelling the token a second time
/// (`review-code.md` F-4).
#[derive(Debug)]
pub enum UnavailableCause {
  /// The channel to the judge closed with this envelope's answer still
  /// outstanding: the host is stopping.
  Stopping,
  /// The clock could not be read (`design.md` §5.4 step 4) — named by
  /// `crates/goad`'s loop, not by this module.
  ClockUnreadable,
  /// The accept task has ended and nothing restarts it: ingress is over for
  /// the life of the process. Named by `crates/goad`'s loop, not by this
  /// module, and never written to a wire.
  IngressStopped,
  /// The connection faulted while the envelope was being read: a reset, an
  /// `EIO`, any transport error. **Not `Malformed`** — that reason names the
  /// writer's serializer as what to fix (`SPEC-003` §6.3), and a writer
  /// whose bytes never arrived did not send bad ones (`review-code.md` F-7).
  Unreadable(io::Error),
}

/// Why an envelope, or a connection, was refused.
///
/// **Seven variants, closing the wire's eight-token reason set.** `reason()`
/// splits `InvalidEnvelope(EnvelopeFault::ReservedSource)` out to its own
/// token, `reserved_source` (`SPEC-003` §6.3, `SPEC-003/R-13`) — the
/// payload stays the one variant; only the wire reads it as two. Exhaustive
/// with no `_` arm, so the compiler — not a reviewer — is what notices the
/// day a ninth reason is needed.
#[derive(Debug)]
pub enum Refusal {
  /// The host cannot act on this envelope. `detail` says which of
  /// [`UnavailableCause`]'s causes it was.
  Unavailable(UnavailableCause),
  /// The bytes are not one JSON document at all.
  Malformed,
  /// A well-formed document that is not an admissible envelope
  /// (`SPEC-003/R-9`, `R-10`), or `source == "host"` (`R-13`) — the latter
  /// reads off the wire as its own reason, `reserved_source` (`reason()`
  /// below).
  InvalidEnvelope(EnvelopeFault),
  /// More than [`ENVELOPE_LIMIT`] bytes arrived before the envelope ended.
  TooLarge { limit: usize },
  /// Nothing complete arrived within [`ENVELOPE_DEADLINE`].
  TimedOut { after: Duration },
  /// An exchange was already in flight when this envelope arrived, and its
  /// shape was good (`SPEC-002/R-9`). Constructed by `crates/goad`'s loop,
  /// never by this module.
  Engaged,
  /// Inside the minimum spacing (`SPEC-002/R-12`, `SPEC-003/R-12`).
  /// `retry_after` is the remaining spacing at the moment of refusal; the
  /// wire's `retry_after_ms` rounds it up (`R-14`). Constructed by
  /// `crates/goad`'s loop, never by this module.
  TooSoon { retry_after: Duration },
}

impl Refusal {
  /// The wire's machine-readable token (`SPEC-003` §6.3). The closed set
  /// of eight: `malformed`, `invalid_envelope`, `reserved_source`,
  /// `too_large`, `timed_out`, `engaged`, `too_soon`, `unavailable`.
  #[must_use]
  pub fn reason(&self) -> &'static str {
    match self {
      Self::Unavailable(_) => "unavailable",
      Self::Malformed => "malformed",
      Self::InvalidEnvelope(EnvelopeFault::ReservedSource) => "reserved_source",
      Self::InvalidEnvelope(_) => "invalid_envelope",
      Self::TooLarge { .. } => "too_large",
      Self::TimedOut { .. } => "timed_out",
      Self::Engaged => "engaged",
      Self::TooSoon { .. } => "too_soon",
    }
  }
}

impl std::fmt::Display for Refusal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unavailable(UnavailableCause::Stopping) => {
        write!(f, "no answer was given for this envelope")
      }
      Self::Unavailable(UnavailableCause::ClockUnreadable) => {
        write!(f, "the clock could not be read")
      }
      Self::Unavailable(UnavailableCause::IngressStopped) => {
        write!(f, "ingress has stopped; no further events will be accepted")
      }
      Self::Unavailable(UnavailableCause::Unreadable(inner)) => {
        write!(f, "the connection could not be read: {inner}")
      }
      Self::Malformed => write!(f, "the bytes are not one JSON document"),
      Self::InvalidEnvelope(inner) => write!(f, "{inner}"),
      Self::TooLarge { limit } => {
        write!(
          f,
          "more than {limit} bytes arrived before the envelope ended"
        )
      }
      Self::TimedOut { after } => {
        write!(f, "nothing complete arrived within {}ms", after.as_millis())
      }
      Self::Engaged => write!(f, "an exchange was already in flight"),
      Self::TooSoon { retry_after } => {
        write!(
          f,
          "inside the minimum spacing; {}ms remain",
          round_up_millis(*retry_after)
        )
      }
    }
  }
}

impl std::error::Error for Refusal {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::InvalidEnvelope(inner) => Some(inner),
      Self::Unavailable(UnavailableCause::Unreadable(inner)) => Some(inner),
      Self::Unavailable(
        UnavailableCause::Stopping
        | UnavailableCause::ClockUnreadable
        | UnavailableCause::IngressStopped,
      )
      | Self::Malformed
      | Self::TooLarge { .. }
      | Self::TimedOut { .. }
      | Self::Engaged
      | Self::TooSoon { .. } => None,
    }
  }
}

/// `remaining` rounded up to the millisecond (`SPEC-003/R-14`): a truncated
/// remainder would leave a writer that waited exactly that long still inside
/// the spacing, which would make R-14's own sentence false of this field.
/// Adding just under a millisecond before truncating, rather than dividing,
/// is what lets [`Duration::as_millis`] do the truncation — in the standard
/// library, not in this crate's own arithmetic — so
/// `clippy::integer_division` and `clippy::as_conversions` (both `deny`) stay
/// clear; `u64::try_from` narrows the `u128` [`Duration::as_millis`] returns,
/// the same fallback-rather-than-panic shape [`read_envelope`]'s own `cap`
/// already uses for the same reason.
fn round_up_millis(remaining: Duration) -> u64 {
  let ceiling = remaining
    .checked_add(Duration::from_nanos(999_999))
    .unwrap_or(Duration::MAX);
  u64::try_from(ceiling.as_millis()).unwrap_or(u64::MAX)
}

/// The reply wire form (`SPEC-003` §6.3): one JSON object naming
/// `protocol`, `accepted`, and — exactly when refused — `reason` and
/// `detail`, plus `retry_after_ms` exactly when `reason` is `too_soon`
/// (`R-14`). Built with `serde_json` rather than interpolated, because
/// `detail` can carry a watcher-chosen key name (`EnvelopeFault::Unknown`,
/// `Duplicate`) and hand-rolled JSON would let it break the reply's own
/// syntax.
#[derive(serde::Serialize)]
struct Wire<'a> {
  protocol: u8,
  accepted: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reason: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  retry_after_ms: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  detail: Option<String>,
}

/// **Newline-terminated** (`SPEC-003` §6.3): the terminator is one byte
/// and strictly widens compatibility — a reader that stops at `\n` and a
/// reader that reads to EOF both work against a host that emits it, and only
/// the second works against one that does not. PHASE-03 reasoned the other
/// way, from the A-1 probe's harvested byte count; "the connection's close is
/// the line's terminator" is a true statement about this host and not a reason
/// for the contract to promise a byte it never sends (`review-code.md` F-1).
fn reply(accepted: bool, refusal: Option<&Refusal>) -> String {
  let retry_after_ms = match refusal {
    Some(Refusal::TooSoon { retry_after }) => Some(round_up_millis(*retry_after)),
    _no_other_reason_carries_it => None,
  };
  let wire = Wire {
    protocol: 1,
    accepted,
    reason: refusal.map(Refusal::reason),
    retry_after_ms,
    detail: refusal.map(ToString::to_string),
  };
  // A host-authored value of primitive fields serializes infallibly; a
  // failure here is a defect in this module, not a caller's mistake — the
  // same argument `process.rs`'s own `#[expect(clippy::unwrap_used, ...)]`
  // rests on for a host-authored `Request`.
  #[expect(
    clippy::unwrap_used,
    reason = "a host-authored reply of primitive fields serializes infallibly (process.rs's \
              own precedent for this argument)"
  )]
  let mut line = serde_json::to_string(&wire).unwrap();
  line.push('\n');
  line
}

/// How long the host tolerates a **persistent** `accept()` fault before it
/// declares ingress over: **five seconds**.
///
/// Stated in elapsed time rather than as a count of retries, because that is
/// what the bound is *about*. `accept(2)` errors that persist rather than
/// clearing on the next call are ordinary — `EMFILE`/`ENFILE`,
/// `ENOBUFS`/`ENOMEM`, `EBADF` — and retrying without a bound pegs a worker for
/// the life of the fault on an application expected to sit idle
/// (`review-code.md` F-2). Spending the budget trades a recoverable fault for
/// an unrecoverable one — ingress is over for the life of the process — and
/// that trade is only legible if the bound says how long a transient fault has
/// to clear. A count of retries would not: with backoff, N failures is
/// anywhere from milliseconds to minutes.
const ACCEPT_FAULT_BUDGET: Duration = Duration::from_secs(5);

/// The shortest wait between retries: **5 ms**, the wait a fault that clears
/// immediately costs.
const ACCEPT_BACKOFF_BASE: Duration = Duration::from_millis(5);

/// The longest wait between retries: **500 ms**, so the whole budget above
/// costs a bounded handful of syscalls rather than a spin, and a fault that
/// does clear is noticed within half a second of clearing.
const ACCEPT_BACKOFF_CEILING: Duration = Duration::from_millis(500);

/// How long to wait before the next `accept()` retry, given how long the
/// current fault has already lasted — or `None` once
/// [`ACCEPT_FAULT_BUDGET`] is spent and the task must end.
///
/// Waiting as long as the fault has lasted is what makes the backoff double:
/// each sleep doubles the elapsed time, so the waits run 5, 10, 20 … ms up to
/// the ceiling and then hold there. It is one expression rather than a
/// schedule because the bound is the elapsed time, and the schedule is only
/// how few syscalls it takes to reach it.
///
/// Ending is not a new reporting surface: dropping the sender is what
/// `Ingress::arrival` already reads as `None`, and the judge already has one
/// word for it — ingress has stopped, permanently, for the life of the process
/// (`SPEC-003` §5, R-15). A persistent `accept()` fault *is* that condition.
fn accept_backoff(faulting_for: Duration) -> Option<Duration> {
  if faulting_for >= ACCEPT_FAULT_BUDGET {
    return None;
  }
  Some(faulting_for.clamp(ACCEPT_BACKOFF_BASE, ACCEPT_BACKOFF_CEILING))
}

/// One envelope, one connection: accept, read (bounded), normalize, hand the
/// arrival to whatever judges it, reply, close. Sequential — the judgement of
/// one arrival is awaited before the next connection is accepted, so at most
/// one is outstanding and the host holds no queue (I-2, `SPEC-003` §5).
///
/// An `accept()` error does not end the task on its own: it is retried after
/// [`accept_backoff`]'s wait, and the fault is forgotten by the next connection
/// that does arrive. A fault that lasts [`ACCEPT_FAULT_BUDGET`] — five seconds
/// — ends the task, which drops the channel to the judge: the ingress-stopped
/// path that already exists, already reported and already parked. The task also
/// ends when that channel closes from the other side.
async fn accept_loop(listener: UnixListener, arrivals: mpsc::Sender<Arrival>) {
  let mut faulting_since: Option<tokio::time::Instant> = None;
  loop {
    let Ok((stream, _addr)) = listener.accept().await else {
      let since = *faulting_since.get_or_insert_with(tokio::time::Instant::now);
      let Some(wait) = accept_backoff(since.elapsed()) else {
        break;
      };
      tokio::time::sleep(wait).await;
      continue;
    };
    faulting_since = None;
    if !handle(stream, &arrivals).await {
      break;
    }
  }
}

/// `false` means the channel to the judge is gone — the loop has ended — and
/// the accept task should stop.
async fn handle(mut stream: UnixStream, arrivals: &mpsc::Sender<Arrival>) -> bool {
  let result = match read_envelope(&mut stream).await {
    Ok(bytes) => envelope::normalize(&bytes).map_err(shape_refusal),
    Err(refusal @ (Refusal::TooLarge { .. } | Refusal::TimedOut { .. })) => {
      // `TooLarge` guarantees bytes the writer already sent are still unread
      // in the kernel's receive buffer; `TimedOut` rarely does. Closing a Unix
      // stream socket with unread bytes pending resets the connection, which
      // can take the reply this function is about to write down with it —
      // R-8 requires the reply regardless, so whatever is *already* queued is
      // cleared first. Non-blocking: it must not wait for more to arrive,
      // which would silently double `ENVELOPE_DEADLINE` for exactly the case
      // (a silent writer) that has nothing to clear at all.
      drain(&stream);
      Err(refusal)
    }
    Err(refusal) => Err(refusal),
  };
  let (tx, rx) = oneshot::channel();
  let arrival = Arrival {
    result,
    answer: Answer(tx),
  };
  // The judge is gone — the loop has ended, and this connection was accepted
  // in the window between that and the process itself going away
  // (`crates/goad/src/main.rs`'s unwind). R-8 admits exactly one unanswered
  // close, *the host process is gone*, and this is not it: the writer is owed
  // the same `unavailable` the adjacent dropped-`Answer` case below already
  // gives it, because the two are one failure at two moments (`review-code.md`
  // F-8).
  if arrivals.send(arrival).await.is_err() {
    respond(&mut stream, &stopping()).await;
    return false;
  }
  let text = rx.await.unwrap_or_else(|_dropped| stopping());
  respond(&mut stream, &text).await;
  true
}

/// The reply for both moments at which the judge can vanish with an envelope
/// outstanding: the channel to it was already closed when this arrival was
/// sent, or the [`Answer`] was dropped after it.
fn stopping() -> String {
  reply(
    false,
    Some(&Refusal::Unavailable(UnavailableCause::Stopping)),
  )
}

/// The one reply, then the close. Both are best-effort: a writer that has gone
/// away cannot be told anything, and there is nothing left to report it to.
async fn respond(stream: &mut UnixStream, text: &str) {
  match stream.write_all(text.as_bytes()).await {
    Ok(()) | Err(_) => (),
  }
  match stream.shutdown().await {
    Ok(()) | Err(_) => (),
  }
}

/// `normalize`'s one fault with no shape of its own (`Malformed` — bytes that
/// are not one JSON document) reads off the wire as `malformed`; every other
/// fault reads off as `invalid_envelope`, `detail` naming which
/// (`SPEC-003/R-9`, `R-10`) — `ReservedSource` included: it stays this
/// payload, and `Refusal::reason()` is what gives it its own wire token,
/// `reserved_source` (`R-13`).
fn shape_refusal(fault: EnvelopeFault) -> Refusal {
  match fault {
    EnvelopeFault::Malformed => Refusal::Malformed,
    other => Refusal::InvalidEnvelope(other),
  }
}

/// One envelope: until the first newline or end of input, whichever comes
/// first (`SPEC-003/R-6`), bounded in both bytes and time (`R-7`). `stream`
/// is borrowed, not consumed, so the caller can still reply and close it
/// after either bound refuses.
///
/// The framing and the byte bound are one read: `stream` is wrapped in
/// `take(ENVELOPE_LIMIT + 1)` before `read_until` runs, so a body that reaches
/// the cap without a newline is distinguishable from one that ends exactly at
/// it — `process.rs::read_capped`'s own `+ 1` idiom, reused for the reason it
/// was written there.
async fn read_envelope(stream: &mut UnixStream) -> Result<Vec<u8>, Refusal> {
  let cap = u64::try_from(ENVELOPE_LIMIT)
    .unwrap_or(u64::MAX)
    .saturating_add(1);
  let mut reader = BufReader::new(stream.take(cap));
  let mut buf = Vec::new();
  let read = tokio::time::timeout(ENVELOPE_DEADLINE, reader.read_until(b'\n', &mut buf)).await;
  match read {
    Err(_elapsed) => Err(Refusal::TimedOut {
      after: ENVELOPE_DEADLINE,
    }),
    Ok(Err(io)) => Err(unreadable(io)),
    Ok(Ok(_bytes_read)) => {
      if buf.last() == Some(&b'\n') {
        buf.pop();
        Ok(buf)
      } else if buf.len() > ENVELOPE_LIMIT {
        Err(Refusal::TooLarge {
          limit: ENVELOPE_LIMIT,
        })
      } else {
        Ok(buf)
      }
    }
  }
}

/// A read that faulted, as the refusal it earns — **`unavailable`, not
/// `malformed`** (`review-code.md` F-7). `malformed` means *the bytes were not
/// one JSON document* and names the writer's serializer as the thing to fix
/// (`SPEC-003` §6.3); a connection that reset mid-envelope sent no bytes
/// this host could read, and `refuse_arrival` puts the same verdict in front of
/// a person, not only in front of the writer. The error is carried into
/// `detail` rather than discarded.
///
/// A function rather than an inline arm because it is the one place the rule is
/// stated, and because a real `io::Error` from a Unix stream read cannot be
/// provoked from a test — a peer's close is EOF, not a fault — so this is where
/// the rule is reachable at all.
fn unreadable(error: io::Error) -> Refusal {
  Refusal::Unavailable(UnavailableCause::Unreadable(error))
}

/// Best-effort, and non-blocking: discards whatever is *already* sitting in
/// the kernel's receive buffer, without waiting for more of it to arrive.
/// Bounded in bytes at [`ENVELOPE_LIMIT`] so a writer that keeps a socket's
/// buffer replenished cannot turn this into a busy loop; unbounded in time on
/// purpose — every call it makes is non-blocking, so there is no wait to
/// bound. `Ok(0)` (the writer closed) and anything not immediately readable
/// both end it the same way: there is nothing more to clear right now.
fn drain(stream: &UnixStream) {
  let mut sink = [0_u8; 4096];
  let mut cleared = 0_usize;
  while cleared < ENVELOPE_LIMIT {
    match stream.try_read(&mut sink) {
      Ok(0) | Err(_) => break,
      Ok(bytes) => cleared = cleared.saturating_add(bytes),
    }
  }
}

// The accept task's retry budget, tested where it can be: as the pure function
// that decides it. **No case drives a real `accept()` error** — `EMFILE` and
// its siblings are process-wide conditions, and `cargo test` runs cases in
// parallel in one process, which is the same argument `SOCKET_MODE`'s doc
// comment makes about `umask(2)`. What is asserted here is the budget itself:
// that it backs off, that it is capped, and that it is spent, after which
// `accept_loop` breaks and the ingress-stopped path — which VT-7 does drive
// end to end (`crates/goad/tests/renderer/ingress.rs`) — takes over.
#[cfg(test)]
mod tests {
  use super::{
    ACCEPT_BACKOFF_BASE, ACCEPT_BACKOFF_CEILING, ACCEPT_FAULT_BUDGET, accept_backoff, lock_path,
    reply, unreadable,
  };

  /// The lock sits **beside** the socket and is named for it — appended, not
  /// substituted, so two configured paths that differ only in extension get
  /// two locks (`goad.sock` and `goad.sock.lock`, never `goad.lock`).
  #[test]
  fn the_lock_is_the_socket_s_own_path_with_a_suffix() {
    assert_eq!(
      lock_path(std::path::Path::new("/run/goad.sock")),
      std::path::PathBuf::from("/run/goad.sock.lock")
    );
    assert_eq!(
      lock_path(std::path::Path::new("/run/goad")),
      std::path::PathBuf::from("/run/goad.lock")
    );
  }

  /// F-7's rule, at the one site that states it. A transport fault is
  /// `unavailable` — *the host cannot act on this envelope* — and never
  /// `malformed`, which names the writer's serializer and would be a lie both
  /// on the wire and on the diagnostics surface `refuse_arrival` folds it onto.
  ///
  /// A unit case rather than an end-to-end one, and honestly so: a peer that
  /// closes a Unix stream socket gives the host EOF, not an error, so no
  /// writer a test can build makes `read_until` fail.
  #[test]
  fn a_connection_that_faults_mid_read_is_unavailable_and_carries_the_error() {
    let refusal = unreadable(std::io::Error::from(std::io::ErrorKind::ConnectionReset));
    assert_eq!(
      refusal.reason(),
      "unavailable",
      "a transport fault is not the writer's serializer"
    );
    let detail = refusal.to_string();
    assert!(
      detail.contains("could not be read"),
      "the detail says the connection faulted: {detail}"
    );
    assert!(
      detail.contains(&std::io::Error::from(std::io::ErrorKind::ConnectionReset).to_string()),
      "the error itself is carried, not discarded: {detail}"
    );
  }

  #[test]
  fn a_retry_waits_as_long_as_the_fault_has_lasted_between_the_two_bounds() {
    assert_eq!(
      accept_backoff(std::time::Duration::ZERO),
      Some(ACCEPT_BACKOFF_BASE),
      "a fault noticed at once waits the shortest wait, not nothing"
    );
    let mid = std::time::Duration::from_millis(40);
    assert_eq!(
      accept_backoff(mid),
      Some(mid),
      "between the bounds the wait is the elapsed fault, which is what doubles it"
    );
    assert_eq!(
      accept_backoff(std::time::Duration::from_secs(2)),
      Some(ACCEPT_BACKOFF_CEILING),
      "and it is capped, so the budget costs a bounded handful of syscalls"
    );
  }

  /// The property the loop turns on: a fault that never clears is not retried
  /// forever. `None` is what makes `accept_loop` break, drop its sender, and
  /// leave the judge to report ingress-stopped.
  #[test]
  fn a_fault_that_outlasts_the_budget_ends_the_task_rather_than_spinning() {
    assert_eq!(
      accept_backoff(ACCEPT_FAULT_BUDGET),
      None,
      "the budget is spent at {ACCEPT_FAULT_BUDGET:?} of continuous fault"
    );
    assert_eq!(
      accept_backoff(std::time::Duration::MAX),
      None,
      "and stays spent past it"
    );
    assert!(
      accept_backoff(ACCEPT_FAULT_BUDGET.saturating_sub(std::time::Duration::from_millis(1)))
        .is_some(),
      "one millisecond inside the budget still retries"
    );
  }

  /// §6.3's reply is one **line**. Asserted here as well as on the wire
  /// (`tests/integration/ingress.rs`) because this is the function that owes
  /// the byte, and the wire case cannot say which of the two writers produced
  /// it.
  #[test]
  fn a_reply_is_one_newline_terminated_line() {
    let accepted = reply(true, None);
    assert!(accepted.ends_with('\n'), "{accepted:?}");
    assert_eq!(accepted.matches('\n').count(), 1, "{accepted:?}");
  }
}
