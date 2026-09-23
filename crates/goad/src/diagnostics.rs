//! Everything a person reads, in one module (design.md §5.2, principle 4).
//!
//! PHASE-04 landed the tray rasteriser: `TrayState`, `ICON_EDGE`, `IDLE`,
//! `FAULT` and `tray_icon`. PHASE-05 added `Reported`, `Refused`,
//! `Diagnostics`, `tooltip`, `BUSY_NOTICE`, the remaining outlets
//! (`goad_shell::report::line_to`, `report_platform`) and the escape/bound
//! pipeline, `finish`, that every composed line on the in-window surface goes
//! through. The host's own standard error takes `one_line` instead — one
//! terminator dropped, then the same escape, and no bound (010
//! `review-code.md` F-10). PHASE-08 adds the
//! startup surface's own outlets, `USAGE` and `print_usage`, and the impure
//! outlet that 010/PHASE-02 replaced with `report_exit` and its pure half,
//! `report_exit_line`, before a tray or a window exists; 006/PHASE-03 adds
//! `version_line` and `print_version`, which answer before one exists too.
//! The module carries
//! the arithmetic deny because both halves compute over lengths a backend or a
//! transport chose (D53, design.md §5.4).
#![deny(clippy::arithmetic_side_effects)]

use std::fmt::{self, Write as _};

use goad_semantics::protocol::canonical::Timestamp;
use goad_semantics::protocol::normalize::Discarded;
use goad_shell::backend::transport::Captured;
use goad_shell::error::CleanupFailure;
use goad_shell::host::Failure;
use goad_shell::report::{line_to, try_line_to};
use slint::{Rgba8Pixel, SharedPixelBuffer};

use crate::exit::Ended;
use crate::startup::StartupError;
use crate::view_model::Undrawn;

/// The tray's severity, derived from the diagnostic surface rather than
/// stored — `Diagnostics::state()` reads it on every `present`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayState {
  Idle,
  Fault,
}

/// Everything an exchange produced that is not the view. Built by `receive`
/// (`reception.rs`) and by nothing else, so no caller can assemble a partial
/// one.
#[derive(Debug)]
pub struct Reported {
  pub failure: Option<Failure>,
  pub cleanup: Option<CleanupFailure>,
  pub discarded: Vec<Discarded>,
  pub stderr: Captured,
}

/// Something the renderer refused before any backend was contacted.
///
/// It lives here rather than beside the controller for the reason
/// `StateError` lives in `error.rs` rather than in `state.rs`: it is a
/// refusal with a user-visible rendering, and every user-visible string in
/// this renderer is in one file. The controller names it; it does not own
/// it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
  /// The click named a presentation that is no longer outstanding (F-13).
  SupersededView,
  /// The click named an option the retained presentation does not carry. A
  /// renderer bug rather than an answer: reported, never sent.
  UnknownOption,
  /// The edit could not be applied to a field of the option it named. Two
  /// ways in: that option's blocks do not declare the field, or the widget's
  /// report is one `view_model::interpret` cannot read against the field that
  /// was drawn — an index no alternative has, a non-finite slider value, or a
  /// report whose variant is not the drawn field's kind.
  ///
  /// Distinguishable from `UnknownOption` deliberately: the line says which of
  /// the two selectors failed. Every path into it is a **renderer bug** — the
  /// first reachable only from a stale or malformed callback, the rest from a
  /// well-formed and current one whose control reported something the drawn
  /// field cannot hold — and all of them take `UnknownOption`'s posture:
  /// reported, nothing recorded (`design.md` §5.2, and §5.5's edge table).
  UnknownField,
  /// The wall clock could not be read, so no request can be stamped.
  NoClock { detail: String },
  /// An event the host was offered was refused. Two **rendered** values —
  /// the wire's machine-readable reason and the prose behind it — never the
  /// refusal itself: this module states what a person reads and holds no
  /// vocabulary of the thing that was refused (`SPEC-003/R-15`).
  Ingress { reason: String, detail: String },
}

/// Every line a person can read on this surface, and whether any of them is
/// a fault. `lines` and `fault` are private: the only way to build one is
/// `of` or `refused`, so neither can be assembled partially.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diagnostics {
  lines: Vec<String>,
  fault: bool,
}

