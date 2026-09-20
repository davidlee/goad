//! `present`, the mapper — design.md §5.2. One function, one direction, no
//! `_ =>` arm: a canonical `View` becomes a displayable `Presentation`.
//!
//! `present` is total, pure and panic-free (I-1): it never refuses a legal
//! view, and every part of it that this renderer cannot draw is degraded
//! into `undrawn` rather than dropped (I-2, R-20). Nothing here reads a
//! clock, a file or a socket — stratum 3, but the pure half of it.

use goad_semantics::protocol::canonical::{
  AlternativeId, Alternatives, Content, Field, FieldId, FieldKind, NumberRange, Opt, OptionId,
  Timestamp, View,
};
use jiff::tz::Offset;
use slint::StyledText;

use crate::draft::{Edited, Finite, Reported};

/// A canonical view, mapped to what this renderer draws.
#[derive(Debug)]
pub struct Presentation {
  pub title: String,
  pub body: Body,
  pub options: Vec<PresentationOption>,
  /// Everything the protocol carried that this renderer did not draw. Never
  /// silently empty: if it is non-empty, the diagnostic surface says so
  /// (`receive`, `reception.rs`, is why that cannot be forgotten).
  pub undrawn: Vec<Undrawn>,
}

impl Presentation {
  /// True when the body reaching the glass is literal text the backend did
  /// not send as text — rejected markdown, HTML, or a URI. Drives the
  /// marker beside the body; the *reason* lives in the diagnostic list and
  /// appears nowhere else. Derived from `undrawn`, not stored, so the rule
  /// has one statement. Note what it excludes: `Undrawn::GroupHint` is
  /// undrawn but says nothing about the body, and so — were it constructible —
  /// would `Undrawn::FieldForm`. It is not: every field kind draws, so
  /// [`FieldForm`] is uninhabited and the exclusion names a variant nothing
  /// can contribute. The arm stays because a sixth protocol kind gives it back.
  #[must_use]
  pub fn body_is_degraded(&self) -> bool {
    self.undrawn.iter().any(|undrawn| {
      matches!(
        undrawn,
        Undrawn::MarkdownUnsupported { .. } | Undrawn::ContentForm { .. }
      )
    })
  }
}

/// The canonical half of an option row — never the label, never an
/// `AlternativeId`. Retained by the controller beside the generated row the
/// `VecModel` holds (design.md §5.2's two-row-type table).
///
/// No `Eq`, here and on the two types beneath it: a drawn field now carries a
/// `DrawnKind`, and both a `NumberRange` and an `Alternatives` hold `f64` or
/// reach one. Nothing needs it.
#[derive(Debug, Clone, PartialEq)]
pub struct PresentationOption {
  pub id: OptionId,
  pub label: String,
  /// The option's drawn fields, in declared order, cut into blocks. Empty
  /// when the option declares no field this renderer draws — and an empty
  /// `blocks` draws nothing at all, not an empty container.
  pub blocks: Vec<FieldBlock>,
}

/// A run of adjacent drawn fields sharing one `group` value, and the heading
/// that names it.
///
/// Layout, not a concept: a block is where a heading is drawn, and the host
/// takes no position on what the backend groups by. `heading: None` is a block
/// with no heading — never a missing block.
///
/// **Three different runs produce `heading: None`, and they do not all merge**
/// (F-P4). `Run::Unreadable` and `Run::Ungrouped` both key `None`, so adjacent
/// ones join into a single block (`:261-266`). `Run::Named("")` — a `group` the
/// backend sent empty — keys `Some("")`, so it does **not** merge with an
/// adjacent ungrouped field, and `blocks_from` opens a second headingless block
/// beside the first. `R-18` leaves grouping to the renderer, so this is
/// admitted rather than wrong; it is written down because the three cases read
/// identically once they are here.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldBlock {
  pub heading: Option<String>,
  pub fields: Vec<PresentationField>,
}

/// One drawn field: the id that answers it, the label beside it, and what
/// kind of thing it is. The person's own state is **not** here and never will
/// be — it lives in `draft.rs`, which is what keeps this value the backend's
/// and that one the person's (design.md §5.3, I-4).
#[derive(Debug, Clone, PartialEq)]
pub struct PresentationField {
  pub id: FieldId,
  pub label: String,
  pub kind: DrawnKind,
}

/// The kinds this renderer draws, with the per-kind data it needs to draw
/// one: a `number`'s bounds, and a `choice`'s alternatives.
///
/// **Host-local, and deliberately not the canonical `FieldKind` cloned**
/// (§7 D10). Carrying the canonical kind would avoid a second enum, but then
/// a sixth protocol kind would be representable in a *drawn* field and would
/// fall silently through the markup's `if` chain drawing nothing. A host-local
/// enum means the sixth kind has to be added here too, which is another place
/// the compiler stops you.
///
/// It is the input to all three of the kind-directed functions below, and
/// matching it exhaustively — never `_` — is what keeps them total.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawnKind {
  Boolean,
  Text,
  Number(NumberRange),
  /// The alternatives in declared order, **and the first one's id beside
  /// them**.
  ///
  /// The duplication is the point. One alternative always exists, because
  /// `Alternatives::new` rejects an empty list — but that is a fact about the
  /// protocol and is invisible to the compiler: `.first()` is an `Option`,
  /// `unwrap_used` / `expect_used` / `indexing_slicing` are `deny` crate-wide,
  /// and `AlternativeId::new` is `pub(super)` so there is no fallback id to
  /// construct. Cloning the id once, where the kind is built, makes `as_drawn`
  /// and every display site total with no lint exception anywhere
  /// (`prototype-notes.md` P-2).
  Choice {
    first: AlternativeId,
    alternatives: Alternatives,
  },
  DateTime,
}

/// What is shown where a view's body is drawn.
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
  None,
  /// The value the backend sent, shown as literal text. Rendered with
  /// `from_plain_text`, a total function that takes no decision, at the
  /// glass.
  Plain(String),
  /// `Content::Markdown` that parsed. The parse is **retained**, not
  /// repeated: `from_markdown` is the classifier and the renderer at once,
  /// and calling it a second time at the setter would put the accept/reject
  /// decision in two places.
  Rich(StyledText),
}

/// Everything a view carried that this renderer does not draw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Undrawn {
  /// A field whose kind this renderer does not draw. Names the option, the
  /// field and the form, because a backend can act on none of the three
  /// without the other two.
  FieldForm {
    option: OptionId,
    field: FieldId,
    form: FieldForm,
  },
  /// A `group` hint whose value is not a JSON string. The field is drawn
  /// where it was declared and under no heading: coercing the value would
  /// invent a heading the backend did not author.
  GroupHint { option: OptionId, field: FieldId },
  /// `Content::Markdown` that `StyledText::from_markdown` rejected.
  MarkdownUnsupported { detail: String },
  /// `Content::Html` or `Content::Uri` — admitted by the protocol, and
  /// shown as literal text because no element draws the *form*.
  ContentForm { form: ContentForm },
}

/// The content forms this renderer shows as literal text because Slint has
/// no element for them. Deliberately **not** a mirror of `Content`: it
/// names only the forms that reach the glass undrawn, so a third content
/// form added to the protocol is a compile error in the mapper's `match`
/// rather than a silent omission.
///
/// It carries no payload on purpose. The bytes already reach the glass as
/// the body; carrying them here too would render one value twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentForm {
  Html,
  Uri,
}

