# A refusal is recorded, not shown — "reported" does not mean the person saw it

Established from the code in slice 009's audit; slice 008 had found the same
thing from the other direction.

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

- **A test asserting a refusal "reaches the diagnostics surface" is usually
  reading the retained model, not the window.** One canon verification case
  reads `served.controller.frame(false).diagnostics` — so a change to *when* a
  refusal becomes visible would not be reported by the instrument canon names
  for it.
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
- Deciding when an idle refusal must become *visible* is a spec amendment with
  its own verification, not a repair. It outlives any one slice.

Related: `getting-eyes-on-the-running-host.md`,
`a-present-that-disturbs-nothing-is-invisible.md`,
`a-green-test-can-assert-a-proxy.md`.
