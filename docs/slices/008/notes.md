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