/// The field forms this renderer does not draw. Named for SPEC-001 §6.2's own
/// heading, *Field forms*, and deliberately **not** a mirror of `FieldKind`:
/// it names only the forms that go undrawn, so a sixth kind added to the
/// protocol is a compile error in the mapper's `match` rather than a field
/// silently dropped.
///
/// **It is empty, and that is the finished state rather than a gap.** A
/// variant left the moment its kind drew — `Text` and `DateTime` at PHASE-07,
/// `Number` at PHASE-08, `Choice` at PHASE-09 — and all five now draw, so
/// there is no form left to name. An empty enum is uninhabited, so
/// `Undrawn::FieldForm` can no longer be constructed and no view can carry a
/// field this renderer did not draw.
///
/// **It is not deleted, and deleting it is the one change that would cost
/// something.** [`drawn_form`] returns `Result<DrawnKind, Self>` and matches
/// the canonical `FieldKind` exhaustively; a sixth protocol kind is a compile
/// error there, and whoever adds it then has to decide — draw it, or give this
/// enum a variant back. Delete the type and that fork disappears along with
/// the error that forces it (`design.md` §5.1, §7 D11; `plan.md` PHASE-09/VA-2).
///
/// Its `Display` **stays, with no arms**, for the same reason the type does.
/// `match *self {}` is total over an uninhabited enum, so `diagnostics.rs`'s
/// undrawn line keeps interpolating `{form}` exactly as it did — and a sixth
/// kind is a second compile error, here, naming the word a person has to be
/// shown. Deleting the impl would take that with it and leave the line to be
/// rewritten by whoever adds the kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldForm {}

impl std::fmt::Display for FieldForm {
  /// The backend's own word for the kind, so the line a person reads names
  /// the value a backend author would search their own view for.
  ///
  /// There is no word to write while the enum is empty, and no value to write
  /// it for: `match *self {}` is how that is said to the compiler rather than
  /// to a reader.
  fn fmt(&self, _formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {}
  }
}

impl std::fmt::Display for ContentForm {
  /// A noun phrase and nothing else, so it drops into the undrawn sentence
  /// without an article being chosen at the call site.
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    formatter.write_str(match self {
      Self::Html => "HTML",
      Self::Uri => "a URI",
    })
  }
}

/// The run a field joins, read off its `group` hint. A run is a maximal
/// sequence of adjacent drawn fields whose keys are equal, and a block is
/// drawn per run — layout, which is the only thing this host has to say about
/// grouping (design.md §8/R-6).
///
/// The hint is read here and nowhere else in this host: `SPEC-001/R-18`
/// permits the renderer, and only the renderer, to branch on a hint key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Run<'hint> {
  /// No `group` key at all.
  Ungrouped,
  /// A `group` key whose value is not a JSON string. Reported, because a hint
  /// that could not be honoured is one the backend should hear about.
  Unreadable,
  /// A `group` key carrying a string, empty or not.
  Named(&'hint str),
}

impl<'hint> Run<'hint> {
  fn of(field: &'hint Field) -> Self {
    match field.hints().as_map().get("group") {
      None => Self::Ungrouped,
      Some(serde_json::Value::String(name)) => Self::Named(name),
      Some(_) => Self::Unreadable,
    }
  }

  /// What two adjacent fields must share to be one run. **Both ungrouped
  /// states key the same**: a `group` that is not a string is not a different
  /// group, it is no group, and the field is drawn in place under no heading
  /// either way.
  fn key(self) -> Option<&'hint str> {
    match self {
      Self::Ungrouped | Self::Unreadable => None,
      Self::Named(name) => Some(name),
    }
  }
}

/// The heading a run is drawn under. `""` is a block with no heading rather
/// than a heading with no text: the backend sent a string, and there is
/// nothing in it to draw.
fn heading_of(key: Option<&str>) -> Option<String> {
  key.filter(|name| !name.is_empty()).map(str::to_owned)
}

/// How this renderer draws a field of one kind — or the form it reports
/// instead, for a kind it draws no control for.
///
/// **One match over the canonical `FieldKind`, and the only one.** Matching it
/// exhaustively is what makes a sixth kind a compile error here rather than a
/// field silently dropped — and it is the site that breaks for that, not
/// `draft.rs`'s `submitted`, whose match is over a host-local type (AC-7,
/// §7 D11).
///
/// **PHASE-05 replaced `undrawn_form` with this**, and the reason is that a
/// second drawn kind left the old pair with no honest spelling. `sift` used to
/// write `DrawnKind::Boolean` as a constant, true because `boolean` was the
/// only kind that reached it. With two kinds drawn it has to choose, and a
/// *second* total function beside `undrawn_form` would have to answer for the
/// three kinds still reported undrawn: `DrawnKind::Choice` carries the first
/// alternative's id, `Alternatives` offers only `as_slice`, `.first()` is an
/// `Option`, and `AlternativeId::new` is `pub(super)` — so there is no id to
/// fall back to. Every way out of that is either a lie in the type or a field
/// that sorts nowhere and is dropped, which is the one thing `I-2` and `R-20`
/// forbid. A `Result` has exactly one arm per kind, no unreachable arm, and no
/// pair of `Option`s whose complementarity the compiler cannot see.
///
/// Each `Err` arm is filled in by the phase that draws its control:
/// `number` PHASE-08, `choice` PHASE-09. When the last
/// one moves across, `FieldForm` has no constructible variant and
/// `Undrawn::FieldForm` becomes the place a *sixth* kind would go rather than
/// a report anything can reach (§5.1's consumer table).
///
/// **The `Result` is kept although its `Err` is now uninhabited**, and
/// `clippy::unnecessary_wraps` is silenced rather than obeyed. The wrapper is
/// the sixth-kind fork: a new `FieldKind` variant must be sorted into `Ok` —
/// draw it — or into `Err` — give [`FieldForm`] a variant back and report it.
/// Collapsing the signature to a bare `DrawnKind` deletes the second half of
/// that choice, and with it the only reason [`FieldForm`] and
/// `Undrawn::FieldForm` still exist (`plan.md` PHASE-09/VA-2).
#[expect(
  clippy::unnecessary_wraps,
  reason = "the `Err` is uninhabited only because all five kinds draw; the `Result` is \
            what makes a sixth kind choose between drawing and reporting, which is the \
            mechanism AC-7 names"
)]
fn drawn_form(kind: &FieldKind) -> Result<DrawnKind, FieldForm> {
  match kind {
    FieldKind::Boolean => Ok(DrawnKind::Boolean),
    FieldKind::Text => Ok(DrawnKind::Text),
    FieldKind::DateTime => Ok(DrawnKind::DateTime),
    FieldKind::Number(range) => Ok(DrawnKind::Number(*range)),
    // The first alternative's id, cloned beside the list here — the one site
    // that has an `Alternatives` in hand, which is what makes `as_drawn` and
    // every display site total (`DrawnKind::Choice`, `prototype-notes.md` P-2).
    //
    // **`Alternatives::first` is why this is a total expression rather than an
    // exception.** Non-emptiness is that type's invariant and it now exposes
    // it, so the argument sits beside the `new` that enforces it instead of
    // being re-derived here — which is what `design.md` §5.2 asked for and
    // could not have while the empty case still had the dead
    // `Err(FieldForm::Choice)` arm to fall into. `crates/goad` carries no lint
    // exception for this at all (`plan-log.md`, 2026-09-20).
    FieldKind::Choice { alternatives } => Ok(DrawnKind::Choice {
      first: alternatives.first().id().clone(),
      alternatives: alternatives.clone(),
    }),
  }
}

