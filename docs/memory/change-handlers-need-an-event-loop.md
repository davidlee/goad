# `changed` handlers fire nowhere under `init_no_event_loop`

Measured by the spike at commit `a698217`.

## The fact

A Slint `changed` callback does not run under
`i_slint_backend_testing::init_no_event_loop()` — **not inside a repeater and
not on a root property**. The same markup, under
`init_integration_test_with_system_time()` and a real
`run_event_loop_until_quit()`, fires exactly as expected.

Change trackers are run by `ChangeTracker::run_change_handlers_once()`, called
from `WindowInner::ensure_tree_instantiated` (`i-slint-core/window.rs:805`),
which the no-event-loop backend never reaches. An `ElementQuery` walk is not a
substitute: it materialises the repeater but does not pump the trackers.

## A `changed` handler fires on a *change*, not on a write

Measured again at `4f93d41`, from the source this time. A change tracker stores
the last value it evaluated and, at flush, re-evaluates and calls the handler
**only if the new value differs**
(`i-slint-core/properties/change_tracker.rs:138-141`). Two consequences:

- Writing a property the value it already holds fires nothing. Any argument of
  the form *"the host writes it, so the widget follows"* is wrong unless the
  written value differs from what is there.
- Writes are **coalesced**: only the final value of a flush is compared. So
  perturbing a property and then setting the real value *inside one handler*
  fires nothing either — the usual first instinct for forcing a re-sync, and it
  does not work. Two separate loop turns do.

A monotone counter — an epoch, a revision — is the reliable trigger, because
every bump is a real change by construction.

## Why it matters more than it looks

**The whole of `crates/goad/tests/renderer/` uses `init_no_event_loop`.** So a
case written there asserting anything a `changed` handler does would be green
while measuring nothing at all — a vacuous pass in the exact shape this project
keeps being bitten by. The failure is silent: no error, no warning, just a
counter that stays at zero.

The observable tier is the `crates/goad/tests/event_loop_schedule/` shape: a
real loop, driven by a `slint::Timer`, quitting itself. That tier is **one
`#[test]` fn per binary**, because `init_integration_test_*` can only be called
once per process — so every case needing a change handler either shares one
binary's single test or gets a binary of its own.

## How to apply

- Before writing a renderer case, ask whether the behaviour needs a change
  handler, a timer or a real frame. If it does, it does not belong in the
  `tests/renderer/` tier and will lie there.
- Negative-control any case that touches this. A control is what tells a real
  green from a vacuous one, and here the vacuous one is the default
  (`a-negative-control-that-does-not-compile.md`).
- The same class of trap as the debug-info one `crates/goad/build.rs` documents:
  without `with_debug_info`, `ElementQuery` returns empty and every renderer
  test passes vacuously. Two different switches, one failure mode.

Related: `a-present-destroys-the-widget-it-writes.md`,
`a-green-test-can-assert-a-proxy.md`,
`slint-testing-backend-initialises-once-per-process.md`.
