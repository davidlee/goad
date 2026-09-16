# Window zoom is the compositor's scale factor, not a property in the markup

Learned at slice 008, L-9. Verified against the pinned Slint 1.17.1 sources.

## The fact

To magnify everything a window draws, dispatch a scale-factor change at it:

```rust
window.dispatch_event(WindowEvent::ScaleFactorChanged { scale_factor });
```

Public API — `i-slint-core/api.rs:648`, `platform.rs:395`. It scales
**everything the window draws**, including the widget library's own padding,
borders and glyph metrics. A `zoom` property multiplying lengths in the markup
would have to touch every literal in the file and would still miss all of that.

**Never store the compositor's base scale.** Divide the zoom already applied
back out of what the window currently reports instead. The compositor dispatches
its own scale change whenever the real scale moves — the window crossing to
another output, `winitwindowadapter.rs:1519` — so a base captured at startup goes
stale and then fights it; reading it back picks the new base up.

**It survives a present**, confirmed on screen. This was the question that
decided whether the affordance was worth building: the window is hidden between
prompts and re-shown every couple of hours, and had the winit adapter re-read the
real scale on show, the zoom would reset at every prompt. It does not.

**No test asserts that**, deliberately: the testing backend has no winit adapter,
so a green case there asserts the core's bookkeeping and nothing about the thing
that would break it. A person watching the window is the honest instrument.

## Why the keyboard is not the cheap input for it

Costed before choosing the tray, and none of the three is a rider on a zoom
lever:

- **`Ctrl +/-`** needs a root `FocusScope`. Key events do bubble from the focused
  item up to the window (`window.rs:1336`), so a focused `CheckBox` does not
  block it — but if **nothing** is focused the bubble list is empty and no key
  reaches anything. *Keyboard focus does not survive a present* (slice 007
  follow-up) makes that a prerequisite, not an aside: the shortcut would work
  until the next present and then silently stop.
- **`Ctrl`+wheel** must be taken off the `ScrollView`, whose `Flickable` accepts
  every wheel event with no modifier check at all (`flickable.rs:251`,
  `accepts_pan_event`). Recovering it means a covering `TouchArea` that accepts
  modifier-carrying scrolls and rejects the rest, with its own consequences for
  the controls underneath.
- **`MenuItem.shortcut`** is matched *before* the event reaches the focused item
  (`window.rs:1299`) — exactly the property that would sidestep the focus problem
  — but it is honoured only inside a `MenuBar`, and the host's menu is a
  `SystemTrayIcon`'s.

The tray needed none of that and cost about twenty lines.

## How to apply

- Zoom is a rendering concern. It never reaches `Command` and never reaches the
  controller: no model state changes, nothing orders against an evaluation, and
  the backend is not told. Wire the tray callbacks straight at the window.
- Before spending on an input path, prove the lever from the cheapest affordance
  available.

Related: `slint-build-mechanics.md`, `getting-eyes-on-the-running-host.md`.
