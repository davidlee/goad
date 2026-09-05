//! `present`, the mapper — design.md §5.2. One function, one direction, no
//! `_ =>` arm: a canonical `View` becomes a displayable `Presentation`.
//!
//! `present` is total, pure and panic-free (I-1): it never refuses a legal
//! view, and every part of it that this renderer cannot draw is degraded
//! into `undrawn` rather than dropped (I-2, R-20). Nothing here reads a
//! clock, a file or a socket — stratum 3, but the pure half of it.

use goad_semantics::protocol::canonical::{Content, OptionId, View};
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
  /// has one statement. Note what it excludes: `Undrawn::OptionFields` is
  /// undrawn but says nothing about the body.
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
  /// `Opt::fields()` was non-empty and this renderer draws no fields.
  OptionFields { option: OptionId, count: usize },
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
          let field_count = option.fields().as_slice().len();
          if field_count > 0 {
            undrawn.push(Undrawn::OptionFields {
              option: option.id().clone(),
              count: field_count,
            });
          }
          PresentationOption {
            id: option.id().clone(),
            label: option.label().to_owned(),
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
