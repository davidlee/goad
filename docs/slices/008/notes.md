# Slice 008 — the renderer gets a look

**Mode:** interactive pairing session. The `docs/AGENTS.md` lifecycle is
suspended for this slice by user instruction (2026-09-16). No design.md, no
plan.md, no review ledgers. This file is the record: what is on the list, what
we did, what we decided.

**Reference material:** `~/.local/src/slint/examples/{layouts,gallery}`.

## The list

<!-- [ ] todo · [~] in progress · [x] done · [!] blocked · [-] dropped -->

Inherited from 007 (`slice-007.md` §Follow-ups, `notes.md` PHASE-06) and
`docs/roadmap.md` §008.

- [x] L-1 — **No content-derived preferred size.** Window is 50×65 for every
      content shape; 1 of 2 option buttons reachable. `ScrollView`
      (`app.slint:49`) does not propagate its content's preferred size and
      `PromptWindow` declares no width/height/min-*. The repair is *propagate a
      size*, not *make it taller*.
- [x] L-2 — Layouts **above** the block container distribute the window's full
      height; title and body are clipped at the top.
- [ ] L-3 — Diagnostic surface does not wrap a line.
- [ ] L-4 — Diagnostic surface text cannot be selected or copied. (This is why
      007's human evidence had to be a screenshot.)
- [x] L-5 — A captured stderr line's trailing `\n` renders as a visible `\n`.
      Host-side: trim at most one line terminator before escaping,
      `diagnostics.rs`.
- [ ] L-6 — **The idle surface.** After an answer nothing is on screen and
      nothing on the console; only the tray says the host is alive, and the next
      check it carries is pointed at by nothing.
- [ ] L-7 — Backend-authored strings reaching the screen are unbounded —
      `title`, `option.label`, `block.heading` — while every diagnostic line
      passes `finish(.., 1024)`. (007 `review-code.md` F-16.)
- [~] L-8 — **The visual pass.** Layout, spacing, typography, in `app.slint`.
      Material was spiked and rejected ("same shit with blue and rounded
      corners") — the ugliness is the layout, not the widget library.
- [~] L-9 — **Magnification.** Not inherited; asked for this session. A
      proof-of-concept is landed on the tray menu. `Ctrl +/-` and `Ctrl`+wheel
      are costed but not built — see the log entry below for why neither is a
      rider on this one.

Adjacent, not on the list unless we pull them in:

- `Diagnostics::of` builds unbounded **lists** (007 F-4).
- Keyboard focus does not survive a present (007 follow-up; its own surface).

## Log

<!-- Append-only. What we did, what we decided, and why. -->

### 2026-09-16 — opened

Methodology suspended by user instruction; scoping interview abandoned after one
question (answer: "everything listed"). Working the list above directly.

### 2026-09-16 — style, structure, sizing

**Decisions taken (user, on screenshots):**

- **D-1 — the style is `fluent`, not `material`.** All four built-ins were
  rendered from the same markup and compared. Material's check labels are
  accent-blue and the colour is **not reachable**: `material/checkbox.slint`
  binds `i-text.color` to `MaterialPalette.control-foreground` and the component
  exposes only `text`, `enabled`, `font-size`, `font-weight`, `has-focus` and
  `checked`. The only way off the colour is the style. Cupertino is grey-on-grey
  on a dark ground; cosmic was the runner-up. `build.rs`'s default changed; the
  `SLINT_STYLE` override is untouched and now reverts to material.
- **D-2 — an option's control sits *after* the fields it carries**, not before.
  Fill, then commit. 007 put it above with the fields indented under it, on the
  reading that the control that answers is flush left and everything it carries
  steps in under it. That indent hung from a parent; with the control below, an
  indent alone hangs from nothing — so **a 2px left rule** now binds the option's
  fields and its control into one block, and is absent for an option with no
  fields.

**Constraints the restructure had to hold, and did** (`tests/renderer/tree.rs`):

- `labels_in_option` scopes by the **groupbox role** so that everything in scope
  was drawn by the field markup. The control therefore stays *outside* the field
  container — inside, its own internal `Text` would enter that scope.
- One case asserts exactly **one** Groupbox exists when one of two options has
  fields, so the container stays guarded on `option.blocks.length > 0`.
- All 185 renderer cases pass unchanged, and the whole workspace is green.

**L-1 — the repair, and the measurement.** `ScrollView` propagates no preferred
size from its content; 007's audit measured the window at **50×65 for every
content shape**. Binding the scroller's `min-height` to `stack.preferred-height`
restores the propagation, capped at 560px so a long form cannot ask for a window
taller than the screen.

| fixture | 007 audit | now |
|---|---|---|
| two plain options | 50×65 | 420×159 |
| two options, two fields each | 50×65 | 420×289 |
| one option, five fields | 50×65 | 420×236 |
| one option, forty fields | — | 420×639 (the cap holding) |

The width is **stated, not derived**: `preferred-width: 420px`, `min-width:
340px`. Every string on this surface is backend-authored and unbounded, and a
word-wrapping `Text` reports its whole string as its preferred width, so a
content-derived width is a width one long title decides.

**`min-height`, not `height` — and this is the one that mattered in practice.**
The first cut fixed the scroller's `height`, which left the window's slack to be
shared out among the *other* children of the outer layout. Under a tiling
compositor, which hands this window a whole column, that put the title at the
top of the screen, the body a third of the way down and the form at the foot of
it — **worse than what it replaced**. Observed by the user on a tiled window
("without me resizing it — not so good"), which is the only configuration that
shows it: a floating window at its own preferred size has no slack to share.

**Also repaired in passing:** a `CheckBox` given the whole block width draws its
focus ring across the whole row. Each field is wrapped in a `HorizontalLayout {
alignment: start }`.

**Open — the void below the form.** Under a tiling compositor the content packs
to the top of a full-height column and the rest is empty. That is now stable and
correct rather than scattered, but it is not *designed*. Candidates: leave it,
centre the block vertically, or let the idle surface (L-6) occupy it.

### 2026-09-16 — the visual pass

Worked live against `just demo` under niri, screenshot per change. The loop is
in the scratchpad (`run.sh` restarts the demo, floats and sizes the window;
`shot.sh` uses `niri msg action screenshot-window --id`, which works wherever
the window is). **`grim -g` with computed geometry does not**: a tiled window
reports no `tile_pos_in_workspace_view`, and the one capture taken that way
photographed a browser.

**D-3 — one card per option, and the card is the option's identity.** A 2px
left rule was tried first and was functional but ugly; dropped, and with it
gone two options' controls read as a pair with nothing saying which one the
fields between them belonged to. The card is unconditional, so an option with a
form and an option with only a control are the same kind of object on screen.

**D-4 — a panel per block, drawn here rather than taken from `GroupBox`.**
Fluent's `GroupBox` draws no border at all — a bold title and some padding
(`fluent/groupbox.slint`) — so it would not do the job; and it binds
`accessible-role: groupbox`, which would put one per *block* into a tree where
exactly one per **option** is asserted, and would put its title into the scope
`labels_in_option` reads. Drawing it kept both cases untouched.

**D-5 — a three-step tonal ramp, measured rather than argued.** Frame 22, card
28, panel 36, each derived from `Palette.background` so a style with another
ground still gets a ramp. Anchored *downwards* from the style's own ground so
the card stays the tone a person looks at most.

The first cut was 12% steps — the ratio asked for — and measured 25 / 28 / 31.
**That is the right ratio and the wrong difference**: three levels out of 255 at
this end of the scale, and the three surfaces read as one flat sheet. Both were
sampled off the screenshot with `ffmpeg` rather than judged by eye, and the two
were rendered side by side to choose between.

**D-6 — the header is a card too**, on the same middle tone, and its text
aligns with the form's rather than sitting left of it. `card-inset` and
`panel-inset` are named on the root and the header's padding is their sum, so
the alignment is derived; a literal 22px would drift the first time either is
tuned.

**D-7 — Inter, embedded.** `import "../../../assets/Inter.ttf"` in `app.slint`
plus `default-font-family`. The weight axis of the variable font works — 600 on
the title and 700 on block headings both render.

**It changes nothing on this machine, and that is the point.** `fc-match
sans-serif` here already answers `Inter`, so the before/after is
indistinguishable; what the embed buys is the same text on a machine whose
default is not Inter, with no fontconfig lookup for this family. The cost is
856K in the binary.

**L-5 — the trailing newline.** `without_one_terminator` in `diagnostics.rs`
drops at most one `\r\n`, `\n` or `\r` before escaping. **At most one**, never a
`trim_end`: a backend that wrote three blank lines wrote them.

Its witness was not a new test. Two rows of `table.rs`'s failure taxonomy — P2
and T3 — already asserted the old output through a real host, and went red on
the change. Those are the binding site, and they are better than the helper
test that was being written when they fired.

### Tests added

`tests/renderer/sizing.rs` — what the window asks the compositor for. Its own
module because every other module here asks the markup what it drew.

- `the_height_a_window_asks_for_follows_what_is_on_it`
- `past_the_cap_more_content_does_not_make_the_window_taller`

**Both were negative-controlled, and a third case was deleted for failing its
control.** Removing the `min-height` binding fails the first; removing the cap
fails the second. A case asserting that no backend string decides the window's
*width* passed with `preferred-width` deleted **and** passed again with the
`ScrollView` replaced by a plain layout — it discriminated nothing and was
removed rather than kept green. The measurement behind it is recorded in the
module's own header and beside the declaration in `app.slint`: the window asks
for 420px with no option on it, with a 420-character option label and with a
450-character checkbox label alike, because **nothing propagates a width
upward** past the scroller, not even a control's own minimum. The comment
originally written on `preferred-width` claimed the opposite and was false.

`just check` exits 0.

### 2026-09-16 — the block heading, and magnification

**D-8 — a block heading's gap is its own.** One `spacing: 4px` governed both
heading→first-field and field→field, so a heading sat exactly as far from the
field it covered as that field sat from the next: the gap said nothing about
which fields the heading claimed. The fields now form their own run inside the
panel at 4px, and the panel's own spacing is the heading's gap.

**This is the structural half of "it needs room to breathe", and it holds
whatever type treatment is chosen.** The typographic half was decided on a
four-way render — baseline and three candidates, side by side, tones sampled
off the screenshot rather than judged:

| | size / weight / tracking | heading tone | reads as |
|---|---|---|---|
| baseline | 13px · 700 · 0 | 255 — the labels' own | a bold checkbox label |
| A | 11px · 600 · +0.5px | 179 (70%) | a section marker |
| B | 15px · 600 · 0 | 255 | a heading |
| C | 10px · 700 · +1.2px | 156 (61%) | a legend |

**B was argued against and dropped: it collides with the window title.** The
title is 17px/600; B is 15px/600 at full white. Two headings two points apart
is not a hierarchy, it is a near-miss — and a block heading is the third level
down (inside an option, inside the question). The treatment that works goes
*down* from the labels, not up: the labels are what a person reads and acts on,
the heading is a wayfinding mark. The user took the rendered comparison and
tuned to 14px / 400 / +1.2px / 65%.

**The tone is `Palette.foreground.transparentize()`, never `.darker()`.**
Transparentize blends towards whatever is behind, so one declaration mutes a
near-white foreground on a dark ground *and* a near-black one on a light
ground. `darker()` only ever goes one way and is wrong in half the themes.

**Not done: uppercasing the heading.** C's scale normally wants it, and it was
deliberately not taken. Uppercasing is the host rewriting a backend-authored
string — it mangles acronyms and does nothing in a non-Latin script. Raise the
tone instead.

#### L-9 — magnification, proved from the tray

**D-9 — the lever is the compositor's own scale factor, not a `zoom` property
in the markup.** `window.dispatch_event(WindowEvent::ScaleFactorChanged { .. })`
is public API (verified against the pinned 1.17.1 sources: `i-slint-core/api.rs:648`,
`platform.rs:395`) and scales *everything the window draws* — the widget
library's own padding, borders and glyph metrics included. A `zoom` property
multiplying lengths would touch every literal in `app.slint` and still miss all
of that.

**D-10 — zoom does not go on the wire.** It never reaches `Command` and never
reaches the controller: no model state changes, nothing needs ordering against
an evaluation, and the backend is not told. `install.rs` wires the three tray
callbacks straight at the window. Sending it down the wire would put a
rendering concern in the controller for nothing.

**D-11 — the compositor's base is never stored.** `rescale` divides the zoom
already applied back out of what the window currently reports. The compositor
dispatches its own scale change whenever the real scale moves (the window
crossing to another output, `winitwindowadapter.rs:1519`), and a base captured
at startup would then be stale and would fight it; reading it back picks the
new base up instead.

**Why the tray and not a key binding.** Both bindings were costed before
choosing, and neither is a rider on this:

- **`Ctrl +/-`** needs a root `FocusScope`. Key events *do* bubble from the
  focused item up to the window (`window.rs:1336`), so a focused `CheckBox`
  does not block it — but if **nothing** is focused the bubble list is empty
  and no key reaches anything. 007 already logged *keyboard focus does not
  survive a present*. That follow-up is a **prerequisite** for this, not an
  aside: the shortcut would work until the next present and then silently stop.
- **`Ctrl`+wheel** has to be taken off the `ScrollView`. Its `Flickable`
  accepts every wheel event with no modifier check at all
  (`flickable.rs:251`, `accepts_pan_event`). Getting it back out means a
  covering `TouchArea` that accepts modifier-carrying scrolls and rejects the
  rest — with its own consequences for clicking the controls underneath. Its
  own piece of work.
- **`MenuItem.shortcut`** exists and is matched *before* the event reaches the
  focused item (`window.rs:1299`) — exactly the property that would sidestep
  the focus problem — but it is honoured only inside a `MenuBar`, and this is a
  `SystemTrayIcon`'s menu.

The tray needs none of that, cost about twenty lines, and de-risks the lever
before anything is spent on the input path.

**Observed:** works. All three items — in, out and reset — were driven from the
tray by the user and reported good. Not photographed: the tray menu cannot be
driven by the screenshot loop, the same limitation that keeps L-3/L-4 unseen,
so the human report *is* the evidence here.

**The zoom survives a present — confirmed on screen.** This was the question
that decided whether the affordance was worth anything: the window is hidden
between prompts and re-shown every two hours, and if the winit adapter re-read
the real scale on show and dispatched its own, the zoom would reset on every
prompt. It does not. Zoomed, then *Check now*, then looked: the level held.

**No test asserts this, deliberately.** The testing backend has no winit
adapter, so a green case there would assert the core's bookkeeping and nothing
about the thing that would actually break it — a proxy, in the exact shape this
slice has already been bitten by twice. The evidence is a human watching the
window, which is the honest instrument for it.

#### Tests added

`src/zoom.rs` — pure arithmetic, inline `#[cfg(test)] mod tests` in the shape
`controller.rs` and `draft.rs` use. Six cases, **all six negative-controlled.**

**Two of the controls nearly lied.** The first pass reported two mutations as
producing no failure. They were **compile errors**: the crate denies warnings,
and stripping `.min(Self::CEILING)` left `Self((x))`, which `unused_parens`
rejects. *A control that does not build looks exactly like a control that
passes, if the check greps only for `FAILED`.* A negative control must be
confirmed to have compiled and run before its red is believed — which is a new
face on the trap already recorded here and in
`docs/memory/a-green-test-can-assert-a-proxy.md`.

**Also noticed, not acted on:** closing the window quits the host
(`install.rs:49`, a deliberate earlier decision). It makes the window something
you cannot dismiss and get back, which is adjacent to L-6.

## Still on the list

- **L-3, L-4 — the diagnostic surface**, untouched this session: no word wrap,
  no text selection. Not yet seen on screen — it is reached through the tray
  menu, which this loop cannot drive.
- **L-6 — the idle surface.** Undecided, and it is a behaviour question rather
  than a layout one: with no view the surface is `Hidden` and the window is
  hidden, so there is nothing to lay out. Candidates: leave it; put the next
  check somewhere a person passes; keep a window up in an idle state.
- **L-7 — backend-authored strings reaching the screen are unbounded.**
  `title`, `option.label`, `block.heading`. Untouched.
- **L-8 — the void below the form** under a tiling compositor. Stable and
  coherent now rather than scattered, but not designed. It is the same space
  L-6 might occupy.
- **L-9 — magnification.** Landed on the tray, and it survives a present — the
  question that decided whether it was worth anything. Two things stay open,
  both by choice rather than by omission: **persistence across a restart**,
  deferred until the user has lived with it for a while; and whether the tray is
  the shipping affordance or scaffolding towards `Ctrl +/-`, which is gated on
  007's keyboard-focus follow-up either way.