/// One drawn field and the run key it carries — and the **only** thing
/// `blocks_from` can see. That is what makes "a run is over the drawn fields"
/// a property of the types rather than a rule somebody has to remember: an
/// undrawn field never becomes one of these, so it cannot break a run or open
/// a block.
#[derive(Debug)]
struct Drawn<'hint> {
  key: Option<&'hint str>,
  field: PresentationField,
}

/// One option's fields, sifted: the drawn ones in declared order, and a
/// report for everything the renderer did not honour — a kind it does not
/// draw, and a `group` hint it could not read.
fn sift(option: &Opt) -> (Vec<Drawn<'_>>, Vec<Undrawn>) {
  let mut drawn = Vec::new();
  let mut undrawn = Vec::new();

  for field in option.fields().as_slice() {
    let run = Run::of(field);
    if run == Run::Unreadable {
      undrawn.push(Undrawn::GroupHint {
        option: option.id().clone(),
        field: field.id().clone(),
      });
    }

    // The drawn kind and the undrawn report are **one** decision, taken once,
    // above: a field is drawn *as* what `drawn_form` answers, or reported *as*
    // what it refuses. There is no third outcome and so no field that sorts
    // nowhere.
    match drawn_form(field.kind()) {
      Err(form) => undrawn.push(Undrawn::FieldForm {
        option: option.id().clone(),
        field: field.id().clone(),
        form,
      }),
      Ok(kind) => drawn.push(Drawn {
        key: run.key(),
        field: PresentationField {
          id: field.id().clone(),
          label: field.label().to_owned(),
          kind,
        },
      }),
    }
  }

  (drawn, undrawn)
}

/// Adjacent fields sharing a run key become one block; a block opens wherever
/// the key changes and is only ever appended to.
///
/// Nothing is sorted, nothing is merged and no field moves (I-6), so two
/// `Morning` runs with an `Evening` between them stay two blocks. A block is
/// opened by a field, never before one, so a group whose every member went
/// undrawn draws no heading over nothing.
fn blocks_from(drawn: Vec<Drawn<'_>>) -> Vec<FieldBlock> {
  let mut blocks: Vec<FieldBlock> = Vec::new();
  let mut open: Option<&str> = None;

  for Drawn { key, field } in drawn {
    match blocks.last_mut() {
      Some(block) if open == key => block.fields.push(field),
      _ => {
        open = key;
        blocks.push(FieldBlock {
          heading: heading_of(key),
          fields: vec![field],
        });
      }
    }
  }

  blocks
}

/// A canonical `View` becomes a `Presentation`. `View` has one variant
/// today; when the protocol gains a second, this is a compile error naming
/// this file, not a blank window at runtime.
#[must_use]
pub fn present(view: &View) -> Presentation {
  match view {
    View::Choice(choice) => {
      let mut undrawn = Vec::new();

      let body = match choice.body() {
        None => Body::None,
        Some(Content::Text(text)) => Body::Plain(text.clone()),
        Some(Content::Markdown(source)) => match StyledText::from_markdown(source) {
          Ok(parsed) => Body::Rich(parsed),
          Err(error) => {
            undrawn.push(Undrawn::MarkdownUnsupported {
              detail: error.to_string(),
            });
            Body::Plain(source.clone())
          }
        },
        Some(Content::Html(source)) => {
          undrawn.push(Undrawn::ContentForm {
            form: ContentForm::Html,
          });
          Body::Plain(source.clone())
        }
        Some(Content::Uri(source)) => {
          undrawn.push(Undrawn::ContentForm {
            form: ContentForm::Uri,
          });
          Body::Plain(source.clone())
        }
      };

      let options = choice
        .options()
        .as_slice()
        .iter()
        .map(|option| {
          let (drawn, reports) = sift(option);
          undrawn.extend(reports);
          let blocks = blocks_from(drawn);
          PresentationOption {
            id: option.id().clone(),
            label: option.label().to_owned(),
            blocks,
          }
        })
        .collect();

      Presentation {
        title: choice.title().to_owned(),
        body,
        options,
        undrawn,
      }
    }
  }
}

// ---------------------------------------------------------------------------
// The kind-directed pure functions
// ---------------------------------------------------------------------------
//
// Three of them, and every one is total over `DrawnKind` by matching it
// exhaustively. **The function that interprets a report is `interpret`**, and
// no production line under `crates/goad/src` may name the identifier
// `resolve`: `goad-boundary`'s
// `structure::no_production_line_in_the_renderer_names_the_identifier_resolve`
// asserts its absence over this whole directory, as an identifier-word match
// rather than a path grep, with string literals kept (design.md §5.2, §7 D30).
// That binds the renderer permanently, not just this function.

/// A number's spelling, which is load-bearing rather than cosmetic: it is what
/// the widget is drawn showing and what its guard compares itself against
/// (design.md §5.2).
///
/// `f64`'s `Display` — the shortest decimal that reads back as itself, and one
/// that never uses scientific notation — and `{:e}` where that spelling
/// exceeds **24** characters.
///
/// The trigger is length because the defect is length. `min: f64::MAX` is a
/// legal `R-17` bound and its `Display` is 309 characters in a `LineEdit`; the
/// smallest normal is 326. An `f64` round-trips in at most 17 significant
/// digits, so 17 digits, a sign and a point is 19 — 24 leaves alone every
/// number a person would type, and catches exactly the spellings that are long
/// only because the exponent is. Switching on magnitude instead, the
/// spreadsheet rule, needs two constants and sends `1e16` to scientific when
/// its plain spelling is 17 characters.
///
/// Both spellings re-parse under `f64::from_str`, which is what the guard's
/// comparand needs and what keeps this free of any numeric grammar of its own.
fn spelled(number: f64) -> String {
  let plain = number.to_string();
  if plain.len() > 24 {
    format!("{number:e}")
  } else {
    plain
  }
}

/// The one place an `f64` becomes the `float` Slint does its arithmetic in,
/// and `None` where the narrowing would lose something.
///
/// `NumberRange` holds `f64` and `R-17` admits any finite one, while Slint's
/// `float` is `f32` — so a legal bound of `1e100` would arrive as an infinity.
/// Narrowing the *contract* by a type rather than by a decision is
/// `CLAUDE.md`'s third invariant failing in the quietest available way, so
/// every crossing goes through here and every caller has to say what it does
/// with `None` (design.md §5.2 *A number at the markup boundary*, §7 D16).
///
/// The `as` below is the crate's only one and is checked on the very next
/// line. There is no `TryFrom<f64> for f32` to use instead, and the rule this
/// enforces is not "a conversion clippy would allow" but "a conversion that
/// loses nothing" — which is narrower.
#[expect(
  clippy::as_conversions,
  clippy::cast_possible_truncation,
  clippy::float_cmp,
  reason = "the crate's only f64 -> f32 narrowing, and the round trip is what \
    makes it a checked conversion rather than a cast; `as` is the only \
    spelling Rust offers for it, and exact equality is the whole of the check \
    — a tolerance here would admit precisely the bounds this refuses"
)]
#[must_use]
pub fn exact_f32(value: f64) -> Option<f32> {
  let narrowed = value as f32;
  (f64::from(narrowed) == value).then_some(narrowed)
}

