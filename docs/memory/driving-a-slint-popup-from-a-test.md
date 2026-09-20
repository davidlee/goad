# Driving a Slint popup from a test: the query reaches in, the pointer does not

Measured in slice 009, PHASE-07, end to end against `DatePickerPopup` and
`TimePickerPopup`.

## The query reaches inside, at every tier

`ElementQuery::find_first` / `find_all` pass `active_popups()` into the walk
(`i-slint-backend-testing-1.17.1/search_api.rs:291-312`), so a shown
`PopupWindow`'s subtree **is** in scope from the window root — including under
`init_no_event_loop`, with no event loop at all.

So a popup's contents are reachable and assertable. What is not reachable is a
click.

## A pointer event cannot be landed inside a popup, at any tier

`ElementHandle::mock_single_click` dispatches at `absolute_center()`.
`absolute_position` is `item.map_to_window(..)`, which for an item inside a
`PopupWindow` stops at the **popup's own root**, because a popup is a separate
item tree with no parent link
(`i-slint-core-1.17.1/item_tree.rs:628-630`). Dispatch then translates by the
popup's origin in the window (`window.rs:848-856`), so a popup-local coordinate
is read as a window one and the click lands elsewhere.

Laying the popup out does not help, and neither does an event loop: the popup
*does* lay out under `init_no_event_loop` after one `mock_elapsed_time`, and the
click still misses.

**What works instead:**

- **`invoke_accessible_default_action`.** Everything a picker needs — a calendar
  day cell, a clock-face selector, a `StandardButton` — declares
  `accessible-role: button` and an `accessible-action-default` that calls its
  own `clicked`. No pointer event is dispatched and nothing depends on layout.
- **Drive the widget's *other* caller.** For a `ComboBox`, click the box itself
  (an ordinary window element) and then arrow: `move-selection-down()` and a
  row's `clicked` are one `select(index)`.

## Two more mechanics that cost time

- **`mock_single_click` cannot be called from inside a timer callback.** It
  advances mock time, which reaches `TimerList::maybe_activate_timers`, which
  asserts *"Recursion in timer code"* (`i-slint-core-1.17.1/timers.rs:262`). In
  a loop-tier stepper, write the three window events out by hand.
- **A keyboard driver that raises one edit per press meets the capacity-1
  channel.** Two arrow presses inside one synchronous helper give the serve loop
  no chance to drain, so the second edit is dropped and the draft keeps the
  first — the guard then corrects the widget, which is right, and is not what a
  case driving *one* choice meant to say. Drive one step and wait for the draft.

## The lesson under all of it

**Absence of a case was twice mistaken for absence of a capability.** The design
doubted a popup could be reached from a test at all; what it had actually
doubted was `mock_single_click`'s dependence on `absolute_center()`. Before
writing off a tier, work out which specific mechanism is missing.

Related: `slint-testing-backend-initialises-once-per-process.md`,
`a-popup-is-rebuilt-on-every-show.md`, `change-handlers-need-an-event-loop.md`,
`a-reading-slint-derives-is-not-a-reading-of-your-value.md`.