/// The bound on a captured backend's stderr, one line on the in-window
/// surface: the only in-window line whose whole value is diagnostic prose a
/// person reads (design.md §5.4). The host's own standard error lines are not
/// this stream and are not bounded — `one_line` says why.
const STDERR_LIMIT: usize = 4096;
/// Every other in-window line's bound: enough for a serde message quoting a
/// document, an OS error, or a discarded `raw`. The host's own standard error
/// is not bounded — `one_line` says why.
const LINE_LIMIT: usize = 1024;
/// The tray tooltip's bound: one line in a panel.
const TOOLTIP_LIMIT: usize = 120;

/// `Captured::truncated`'s own fixed sentence — the outer fact, named once,
/// with no number: `STDERR_LIMIT` is a private stratum-2 const and copying
/// its value here would be a second statement of one fact that can drift.
const CAPTURE_TRUNCATED: &str =
  "stderr was cut at the host's capture limit; the backend wrote more than the host kept";

impl Diagnostics {
  /// The reducer. Consuming, because `Outcome`'s parts are owned and not
  /// `Clone`; total, because every field of every input has a rendering.
  /// `undrawn` is a **separate argument** rather than a field of `Reported`
  /// because it is the mapper's fact and `Outcome` has no field for it — and
  /// it is passed by the one function that has both in hand, `receive`, so it
  /// cannot be forgotten (F-7, I-2).
  #[must_use]
  pub fn of(reported: Reported, undrawn: &[Undrawn]) -> Self {
    let Reported {
      failure,
      cleanup,
      discarded,
      stderr,
    } = reported;

    let mut lines = Vec::new();
    let mut fault = false;

    if let Some(failure) = &failure {
      lines.push(finish(&format!("no action taken: {failure}"), LINE_LIMIT));
      fault = true;
    }
    if let Some(cleanup) = &cleanup {
      lines.push(finish(
        &format!("cleanup unverified: {cleanup}"),
        LINE_LIMIT,
      ));
      fault = true;
    }
    for part in undrawn {
      lines.push(finish(&undrawn_line(part), LINE_LIMIT));
      fault = true;
    }
    for part in &discarded {
      lines.push(finish(&part.to_string(), LINE_LIMIT));
      fault = true;
    }
    if stderr.truncated {
      lines.push(finish(CAPTURE_TRUNCATED, LINE_LIMIT));
    }
    if !stderr.bytes.is_empty() {
      let decoded = String::from_utf8_lossy(&stderr.bytes);
      let decoded = without_one_terminator(&decoded);
      lines.push(finish(&format!("stderr: {decoded}"), STDERR_LIMIT));
    }

    Self { lines, fault }
  }

  /// A refusal the host made with no backend involved, so there is no
  /// `Outcome` to reduce. One line, with the same prefix as any other
  /// refusal, because it is the same fact: the host took no action and is
  /// saying so.
  #[must_use]
  pub fn refused(refused: &Refused) -> Self {
    let composed = match refused {
      Refused::SupersededView => {
        "no action taken: that answer belongs to a question that has since been replaced".to_owned()
      }
      Refused::UnknownOption => {
        "no action taken: the host could not match that control to the question it is holding"
          .to_owned()
      }
      Refused::UnknownField => {
        "no action taken: the host could not match that control to a field of the option it names"
          .to_owned()
      }
      Refused::NoClock { detail } => format!(
        "no action taken: the system clock could not be read, so no request could be stamped ({detail})"
      ),
      Refused::Ingress { reason, detail } => {
        format!("no action taken: an event was refused ({reason}): {detail}")
      }
    };
    Self {
      lines: vec![finish(&composed, LINE_LIMIT)],
      fault: true,
    }
  }

  #[must_use]
  pub fn is_clear(&self) -> bool {
    self.lines.is_empty()
  }

  #[must_use]
  pub fn lines(&self) -> &[String] {
    &self.lines
  }

  #[must_use]
  pub fn state(&self) -> TrayState {
    if self.fault {
      TrayState::Fault
    } else {
      TrayState::Idle
    }
  }
}