/// A `Slider`'s step: a hundredth of the span it is drawn over.
///
/// Named rather than written twice because [`slider_bounds`] **checks** this
/// step and `glass.rs` **ships** it, and a second spelling is a second thing
/// that can drift from the one that did the proving.
///
/// Slint defaults `step` to `1`, which crosses a range declared `min: 0,
/// max: 1` in one key press, and rejects every key when it is `0`
/// (`common/slider-base.slint:8`, `:79`). Slint's own rule for
/// `accessible-value-step` is `min(root.step, (maximum - minimum) / 100)`
/// (`fluent/slider.slint:29`) — a **cap**, not a floor — so under this step
/// the cap binds at equality and the increment an assistive technology is told
/// about is the one the keyboard gives. This is presentation, which `R-18`
/// leaves to the renderer; the `step` `slice-009.md` excludes is the
/// **protocol** one (design.md §5.2).
#[must_use]
pub fn slider_step(minimum: f32, maximum: f32) -> f32 {
  (maximum - minimum) / 100.0
}

/// The bounds a `Slider` may be drawn over, or `None` for the numeric text
/// control. **The only place a `number`'s control is chosen** (§7 D17).
///
/// Every clause is evaluated in `f32`, because `f32` is what Slint will be
/// doing the arithmetic in. It answers `Some` when all three hold:
///
/// 1. both bounds are present, and each round-trips `f64` → `f32` → `f64`
///    unchanged ([`exact_f32`]);
/// 2. the span `maximum - minimum` is finite and strictly positive;
/// 3. the step, [`slider_step`], **moves the value**: `minimum + step` is
///    greater than `minimum`, and `maximum - step` is less than `maximum`.
///
/// An exact endpoint round-trip on its own is not enough, because Slint's
/// slider arithmetic is what has to work afterwards. The thumb is placed by
/// dividing by `maximum - minimum` (`fluent/slider.slint:75`), so equal bounds
/// such as `[1, 1]` divide by zero and `[-f32::MAX, f32::MAX]` has an infinite
/// span even though both endpoints are exact. That is what clause 2 is for,
/// and it is also what makes the division safe to perform at all.
///
/// **Clause 3 states operability directly rather than approximating it**, and
/// it subsumes the `step > 0` test it replaces — not clause 2. `increment()`
/// is exactly `set-value(value + step)` and `set-value` returns immediately
/// when the result equals the value it already holds
/// (`common/slider-base.slint:117-131`), so a step below half an ulp freezes
/// the slider even where the bounds round-trip exactly and the span is finite
/// and positive: at `minimum = 2^100` the `f32` ulp is `2^77`, and a one-ulp
/// span gives a step of about `2^70.3`. A step that underflows to zero fails
/// clause 3, and so would a non-finite one — but an infinite **span** passes
/// it, because an infinite step does move the value in both directions, which
/// is why clause 2 is not subsumed and is evaluated first.
///
/// None of what this rejects is a loss: each takes the text control, which is
/// where a range a slider cannot operate belongs anyway. So the text control
/// is the one that always works and the `Slider` is the one with an
/// admissibility condition — the opposite of how a bounds-driven reading makes
/// it look. A later slice that wants the choice to read a hint (`R-18` permits
/// the renderer, and only the renderer, to branch on one), a configuration, or
/// nothing at all, changes this body and nothing else: not the markup, not the
/// wire, not `Edited` (design.md §5.2).
#[must_use]
pub fn slider_bounds(range: &NumberRange) -> Option<(f32, f32)> {
  let minimum = exact_f32(range.min()?)?;
  let maximum = exact_f32(range.max()?)?;

  let span = maximum - minimum;
  if !span.is_finite() || span <= 0.0 {
    return None;
  }

  let step = slider_step(minimum, maximum);
  (minimum + step > minimum && maximum - step < maximum).then_some((minimum, maximum))
}

/// The number a `number` field is drawn showing: its declared minimum, or zero
/// where none was declared.
///
/// `R-17` already guarantees a declared bound is finite, so `Finite::new`
/// cannot refuse one. It is still the constructor that is called, falling back
/// to `ZERO`, because a total expression is cheaper than an argument about why
/// an `expect` is unreachable (design.md §5.2).
///
/// A range carrying only a `max` is legal, so this can answer a number above
/// that maximum — `max: -10` and no `min` is drawn showing, and submits, `0`.
/// Not a defect: `R-35` puts the judgement of whether an answer is acceptable
/// in the backend, and `R-58` requires a value for every drawn field. It is a
/// consequence a backend author cannot discover from `R-58`, which is why
/// `canon-delta.md` CD-1 states it.
fn drawn_number(range: &NumberRange) -> Finite {
  Finite::new(range.min().unwrap_or(0.0)).unwrap_or(Finite::ZERO)
}

/// An `Adjusted` built from the number alone, spelling the text beside it.
///
/// Every site that produces one **without a person having typed** comes
/// through here — `as_drawn`, and a `Slider`'s `AdjustedValue` — so the format
/// rule has one application rather than one per site (design.md §5.2).
fn adjusted(number: Finite) -> Edited {
  Edited::Adjusted {
    text: spelled(number.get()),
    number,
  }
}

/// The number a numeric field already holds, where the draft holds one at all.
///
/// Total over `Edited` rather than `_`-terminated: a sixth variant is a
/// compile error here, which is where the question *is this a number?* would
/// otherwise be answered by omission.
fn held_number(held: Option<&Edited>) -> Option<Finite> {
  match held {
    Some(Edited::Adjusted { number, .. }) => Some(*number),
    Some(Edited::Checked(_) | Edited::Typed(_) | Edited::Chosen(_) | Edited::Picked { .. })
    | None => None,
  }
}

/// What the wire carries for a field nobody has touched — what the widget is
/// drawn showing, so the screen and the wire agree (§4's P-3, §7 D1).
///
/// **Three call sites, and `glass.rs` is deliberately not one of them.**
/// `controller::answer` applies it because `R-58` forbids omitting a value for
/// a drawn field; `interpret` applies it to supply the number a numeric text
/// falls back to; [`untouched`] applies it for the four kinds whose screen and
/// wire agree. Keeping the glass out is what makes the `datetime` epoch a fact
/// about the wire rather than a fact about the screen — a button reading
/// `1970-01-01T00:00:00+00:00` would be the host showing a person an answer
/// nobody gave (design.md §5.2, §7 D2). `datetime` is the only kind whose
/// display can tell *untouched* from *answered*, and [`untouched`] is where that
/// is said.
#[must_use]
pub fn as_drawn(kind: &DrawnKind) -> Edited {
  match kind {
    DrawnKind::Boolean => Edited::Checked(false),
    DrawnKind::Text => Edited::Typed(String::new()),
    DrawnKind::Number(range) => adjusted(drawn_number(range)),
    // The first alternative's id, cloned off the kind rather than read out of
    // the list — which is the whole reason the kind carries it.
    DrawnKind::Choice { first, .. } => Edited::Chosen(first.clone()),
    // `+00:00` rather than `Z`, because it falls out of the same
    // `display_with_offset` call as every other datetime and a second code
    // path for the untouched case was the worse trade (§7 D3). The sentinel a
    // backend would recognise is the 1970, not the offset.
    DrawnKind::DateTime => Edited::Picked {
      instant: Timestamp::new(jiff::Timestamp::UNIX_EPOCH),
      offset: Offset::UTC,
    },
  }
}

