//! design.md §9 items 4-5 (AC-9): `present`, tested without a component.
//!
//! `present` is total, pure and panic-free — every row of §5.2's content
//! table is asserted here, against the canonical `View` `read_response`
//! produces, never against a `PromptWindow`. Fixtures are built with
//! `serde_json::json!` rather than hand-escaped byte strings, so a fixture
//! carrying U+E541 (E-7) is exactly the string that reaches `present`, not a
//! string plus an escaping mistake.

use goad::view_model::{Body, ContentForm, Undrawn, present};
use goad_semantics::protocol::canonical::{Timestamp, View};
use goad_semantics::protocol::normalize::read_response;
use slint::StyledText;

fn now() -> Timestamp {
  Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture must be an instant"),
  )
}

fn view_from(document: &serde_json::Value) -> View {
  let bytes = document.to_string();
  read_response(bytes.as_bytes(), now())
    .expect("the fixture must normalize")
    .value
    .view()
    .expect("the fixture carries a view")
    .clone()
}

fn one_option() -> serde_json::Value {
  serde_json::json!([{ "id": "ok", "label": "Fine" }])
}

fn choice_with_body(body: Option<serde_json::Value>) -> View {
  let mut view = serde_json::json!({ "kind": "choice", "title": "T", "options": one_option() });
  if let Some(body) = body {
    view["body"] = body;
  }
  view_from(&serde_json::json!({ "view": view }))
}

fn choice_with_markdown(source: &str) -> View {
  choice_with_body(Some(
    serde_json::json!({ "kind": "markdown", "value": source }),
  ))
}

/// VT-1 / item 4, row 1: no body at all.
#[test]
fn an_absent_body_renders_as_nothing_and_is_not_undrawn() {
  let presentation = present(&choice_with_body(None));
  assert_eq!(presentation.body, Body::None);
  assert_eq!(presentation.undrawn, Vec::<Undrawn>::new());
  assert!(!presentation.body_is_degraded());
}

/// VT-1 / item 4, row 2: plain text, tagged.
#[test]
fn text_renders_as_plain_and_is_not_undrawn() {
  let body = Some(serde_json::json!({ "kind": "text", "value": "hello" }));
  let presentation = present(&choice_with_body(body));
  assert_eq!(presentation.body, Body::Plain("hello".to_owned()));
  assert_eq!(presentation.undrawn, Vec::<Undrawn>::new());
  assert!(!presentation.body_is_degraded());
}

/// A bare string body is text too (brief §10.1) — the implicit wire form,
/// not a fourth row of the table.
#[test]
fn a_bare_string_body_is_also_text() {
  let presentation = present(&choice_with_body(Some(serde_json::json!("hello"))));
  assert_eq!(presentation.body, Body::Plain("hello".to_owned()));
}

/// VT-1 / item 4, row 3: markdown that parses is retained, not re-rendered
/// from the source string.
#[test]
fn accepted_markdown_is_retained_as_rich_and_is_not_undrawn() {
  let presentation = present(&choice_with_markdown("**bold**"));
  let expected = StyledText::from_markdown("**bold**").expect("this markdown must parse");
  assert_eq!(presentation.body, Body::Rich(expected));
  assert_eq!(presentation.undrawn, Vec::<Undrawn>::new());
  assert!(!presentation.body_is_degraded());
}

/// VT-1 / item 4, row 4: markdown that does not parse degrades rather than
/// refuses. E-7's own example: U+E541 is Slint's private-use interpolation
/// placeholder, and a backend-authored string can carry it.
#[test]
fn rejected_markdown_degrades_to_plain_and_is_reported_undrawn() {
  let source = "\u{e541}";
  let presentation = present(&choice_with_markdown(source));
  assert_eq!(
    presentation.body,
    Body::Plain(source.to_owned()),
    "a rejected body still reaches the glass, as literal text"
  );
  assert!(matches!(
    presentation.undrawn.as_slice(),
    [Undrawn::MarkdownUnsupported { .. }]
  ));
  assert!(presentation.body_is_degraded());
}