/// One `Undrawn`'s line, before escaping and bounding.
fn undrawn_line(undrawn: &Undrawn) -> String {
  match undrawn {
    Undrawn::FieldForm {
      option,
      field,
      form,
    } => {
      let option = option.as_str();
      let field = field.as_str();
      // **Names the field's own form and no subset of the drawn ones.** The
      // clause that used to be here — *"this renderer draws boolean fields
      // only"* — was true while `boolean` was the one kind drawn, and went
      // false the moment a second did. Naming the set is the mistake, not the
      // count: any enumeration is stale again at the next phase that draws a
      // kind. What a backend author can act on is which of *their* field was
      // not drawn, which is the part that never goes stale
      // (`plan.md` PHASE-05 Surfaces, `plan-log.md` 2026-09-19).
      format!(
        "not drawn: option {option} field {field} is a {form} field, which this renderer has no control for"
      )
    }
    Undrawn::GroupHint { option, field } => {
      let option = option.as_str();
      let field = field.as_str();
      format!(
        "not grouped: option {option} field {field} carries a group hint that is not a string"
      )
    }
    Undrawn::MarkdownUnsupported { detail } => {
      format!("shown as plain text: this body's markdown was not understood ({detail})")
    }
    Undrawn::ContentForm { form } => {
      format!("shown as plain text: this body is {form}, and nothing here draws that form")
    }
  }
}

/// Drops **at most one** trailing line terminator — `\r\n`, `\n` or `\r`.
///
/// A capture is one line of this surface, so an embedded newline must be
/// escaped: without that, one capture would break the list's shape. The
/// *trailing* one is the only case where the escape shows a person something
/// that was never content — a shell line written to stderr ends in a newline,
/// and the surface rendered it as a visible `\n` at the end of the record.
///
/// **At most one**, and never a `trim_end`: a backend that deliberately wrote
/// three blank lines wrote them, and only the terminator of the last line is
/// the writing convention rather than the message.
fn without_one_terminator(decoded: &str) -> &str {
  decoded
    .strip_suffix("\r\n")
    .or_else(|| decoded.strip_suffix('\n'))
    .or_else(|| decoded.strip_suffix('\r'))
    .unwrap_or(decoded)
}

/// Escapes `source` for display: `\` and the C0/DEL/C1 control characters
/// (`\n`, `\r`, `\t` by name, the rest as `\u{…}`), everything else verbatim.
/// A `Display` adapter, not a `String` accumulator — §5.4's *shapes* table,
/// rule 8: the two accumulating spellings are each other's denied suggested
/// repair (`clippy::format_push_string` / `clippy::let_underscore_must_use`),
/// and this is the one shape that satisfies both by materialising nothing
/// that can fail to write.
struct Escaped<'a>(&'a str);

impl fmt::Display for Escaped<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for ch in self.0.chars() {
      match ch {
        '\\' => f.write_str("\\\\")?,
        '\n' => f.write_str("\\n")?,
        '\r' => f.write_str("\\r")?,
        '\t' => f.write_str("\\t")?,
        other if other.is_control() => write!(f, "\\u{{{:x}}}", u32::from(other))?,
        other => f.write_char(other)?,
      }
    }
    Ok(())
  }
}

/// Step 4, last: `chars().count()`, and if it exceeds `limit`, keep the
/// first `limit` chars and append the marker naming how many were elided.
/// Counting `char`s rather than bytes makes a split codepoint
/// unrepresentable — the cut is always on a scalar-value boundary.
fn bound(escaped: &str, limit: usize) -> String {
  let total = escaped.chars().count();
  if total <= limit {
    return escaped.to_owned();
  }
  let elided = total.saturating_sub(limit);
  let kept: String = escaped.chars().take(limit).collect();
  format!("{kept} [{elided} more characters not shown]")
}

/// Steps 3 and 4 together: escape, then bound. Every composed line on the
/// in-window surface passes through this — decoding (step 2) happens only for
/// a captured backend's stderr, before `composed` is built, so `composed` here
/// is always already-decoded text (design.md §5.4's pipeline).
fn finish(composed: &str, limit: usize) -> String {
  bound(&Escaped(composed).to_string(), limit)
}