/// What an untouched field **shows**, where that is one of the draft's values
/// at all — the screen's half of [`as_drawn`], and the one place the two part.
///
/// `None` means *this kind's untouched display is not a value the draft can
/// hold*, and `datetime` is the only kind it answers for: the button reads
/// *not set* while the wire carries the epoch, because a button reading
/// `1970-01-01T00:00:00+00:00` would be the host showing a person an answer
/// nobody gave (§7 D1, D2). `glass.rs` supplies that sentinel, which is
/// presentation and belongs there.
///
/// For the other four the screen and the wire agree — §4's P-3 — and this
/// function is where that agreement is **stated** rather than left as a
/// comment beside a defaulted slot. It stopped being safe to leave implicit
/// when `number` began drawing: a field declared `min: 2.5` shows `2.5`
/// because that is what it submits, and a defaulted slot would have shown an
/// empty box and submitted `2.5` (design.md §5.2).
///
/// So `glass.rs` still never calls `as_drawn`, and the divergence D-6 turns on
/// has exactly one statement, here, beside the rule it diverges from.
#[must_use]
pub fn untouched(kind: &DrawnKind) -> Option<Edited> {
  match kind {
    DrawnKind::DateTime => None,
    DrawnKind::Boolean | DrawnKind::Text | DrawnKind::Number(_) | DrawnKind::Choice { .. } => {
      Some(as_drawn(kind))
    }
  }
}

/// What the draft should hold, given what one widget reported, what the host
/// holds for that field now, and what kind the field was drawn as. `None` out
/// is a renderer bug.
///
/// Two of the five values can only be formed where the retained presentation
/// is, and a Slint callback is not there: an `AlternativeId` can only be
/// cloned off a drawn view, so a `ComboBox`'s index travels and is interpreted
/// against the drawn field's alternatives; and a numeric text that does not
/// parse finitely keeps the number the field already holds, which only the
/// draft — or, for an untouched field, the minimum it was drawn showing —
/// knows (design.md §5.2, §7 D25).
///
/// `held` is an `Option` and this applies `as_drawn`'s rule itself, so every
/// caller passes `state_of(…)` straight through rather than writing
/// `unwrap_or_else(|| as_drawn(kind))` at each site.
///
/// **The `None` surface is all three of its cases, and there is no fourth:**
///
/// 1. a `Chosen` index no alternative of the drawn field has;
/// 2. an `AdjustedValue` that is not finite;
/// 3. a report whose variant does not match the drawn kind (§7 D31).
///
/// All three are renderer bugs and take the `Refused::UnknownField` posture:
/// reported, nothing recorded. The third is here because the signature admits
/// every pair — six reports against five kinds is thirty, of which six are
/// in-kind — and the first two do not cover the other twenty-four. Two of them
/// look covered and are not. A mismatched `Chosen` falls into case 1 only if
/// the implementation happens to answer *no alternatives* for the four kinds
/// that have none, which is a coincidence of spelling rather than a rule. And
/// a mismatched `AdjustedText` is **not** an exception to *the text is
/// recorded verbatim, always*: that rule is about an in-kind `AdjustedText`,
/// where there is a number to keep beside the text, and on a mismatch there is
/// no in-kind rule left to honour. Recording a value in answer to a renderer
/// bug is a value nobody gave, held as though someone gave it.
///
/// That is why the match below carries no `_` arm in either position. A
/// `_ => None` would satisfy a two-case reading, and would answer a sixth
/// `Reported` variant or a sixth `DrawnKind` by silence.
#[must_use]
pub fn interpret(reported: &Reported, held: Option<&Edited>, kind: &DrawnKind) -> Option<Edited> {
  match kind {
    DrawnKind::Boolean => match reported {
      Reported::Checked(checked) => Some(Edited::Checked(*checked)),
      Reported::Typed(_)
      | Reported::AdjustedText(_)
      | Reported::AdjustedValue(_)
      | Reported::Chosen(_)
      | Reported::Picked { .. } => None,
    },
    DrawnKind::Text => match reported {
      Reported::Typed(text) => Some(Edited::Typed(text.clone())),
      Reported::Checked(_)
      | Reported::AdjustedText(_)
      | Reported::AdjustedValue(_)
      | Reported::Chosen(_)
      | Reported::Picked { .. } => None,
    },
    DrawnKind::Number(range) => match reported {
      // **One rule covers every text the control admits.** The text is
      // recorded verbatim, always; the number is replaced only where the
      // parse yields a *finite* `f64`, and otherwise the field keeps the
      // number it had — for a field nobody has touched, the number it was
      // drawn showing. So `-`, `.`, `12/25`, `inf` and `1e400` all reach the
      // draft as text and leave the number alone, and the host repairs
      // nothing (§7 D23, D33).
      Reported::AdjustedText(text) => Some(Edited::Adjusted {
        number: text
          .parse::<f64>()
          .ok()
          .and_then(Finite::new)
          .or_else(|| held_number(held))
          .unwrap_or_else(|| drawn_number(range)),
        text: text.clone(),
      }),
      // A `Slider` has nothing to display, so the host's own spelling of the
      // number is the text. `f64::from` widens losslessly; a non-finite is
      // case 2 of the `None` surface, refused here rather than inside a Slint
      // closure that has nothing to report a refusal to (§5.5 I-G).
      Reported::AdjustedValue(value) => Finite::new(f64::from(*value)).map(adjusted),
      Reported::Checked(_) | Reported::Typed(_) | Reported::Chosen(_) | Reported::Picked { .. } => {
        None
      }
    },
    DrawnKind::Choice { alternatives, .. } => match reported {
      // Case 1: an index no alternative has. `AlternativeId::new` is
      // `pub(super)`, so the id this produces was necessarily cloned off the
      // view the backend sent (§5.5 I-D, §7 D12).
      Reported::Chosen(index) => usize::try_from(*index)
        .ok()
        .and_then(|at| alternatives.as_slice().get(at))
        .map(|alternative| Edited::Chosen(alternative.id().clone())),
      Reported::Checked(_)
      | Reported::Typed(_)
      | Reported::AdjustedText(_)
      | Reported::AdjustedValue(_)
      | Reported::Picked { .. } => None,
    },
    DrawnKind::DateTime => match reported {
      Reported::Picked { instant, offset } => Some(Edited::Picked {
        instant: *instant,
        offset: *offset,
      }),
      Reported::Checked(_)
      | Reported::Typed(_)
      | Reported::AdjustedText(_)
      | Reported::AdjustedValue(_)
      | Reported::Chosen(_) => None,
    },
  }
}

