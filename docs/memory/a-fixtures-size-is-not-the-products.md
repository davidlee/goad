# A fixture's size is not the product's, in either direction

Learned at slice 007, PHASE-02 through PHASE-06, and confirmed by direct
measurement at audit.

## The fact

Three fixtures in two files declare a 600×600 or 400×400 viewport so the
element query can reach the controls they assert on. All of them are green. The
product, left to itself, has a preferred size of **50×65 regardless of its
content** — measured at audit for two plain options, for two options with two
fields each, and for one option with five fields: identical every time. At that
size one of two option buttons is reachable; at 600×600 both are.

So the fixtures hide a real defect, and they had to, because without a declared
size the query sees nothing. **The size a test declares is what the test can
see; it is never a claim about what the window should be.**

The second half is the one that costs more: the same declared viewport that lets
a fixture see the form is what hides **how the form looks**. All 535 tests were
green while the layout distributed 1723px of slack between five checkboxes, so
that two members of one block sat further apart than two blocks did. The defect
existed only at a size no test declared, and no test declares a size for the
reason a person would.

## Why

Two mechanisms, neither obvious:

- `ElementQuery` skips any element clipped out of view — a geometric test
  against the nearest clipping ancestor
  (`i-slint-backend-testing/search_api.rs` → `i-slint-core/item_tree.rs`). A
  shown window clips; an unshown one does not, which is why `tree.rs`'s
  exhaustive queries are sound and `wiring.rs`'s are not.
- Content inside a `ScrollView` does not propagate its preferred size upward —
  that is what a scroll view is for — so a window whose markup declares no
  width, height or min-size takes the layout engine's default and never grows.
  This is a property of the layout engine, not of the testing backend: the same
  number reaches a real compositor.

And the reason nobody saw it: **a tiling compositor makes a sizing defect
unobservable.** Under niri the window is tiled to the full column (617×1723
measured), so it never takes a preferred size and never clips. A kiosk
compositor like `cage` fullscreens it for the same effect. Two human acceptance
criteria were named as the observers of this defect and neither could have seen
it.

## How to apply

- When a test declares a window size, say in its doc **why the test needs it**
  and that it claims nothing about the product. Slice 007's three copies each do
  this; keep it.
- **Never conclude a layout is sound from a green test.** Ask what size the
  assertion ran at and whether a person will ever see that size.
- Naming a human criterion as the observer of a defect is only sound if the
  defect's precondition is **reachable in the environment the person will use**.
  Check that before writing the criterion. The general form: *a human run
  answers the question the environment lets it ask.*
- To observe a sizing defect deliberately, measure the preferred size directly
  — show the window, read `window.size()` — rather than hoping a screenshot
  shows it. A floating compositor, or an explicitly sized window, is the other
  route.

Related: `a-green-test-can-assert-a-proxy.md`,
`wayland-window-placement-is-the-compositors.md`, `a-bound-is-not-tested-at-the-bound.md`.