/// A line for the host's own standard error: at most one trailing terminator
/// dropped, then escaped — **not** bounded. The bound is for lengths a backend
/// or a transport chose (D53); these lines carry the platform's text or the
/// person's own configuration's parse error, whose message `toml` writes last,
/// so a bound that keeps a prefix would cut the cause (010 `review-code.md`
/// F-10). The terminator is dropped for the reason it is dropped from a
/// backend's stderr: escaped, it would show a `\n` that was never content.
///
/// **What no bound costs**: a line past journald's `LineMax=` (48K by default)
/// is split into several records, and the last record the journal shows for
/// the run then does not begin `goad: `. The length is the person's own — their
/// configuration, its paths — or the OS's or the platform's; never a backend's.
fn one_line(composed: &str) -> String {
  Escaped(without_one_terminator(composed)).to_string()
}

/// Composed here, with the other user-visible strings, rather than beside
/// the controller — the controller supplies the one bit it owns, `waiting`.
#[must_use]
pub fn tooltip(diagnostics: &Diagnostics, waiting: bool) -> String {
  match diagnostics.lines.split_first() {
    None if waiting => "goad — waiting for an answer".to_owned(),
    None => "goad — nothing to show".to_owned(),
    Some((first, rest)) => {
      let summary = bound(first, TOOLTIP_LIMIT);
      if rest.is_empty() {
        format!("goad — {summary}")
      } else {
        let more = rest.len();
        format!("goad — {summary} (+{more} more)")
      }
    }
  }
}

/// The standing schedule's own line. Not a `Diagnostics` line and not one
/// `Diagnostics` renders: a standing schedule is not an exchange's product,
/// and putting it among the tray's own rows would retire the "Nothing to
/// report." sentinel by side effect. It names what it renders — the resolved
/// next check the host holds and reports, which is the **instruction**,
/// never a prediction of when the host will actually fire (SPEC-002 §6:
/// a host surfacing its next check MUST NOT present the instruction as a
/// prediction) — because the spacing, a refused firing, or a suspend can each
/// move the real deadline later without moving this value (SPEC-002/R-6).
///
/// Second precision, **truncated**: sub-second detail is not meaningful to a
/// person reading a clock, and jiff's default half-expand would render an
/// instant up to half a second later than the one the host holds — a value
/// it never stored, in the one direction that can be read as promising a
/// later check. Falls back to the untruncated instant if the arithmetic
/// errors at the edge of representable time, rather than panicking (VT-1).
#[must_use]
pub fn next_check_line(at: Timestamp) -> String {
  let truncated = at
    .instant()
    .round(
      jiff::TimestampRound::new()
        .smallest(jiff::Unit::Second)
        .mode(jiff::RoundMode::Trunc),
    )
    .unwrap_or(at.instant());
  finish(&format!("next check (instructed): {truncated}"), LINE_LIMIT)
}

/// The back-pressure line. `Wire::send` raises the signal that asks for it
/// and `Glass::present` is the only thing that ever writes it to the window
/// (design.md §5.3). Back-pressure, not a fault: it never enters
/// `Diagnostics` and never touches the tray.
pub const BUSY_NOTICE: &str = "still working on the last request — try again in a moment";

/// One `const`, no trailing newline — [`try_line_to`]'s `writeln!` supplies the
/// one, and two sources of that newline would be a fact stated twice. Its
/// only destination is `--help`; a usage error names the flag instead and
/// does not reprint this block (principle 4).
pub const USAGE: &str = "usage: goad [<config-path>]
       goad -h | --help
       goad --version

With no argument the configuration is read from
$XDG_CONFIG_HOME/goad/config.toml, and from $HOME/.config/goad/config.toml when
XDG_CONFIG_HOME is unset, empty, or not absolute.";

/// stdout. The only caller is `--help`, which is answered only if this
/// arrived: a failed write is the caller's to report, not this outlet's to
/// swallow (010 `review-code.md` F-3).
///
/// # Errors
/// Standard output's own, when the block could not be written to it.
pub fn print_usage() -> std::io::Result<()> {
  try_line_to(std::io::stdout().lock(), USAGE)
}

