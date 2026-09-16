//! `crates/goad/src/zoom.rs` — stratum 3.
//!
//! The host's own magnification, and nothing else: no window, no event, no
//! clock. What is here is arithmetic, so it is tested without a compositor;
//! `install.rs` is what turns a `Zoom` into a scale factor the window is
//! given.
//!
//! **This is presentation, not a command.** It never reaches `Command` and
//! never reaches the controller: no model state changes, nothing needs
//! ordering against an evaluation, and the backend is not told. Sending it
//! down the wire would put a rendering concern in the controller and buy
//! nothing.

/// A multiplier over the scale the compositor hands the window.
///
/// `NONE` is not "unscaled" — it is *whatever the compositor said*, which on
/// a high-DPI output is already 1.2 or 2. The host magnifies on top of that
/// rather than replacing it, so a window at `NONE` looks exactly like a build
/// without this file.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zoom(f32);

impl Zoom {
  /// No magnification: the compositor's own scale, untouched.
  pub const NONE: Self = Self(1.0);

  /// One press. Multiplicative rather than additive, so a step is the same
  /// proportion at every size; an additive step is coarse at the bottom of
  /// the range and imperceptible at the top.
  const STEP: f32 = 1.1;

  /// The range. Under the floor the controls stop being hittable; over the
  /// ceiling one option fills the screen.
  const FLOOR: f32 = 0.5;
  const CEILING: f32 = 3.0;

  pub fn larger(self) -> Self {
    Self((self.0 * Self::STEP).min(Self::CEILING))
  }

  pub fn smaller(self) -> Self {
    Self((self.0 / Self::STEP).max(Self::FLOOR))
  }

  /// The scale factor to hand a window whose compositor scale is `base`.
  pub fn applied_to(self, base: f32) -> f32 {
    base * self.0
  }

  /// The compositor's own scale, recovered from a window *currently* showing
  /// this zoom.
  ///
  /// Read back rather than remembered. The compositor dispatches its own
  /// scale change whenever the real scale moves — the window crossing to
  /// another output — and a base captured once would then be stale and would
  /// fight it. Dividing the live figure by the zoom already applied picks the
  /// new base up instead of arguing with it.
  pub fn base_of(self, shown: f32) -> f32 {
    shown / self.0
  }
}

// Pure arithmetic, tested inline in the `#[cfg(test)] mod tests` shape
// `controller.rs` and `draft.rs` use for a stratum-internal pure function.
#[cfg(test)]
mod tests {
  use super::*;

  /// Within a tenth of a scale step: close enough that no reader could see it,
  /// loose enough that `f32` division does not decide the test.
  fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.001
  }

  #[test]
  fn no_zoom_hands_the_window_back_the_compositors_own_scale() {
    assert!(near(Zoom::NONE.applied_to(1.2), 1.2));
    assert!(near(Zoom::NONE.applied_to(2.0), 2.0));
  }

  #[test]
  fn a_step_is_the_same_proportion_wherever_it_is_taken() {
    let one = Zoom::NONE.larger().applied_to(1.0) / Zoom::NONE.applied_to(1.0);
    let far = Zoom::NONE.larger().larger().larger().larger();
    let five = far.larger().applied_to(1.0) / far.applied_to(1.0);
    assert!(near(one, five), "{one} vs {five}");
  }

  #[test]
  fn out_undoes_in() {
    assert!(near(Zoom::NONE.larger().smaller().applied_to(1.2), 1.2));
  }

  #[test]
  fn the_range_holds_however_long_a_key_is_held() {
    let up = (0..100).fold(Zoom::NONE, |z, _| z.larger());
    let down = (0..100).fold(Zoom::NONE, |z, _| z.smaller());
    assert!(near(up.applied_to(1.0), 3.0), "{up:?}");
    assert!(near(down.applied_to(1.0), 0.5), "{down:?}");
  }

  /// The property `install.rs` leans on: the base is never stored, it is
  /// divided back out of what the window currently reports.
  #[test]
  fn the_compositors_scale_is_recoverable_from_a_zoomed_window() {
    let zoom = Zoom::NONE.larger().larger();
    assert!(near(zoom.base_of(zoom.applied_to(1.2)), 1.2));
  }

  /// And it is recovered from the *live* figure, so a compositor that moved
  /// the scale under us is picked up rather than fought.
  #[test]
  fn a_scale_the_compositor_changed_is_read_back_not_overwritten() {
    let zoom = Zoom::NONE.larger();
    // The window was at base 1.2 when zoomed; the compositor has since moved
    // it to 2.0, which arrives multiplied by the zoom already applied.
    assert!(near(zoom.base_of(zoom.applied_to(2.0)), 2.0));
  }
}
