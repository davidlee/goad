//! design.md §9 items 4-5 (AC-9): `present`, tested without a component.
//!
//! `present` is total, pure and panic-free — every row of §5.2's content
//! table is asserted here, against the canonical `View` `read_response`
//! produces, never against a `PromptWindow`. Fixtures are built with
//! `serde_json::json!` rather than hand-escaped byte strings, so a fixture
//! carrying U+E541 (E-7) is exactly the string that reaches `present`, not a
//! string plus an escaping mistake.

use goad::view_model::{Body, ContentForm, Presentation, Undrawn, present};
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

/// One option carrying the fields given — the shape every field case needs
/// and no body case does. The option's own id is the one every report below
/// names.
fn one_option_with_fields(fields: &serde_json::Value) -> View {
  view_from(&serde_json::json!({
    "view": {
      "kind": "choice",
      "title": "T",
      "options": [{ "id": "opt", "label": "Fine", "fields": fields }]
    }
  }))
}

/// The only option's blocks, as (heading, the ids of its fields). Ids rather
/// than labels because the id is what answers the field, and the whole
/// subject here is which field is drawn where.
fn blocks_of(presentation: &Presentation) -> Vec<(Option<&str>, Vec<&str>)> {
  presentation.options[0]
    .blocks
    .iter()
    .map(|block| {
      (
        block.heading.as_deref(),
        block
          .fields
          .iter()
          .map(|field| field.id.as_str())
          .collect::<Vec<_>>(),
      )
    })
    .collect()
}

/// A boolean field, optionally carrying a `group` hint. Flat on the field
/// object, which is where `normalize_field` collects an unmodelled key into
/// `hints` (SPEC-001/R-18).
fn boolean(id: &str, group: Option<serde_json::Value>) -> serde_json::Value {
  let mut field = serde_json::json!({ "id": id, "kind": "boolean", "label": "F" });
  if let Some(group) = group {
    field["group"] = group;
  }
  field
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

/// VT-3 — AC-2's rule half. A heading wherever the `group` value changes, and
/// **no merging**: two `Morning` runs separated by an `Evening` are two
/// blocks, both titled, because merging them would move a field and the host
/// chooses no field's position (I-6).
#[test]
fn a_block_starts_wherever_the_group_value_changes_and_runs_are_never_merged() {
  let presentation = present(&one_option_with_fields(&serde_json::json!([
    boolean("a", Some(serde_json::json!("Morning"))),
    boolean("b", Some(serde_json::json!("Morning"))),
    boolean("c", Some(serde_json::json!("Evening"))),
    boolean("d", Some(serde_json::json!("Morning"))),
  ])));

  assert_eq!(
    blocks_of(&presentation),
    vec![
      (Some("Morning"), vec!["a", "b"]),
      (Some("Evening"), vec!["c"]),
      (Some("Morning"), vec!["d"]),
    ]
  );
  assert_eq!(presentation.undrawn, Vec::<Undrawn>::new());
}

/// VT-3 — the two headingless cases, drawn in place and in declared order.
/// `""` is an **untitled block**: the backend sent a string, so the `group`
/// value changed and a new run starts, and there is simply no heading to
/// draw. An absent `group` is not that value, which is why `b` does not join
/// `a`.
#[test]
fn an_absent_group_and_an_empty_one_are_blocks_with_no_heading() {
  let presentation = present(&one_option_with_fields(&serde_json::json!([
    boolean("a", None),
    boolean("b", Some(serde_json::json!(""))),
    boolean("c", Some(serde_json::json!("Morning"))),
  ])));

  assert_eq!(
    blocks_of(&presentation),
    vec![
      (None, vec!["a"]),
      (None, vec!["b"]),
      (Some("Morning"), vec!["c"]),
    ]
  );
  assert_eq!(
    presentation.undrawn,
    Vec::<Undrawn>::new(),
    "an empty group is a string the backend sent; there is nothing to report"
  );
}

/// VT-4 — a `group` that is not a JSON string is reported and the field is
/// drawn where it was declared: coercing `7` into a heading would invent one
/// the backend did not author (P-3). An empty group is not reported, and an
/// absent one is not either.
#[test]
fn a_group_hint_that_is_not_a_string_is_reported_and_the_field_is_drawn_in_place() {
  let presentation = present(&one_option_with_fields(&serde_json::json!([
    boolean("counted", Some(serde_json::json!(7))),
    boolean("empty", Some(serde_json::json!(""))),
    boolean("bare", None),
  ])));

  let reported: Vec<(&str, &str)> = presentation
    .undrawn
    .iter()
    .filter_map(|undrawn| match undrawn {
      Undrawn::GroupHint { option, field } => Some((option.as_str(), field.as_str())),
      _ => None,
    })
    .collect();

  assert_eq!(reported, vec![("opt", "counted")]);
  assert_eq!(presentation.undrawn.len(), 1, "and nothing else");
  assert_eq!(
    blocks_of(&presentation),
    vec![
      (None, vec!["counted"]),
      (None, vec!["empty"]),
      (None, vec!["bare"]),
    ]
  );
}

/// VT-4 — a field report is orthogonal to the body, which is the exclusion
/// `Presentation::body_is_degraded`'s own doc states. A body that parsed is not
/// degraded by a `group` hint the renderer could not read.
///
/// **It was two reports and is now one, and the half that went is not
/// replaceable.** The second field was a kind this renderer did not draw;
/// PHASE-09 drew the last of them, so `Undrawn::FieldForm` is unconstructible
/// and `GroupHint` is the only field-level report left. The claim is unchanged
/// — the exclusion is over both field variants and `body_is_degraded` still
/// matches neither — and the case now measures it over the one that can still
/// be built (`canon-delta.md` CD-2, `prototype-notes.md` P-13).
#[test]
fn field_reports_leave_a_parsed_body_undegraded() {
  let view = view_from(&serde_json::json!({
    "view": {
      "kind": "choice",
      "title": "T",
      "body": { "kind": "markdown", "value": "**bold**" },
      "options": [{
        "id": "opt",
        "label": "Fine",
        "fields": [
          { "id": "counted", "kind": "boolean", "label": "Counted", "group": 7 }
        ]
      }]
    }
  }));
  let presentation = present(&view);

  assert!(matches!(presentation.body, Body::Rich(_)));
  assert_eq!(presentation.undrawn.len(), 1, "the field is reported");
  assert!(
    !presentation.body_is_degraded(),
    "a field report says nothing about the body"
  );
}

/// VT-5 — the option path as it is today survives: an option carrying no
/// field is a bare button with nothing beside it, and two of them map in
/// declared order with nothing reported (AC-6).
#[test]
fn options_carrying_no_fields_map_as_they_do_today() {
  let view = view_from(&serde_json::json!({
    "view": {
      "kind": "choice",
      "title": "T",
      "options": [
        { "id": "ok", "label": "Fine" },
        { "id": "skip", "label": "Skip" }
      ]
    }
  }));
  let presentation = present(&view);

  assert_eq!(presentation.options.len(), 2);
  assert_eq!(presentation.options[0].label, "Fine");
  assert_eq!(presentation.options[1].label, "Skip");
  assert!(
    presentation.options[0].blocks.is_empty(),
    "a bare option draws nothing beside its button"
  );
  assert!(
    presentation.options[1].blocks.is_empty(),
    "and so does the second"
  );
  assert_eq!(presentation.undrawn, Vec::<Undrawn>::new());
  assert!(!presentation.body_is_degraded());
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