/// The `--version` answer: the package version, and the revision the build
/// stamped — when it stamped one.
///
/// **No placeholder for the unstamped case.** `cargo install` stamps nothing
/// and a tarball consumer of the flake stamps nothing either; both say
/// `0.1.0` and stop there. A `(revision unknown)` would be this module
/// reporting a fact it does not have, which is the one thing every line on
/// this surface may not do.
///
/// No `"goad: "` prefix, unlike [`report_startup_line`]: the prefix is there
/// because stderr must say who spoke, and a direct answer to a direct
/// question on stdout need not — `goad-emit`'s `--version` is the precedent.
/// The revision is taken as an argument rather than read here, so this stays
/// pure and every branch is a test rather than a build configuration.
///
/// **Set-but-empty is unset**, the rule `build.rs` already states for
/// `SLINT_STYLE`: a flake consumed as a tarball has no revision to stamp and
/// stamps `""`, which must read as unstamped and not as a revision whose name
/// is empty. The test is **here**, inside the one function that decides the
/// line, and not at the call site: a rule spelled at each caller is a rule
/// the unit tier cannot reach and two transcriptions that can disagree
/// (`review-code.md` F-2).
#[must_use]
pub fn version_line(revision: Option<&str>) -> String {
  match revision.filter(|revision| !revision.is_empty()) {
    Some(revision) => format!("{} ({revision})", env!("CARGO_PKG_VERSION")),
    None => env!("CARGO_PKG_VERSION").to_owned(),
  }
}

/// stdout. The only caller is `--version`, and it hands the revision its own
/// build was stamped with; as with [`print_usage`], the question is answered
/// only if the line arrived.
///
/// # Errors
/// Standard output's own, when the line could not be written to it.
pub fn print_version(revision: Option<&str>) -> std::io::Result<()> {
  try_line_to(std::io::stdout().lock(), &version_line(revision))
}

/// The exact string `report_exit` writes for a startup failure, via
/// `report_exit_line`'s `Err` arm — with no destination here, the pure half,
/// so a test can assert it with no sink to fake (F-7). `{error}` is
/// `StartupError`'s `Display`, one rendering, no `source()` walk, and the
/// whole line goes through `one_line`, as every composed line on standard
/// error does. The escape is what keeps it **one** line: a platform error can
/// carry several, and a raw interpolation would leave a last line naming
/// neither the binary nor what happened (010 `review-code.md` F-2).
#[must_use]
pub fn report_startup_line(error: &StartupError) -> String {
  one_line(&format!("goad: {error}"))
}

/// stderr, once, last — or nothing at all, for `Ended::AsAsked`, whose
/// status has nothing to report.
pub fn report_exit(outcome: &Result<Ended, StartupError>) {
  if let Some(line) = report_exit_line(outcome) {
    line_to(std::io::stderr().lock(), &line);
  }
}

/// The line a non-zero exit is accompanied by, and `None` for the status that
/// has nothing to report. The pure half, so every arm is a test with no sink
/// to fake (F-7).
///
/// Each sentence is true of exactly one situation, which is what makes the
/// line worth reading — save where the event-loop call fails on entry, when
/// the *stopped running* sentence says the host had been running and it had
/// not, because the host cannot tell that call from a loop that ran (the
/// seam's cost, SPEC-004 §5 *What the seam costs*). Otherwise: a host
/// that never started writes `report_startup_line`'s, and a host that started
/// and stopped writes one of the two below. Both name the **phase** and not the cause — the call can
/// fail for whatever the platform backend decides, and the host does not know
/// which, so the cause travels in `{error}` where it belongs.
#[must_use]
pub fn report_exit_line(outcome: &Result<Ended, StartupError>) -> Option<String> {
  match outcome {
    Ok(Ended::AsAsked) => None,
    Ok(Ended::StoppedRunning(Some(error))) => Some(one_line(&format!(
      "goad: the host was running and stopped: {error}"
    ))),
    Ok(Ended::StoppedRunning(None)) => {
      Some("goad: the host was running and stopped, and no error was reported".to_owned())
    }
    Err(error) => Some(report_startup_line(error)),
  }
}