/// VT-1 / item 4, row 5: HTML is shown as literal text and named undrawn by
/// form — the markup is undrawn, not the bytes, which are never omitted.
#[test]
fn html_renders_as_plain_and_is_reported_undrawn_by_form() {
  let body = Some(serde_json::json!({ "kind": "html", "value": "<b>hi</b>" }));
  let presentation = present(&choice_with_body(body));
  assert_eq!(presentation.body, Body::Plain("<b>hi</b>".to_owned()));
  assert_eq!(
    presentation.undrawn,
    vec![Undrawn::ContentForm {
      form: ContentForm::Html
    }]
  );
  assert!(presentation.body_is_degraded());
}

/// VT-1 / item 4, row 6: a URI is shown as literal text, never dereferenced
/// (R-19) — displaying the string is not fetching what it names.
#[test]
fn uri_renders_as_plain_and_is_reported_undrawn_by_form() {
  let body = Some(serde_json::json!({ "kind": "uri", "value": "https://example.invalid/x" }));
  let presentation = present(&choice_with_body(body));
  assert_eq!(
    presentation.body,
    Body::Plain("https://example.invalid/x".to_owned())
  );
  assert_eq!(
    presentation.undrawn,
    vec![Undrawn::ContentForm {
      form: ContentForm::Uri
    }]
  );
  assert!(presentation.body_is_degraded());
}

/// `ContentForm`'s only rendering (F-19): a noun phrase, no article chosen
/// at the call site.
#[test]
fn content_form_displays_as_a_noun_phrase() {
  assert_eq!(ContentForm::Html.to_string(), "HTML");
  assert_eq!(ContentForm::Uri.to_string(), "a URI");
}

/// VT-1 / item 4: a non-empty `Opt::fields()` yields `Undrawn::OptionFields`
/// with the right count — orthogonal to the body, so `body_is_degraded`
/// stays false (the exclusion `Presentation::body_is_degraded`'s own doc
/// states).
#[test]
fn an_option_with_fields_is_reported_undrawn_by_id_and_count() {
  let view = view_from(&serde_json::json!({
    "view": {
      "kind": "choice",
      "title": "T",
      "options": [
        {
          "id": "ok",
          "label": "Fine",
          "fields": [
            { "id": "note", "kind": "text", "label": "Note" },
            { "id": "other", "kind": "text", "label": "Other" }
          ]
        },
        { "id": "skip", "label": "Skip" }
      ]
    }
  }));
  let View::Choice(choice) = &view;
  let with_fields = choice
    .options()
    .as_slice()
    .first()
    .expect("the fixture's first option")
    .id()
    .clone();

  let presentation = present(&view);

  assert_eq!(presentation.options.len(), 2);
  assert_eq!(presentation.options[0].label, "Fine");
  assert_eq!(presentation.options[1].label, "Skip");
  assert!(!presentation.body_is_degraded());
  assert_eq!(
    presentation.undrawn,
    vec![Undrawn::OptionFields {
      option: with_fields,
      count: 2,
    }]
  );
}

/// VT-2 / item 5: a corpus straddling the parse/reject boundary. Every
/// accepted form's `Body::Rich` equals an independent parse of the same
/// source — the mapper's parse is retained, not re-run with a possibly
/// different result — and every rejected form degrades rather than
/// refuses.
#[test]
fn a_markdown_corpus_straddles_the_parse_reject_boundary() {
  let accepted = [
    "plain text, no markdown syntax at all",
    "**bold** and *italic*",
    "- one\n- two",
    "a [link](https://example.invalid/x)",
  ];
  for source in accepted {
    let presentation = present(&choice_with_markdown(source));
    let expected =
      StyledText::from_markdown(source).unwrap_or_else(|error| panic!("{source:?}: {error}"));
    assert_eq!(
      presentation.body,
      Body::Rich(expected),
      "accepted source: {source:?}"
    );
    assert_eq!(
      presentation.undrawn,
      Vec::<Undrawn>::new(),
      "accepted source: {source:?}"
    );
  }

  let rejected = ["\u{e541}", "two placeholders \u{e541}\u{e541}"];
  for source in rejected {
    let presentation = present(&choice_with_markdown(source));
    assert_eq!(
      presentation.body,
      Body::Plain(source.to_owned()),
      "rejected source degrades to plain: {source:?}"
    );
    assert!(
      matches!(
        presentation.undrawn.as_slice(),
        [Undrawn::MarkdownUnsupported { .. }]
      ),
      "rejected source: {source:?}"
    );
  }
}
