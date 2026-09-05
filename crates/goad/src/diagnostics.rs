//! Everything a person reads, in one module (design.md §5.2, principle 4).
//!
//! This phase (PHASE-04) lands only the tray rasteriser: `TrayState`,
//! `ICON_EDGE`, `IDLE`, `FAULT` and `tray_icon`. `Reported`, `Refused`,
//! `Diagnostics`, `tooltip` and `BUSY_NOTICE` are PHASE-05's — the module
//! carries the arithmetic deny now because the rasteriser needs it
//! (design.md §5.4, DF-1: the rasteriser's home is here, not a separate
//! `tray_icon.rs`).
#![deny(clippy::arithmetic_side_effects)]

use slint::{Rgba8Pixel, SharedPixelBuffer};

/// The tray's severity, derived from the diagnostic surface rather than
/// stored — PHASE-05's `Diagnostics::state()` reads it on every `present`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayState {
  Idle,
  Fault,
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