/// The exact string `report_platform` writes, with no destination — the pure
/// half, so a test can assert it with no sink to fake (F-7).
///
/// It takes the **rendered** detail rather than a Slint error type, for
/// `Refused::NoClock`'s reason above: the `Display` happens at the one site
/// that has the value, so this module names no Slint error type and stays
/// testable with a literal.
///
/// **"could not be drawn" is the shared half of two failures** — a window that
/// refused to be shown, and a window that was gone before it could be adjusted
/// (`review-code.md` F-B5). Each caller supplies the half that distinguishes
/// them, in `detail`, which is why this line stops where it does.
#[must_use]
pub fn report_platform_line(detail: &str) -> String {
  one_line(&format!("goad: the window could not be drawn: {detail}"))
}

/// stderr, and the process keeps running.
///
/// **Two callers** (`review-code.md` F-B5): `SlintGlass::present`, when
/// `show()` or `hide()` fails after the loop has started, and `install`'s
/// `rescale`, when the one weak
/// handle in the crate fails to upgrade. The second arrived with F-R8's repair
/// and this sentence was not amended with it — recorded here because the commit
/// that added it set out to correct five false doc claims and created a sixth.
pub fn report_platform(detail: &str) {
  line_to(std::io::stderr().lock(), &report_platform_line(detail));
}

/// Physical pixels. `StatusNotifierItem` consumers scale what they are
/// given; 32 is the smallest edge that still reads as a disc after a
/// panel's downscale.
const ICON_EDGE: u32 = 32;
/// Idle: a ring. Slate, readable on a light or a dark panel.
const IDLE: Rgba8Pixel = Rgba8Pixel {
  r: 0x5A,
  g: 0x6B,
  b: 0x7D,
  a: 0xFF,
};
/// Fault: a filled disc. Brick.
const FAULT: Rgba8Pixel = Rgba8Pixel {
  r: 0xC0,
  g: 0x39,
  b: 0x2B,
  a: 0xFF,
};

/// The geometry, pinned, in eighth-of-a-pixel units so a 4x4 sample grid has
/// integral sample centres (design.md §5.4).
const CENTRE: u32 = 128; // 16.0 px, both axes
const OUTER_SQ: u32 = 14_400; // 15.0 px, squared
const INNER_SQ_IDLE: u32 = 5_184; // 9.0 px, squared
const INNER_SQ_FAULT: u32 = 0;
/// Samples per pixel edge (a 4x4 grid), and per pixel in total.
const SAMPLES_PER_EDGE: u32 = 4;
const SAMPLES_PER_PIXEL: u32 = 16;

/// One rasteriser, one parameter — the inner radius, zero for the disc.
/// Idle is an annulus and Fault a filled disc: the two states differ in
/// form as well as hue, so the pair survives a monochrome panel and a
/// colour-blind viewer.
///
/// No icon file, no build-time generator, no image decoder: this is the
/// whole rule.
/// **Rasterised once per thread, and handed out as clones**, because slint
/// compares an `Image` by its *buffer address* and not by its contents:
/// `SharedImageBuffer::eq` is `data.as_ptr()` equality
/// (`i-slint-core-1.17.1/graphics/image.rs:211-223`), reached through
/// `ImageInner`'s `EmbeddedImage` arm (`:643`) and `Image`'s derived
/// `PartialEq` (`:774`). A freshly allocated buffer never shares an address
/// with the one it replaces, so two byte-identical idle icons compared
/// unequal, `SystemTrayIcon`'s `icon_tracker` fired
/// (`items/system_tray.rs:326-341`, and `ChangeTracker` fires only on `!=`,
/// `properties/change_tracker.rs:131-134`) and the platform tray icon was
/// re-set on **every** present — every command, every scheduled poll, and
/// every refused ingress arrival (F-R5).
///
/// Cloning an `Image` shares the buffer, so the address is stable and the
/// tracker now fires exactly when the state changes. The sibling
/// `set_hover_text` needed none of this: a `SharedString` compares by content,
/// which is why `tooltip_tracker` was always well behaved.
///
/// Thread-local rather than a global, because `slint::Image` is neither `Send`
/// nor `Sync`. Each test thread rasterises its own pair; stability within a
/// thread is what the tracker reads.
#[must_use]
pub fn tray_icon(state: TrayState) -> slint::Image {
  thread_local! {
    static ICONS: (slint::Image, slint::Image) =
      (rasterise(TrayState::Idle), rasterise(TrayState::Fault));
  }

  ICONS.with(|icons| match state {
    TrayState::Idle => icons.0.clone(),
    TrayState::Fault => icons.1.clone(),
  })
}

