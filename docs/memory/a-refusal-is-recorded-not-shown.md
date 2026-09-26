# A refusal is recorded, not shown — "reported" does not mean the person saw it

Established from the code in slice 009's audit; slice 008 had found the same
thing from the other direction. Amended at slice 011's close, when `SPEC-003`
R-15 was promoted with *when* a refusal decided while idle updates the surface,
and its verification row moved to the window.

## The fact

The host's failure taxonomy says every refusal is reported. In this renderer,
**reported means recorded**:

- `Diagnostics` is rendered only under `WindowMode.diagnostic`.
- `Controller::surface()` answers `Diagnostics` only when the focus is
  `Focus::Diagnostics`, which **only the Diagnostics menu item sets**.
- `refuse()` writes the diagnostics and never touches the focus.

So for anyone in prompt mode — which is everyone who is answering a form — a
refusal lands in retained state and nothing on screen changes. It is visible
only to someone who goes and opens the pane.

Two consequences that have both bitten:

- **A test asserting a refusal "reaches the diagnostics surface" can be
  reading the retained model, not the window.** Until slice 011, R-15's own
  canon case read `served.controller.frame(false).diagnostics` after the loop
  stopped, so a change to *when* a refusal became visible would not have been
  reported by the instrument canon named for it. R-15's row now reads the
  window, through `RecordingGlass` in `crates/goad/tests/renderer/ingress.rs`,
  which records each update's instant, mode and diagnostic lines after the
  real glass. Other cases elsewhere may still read the model; check which.
- **The diagnostic surface shows the current frame only.** It does not
  accumulate, so there is no history to scroll, and "a new line every N seconds"
  is never the available evidence that something is still happening.

## How to apply

- When a spec says a refusal is *reported*, check what that means in this
  renderer before treating it as an observable. It is retained state.
- When writing a case about visibility, assert against the **window**, and say
  in the case which of the two you are reading.
- When a human run needs to see a refusal, the run has to **open the
  diagnostics pane** — put that in the observation table as a step, not as an
  assumption.
- *When* an idle refusal updates the surface is now `SPEC-003` R-15's: within
  a fixed interval, at most one refusal-caused update per interval, and the
  update shows the latest refusal, so an overwritten one is never shown. What
  the surface *retains* is still open (`docs/follow-ups.md` FU-3). Updating
  the surface is not the same as a person seeing it: in prompt mode the pane
  is still not displayed.

Related: `getting-eyes-on-the-running-host.md`,
`a-present-that-disturbs-nothing-is-invisible.md`,
`a-green-test-can-assert-a-proxy.md`.
