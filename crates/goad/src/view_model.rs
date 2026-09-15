//! `present`, the mapper — design.md §5.2. One function, one direction, no
//! `_ =>` arm: a canonical `View` becomes a displayable `Presentation`.
//!
//! `present` is total, pure and panic-free (I-1): it never refuses a legal
//! view, and every part of it that this renderer cannot draw is degraded
//! into `undrawn` rather than dropped (I-2, R-20). Nothing here reads a
//! clock, a file or a socket — stratum 3, but the pure half of it.

use goad_semantics::protocol::canonical::{
  Content, Field, FieldId, FieldKind, Opt, OptionId, View,
};
use slint::StyledText;

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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldBlock {
  pub heading: Option<String>,
  pub fields: Vec<PresentationField>,
}

/// One drawn field: the id that answers it and the label beside it. The
/// person's own state is **not** here and never will be — it lives in
/// `draft.rs`, which is what keeps this value the backend's and that one the
/// person's (design.md §5.3, I-4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationField {
  pub id: FieldId,
  pub label: String,
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

/// The form of a field this renderer does not draw, or `None` for one it
/// does. Matching `FieldKind` exhaustively is what makes a sixth kind a
/// compile error here rather than a field silently dropped — and it is the
/// site that breaks for that, not `draft.rs`'s `submitted`, whose match is
/// over a host-local type.
fn undrawn_form(kind: &FieldKind) -> Option<FieldForm> {
  match kind {
    FieldKind::Boolean => None,
    FieldKind::Text => Some(FieldForm::Text),
    FieldKind::DateTime => Some(FieldForm::DateTime),
    FieldKind::Number(_) => Some(FieldForm::Number),
    FieldKind::Choice { .. } => Some(FieldForm::Choice),
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

    match undrawn_form(field.kind()) {
      Some(form) => undrawn.push(Undrawn::FieldForm {
        option: option.id().clone(),
        field: field.id().clone(),
        form,
      }),
      None => drawn.push(Drawn {
        key: run.key(),
        field: PresentationField {
          id: field.id().clone(),
          label: field.label().to_owned(),
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
