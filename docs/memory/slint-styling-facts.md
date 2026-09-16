# Slint styling facts found by rendering them

Learned at slice 008's visual pass, each by rendering the alternatives and
sampling the result rather than by reading the documentation.

## Mute a tone with `transparentize()`, never `darker()`

`Palette.foreground.transparentize(0.35)` blends towards whatever is behind it,
so **one declaration** mutes a near-white foreground on a dark ground *and* a
near-black one on a light ground. `darker()` only ever goes one way and is wrong
in half the themes.

## The built-in styles are not interchangeable, and the difference may be unreachable

`build.rs`'s `with_style()` sets the default; `SLINT_STYLE` overrides it.

- **Material's checkbox label colour is not reachable from outside.**
  `material/checkbox.slint` binds `i-text.color` to
  `MaterialPalette.control-foreground`, and the component exposes only `text`,
  `enabled`, `font-size`, `font-weight`, `has-focus` and `checked`. Accent-blue
  labels are a property of the style, and the only way off them is another
  style. 008 chose **fluent**; cosmic was the runner-up; cupertino is
  grey-on-grey on a dark ground.
- **Fluent's `GroupBox` draws no border** — a bold title and some padding
  (`fluent/groupbox.slint`). If a bordered container is wanted, draw it.

## `accessible-role` is part of a widget's contract with the test tier

Fluent's `GroupBox` binds `accessible-role: groupbox`. This codebase's
`tests/renderer/tree.rs` scopes `labels_in_option` by that role and asserts the
count of groupboxes per option, so importing a `GroupBox` for its *look* would
have silently changed what several cases mean. Drawing the panel by hand kept
both intact.

The same rule made two other structural decisions in 008's diagnostic pane:
`Nothing to report.` stays **outside** the diagnostics list, because inside it is
one more `Text` in a scope whose `accessible-item-count` states how many
diagnostics there are; and the list stays **unconditional**, because
`wiring.rs::in_diagnostic_mode` finds the pane by the element labelled
`diagnostics`, so guarding it on the line count would make an empty report look
like the prompt pane to every test that asks.

## Do not uppercase a backend-authored string

A small-caps section marker normally wants uppercasing at that scale. It was
deliberately not taken: uppercasing is the host rewriting a string it did not
author — it mangles acronyms and does nothing in a non-Latin script. Raise the
tone instead.

## An embedded face costs what it weighs

`import "../../../assets/Inter.ttf"` plus `default-font-family`; the variable
font's weight axis works. It changes nothing on a machine whose `fc-match
sans-serif` already answers Inter — what it buys is the same text elsewhere, with
no fontconfig lookup for the family. 856K in the binary.

Related: `slint-build-mechanics.md`, `slint-markdown-subset.md`,
`fontconfig-needs-makefontsconf-not-just-buildinputs.md`.
