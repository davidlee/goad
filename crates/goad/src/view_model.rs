//! `present`, the mapper — design.md §5.2. One function, one direction, no
//! `_ =>` arm: a canonical `View` becomes a displayable `Presentation`.
//!
//! `present` is total, pure and panic-free (I-1): it never refuses a legal
//! view, and every part of it that this renderer cannot draw is degraded
//! into `undrawn` rather than dropped (I-2, R-20). Nothing here reads a
//! clock, a file or a socket — stratum 3, but the pure half of it.

use std::str::FromStr;

use goad_semantics::protocol::canonical::{
  Alternative, AlternativeId, Alternatives, Content, Field, FieldId, FieldKind, NumberRange, Opt,
  OptionId, Timestamp, View,
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
  /// has one statement. Note what it excludes: `Undrawn::FieldForm` and
  /// `Undrawn::GroupHint` are undrawn but say nothing about the body.
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
/// with no heading — an ungrouped run, or a `group` the backend sent empty —
/// never a missing block.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldBlock {
  pub heading: Option<String>,
  pub fields: Vec<PresentationField>,
}

/// One drawn field: the id that answers it and the label beside it. The
/// person's own state is **not** here and never will be — it lives in
/// `draft.rs`, which is what keeps this value the backend's and that one the
/// person's (design.md §5.3, I-4).
///
/// `kind` is the renderer's own [`DrawnKind`], not the canonical one: it
/// carries exactly the per-kind data a control needs, and carrying it here is
/// what lets `as_drawn` and `resolve` be reached from the walk `answer` and
/// `edit` already make.
#[derive(Debug, Clone, PartialEq)]
pub struct PresentationField {
  pub id: FieldId,
  pub label: String,
  pub kind: DrawnKind,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldForm {
  Text,
  DateTime,
  Number,
  Choice,
}

impl std::fmt::Display for FieldForm {
  /// The backend's own word for the kind, so the line a person reads names
  /// the value a backend author would search their own view for.
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    formatter.write_str(match self {
      Self::Text => "text",
      Self::DateTime => "datetime",
      Self::Number => "number",
      Self::Choice => "choice",
    })
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

/// The per-kind data a drawn field's controls need.
///
/// Host-local, and deliberately **not** the canonical `FieldKind` cloned.
/// `FieldKind` is what the protocol admits; this is what this renderer draws.
/// The only way to make one is [`drawn_kind`]'s exhaustive match, so a sixth
/// protocol kind cannot reach a drawn field without that match being amended
/// — the same guarantee [`FieldForm`] gives in the other direction (D10).
///
/// Every variant exists and every variant is answered for; only `Boolean` is
/// *produced* so far, because the markup draws one control. Each later phase
/// moves one arm of `drawn_kind` from `Err` to `Ok` and removes the matching
/// [`FieldForm`], which is why that enum is not yet uninhabited.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawnKind {
  Boolean,
  Text,
  Number(NumberRange),
  /// The alternatives in declared order — the `ComboBox`'s labels, and what
  /// [`resolve`] resolves a reported index against — beside the first of
  /// them, which is what an untouched `choice` submits.
  ///
  /// `first` is carried rather than looked up because `Alternatives::new`
  /// rejects an empty list (`canonical.rs:362`) and the compiler cannot see
  /// that at [`as_drawn`] — where there is no fallback to write either, since
  /// `AlternativeId::new` is `pub(super)` and this crate cannot mint one. One
  /// clone where the kind is built is what makes `as_drawn` and the display
  /// path total without a lint exception anywhere
  /// (`prototype-notes.md` P-2).
  Choice {
    first: AlternativeId,
    alternatives: Alternatives,
  },
  DateTime,
}

/// The kind this renderer draws a field as, or the form it reports for one it
/// does not.
///
/// Matching `FieldKind` exhaustively with no `_` arm is what makes a sixth
/// protocol kind a compile error here rather than a field silently dropped —
/// and this is the site that breaks for it, not `draft.rs`'s `submitted`,
/// whose match is over a host-local type. `Err` is not a failure; it is the
/// report.
fn drawn_kind(kind: &FieldKind) -> Result<DrawnKind, FieldForm> {
  match kind {
    FieldKind::Boolean => Ok(DrawnKind::Boolean),
    FieldKind::Text => Err(FieldForm::Text),
    FieldKind::DateTime => Err(FieldForm::DateTime),
    FieldKind::Number(_) => Err(FieldForm::Number),
    // P3: this becomes `Ok(DrawnKind::Choice { .. })` when a `ComboBox` is
    // drawn. The arm that lands here needs the first alternative, and
    // `Alternatives::new`'s non-empty guarantee is not visible to the
    // compiler — so it answers `Err(FieldForm::Choice)` for an empty list
    // as well as for an undrawn one, which puts the one case the guarantee
    // does not cover where this renderer already has somewhere to report it.
    FieldKind::Choice { .. } => Err(FieldForm::Choice),
  }
}

/// What the wire answers for a field nobody has touched, following P-3: the
/// as-drawn value is what the widget shows.
///
/// Applied by `controller.rs::answer`, because `SPEC-001/R-58` forbids
/// omitting a value for a drawn field, and by [`resolve`] below. Not by
/// `glass.rs`: for four of the five kinds the drawn value and the as-drawn
/// value coincide by construction, and for `datetime` they do not — a button
/// reading `1970-01-01T00:00:00+00:00` would be the host showing a person an
/// answer nobody gave, so the button reads *not set* and the epoch is a fact
/// about the wire alone (D-6, design.md §5.2).
#[must_use]
pub fn as_drawn(kind: &DrawnKind) -> Edited {
  match kind {
    DrawnKind::Boolean => Edited::Checked(false),
    DrawnKind::Text => Edited::Typed(String::new()),
    // The declared minimum where one was given, otherwise zero: the widget is
    // drawn showing that number, so the screen and the wire agree. `R-17`
    // already guarantees a declared bound is finite, so `Finite::new` cannot
    // refuse one — it is still the constructor that is called, because a
    // total expression is cheaper than an argument about why an `expect` is
    // unreachable. A range carrying only a `max` therefore submits `0`, which
    // may exceed it; that is `R-35`'s judgement to make, not this host's, and
    // `canon-delta.md` CD-1 states it.
    DrawnKind::Number(range) => {
      let number = range.min().and_then(Finite::new).unwrap_or(Finite::ZERO);
      Edited::Adjusted {
        text: number_text(number),
        number,
      }
    }
    DrawnKind::Choice { first, .. } => Edited::Chosen(first.clone()),
    DrawnKind::DateTime => Edited::Picked {
      instant: Timestamp::new(jiff::Timestamp::UNIX_EPOCH),
      offset: Offset::UTC,
    },
  }
}

/// What the draft should hold, given what the widget reported and what the
/// field shows now.
///
/// `shown` is [`Draft::state_of`](crate::draft::Draft::state_of)'s answer
/// passed straight through. This function has the kind, so it consults
/// [`as_drawn`] itself rather than making every caller spell
/// `state_of(..).unwrap_or_else(|| as_drawn(kind))` — which also keeps
/// `as_drawn`'s call sites in the module it lives in.
///
/// # `None`
///
/// `None` is a renderer bug in both of its cases, and there are exactly two.
/// They are named in full because a case left off this list is a case that
/// becomes an `unwrap` in someone's implementation — the same reason
/// `compose` names its four fallible steps (F-42):
///
/// 1. a [`Reported::Chosen`] index no alternative has;
/// 2. a non-finite [`Reported::AdjustedValue`]. The `Slider` reports a bare
///    `f32` because a Slint callback has nothing to report a refusal to and
///    no draft to leave alone, so I-G is held here and by `Finite` at the
///    wire — **not** by the shape of `Reported`.
///
/// Both take the `Refused::UnknownField` posture: reported, nothing recorded.
#[must_use]
pub fn resolve(reported: &Reported, shown: Option<&Edited>, kind: &DrawnKind) -> Option<Edited> {
  match reported {
    Reported::Checked(checked) => Some(Edited::Checked(*checked)),
    Reported::Typed(text) => Some(Edited::Typed(text.clone())),
    // One rule covers every text the control admits: the text is recorded
    // verbatim, always, and the number is replaced only where the parse
    // yields a finite `f64`. So `-`, `.` and `-.` — the three texts the
    // control admits that no parse accepts — leave the number alone, and
    // `1e400` stays on screen while the host keeps the number it can defend.
    // That is what keeps the guard quiet mid-entry and keeps a non-finite off
    // the wire at once (F-30, F-34).
    Reported::AdjustedText(text) => Some(Edited::Adjusted {
      number: parsed_number(text)
        .and_then(Finite::new)
        .unwrap_or_else(|| standing_number(shown, kind)),
      text: text.clone(),
    }),
    Reported::AdjustedValue(value) => {
      Finite::new(f64::from(*value)).map(|number| Edited::Adjusted {
        // Nothing displays a `Slider`'s value but the host's own formatting of
        // it, so the text is derived rather than reported.
        text: number_text(number),
        number,
      })
    }
    // The index travels and is resolved here because `AlternativeId::new` is
    // `pub(super)` in `goad-semantics`: a callback can only report what it
    // can see, and an id can only be cloned off a drawn view (D12, I-D).
    Reported::Chosen(index) => alternatives_of(kind)
      .get(usize::try_from(*index).ok()?)
      .map(|alternative| Edited::Chosen(alternative.id().clone())),
    Reported::Picked { instant, offset } => Some(Edited::Picked {
      instant: *instant,
      offset: *offset,
    }),
  }
}

/// The number a `number` field holds right now: what the draft recorded, or —
/// for a field nobody has touched — the value it was drawn showing.
///
/// The `_` arm is a report whose kind is not the field's, which is a renderer
/// bug but deliberately **not** one of `resolve`'s two `None` cases: the text
/// is still recorded verbatim either way, and zero is what an unbounded
/// untouched `number` would have held.
fn standing_number(shown: Option<&Edited>, kind: &DrawnKind) -> Finite {
  match shown.cloned().unwrap_or_else(|| as_drawn(kind)) {
    Edited::Adjusted { number, .. } => number,
    _ => Finite::ZERO,
  }
}

/// The alternatives a reported index is resolved against — empty for every
/// kind that has none, so a `Chosen` reported against a `boolean` is an index
/// no alternative has and lands in `resolve`'s first `None` case rather than
/// in a third one.
fn alternatives_of(kind: &DrawnKind) -> &[Alternative] {
  match kind {
    DrawnKind::Choice { alternatives, .. } => alternatives.as_slice(),
    _ => &[],
  }
}

/// How the host spells a number for the control that displays it.
///
/// `f64`'s own `Display` is the shortest decimal that reads back as the same
/// number, which is what a person typing it would have written. The spelling
/// is load-bearing rather than cosmetic: an untouched field is drawn showing
/// this string and submits the number beside it, so a format that lost digits
/// would show a person a different number from the one their own field would
/// answer with.
#[must_use]
pub fn number_text(number: Finite) -> String {
  number.get().to_string()
}

/// Parse a numeric `LineEdit`'s text under the rule the control validated it
/// with. `None` is a text the control admits that no parse accepts.
///
/// `input-type: decimal` validates each insertion through Slint's
/// `string_to_float`, which is **locale-aware**: it takes the locale's decimal
/// separator from ICU as an arbitrary `char`, substitutes that character, and
/// rejects a `.` outright where the separator is something else
/// (`i-slint-core/string.rs:398-412`). It is live in this build —
/// `i-slint-core`'s default `std` feature enables
/// `i-slint-common/locale-decimal-separator` — and it is not reachable from
/// host code: `SlintContext::locale_decimal_separator` is `i-slint-core`,
/// which this crate does not depend on, and the `slint` crate re-exports
/// neither it nor `string_to_float`. A host that called `f64::from_str` on the
/// raw text would refuse `1,5` in a comma-decimal locale — text the control
/// had just approved — record nothing, and let the next guard write over the
/// person.
///
/// So the host accepts the class the control admits **without knowing which
/// separator it is in**: parse the text as it stands; failing that, if the
/// text holds exactly one character outside the numeric grammar, replace that
/// one character with `.` and parse again. Empty text is zero, which is what
/// Slint's own `to-float` reads an empty field as — and that is what makes the
/// guard's one exception correspond to a value the host actually holds.
///
/// All of it is host-side and testable without a locale fixture (D-16, and
/// F-26, which widened D-16's comma-only rule to the class).
fn parsed_number(text: &str) -> Option<f64> {
  if text.is_empty() {
    return Some(0.0);
  }
  if let Ok(number) = f64::from_str(text) {
    return Some(number);
  }

  let mut foreign = text
    .char_indices()
    .filter(|(_, character)| !is_numeric_grammar(*character));
  let (at, separator) = foreign.next()?;
  if foreign.next().is_some() {
    return None;
  }

  let mut substituted = String::with_capacity(text.len());
  substituted.push_str(text.get(..at)?);
  substituted.push('.');
  substituted.push_str(text.get(at.checked_add(separator.len_utf8())?..)?);
  f64::from_str(&substituted).ok()
}

/// The characters `f64::from_str` reads as part of a *number*. Deliberately
/// not its whole grammar: `inf`, `infinity` and `nan` are spellings it also
/// accepts, and they parse at the first attempt, so admitting their letters
/// here would only widen what counts as a separator.
fn is_numeric_grammar(character: char) -> bool {
  character.is_ascii_digit() || matches!(character, '+' | '-' | '.' | 'e' | 'E')
}

/// The bounds a `Slider` may be drawn over, or `None` for the numeric text
/// control. The only place a `number`'s control is chosen — `slider` is a
/// decision the row carries, not a fact the markup reasons from.
///
/// `Some` when all three hold, and `None` otherwise:
///
/// - both bounds are present, and each round-trips `f64` -> `f32` -> `f64`
///   unchanged;
/// - the `f32` span `maximum - minimum` is finite and strictly positive;
/// - `minimum + step` exceeds `minimum` and `maximum - step` falls below
///   `maximum`, both evaluated in `f32`, where `step` is that span's
///   hundredth.
///
/// An exact endpoint round-trip on its own is not enough, because Slint's
/// slider arithmetic is what has to work afterwards. The thumb is placed by
/// dividing by `maximum - minimum` (`fluent/slider.slint:75-76`), so equal
/// bounds such as `[1, 1]` divide by zero and `[-f32::MAX, f32::MAX]` has an
/// infinite span even though both its endpoints are exact — the span clause is
/// what makes the division safe to perform at all.
///
/// The third clause is *Slint's arithmetic has to work afterwards*, and it
/// subsumes the "finite and strictly positive step" it replaced (F-20): a step
/// that underflows to zero fails it, and so does a non-finite one. What it
/// additionally rejects is `[2^100, 2^100 + 2^77]`, whose step is finite,
/// positive, and below half an ulp at `minimum`, so `minimum + step` rounds
/// back to `minimum`. `increment()` is exactly
/// `root.set-value(root.value + root.step)`
/// (`widgets/common/slider-base.slint:126-131`), so that slider is drawable
/// and frozen. Positivity does not establish operability.
///
/// Nothing this rejects is a loss: each case takes the text control, which is
/// where a range a slider cannot operate belongs anyway. So the text control
/// is the one that always works and the `Slider` is the one with an
/// admissibility condition, which is the opposite of how a bounds-driven
/// reading makes it look. A later slice that wants the choice to read a hint
/// (`R-18` permits the renderer, and only the renderer, to branch on one)
/// changes this body and nothing else: not the markup, not the wire, not
/// `Edited`.
#[expect(
  clippy::as_conversions,
  clippy::cast_possible_truncation,
  clippy::float_cmp,
  reason = "narrowing `f64` to `f32` and comparing the result for exact \
            equality is the operation this function exists to perform: the \
            first clause round-trips each bound and refuses any the narrowing \
            changed. `std` has no checked `f64` -> `f32`, so the cast is the \
            only spelling there is, and the exception sits on the one function \
            whose whole body is the check."
)]
#[must_use]
pub fn slider_bounds(range: &NumberRange) -> Option<(f32, f32)> {
  let (minimum, maximum) = (range.min()?, range.max()?);
  let (low, high) = (minimum as f32, maximum as f32);
  if f64::from(low) != minimum || f64::from(high) != maximum {
    return None;
  }

  let span = high - low;
  if !span.is_finite() || span <= 0.0 {
    return None;
  }

  let step = span / 100.0;
  (low + step > low && high - step < high).then_some((low, high))
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

    match drawn_kind(field.kind()) {
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

#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::{NumberRange, Timestamp};
  use jiff::tz::Offset;

  use super::{
    DrawnKind, Edited, Finite, Reported, as_drawn, number_text, parsed_number, resolve,
    slider_bounds,
  };
  use crate::fixture::{alternative_id, alternatives};

  fn range(min: Option<f64>, max: Option<f64>) -> NumberRange {
    NumberRange::new(min, max).expect("the fixture must be a legal range")
  }

  fn finite(value: f64) -> Finite {
    Finite::new(value).expect("the fixture must be finite")
  }

  fn choice(ids: &[&str]) -> DrawnKind {
    DrawnKind::Choice {
      first: alternative_id(ids, 0),
      alternatives: alternatives(ids),
    }
  }

  // --- slider_bounds -----------------------------------------------------
  //
  // Every rejection below draws the numeric text control instead, so none of
  // them loses a legal `R-17` range — what is being asserted is which control
  // is chosen, not whether the field is answerable.

  #[test]
  fn an_ordinary_range_is_drawn_as_a_slider() {
    assert_eq!(
      slider_bounds(&range(Some(0.0), Some(100.0))),
      Some((0.0, 100.0))
    );
  }

  /// A range needs **both** bounds before a slider has anything to span.
  #[test]
  fn a_range_missing_a_bound_takes_the_text_control() {
    assert_eq!(slider_bounds(&range(Some(0.0), None)), None);
    assert_eq!(slider_bounds(&range(None, Some(100.0))), None);
    assert_eq!(slider_bounds(&range(None, None)), None);
  }

  /// First clause. `1e100` is a legal `R-17` bound and is not an `f32`, so
  /// narrowing it would change the range the person is shown from the range
  /// the backend declared — which is the narrowing-by-type that §5.2 exists
  /// to refuse.
  #[test]
  fn a_bound_that_does_not_round_trip_through_f32_takes_the_text_control() {
    assert_eq!(slider_bounds(&range(Some(0.0), Some(1e100))), None);
    assert_eq!(slider_bounds(&range(Some(-1e100), Some(0.0))), None);
  }

  /// Second clause, both halves. Slint places the thumb by dividing by the
  /// span (`fluent/slider.slint:75-76`), so `[1, 1]` divides by zero, and
  /// `[-f32::MAX, f32::MAX]` overflows the span to infinity even though both
  /// endpoints are exact.
  #[test]
  fn a_span_that_is_not_finite_and_positive_takes_the_text_control() {
    assert_eq!(slider_bounds(&range(Some(1.0), Some(1.0))), None);
    assert_eq!(
      slider_bounds(&range(
        Some(f64::from(-f32::MAX)),
        Some(f64::from(f32::MAX))
      )),
      None
    );
  }

  /// Third clause, the underflow half — the case the clause it replaced also
  /// caught. The span is the smallest positive `f32` there is, so its
  /// hundredth is below the smallest subnormal and rounds to zero, which
  /// `SliderBase` answers by rejecting every key
  /// (`common/slider-base.slint:79-80`).
  #[test]
  fn a_span_whose_hundredth_underflows_takes_the_text_control() {
    let smallest = f64::from(f32::from_bits(1));
    assert_eq!(slider_bounds(&range(Some(0.0), Some(smallest))), None);
  }

  /// Third clause, the half that is new (F-20). Both endpoints round-trip,
  /// the span is finite and positive, and the step is finite and positive —
  /// so the predicate this replaced admitted it — but the step is below half
  /// an ulp at `minimum`, so `minimum + step` rounds back to `minimum` and
  /// `increment()` freezes the thumb. Positivity does not establish
  /// operability.
  #[test]
  fn a_step_too_small_to_move_the_thumb_takes_the_text_control() {
    let minimum = 2.0_f64.powi(100);
    let maximum = minimum + 2.0_f64.powi(77);
    assert_eq!(
      slider_bounds(&range(Some(minimum), Some(maximum))),
      None,
      "both bounds are exact `f32`s and the step is positive; it is still inoperable"
    );
  }

  // --- the number parse rule ---------------------------------------------

  /// What Slint's own `to-float` reads an empty field as, and what makes the
  /// guard's cleared-field exception correspond to a value the host holds.
  #[test]
  fn empty_text_parses_as_zero() {
    assert_eq!(parsed_number(""), Some(0.0));
  }

  /// The rule the control validated the text with, in the two locales that
  /// matter: the separator may be `.` or may be something else, and the host
  /// accepts the class without knowing which it is in.
  #[test]
  fn a_single_foreign_character_is_read_as_the_decimal_separator() {
    assert_eq!(parsed_number("1.5"), Some(1.5));
    assert_eq!(parsed_number("1,5"), Some(1.5));
    assert_eq!(parsed_number("-1,5"), Some(-1.5));
  }

  /// A group separator beside a decimal point is **not** a decimal
  /// separator, and the rule answers `None` rather than guessing: the
  /// substitution yields `1.000.5`, which no parse accepts. The caller's rule
  /// then records the text verbatim and keeps the number it had.
  #[test]
  fn a_grouped_number_is_not_parsed() {
    assert_eq!(parsed_number("1,000.5"), None);
    assert_eq!(
      parsed_number("1,000,5"),
      None,
      "two foreign characters are not one separator"
    );
  }

  /// The three texts `input-type: decimal` admits that no parse accepts —
  /// allowed as `len <= 2` starts so a person can begin typing a negative or
  /// a fractional number at all (`items/text.rs:2211-2226`).
  #[test]
  fn the_three_admitted_non_numbers_do_not_parse() {
    for text in ["-", ".", "-."] {
      assert_eq!(parsed_number(text), None, "`{text}` parses to no number");
    }
  }

  /// It parses, and it is not a number the host may hold: `Finite` is where
  /// that is refused, one step later, so the text still reaches the widget
  /// verbatim.
  #[test]
  fn an_overflowing_text_parses_to_an_infinity() {
    assert_eq!(parsed_number("1e400"), Some(f64::INFINITY));
    assert_eq!(parsed_number("1e400").and_then(Finite::new), None);
  }

  // --- as_drawn ----------------------------------------------------------

  #[test]
  fn an_untouched_boolean_is_drawn_unticked() {
    assert_eq!(as_drawn(&DrawnKind::Boolean), Edited::Checked(false));
  }

  #[test]
  fn an_untouched_text_is_drawn_empty() {
    assert_eq!(as_drawn(&DrawnKind::Text), Edited::Typed(String::new()));
  }

  /// The declared minimum where there is one: the widget is drawn showing
  /// that number, so the screen and the wire agree (P-3).
  #[test]
  fn an_untouched_number_is_drawn_at_its_declared_minimum() {
    assert_eq!(
      as_drawn(&DrawnKind::Number(range(Some(2.5), Some(10.0)))),
      Edited::Adjusted {
        number: finite(2.5),
        text: "2.5".to_owned()
      }
    );
  }

  /// A range carrying only a `max` is legal (`R-17`), and zero is what such a
  /// field submits untouched — possibly above that `max`. `R-35` puts that
  /// judgement in the backend and `R-58` requires a value, so this is a
  /// consequence rather than a defect (`canon-delta.md` CD-1).
  #[test]
  fn an_untouched_number_with_no_minimum_is_drawn_at_zero() {
    assert_eq!(
      as_drawn(&DrawnKind::Number(range(None, Some(-10.0)))),
      Edited::Adjusted {
        number: Finite::ZERO,
        text: "0".to_owned()
      }
    );
  }

  /// The first alternative, always defined because `Alternatives::new`
  /// rejects an empty list.
  #[test]
  fn an_untouched_choice_is_drawn_at_its_first_alternative() {
    assert_eq!(
      as_drawn(&choice(&["red", "green"])),
      Edited::Chosen(alternative_id(&["red", "green"], 0))
    );
  }

  /// The epoch — a fact about the wire alone. The button reads *not set*,
  /// which is why `glass.rs` reads `state_of`'s `None` directly instead of
  /// applying this (D-6).
  #[test]
  fn an_untouched_datetime_is_the_epoch() {
    assert_eq!(
      as_drawn(&DrawnKind::DateTime),
      Edited::Picked {
        instant: Timestamp::new(jiff::Timestamp::UNIX_EPOCH),
        offset: Offset::UTC
      }
    );
  }

  // --- resolve -----------------------------------------------------------

  /// `resolve`'s first `None` case, named in full on the function: an index
  /// no alternative has. A renderer bug, and it takes the
  /// `Refused::UnknownField` posture — nothing is recorded.
  #[test]
  fn an_index_no_alternative_has_resolves_to_nothing() {
    let kind = choice(&["red", "green"]);
    assert_eq!(resolve(&Reported::Chosen(2), None, &kind), None);
    assert!(
      resolve(&Reported::Chosen(1), None, &kind).is_some(),
      "the last declared index is in range; the bug is the one past it"
    );
  }

  /// `resolve`'s second `None` case. `Reported::AdjustedValue` is a bare
  /// `f32` and can express all three non-finites, so I-G is held **here**
  /// rather than by the shape of `Reported` (F-42).
  #[test]
  fn a_non_finite_slider_value_resolves_to_nothing() {
    let kind = DrawnKind::Number(range(Some(0.0), Some(1.0)));
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
      assert_eq!(
        resolve(&Reported::AdjustedValue(value), None, &kind),
        None,
        "a `Slider` reporting {value} is a renderer bug, not a recorded number"
      );
    }
  }

  /// A finite one resolves, and its text is the host's own spelling: nothing
  /// displays a `Slider`'s value but that.
  #[test]
  fn a_finite_slider_value_resolves_to_the_number_and_its_spelling() {
    let kind = DrawnKind::Number(range(Some(0.0), Some(1.0)));
    assert_eq!(
      resolve(&Reported::AdjustedValue(0.25), None, &kind),
      Some(Edited::Adjusted {
        number: finite(0.25),
        text: "0.25".to_owned()
      })
    );
  }

  /// The rule that covers every text the control admits: the text is
  /// recorded **verbatim, always**, and the number is replaced only where the
  /// parse yields a finite `f64`.
  #[test]
  fn a_number_text_that_does_not_parse_keeps_the_number_the_field_had() {
    let kind = DrawnKind::Number(range(Some(0.0), Some(100.0)));
    let held = Edited::Adjusted {
      number: finite(42.0),
      text: "42".to_owned(),
    };

    assert_eq!(
      resolve(&Reported::AdjustedText("-".to_owned()), Some(&held), &kind),
      Some(Edited::Adjusted {
        number: finite(42.0),
        text: "-".to_owned()
      }),
      "the person keeps what they typed and the host keeps the number it can defend"
    );
    assert_eq!(
      resolve(
        &Reported::AdjustedText("1e400".to_owned()),
        Some(&held),
        &kind
      ),
      Some(Edited::Adjusted {
        number: finite(42.0),
        text: "1e400".to_owned()
      }),
      "an overflow parses but is not finite, so the last representable number stands"
    );
  }

  /// The same rule for a field nobody has touched: `shown` is `None` and the
  /// number it falls back to is the one the widget was drawn showing, which
  /// is what `as_drawn` says. This is why `resolve` takes the `Option` rather
  /// than making both callers spell the fallback.
  #[test]
  fn an_untouched_number_falls_back_to_what_it_was_drawn_showing() {
    let kind = DrawnKind::Number(range(Some(7.0), Some(100.0)));
    assert_eq!(
      resolve(&Reported::AdjustedText("-".to_owned()), None, &kind),
      Some(Edited::Adjusted {
        number: finite(7.0),
        text: "-".to_owned()
      })
    );
  }

  /// A text that does parse replaces the number, and still records the text
  /// verbatim — `1,5` is kept as typed rather than re-spelled as `1.5`, so
  /// the guard's string comparison is an identity.
  #[test]
  fn a_number_text_that_parses_replaces_the_number_and_keeps_the_text() {
    let kind = DrawnKind::Number(range(Some(0.0), Some(100.0)));
    assert_eq!(
      resolve(&Reported::AdjustedText("1,5".to_owned()), None, &kind),
      Some(Edited::Adjusted {
        number: finite(1.5),
        text: "1,5".to_owned()
      })
    );
  }

  /// The three reports that carry their own answer whole.
  #[test]
  fn the_three_reports_that_need_no_resolution_pass_straight_through() {
    assert_eq!(
      resolve(&Reported::Checked(true), None, &DrawnKind::Boolean),
      Some(Edited::Checked(true))
    );
    assert_eq!(
      resolve(&Reported::Typed("x".to_owned()), None, &DrawnKind::Text),
      Some(Edited::Typed("x".to_owned()))
    );
    let instant = Timestamp::new(jiff::Timestamp::UNIX_EPOCH);
    assert_eq!(
      resolve(
        &Reported::Picked {
          instant,
          offset: Offset::constant(-5)
        },
        None,
        &DrawnKind::DateTime
      ),
      Some(Edited::Picked {
        instant,
        offset: Offset::constant(-5)
      })
    );
  }

  /// An index resolves against the drawn field's alternatives, and the value
  /// recorded is the **id** — the label and the index both stay behind.
  #[test]
  fn a_reported_index_resolves_to_the_alternative_it_names() {
    assert_eq!(
      resolve(&Reported::Chosen(1), None, &choice(&["red", "green"])),
      Some(Edited::Chosen(alternative_id(&["red", "green"], 1)))
    );
  }

  /// The spelling an untouched field is drawn with and the number it submits
  /// are the same value, so the format must not lose digits.
  #[test]
  fn a_number_is_spelled_so_it_reads_back_as_itself() {
    for value in [0.0, 1.5, -0.25, 1e40, f64::MIN_POSITIVE] {
      assert_eq!(
        parsed_number(&number_text(finite(value))),
        Some(value),
        "{value} must survive its own spelling"
      );
    }
  }
}
