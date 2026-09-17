# A `PopupWindow` is rebuilt on every `show()`, and cannot be written into

Measured by the spike at commit `4f93d41` (`spike-fields/tests/picker_seed.rs`)
after a design review raised a blocker that rests on the opposite assumption.

## The fact

`show-popup` compiles to a **fresh construction** every time:

```rust
let popup_instance = #popup_window_id::new(…).unwrap();
…
if let Some(current_id) = …popup_id.take() { window.close_popup(current_id); }
let popup_id = window.show_popup(…);
```

(`i-slint-compiler/generator/rust.rs:3736-3763`.) The closed instance is popped
from `active_popups` and dropped (`i-slint-core/window.rs:1955-1990`), and
`active_popups` holds the only `ItemTreeRc`. **No state inside a popup survives
a close.** Every `in` property is re-bound from scratch on the next show.

So a single root `DatePickerPopup` shared by many fields is a shared
*declaration*, not a shared *instance*. There is no cross-field leakage to
defend against: two fields seeded with the same date each open on it, and the
field whose pick would have leaked reopens on its seed too. Measured, with a
differing seed for the second field as the positive control.

## The second fact, which is a compile error

A `PopupWindow`'s properties **cannot be assigned from an enclosing
component's handler**:

```
Cannot access property or callback 'picker.date' inside of a Window
from enclosing component
```

`PopupWindow` inherits `Window`, and that boundary is one-way for property
access. What works instead is a binding at the popup's own declaration site,
reading a property the enclosing component owns:

```slint
in-out property <Date> seed-date;
function open-for(slot: int) {
    root.seed-date = root.seeds[slot].date;   // root's own property: legal
    picker.show();
}
picker := DatePickerPopup { date: root.seed-date; }
```

`show()` **is** callable from outside, which is how a spike that only ever
called `show()` missed this entirely.

Because the popup is rebuilt per show, that binding is evaluated fresh on every
open — so a seed written immediately before `show()` is picked up without any
`changed` handler being involved, and therefore without needing a real event
loop for that reason (`change-handlers-need-an-event-loop.md` still governs
anything that genuinely is a `changed` handler).

## Driving one in a test

The date-picker chain needs **no pointer event and no layout**:

| control | found by | driven by |
|---|---|---|
| a calendar day | `accessible-label` = the day number | `invoke_accessible_default_action` |
| the dialog's OK | `accessible-label` = `OK` | `invoke_accessible_default_action` |

(`widgets/common/datepicker_base.slint:59-63`,
`widgets/common/standardbutton.slint:17-31`.) `ElementQuery::from_root` walks
`active_popups`, so a query from the window root reaches inside.

A `ComboBox`'s `ListItem` is the exception worth knowing: it carries
`accessible-role: list-item`, a label, an index and a selected flag, but **no**
`accessible-action-default` (`widgets/fluent/components.slint:15-19`). That one
needs `mock_single_click`, which dispatches at the element's absolute centre and
so does depend on the popup being laid out.

## How to apply

- Reading a widget's source tells you what an **instance** does. It never tells
  you how long the instance lives. Check the lifetime before building an
  argument on retained state — that is the failure this note exists for, and it
  is the mirror of inferring a missing capability from no local case using one.
- Do not design a handler that writes into a popup. Own the value outside and
  let the popup bind to it.
- Before assigning a popup case to a test tier, check whether its driver has an
  accessible default action. If it does, the no-loop tier reaches it.

Related: `change-handlers-need-an-event-loop.md`,
`slint-form-widget-shapes.md`, `a-green-test-can-assert-a-proxy.md`.