// The three kind-directed functions above are pure and stratum-internal, and
// two of them are `pub` only because `controller.rs` and `glass.rs` reach
// them. Tested here, inline, in the `#[cfg(test)] mod tests` shape
// `draft.rs`, `controller.rs`, `goad-shell/src/state.rs` and
// `goad-semantics/src/schedule.rs` already use.
#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::{FieldKind, NumberRange, Timestamp, View};
  use goad_semantics::protocol::normalize::read_response;
  use jiff::tz::Offset;

  use crate::draft::{Edited, Finite, Reported, submitted};

  use super::{
    DrawnKind, Undrawn, as_drawn, drawn_form, exact_f32, interpret, present, slider_bounds, spelled,
  };

  /// The view a document normalizes to, for a unit that needs a real
  /// `FieldKind` rather than one it constructed.
  fn viewed(document: &serde_json::Value) -> View {
    let now = Timestamp::new(
      "2026-01-01T00:00:00Z"
        .parse()
        .expect("the fixture must be an instant"),
    );
    read_response(document.to_string().as_bytes(), now)
      .expect("the fixture must normalize")
      .value
      .view()
      .expect("the fixture carries a view")
      .clone()
  }

  /// PHASE-09/**VT-7** — **AC-7.** Every canonical `FieldKind` is drawn, and
  /// the only field-level report left is the one that is not about a kind.
  ///
  /// **The property AC-7 names is a compile-time one and is identifier-free**
  /// (`design.md:149-151`): `drawn_form` matches the canonical `FieldKind`
  /// with no `_` arm, so a sixth protocol kind is an error there and whoever
  /// adds it must choose — draw it, or give `FieldForm` a variant back. No
  /// test can assert a match's exhaustiveness; what this asserts is the half
  /// that is observable, which is that today the choice went the same way five
  /// times. A sixth kind added and drawn passes this; a sixth kind added and
  /// *not* drawn does not compile at all.
  ///
  /// The second half is `Undrawn` still doing its job for the one thing that
  /// can still be undrawn: a `group` hint that is not a string. The field is
  /// drawn where it was declared — coercing `7` into a heading would invent
  /// one the backend did not author — and the report is beside it.
  #[test]
  fn every_canonical_field_kind_is_drawn_and_a_group_hint_is_still_reported() {
    let view = viewed(&serde_json::json!({
      "view": {
        "kind": "choice",
        "title": "T",
        "options": [{
          "id": "opt",
          "label": "L",
          "fields": [
            { "id": "a", "kind": "boolean", "label": "A" },
            { "id": "b", "kind": "text", "label": "B" },
            { "id": "c", "kind": "number", "label": "C", "min": 0, "max": 10 },
            { "id": "d", "kind": "choice", "label": "D",
              "options": [{ "id": "one", "label": "One" }] },
            { "id": "e", "kind": "datetime", "label": "E", "group": 7 }
          ]
        }]
      }
    }));
    let View::Choice(choice) = &view;
    let fields = choice.options().as_slice()[0].fields();

    let drawn: Vec<bool> = fields
      .as_slice()
      .iter()
      .map(|field| drawn_form(field.kind()).is_ok())
      .collect();
    assert_eq!(
      drawn,
      vec![true, true, true, true, true],
      "all five kinds draw, so `drawn_form` has no `Err` left to answer with"
    );

    let presentation = present(&view);
    assert_eq!(
      presentation.undrawn,
      vec![Undrawn::GroupHint {
        option: choice.options().as_slice()[0].id().clone(),
        field: fields.as_slice()[4].id().clone(),
      }],
      "and the one report a field can still earn is the one that is not about its kind"
    );
    assert_eq!(
      presentation.options.as_slice()[0]
        .blocks
        .iter()
        .flat_map(|block| block.fields.iter())
        .count(),
      5,
      "every field is drawn, the badly grouped one included"
    );
  }

  /// A `choice` over two declared alternatives, as the mapper would build it.
  ///
  /// `AlternativeId::new` is `pub(super)` in `goad-semantics`, so a unit that
  /// needs one parses a view and clones it off rather than minting it
  /// (§5.5 I-D). The wire key is `options`, because that is the key a backend
  /// author writes (`R-16`, `R-53`).
  fn a_choice() -> DrawnKind {
    let document = serde_json::json!({
      "view": {
        "kind": "choice",
        "title": "T",
        "options": [{
          "id": "opt",
          "label": "L",
          "fields": [{
            "id": "pick",
            "kind": "choice",
            "label": "F",
            "options": [
              { "id": "first", "label": "First" },
              { "id": "second", "label": "Second" }
            ]
          }]
        }]
      }
    });
    let now = Timestamp::new(
      "2026-01-01T00:00:00Z"
        .parse()
        .expect("the fixture must be an instant"),
    );
    let view = read_response(document.to_string().as_bytes(), now)
      .expect("the fixture must normalize")
      .value
      .view()
      .expect("the fixture carries a view")
      .clone();
    let View::Choice(choice) = &view;
    let field = choice.options().as_slice()[0].fields().as_slice()[0].clone();
    let FieldKind::Choice { alternatives } = field.kind() else {
      panic!("the fixture declares a choice");
    };
    DrawnKind::Choice {
      first: alternatives.as_slice()[0].id().clone(),
      alternatives: alternatives.clone(),
    }
  }

  fn a_number(min: Option<f64>, max: Option<f64>) -> DrawnKind {
    DrawnKind::Number(NumberRange::new(min, max).expect("the fixture's bounds are legal"))
  }

  fn finite(value: f64) -> Finite {
    Finite::new(value).expect("the fixture's number is finite")
  }

  fn epoch() -> Timestamp {
    Timestamp::new(jiff::Timestamp::UNIX_EPOCH)
  }

  /// PHASE-02/VT-3 — `as_drawn` over all five kinds: what the widget is drawn
  /// showing before anybody touches it (§4's P-3, §7 D1).
  #[test]
  fn as_drawn_answers_every_kind() {
    assert_eq!(as_drawn(&DrawnKind::Boolean), Edited::Checked(false));
    assert_eq!(as_drawn(&DrawnKind::Text), Edited::Typed(String::new()));
    assert_eq!(
      as_drawn(&a_number(Some(2.5), Some(10.0))),
      Edited::Adjusted {
        number: finite(2.5),
        text: "2.5".to_owned(),
      },
      "the declared minimum, beside its own spelling — the widget is drawn \
       showing both, so the screen and the wire agree and the guard has a \
       comparand"
    );
    assert_eq!(
      as_drawn(&a_number(None, None)),
      Edited::Adjusted {
        number: Finite::ZERO,
        text: "0".to_owned(),
      },
      "no declared bound falls back to zero"
    );
    let DrawnKind::Choice { first, .. } = a_choice() else {
      panic!("the fixture is a choice");
    };
    assert_eq!(
      as_drawn(&a_choice()),
      Edited::Chosen(first),
      "the first alternative in declared order"
    );
    assert_eq!(
      as_drawn(&DrawnKind::DateTime),
      Edited::Picked {
        instant: epoch(),
        offset: Offset::UTC,
      }
    );
  }

  /// PHASE-02/VT-3, the wire half — `canon-delta.md` CD-1 in one case: what
  /// this host submits for a field nobody touched, per kind.
  ///
  /// The two clauses CD-1 exists for are the last two assertions. The
  /// `datetime` epoch has no neutral value to be, so the host submits one
  /// nobody would pick rather than one that looks like an answer; and a range
  /// carrying only a `max` submits `0`, **outside the bound its own backend
  /// declared**. That breaches nothing — `R-35` puts the judgement in the
  /// backend and `R-58` requires a value — but a backend author cannot
  /// discover it from either rule.
  #[test]
  fn an_untouched_field_submits_what_canon_delta_cd_1_states() {
    assert_eq!(
      submitted(&as_drawn(&DrawnKind::Boolean)),
      serde_json::json!(false)
    );
    assert_eq!(
      submitted(&as_drawn(&DrawnKind::Text)),
      serde_json::json!("")
    );
    assert_eq!(
      submitted(&as_drawn(&a_number(Some(3.0), None))),
      serde_json::json!(3.0)
    );
    assert_eq!(
      submitted(&as_drawn(&a_choice())),
      serde_json::json!("first")
    );
    assert_eq!(
      submitted(&as_drawn(&a_number(None, Some(-10.0)))),
      serde_json::json!(0.0),
      "a `max`-only range submits zero, which may exceed that maximum"
    );
    assert_eq!(
      submitted(&as_drawn(&DrawnKind::DateTime)),
      serde_json::json!("1970-01-01T00:00:00+00:00"),
      "the epoch, with an explicit offset rather than `Z`"
    );
  }

  /// PHASE-02/VT-4, the in-kind half. Written beside the three refusals below
  /// so that neither half can pass by answering the same thing to everything
  /// (`docs/memory/a-green-test-can-assert-a-proxy.md`).
  #[test]
  fn an_in_kind_report_becomes_what_the_draft_holds() {
    assert_eq!(
      interpret(&Reported::Checked(true), None, &DrawnKind::Boolean),
      Some(Edited::Checked(true))
    );
    assert_eq!(
      interpret(&Reported::Typed("typed".to_owned()), None, &DrawnKind::Text),
      Some(Edited::Typed("typed".to_owned()))
    );
    assert_eq!(
      interpret(
        &Reported::AdjustedText("1.5".to_owned()),
        None,
        &a_number(None, None)
      ),
      Some(Edited::Adjusted {
        number: finite(1.5),
        text: "1.5".to_owned(),
      })
    );
    assert_eq!(
      interpret(&Reported::AdjustedValue(1.5), None, &a_number(None, None)),
      Some(Edited::Adjusted {
        number: finite(1.5),
        text: "1.5".to_owned(),
      }),
      "a slider has nothing to display, so the host's own spelling is the text"
    );
    let DrawnKind::Choice {
      alternatives: declared,
      ..
    } = a_choice()
    else {
      panic!("the fixture is a choice");
    };
    assert_eq!(
      interpret(&Reported::Chosen(1), None, &a_choice()),
      Some(Edited::Chosen(declared.as_slice()[1].id().clone())),
      "the index is interpreted against the drawn field's alternatives"
    );
    assert_eq!(
      interpret(
        &Reported::Picked {
          instant: epoch(),
          offset: Offset::constant(-5),
        },
        None,
        &DrawnKind::DateTime
      ),
      Some(Edited::Picked {
        instant: epoch(),
        offset: Offset::constant(-5),
      })
    );
  }

  /// PHASE-02/VT-4, `None` case 1 — a `Chosen` index no alternative of the
  /// drawn field has. A renderer bug, and it takes the
  /// `Refused::UnknownField` posture: reported, nothing recorded.
  #[test]
  fn a_chosen_index_no_alternative_has_is_refused() {
    assert_eq!(interpret(&Reported::Chosen(2), None, &a_choice()), None);
    assert_eq!(
      interpret(&Reported::Chosen(u32::MAX), None, &a_choice()),
      None
    );
    assert!(
      interpret(&Reported::Chosen(1), None, &a_choice()).is_some(),
      "the last in-range index is accepted, so the refusals above are about \
       the range and not about the walk"
    );
  }

  /// PHASE-02/VT-4, `None` case 2 — a `Slider` reporting a number that is not
  /// finite. `AdjustedValue` is an `f32` and admits `NaN` and both infinities
  /// like any other; §5.5 I-G is held by `interpret` here and by `Finite` at
  /// the wire, and by neither boundary type.
  #[test]
  fn a_slider_reporting_a_non_finite_number_is_refused() {
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
      assert_eq!(
        interpret(&Reported::AdjustedValue(value), None, &a_number(None, None)),
        None
      );
    }
  }

  /// PHASE-02/VT-4, `None` case 3 — **all twenty-four** of the pairs the other
  /// two cases do not cover.
  ///
  /// The signature admits every pair: six reports against five kinds is
  /// thirty, of which six are in-kind. Enumerated rather than sampled, because
  /// two of the twenty-four look covered and are not — a mismatched `Chosen`
  /// falls into case 1 only by a coincidence of spelling, and a mismatched
  /// `AdjustedText` is not an exception to *the text is recorded verbatim,
  /// always*, which is a rule about an **in-kind** report.
  #[test]
  fn a_report_whose_variant_is_not_the_drawn_kind_is_refused() {
    let reports = [
      Reported::Checked(true),
      Reported::Typed("typed".to_owned()),
      Reported::AdjustedText("1.5".to_owned()),
      Reported::AdjustedValue(1.5),
      Reported::Chosen(0),
      Reported::Picked {
        instant: epoch(),
        offset: Offset::UTC,
      },
    ];
    let kinds = [
      DrawnKind::Boolean,
      DrawnKind::Text,
      a_number(None, None),
      a_choice(),
      DrawnKind::DateTime,
    ];

    let mut refused = 0;
    for report in &reports {
      for kind in &kinds {
        let in_kind = matches!(
          (report, kind),
          (Reported::Checked(_), DrawnKind::Boolean)
            | (Reported::Typed(_), DrawnKind::Text)
            | (
              Reported::AdjustedText(_) | Reported::AdjustedValue(_),
              DrawnKind::Number(_)
            )
            | (Reported::Chosen(_), DrawnKind::Choice { .. })
            | (Reported::Picked { .. }, DrawnKind::DateTime)
        );
        if in_kind {
          continue;
        }
        assert_eq!(
          interpret(report, None, kind),
          None,
          "{report:?} against a drawn {kind:?}"
        );
        refused += 1;
      }
    }
    assert_eq!(
      refused, 24,
      "six reports against five kinds is thirty, of which six are in kind"
    );
  }

  /// PHASE-02/VT-4 — **the text is recorded verbatim, always; the number is
  /// replaced only where the parse yields a finite `f64`.** One rule, holding
  /// two properties no choice of comparand could hold on its own: nothing
  /// non-finite reaches the wire, and nothing is written back over a person
  /// mid-entry (§5.2, §7 D23).
  ///
  /// The three groups are the three classes `prototype-notes.md` P-7 found:
  /// texts no parse accepts (the control's own two-byte escape), texts a parse
  /// accepts **non-finitely** (reachable by pasting), and texts the control
  /// never validated at all.
  #[test]
  fn a_numeric_text_no_finite_parse_accepts_keeps_the_number_the_field_held() {
    let held = Edited::Adjusted {
      number: finite(7.25),
      text: "7.25".to_owned(),
    };

    for text in ["-", ".", "-.", "inf", "nan", "1e400", "12/25", "", "$5"] {
      assert_eq!(
        interpret(
          &Reported::AdjustedText(text.to_owned()),
          Some(&held),
          &a_number(None, None)
        ),
        Some(Edited::Adjusted {
          number: finite(7.25),
          text: text.to_owned(),
        }),
        "`{text}` is recorded verbatim and the last representable number stands"
      );
    }

    assert_eq!(
      interpret(
        &Reported::AdjustedText("1e40".to_owned()),
        Some(&held),
        &a_number(None, None)
      ),
      Some(Edited::Adjusted {
        number: finite(1e40),
        text: "1e40".to_owned(),
      }),
      "a text that does parse finitely replaces the number, so the cases \
       above are about the parse and not about the arm"
    );
  }

  /// PHASE-02/VT-4 — where the field is untouched, the number a refused text
  /// falls back to is the one it was **drawn** showing. `interpret` applies
  /// that rule itself, so no caller writes
  /// `state_of(…).unwrap_or_else(|| as_drawn(kind))`.
  #[test]
  fn an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing() {
    assert_eq!(
      interpret(
        &Reported::AdjustedText("-".to_owned()),
        None,
        &a_number(Some(3.5), None)
      ),
      Some(Edited::Adjusted {
        number: finite(3.5),
        text: "-".to_owned(),
      })
    );
    assert_eq!(
      interpret(
        &Reported::AdjustedText("-".to_owned()),
        None,
        &a_number(None, None)
      ),
      Some(Edited::Adjusted {
        number: Finite::ZERO,
        text: "-".to_owned(),
      })
    );
  }

  /// PHASE-02/VT-5 — the formatter, and the round trip the guard depends on
  /// (§5.2, §7 D32).
  ///
  /// `f64`'s `Display` up to 24 characters and `{:e}` beyond it. The trigger
  /// is length because the defect is length: `min: f64::MAX` is a legal `R-17`
  /// bound whose `Display` is 309 characters in a `LineEdit`.
  #[test]
  fn a_number_spells_short_and_re_parses_to_the_number_it_came_from() {
    assert_eq!(spelled(1.5), "1.5");
    assert_eq!(spelled(0.0), "0");
    assert_eq!(spelled(-3.25), "-3.25");
    assert_eq!(
      spelled(f64::MAX),
      "1.7976931348623157e308",
      "309 characters under `Display`, and this is what the field shows instead"
    );
    assert_eq!(spelled(f64::MIN_POSITIVE), "2.2250738585072014e-308");

    // The threshold itself, from both sides. `1e23` spells 24 characters
    // plain and stays plain; `1e24` spells 25 and switches. Without this pair
    // a formatter that always used `{:e}` — or never did — would pass every
    // other assertion in this case that is about length alone.
    assert_eq!(spelled(1e23), "100000000000000000000000");
    assert_eq!(spelled(1e24), "1e24");

    for number in [
      0.0,
      1.5,
      -3.25,
      1e40,
      f64::MAX,
      f64::MIN,
      f64::MIN_POSITIVE,
      -1.234_567_890_123_456_7e-5,
    ] {
      let spelling = spelled(number);
      assert!(
        spelling.len() <= 24,
        "`{spelling}` is {} characters",
        spelling.len()
      );
      assert_eq!(
        spelling.parse::<f64>().ok(),
        Some(number),
        "`{spelling}` must re-parse to the number it came from"
      );
    }
  }

  /// A range built straight, for the one function that takes a `NumberRange`
  /// rather than a `DrawnKind`.
  fn bounds(min: f64, max: f64) -> NumberRange {
    NumberRange::new(Some(min), Some(max)).expect("the fixture's bounds are legal")
  }

  /// Slice 009 `plan.md` PHASE-08/**VT-1** — the three clauses of
  /// `slider_bounds`, one case each, and an ordinary range that passes all
  /// three.
  ///
  /// Each of the three is a range a **backend may legally send**:
  /// `NumberRange::new` refuses a non-finite bound and `min > max`, and admits
  /// `min == max`. So none of these is a renderer-only construction.
  ///
  /// The clauses are not interchangeable and the case is written so that a
  /// missing one shows. Deleting the **span** clause leaves
  /// `[-f32::MAX, f32::MAX]` admitted, because an infinite step *does* move
  /// the value in both directions. Deleting the **move** clause leaves the
  /// `2^100` range admitted, because its span is finite and strictly
  /// positive. That is what *the third clause subsumes the positivity test*
  /// means and does not mean: it replaces a `step > 0` test, and it does not
  /// replace the span test above it.
  #[test]
  fn slider_bounds_admits_only_a_range_a_slider_can_be_operated_over() {
    assert_eq!(
      slider_bounds(&bounds(0.0, 10.0)),
      Some((0.0, 10.0)),
      "an ordinary range is a slider"
    );

    assert_eq!(
      slider_bounds(&NumberRange::new(Some(0.0), None).expect("one bound is legal")),
      None,
      "a range missing a bound has no span to divide by, so there is no slider to draw"
    );

    assert_eq!(
      slider_bounds(&bounds(0.1, 10.0)),
      None,
      "`0.1` is not an `f32`, so the thumb's own arithmetic would run over a \
       minimum the backend never sent — and the span here is finite, positive \
       and moved by its step, so no later clause catches it"
    );

    assert_eq!(
      slider_bounds(&bounds(1.0, 1.0)),
      None,
      "equal bounds divide by zero where the thumb is placed        (`fluent/slider.slint:75`), and a slider over a single value offers nothing"
    );

    let edge = f64::from(f32::MAX);
    assert_eq!(
      slider_bounds(&bounds(-edge, edge)),
      None,
      "both endpoints round-trip exactly and the span is still an `f32` infinity"
    );

    // `2^100` is exact in `f32` and the next `f32` above it is `2^100 + 2^77`,
    // so this range round-trips and its span is one ulp. A hundredth of one
    // ulp is below half an ulp, so `minimum + step` rounds back to `minimum`
    // and `increment()` — which is exactly `set-value(value + step)`, and
    // `set-value` returns when the result equals the value it holds
    // (`common/slider-base.slint:117-131`) — moves nothing. The slider would
    // be frozen at every magnitude the range covers.
    let base = 2f64.powi(100);
    let ulp = 2f64.powi(77);
    assert_eq!(
      slider_bounds(&bounds(base, base + ulp)),
      None,
      "a finite, strictly positive step can still be too small to move the value"
    );
  }

  /// Slice 009 PHASE-08/VT-1's other half: the narrowing that guards the first
  /// clause is a **checked** conversion and not a cast.
  #[test]
  fn only_an_f64_that_survives_the_round_trip_crosses_as_a_float() {
    assert_eq!(exact_f32(0.0), Some(0.0));
    assert_eq!(exact_f32(10.5), Some(10.5));
    assert_eq!(exact_f32(f64::from(f32::MAX)), Some(f32::MAX));
    assert_eq!(
      exact_f32(1e100),
      None,
      "a legal `R-17` bound `f32` reads as infinity"
    );
    assert_eq!(
      exact_f32(0.1),
      None,
      "and one an `f32` can only approximate"
    );
    assert_eq!(
      exact_f32(f64::MIN_POSITIVE),
      None,
      "a subnormal `f64` flushes to zero, which is the silent half of the class"
    );
  }
}