/// The rule itself: idle is an annulus and fault a filled disc, sampled
/// 4x4 per pixel. Called exactly twice per thread, by `tray_icon`.
fn rasterise(state: TrayState) -> slint::Image {
  let (colour, inner_sq) = match state {
    TrayState::Idle => (IDLE, INNER_SQ_IDLE),
    TrayState::Fault => (FAULT, INNER_SQ_FAULT),
  };

  let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(ICON_EDGE, ICON_EDGE);
  let pixels = (0..ICON_EDGE)
    .flat_map(|y| (0..ICON_EDGE).map(move |x| (x, y)))
    .map(|(x, y)| pixel_at(x, y, colour, inner_sq));
  for (slot, pixel) in buffer.make_mut_slice().iter_mut().zip(pixels) {
    *slot = pixel;
  }

  slint::Image::from_rgba8(buffer)
}

/// The pixel at `(x, y)`: `colour` with an alpha proportional to how many of
/// its 4x4 samples are covered by the annulus (or disc, when `inner_sq` is
/// `0`).
fn pixel_at(x: u32, y: u32, colour: Rgba8Pixel, inner_sq: u32) -> Rgba8Pixel {
  let covered = (0..SAMPLES_PER_EDGE)
    .flat_map(|j| (0..SAMPLES_PER_EDGE).map(move |i| (i, j)))
    .filter(|&(i, j)| sample_covered(x, y, i, j, inner_sq))
    .count();
  let covered = u32::try_from(covered).unwrap_or(SAMPLES_PER_PIXEL);

  let alpha =
    u8::try_from(covered.saturating_mul(255).checked_div(16).unwrap_or(0)).unwrap_or(u8::MAX);

  Rgba8Pixel {
    r: colour.r,
    g: colour.g,
    b: colour.b,
    a: alpha,
  }
}

/// Sample `(i, j)` of pixel `(x, y)` sits at `(8x + 2i + 1, 8y + 2j + 1)`,
/// in eighth-of-a-pixel units. Covered iff its squared distance from the
/// centre is at most `OUTER_SQ` and at least `inner_sq` — both comparisons
/// inclusive, so a sample exactly on either boundary is inside.
fn sample_covered(x: u32, y: u32, i: u32, j: u32, inner_sq: u32) -> bool {
  let sample_x = x
    .saturating_mul(8)
    .saturating_add(i.saturating_mul(2))
    .saturating_add(1);
  let sample_y = y
    .saturating_mul(8)
    .saturating_add(j.saturating_mul(2))
    .saturating_add(1);
  let dx = sample_x.abs_diff(CENTRE);
  let dy = sample_y.abs_diff(CENTRE);
  let distance_sq = dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy));
  distance_sq <= OUTER_SQ && distance_sq >= inner_sq
}

// `next_check_line` is tested here, inline, alongside every other pure
// rendering function on this surface (VT-1) — `mod tests`'s cut is also
// what `goad-boundary`'s AC-6 (a) instrument relies on to stay clear of
// this file's own test fixtures.
#[cfg(test)]
mod tests {
  use super::{next_check_line, version_line};
  use goad_semantics::protocol::canonical::Timestamp;

  /// PHASE-03/VT-1, AC-5's rendering half. The **stamped** branch is
  /// unreachable anywhere else under `cargo test`: nothing in the gate sets
  /// `GOAD_REVISION`, so the binary tier can only ever see the bare form.
  /// This case is the real assertion for it, not a proxy for one
  /// (`docs/memory/tests-asserting-proxies.md`).
  #[test]
  fn a_stamped_build_names_its_revision_beside_the_version() {
    assert_eq!(version_line(Some("08528b5")), "0.1.0 (08528b5)");
  }

