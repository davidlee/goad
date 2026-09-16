# A scroll view is a size barrier in both directions

Learned at slice 008, which met both halves of it in one session — first
repairing the upward direction on the prompt pane, then meeting the downward
direction on the diagnostic pane and not recognising it for what it was.

`a-fixtures-size-is-not-the-products.md` records the *upward* half as a defect
found at slice 007's audit. This note is the pair of mechanisms and the pair of
repairs.

## The fact

A `ScrollView` decouples its content's geometry from its parent's. That is what
it is for, and it cuts **both** ways:

- **Upward** — content inside it propagates no preferred size out. A window
  whose markup declares no width, height or min-size therefore takes the layout
  engine's default and never grows: measured at **50×65 for every content
  shape**, from two plain options to one option with five fields.
- **Downward** — it gives its content an **unconstrained width**. A `Text`
  reports its whole string as its preferred width, so `wrap: word-wrap` has
  nothing to wrap against: the line is laid out at full length and the scroller
  grows a horizontal bar to reach it.

One element, two defects, opposite directions. Neither is visible under a
tiling compositor, which sizes the window itself.

## Why each repair is what it is

**Upward: bind the scroller's `min-height`, never its `height`.** Fixing
`height` leaves the outer layout's slack to be shared among the scroller's
*siblings*. Under a tiler — which hands the window a whole column — that put the
title at the top of the screen, the body a third of the way down and the form at
the foot: **worse than the defect it replaced**, and observable only on a tiled
window, because a floating window at its preferred size has no slack to share.

```
  height: stack.preferred-height        min-height: stack.preferred-height
  ┌──────────────┐  slack goes to       ┌──────────────┐  slack goes to
  │ title        │  the siblings        │ title        │  the scroller
  │              │  ↕                   │ body         │
  │ body         │  ↕                   │ ┌──────────┐ │
  │              │  ↕                   │ │ form     │ │
  │ ┌──────────┐ │                      │ │          │ │  ↕
  │ │ form     │ │                      │ └──────────┘ │  ↕
  └─┴──────────┴─┘                      └──────────────┘
```

Cap the binding (008 used 560px) so a long form cannot ask for a window taller
than the screen.

**Width is stated, not derived.** Every string on a backend-authored surface is
unbounded, and a word-wrapping `Text` reports its whole string as its preferred
width — so a content-derived width is a width one long title decides. 008
declares `preferred-width: 420px` / `min-width: 340px`. Measured: the window
asks for 420px with no option on it, with a 420-character option label, and with
a 450-character checkbox label alike, because nothing propagates a width upward
past the scroller — not even a control's own minimum.

**Downward: pin the inner layout to the scroller's `visible-width`.** That is
what gives the wrap something to wrap against; `horizontal-scrollbar-policy:
always-off` then retires a bar with nothing left to scroll. `wrap: word-wrap`
alone is not the fix and will look like one until the string is long enough.

## How to apply

- Any `ScrollView` in this markup owes **two** bindings, and they are not
  symmetrical: `min-height` out of it (capped), `visible-width` into it.
- When a layout misbehaves only under a tiling compositor, suspect slack
  distribution: ask which children are free to grow, not which one is wrong.
- Both defects are invisible to the test tier *and* to a person on a tiler.
  Measuring a preferred size directly — show the window, read `window.size()` —
  is the instrument; a screenshot under niri is not.

Related: `a-fixtures-size-is-not-the-products.md`,
`wayland-window-placement-is-the-compositors.md`.
