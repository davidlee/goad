# A reading Slint derives from your value is not a reading of your value

Measured in slice 009, PHASE-08, against `fluent/slider.slint`.

## The fact

Slint's widget library computes some accessibility properties rather than
forwarding them, so reading one back does **not** tell you what the markup
holds.

**`accessible-value-step` is a cap, not the step.** `fluent/slider.slint:29`
binds it to:

```slint
min(root.step, (maximum - minimum) / 100)
```

So any design whose step *is* that hundredth gets the hundredth back whatever
the markup says — and a slider shipping a step of a **tenth** of the span still
reports `0.1` while moving the value by `1`. Measured.

What measures the shipped step is
**`invoke_accessible_increment_action`**, because `increment()` is exactly
`set-value(value + step)`. Drive the action and read the value, rather than
reading the property.

**A `Slider` cannot report a non-finite.** `set_accessible_value("NaN")` leaves
the widget on its **maximum**: `set-value` clamps with
`max(minimum, min(maximum, v))` before raising `changed`, and `"inf"` clamps
identically. So a host-side non-finite refusal guards against a renderer bug and
not against this renderer — worth knowing before writing a case that tries to
reach it.

**A Slint `Button` declares `accessible-checked`.** It has a `checkable`
property and binds the attribute whether or not anything set it, so *"declares
no checked state"* is a false discriminant that happens to pass against a
`LineEdit` and says nothing about a `CheckBox`. What tells the drawn controls
apart in the element tree is the **role**: `button`, `checkbox`, `text-input`.

**`set_accessible_value` on a `LineEdit` reaches no text-input logic.**
`fluent/lineedit.slint:16` implements it as an assignment plus a call to
`edited` — so it never touches `TextInput::key_event`, never moves a caret, and
never passes the `enabled` gate. A suite built entirely on it cannot observe a
caret and cannot observe a disabled control refusing input. Dispatch real
`WindowEvent::KeyPressed`/`KeyReleased` when either of those is the subject.

## Why

The accessibility surface is a convenience for driving widgets without layout,
and it is the only surface available to a test tier with no event loop. That
makes it the default, and its derived-ness is invisible from the call site: you
set a value, you read a value, and the round trip looks like a measurement.

## How to apply

- Before asserting on any `accessible-*` property, **read the widget's own
  `.slint` source** for how it is bound. A `min`, a `clamp` or a literal there
  means the property is not your value.
- Prefer driving an **action** (`invoke_accessible_increment_action`,
  `invoke_accessible_default_action`) and reading the *model* over reading an
  accessibility property back.
- Where the subject is input handling itself — carets, focus, `enabled` —
  nothing but a real window event will do.

Related: `slint-form-widget-shapes.md`, `slint-styling-facts.md`,
`a-green-test-can-assert-a-proxy.md`.