  /// No placeholder. A build that stamped no revision says only what is
  /// known — `(revision unknown)` would be the host reporting a fact it does
  /// not have (design.md §4, principle 2).
  #[test]
  fn an_unstamped_build_says_only_the_version() {
    assert_eq!(version_line(None), "0.1.0");
  }

  /// **Set-but-empty is unset** (`review-code.md` F-2). `flake.nix` stamps
  /// `self.shortRev or self.dirtyShortRev or ""`, and the third branch is
  /// what a tarball consumer gets: `GOAD_REVISION=""` reaches `option_env!`
  /// as `Some("")`, which must read as unstamped. Without the filter this
  /// prints `0.1.0 ()` — measured, and green everywhere else.
  #[test]
  fn a_build_stamped_with_an_empty_revision_is_an_unstamped_build() {
    assert_eq!(version_line(Some("")), "0.1.0");
  }

  /// Truncated, not rounded to nearest. Half-expand would render
  /// `04:34:14.987Z` as `04:34:15Z` — an instant up to half a second
  /// **later** than the one the host holds, and one it never stored. What is
  /// reported is the instruction the host holds (SPEC-002/R-6), and the line
  /// must not read as a promise about when the check will happen
  /// (SPEC-002 §6); erring later is the one direction that can.
  #[test]
  fn an_ordinary_instant_is_truncated_to_second_precision_never_rounded_up() {
    let at = Timestamp::new("2026-09-07T04:34:14.987654321Z".parse().unwrap());
    assert_eq!(
      next_check_line(at),
      "next check (instructed): 2026-09-07T04:34:14Z"
    );
  }

  #[test]
  fn the_line_names_the_instruction_not_a_deadline() {
    let at = Timestamp::new("2026-09-07T04:34:14Z".parse().unwrap());
    assert!(next_check_line(at).starts_with("next check (instructed): "));
  }

  #[test]
  fn rounding_at_the_edge_of_representable_time_does_not_panic() {
    let at = Timestamp::new(jiff::Timestamp::MAX);
    let line = next_check_line(at);
    assert!(line.starts_with("next check (instructed): "), "{line}");
  }

  /// A bash line written to stderr ends in a newline, and every capture is
  /// escaped so that an embedded newline cannot break the list's shape. The
  /// trailing one is the writing convention rather than the message, and
  /// rendering it as a visible `\n` shows a person something no backend
  /// wrote.
  #[test]
  fn one_trailing_terminator_is_dropped_before_escaping() {
    assert_eq!(
      super::without_one_terminator("answered v1\n"),
      "answered v1"
    );
    assert_eq!(
      super::without_one_terminator("answered v1\r\n"),
      "answered v1"
    );
    assert_eq!(
      super::without_one_terminator("answered v1\r"),
      "answered v1"
    );
  }

  /// **At most one**, and never a `trim_end`. A backend that wrote three
  /// blank lines wrote them: only the last line's terminator is convention,
  /// and the rest are the message.
  #[test]
  fn a_second_terminator_is_message_and_survives() {
    assert_eq!(super::without_one_terminator("a\n\n\n"), "a\n\n");
    assert_eq!(super::without_one_terminator("a"), "a");
    assert_eq!(super::without_one_terminator("\n"), "");
  }

  #[test]
  fn the_line_goes_through_the_escape_and_bound_pipeline() {
    // `jiff::Timestamp`'s `Display` contains no control character and is
    // far short of `LINE_LIMIT`, so this asserts the pipeline ran (the
    // fixed prefix survives verbatim) rather than that it changed anything
    // — `Escaped`/`bound` are exercised for real by `Diagnostics::of`'s and
    // `refused`'s own tests elsewhere on this surface.
    let at = Timestamp::new(jiff::Timestamp::MIN);
    let line = next_check_line(at);
    assert!(line.starts_with("next check (instructed): "), "{line}");
    assert!(line.chars().count() <= super::LINE_LIMIT);
  }
}
