//! Everything a person reads, in one module (design.md §5.2, principle 4).
//!
//! PHASE-04 landed the tray rasteriser: `TrayState`, `ICON_EDGE`, `IDLE`,
//! `FAULT` and `tray_icon`. PHASE-05 adds the rest: `Reported`, `Refused`,
//! `Diagnostics`, `tooltip`, `BUSY_NOTICE`, the two remaining outlets
//! (`line_to`, `report_platform`) and the escape/bound pipeline every line on
//! this surface goes through. The module carries the arithmetic deny because
//! both halves compute over lengths a backend or a transport chose (D53,
//! design.md §5.4).
#![deny(clippy::arithmetic_side_effects)]

use std::fmt::{self, Write as _};

use goad_semantics::protocol::normalize::Discarded;
use goad_shell::backend::transport::Captured;
use goad_shell::error::CleanupFailure;
use goad_shell::host::Failure;
use slint::{Rgba8Pixel, SharedPixelBuffer};

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
  SupersededView { named: String },
  /// The click named an option the retained presentation does not carry. A
  /// renderer bug rather than an answer: reported, never sent.
  UnknownOption { named: String },
  /// The wall clock could not be read, so no request can be stamped.
  NoClock { detail: String },
}

/// Every line a person can read on this surface, and whether any of them is
/// a fault. `lines` and `fault` are private: the only way to build one is
/// `of` or `refused`, so neither can be assembled partially.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diagnostics {
  lines: Vec<String>,
  fault: bool,
}

/// The stderr line's own bound: the only line whose whole value is
/// diagnostic prose a person reads (design.md §5.4).
const STDERR_LIMIT: usize = 4096;
/// Every other line's bound: enough for a serde message quoting a document,
/// an OS error, or a discarded `raw`.
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
      Refused::SupersededView { .. } => {
        "no action taken: that answer belongs to a question that has since been replaced".to_owned()
      }
      Refused::UnknownOption { .. } => {
        "no action taken: the host could not match that control to the question it is holding"
          .to_owned()
      }
      Refused::NoClock { detail } => format!(
        "no action taken: the system clock could not be read, so no request could be stamped ({detail})"
      ),
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
    Undrawn::OptionFields { option, count } => {
      let id = option.as_str();
      let plural = if *count == 1 { "" } else { "s" };
      format!(
        "not drawn: option {id} carries {count} field{plural}; this renderer draws options and their labels only"
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

/// Steps 3 and 4 together: escape, then bound. Every line on this surface
/// passes through this — decoding (step 2) happens only for stderr, before
/// `composed` is built, so `composed` here is always already-decoded text
/// (design.md §5.4's pipeline).
fn finish(composed: &str, limit: usize) -> String {
  bound(&Escaped(composed).to_string(), limit)
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

/// The transient back-pressure line. `Wire::send` is its only writer
/// (design.md §5.3). Back-pressure, not a fault: it never enters
/// `Diagnostics` and never touches the tray.
pub const BUSY_NOTICE: &str = "still working on the last request — try again in a moment";

/// Best effort: if the handle cannot be written there is nowhere left to
/// report that, and the exit code still carries the fact. `.ok()` and
/// `drop(..)` both pass the lint table too; this spelling is chosen because
/// it is the only one of the three that says *both outcomes were considered*
/// rather than merely *discarded* (design.md §5.4).
fn line_to(mut sink: impl std::io::Write, line: &str) {
  match writeln!(sink, "{line}") {
    Ok(()) | Err(_) => (),
  }
}

/// stderr, and the process keeps running. The only caller is
/// `SlintGlass::present`, when `show()` or `hide()` fails after the loop has
/// started.
///
/// It takes the **rendered** detail rather than a Slint error type, for
/// `Refused::NoClock`'s reason above: the `Display` happens at the one site
/// that has the value, so this module names no Slint error type and stays
/// testable with a literal.
pub fn report_platform(detail: &str) {
  line_to(
    std::io::stderr().lock(),
    &format!("goad: the window could not be drawn: {detail}"),
  );
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
#[must_use]
pub fn tray_icon(state: TrayState) -> slint::Image {
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
