# A Wayland client cannot place, raise, or focus its own window

Learned at slice 002, PHASE-08/09 (`design.md` §5.4, `crates/goad/README.md`).

## The fact

On Wayland, placement, raising, and focus are the compositor's alone; a
client asking for any of the three is a no-op, verified rather than assumed.
The only thing an application can guarantee is a stable xdg app id. `goad`'s
is `goad`, and it does not change.

## Why

A prompt reaching the user therefore depends on a compositor-side rule, not
application code — `crates/goad/README.md` carries the literal niri
`window-rule` block (`match app-id="^goad$"`, `open-floating true`,
`open-focused true`), validated against niri 26.04. The README is explicit
that this is niri's syntax and another compositor matches the same app id
its own way — the app id is the only cross-compositor contract.

## How to apply

- Do not write code that tries to force a goad window to the front; it
  cannot work on Wayland and any test asserting it would be testing a no-op.
- If the app id ever changes, the README's window-rule block and every
  compositor config a user has written against it goes stale — treat the id
  as a stable public contract, not an implementation detail.
- A future renderer or packaging change documents its own compositor's rule
  the same way rather than inventing host-side placement logic.
