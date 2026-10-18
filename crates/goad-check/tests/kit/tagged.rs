//! The fence gate (`design.md` §5.2.6, *The tagged-fence convention*).
//!
//! Every `json` and `toml` fence in the kit's Markdown carries a `goad:` role,
//! and goes through the door the host itself reads that kind of document
//! with. A fence that is untagged, misspelt or of an unknown role is refused,
//! so the checked set cannot shrink unseen.
use std::collections::BTreeMap;
use std::fmt;
use std::mem::discriminant;
use std::path::PathBuf;

use goad_semantics::error::Requirement;
use goad_semantics::protocol::canonical::{
  Evaluate, Event, FieldKind, Request, Respond, Submitted, Timestamp, UserResponse, View, ViewId,
};
use goad_semantics::protocol::normalize::{Discarded, read_response};
use goad_shell::config::Config;
use goad_shell::ingress::envelope;
use serde_json::Value;

use crate::fences::fences;
use crate::tree;

/// The `now` every response fence is read at, so a relative `next_check` in
/// one resolves the same way on every run.
const NOW: &str = "2026-08-23T04:12:00Z";

/// Why a fence is refused. Closed: each negative control asserts one.
#[derive(Debug)]
enum Refusal {
  /// `json` or `toml` with no `goad:` role.
  Untagged,
  /// A first word that, lowercased, begins `json` or `toml` but is not
  /// exactly one (`jsonc`, `json5`, `JSON`).
  Misspelt,
  /// `json` or `toml` followed by anything but a role of the convention.
  UnknownRole,
  /// A `json` role on a `toml` fence, or the reverse.
  WrongLanguage,
  /// `read_response` refused a response tagged to be accepted.
  ResponseRefused(String),
  /// `read_response` accepted a response tagged `refused R-N`.
  ResponseAccepted,
  /// A refusal or a discard answered by another id than the tag's.
  WrongRequirement { tagged: String, found: String },
  /// Not the tag's number of discards: none for a plain response, one for
  /// `discarded R-N`.
  WrongDiscards { tagged: usize, found: usize },
  /// A request fence the gate cannot read as one.
  MalformedRequest(String),
  /// What the host would send differs from the fence.
  FramingDiffers { host: Value },
  /// A respond with no view-carrying response before it in the document.
  NoPrecedingView,
  /// A respond selecting an option the view does not offer.
  UnknownOption(String),
  /// A respond whose values are not exactly the option's fields (R-58).
  FieldSet {
    missing: Vec<String>,
    undrawn: Vec<String>,
  },
  /// A value of another JSON type than its field submits.
  WrongJsonType(String),
  /// A `choice` value that is none of its alternatives' ids.
  NotAnAlternative(String),
  /// `envelope::normalize` refused an envelope.
  EnvelopeRefused(String),
  /// `Config::parse` refused a config.
  ConfigRefused(String),
}

impl fmt::Display for Refusal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Untagged => write!(f, "untagged: add its `goad:` role"),
      Self::Misspelt => write!(f, "misspelt: the language is exactly `json` or `toml`"),
      Self::UnknownRole => write!(f, "not a role of the convention"),
      Self::WrongLanguage => write!(f, "the role is of the other language"),
      Self::ResponseRefused(error) => write!(f, "read_response refused it: {error}"),
      Self::ResponseAccepted => write!(f, "read_response accepted it"),
      Self::WrongRequirement { tagged, found } => write!(f, "tagged {tagged}, answered {found}"),
      Self::WrongDiscards { tagged, found } => write!(f, "{found} discards, tagged {tagged}"),
      Self::MalformedRequest(why) => write!(f, "not a request: {why}"),
      Self::FramingDiffers { host } => write!(f, "the host would send {host}"),
      Self::NoPrecedingView => write!(f, "no view-carrying response before it"),
      Self::UnknownOption(id) => write!(f, "the view offers no option {id}"),
      Self::FieldSet { missing, undrawn } => {
        let (missing, undrawn) = (missing.join(", "), undrawn.join(", "));
        write!(
          f,
          "drawn fields missing [{missing}], undrawn fields sent [{undrawn}]"
        )
      }
      Self::WrongJsonType(id) => write!(f, "{id} is not of the JSON type its field submits"),
      Self::NotAnAlternative(id) => write!(f, "{id} is none of its alternatives"),
      Self::EnvelopeRefused(error) => write!(f, "envelope::normalize refused it: {error}"),
      Self::ConfigRefused(error) => write!(f, "Config::parse refused it: {error}"),
    }
  }
}

/// A `goad:` role, as the convention's table names them.
enum Role {
  Response,
  Refused(String),
  Discarded(String),
  Request,
  Envelope,
  Config,
}

/// What the gate made of one Markdown document: how many fences it checked,
/// and each refusal with the line of its opener.
struct Verdict {
  checked: usize,
  refusals: Vec<(usize, Refusal)>,
}

/// `None` for a fence the gate does not check: another language, or no info
/// string.
fn role(info: &str) -> Option<Result<Role, Refusal>> {
  let words: Vec<&str> = info.split_whitespace().collect();
  let (first, rest) = words.split_first()?;
  let lowered = first.to_lowercase();
  if !(lowered.starts_with("json") || lowered.starts_with("toml")) {
    return None;
  }
  if !matches!(*first, "json" | "toml") {
    return Some(Err(Refusal::Misspelt));
  }
  let (language, role) = match rest {
    [] => return Some(Err(Refusal::Untagged)),
    ["goad:response"] => ("json", Role::Response),
    ["goad:response", "refused", id] => ("json", Role::Refused((*id).to_owned())),
    ["goad:response", "discarded", id] => ("json", Role::Discarded((*id).to_owned())),
    ["goad:request"] => ("json", Role::Request),
    ["goad:envelope"] => ("json", Role::Envelope),
    ["goad:config"] => ("toml", Role::Config),
    _ => return Some(Err(Refusal::UnknownRole)),
  };
  Some(if *first == language {
    Ok(role)
  } else {
    Err(Refusal::WrongLanguage)
  })
}

fn check(markdown: &str) -> Verdict {
  let mut verdict = Verdict {
    checked: 0,
    refusals: Vec::new(),
  };
  // The nearest preceding view a respond fence answers.
  let mut view: Option<View> = None;
  for fence in fences(markdown) {
    let Some(role) = role(fence.info) else {
      continue;
    };
    verdict.checked += 1;
    let body = &markdown[fence.body.clone()];
    let outcome = role.and_then(|role| match role {
      Role::Request => request(body, view.as_ref()),
      Role::Envelope => envelope::normalize(body.as_bytes())
        .map(drop)
        .map_err(|e| Refusal::EnvelopeRefused(e.to_string())),
      Role::Config => Config::parse(body)
        .map(drop)
        .map_err(|e| Refusal::ConfigRefused(e.to_string())),
      response => {
        let (drawn, outcome) = self::response(&response, body);
        view = drawn.or(view.take());
        outcome
      }
    });
    if let Err(refusal) = outcome {
      let line = markdown[..fence.body.start].lines().count();
      verdict.refusals.push((line, refusal));
    }
  }
  verdict
}

fn now() -> Timestamp {
  Timestamp::new(NOW.parse().expect("NOW is an instant"))
}

/// A response fence through `read_response`: its outcome against the tag, and
/// the view it carries if it was accepted with one.
fn response(role: &Role, body: &str) -> (Option<View>, Result<(), Refusal>) {
  let read = read_response(body.as_bytes(), now());
  let view = read
    .as_ref()
    .ok()
    .and_then(|normalized| normalized.value.view().cloned());
  let outcome = match (role, read) {
    (Role::Refused(tagged), Err(error)) => same(tagged, &error.requirement().to_string()),
    (Role::Refused(_), Ok(_)) => Err(Refusal::ResponseAccepted),
    (_, Err(error)) => Err(Refusal::ResponseRefused(error.to_string())),
    (Role::Discarded(tagged), Ok(normalized)) => match normalized.discarded.as_slice() {
      [Discarded::Schedule { reason, .. }] => same(tagged, &reason.requirement().to_string()),
      other => Err(Refusal::WrongDiscards {
        tagged: 1,
        found: other.len(),
      }),
    },
    (_, Ok(normalized)) => match normalized.discarded.len() {
      0 => Ok(()),
      found => Err(Refusal::WrongDiscards { tagged: 0, found }),
    },
  };
  (view, outcome)
}

fn same(tagged: &str, found: &str) -> Result<(), Refusal> {
  if tagged == found {
    Ok(())
  } else {
    Err(Refusal::WrongRequirement {
      tagged: tagged.to_owned(),
      found: found.to_owned(),
    })
  }
}

/// A request fence: the request the host would build from it, serialized,
/// equals it. So the fence holds the host's framing, spelt as the host spells
/// it.
fn request(body: &str, view: Option<&View>) -> Result<(), Refusal> {
  let fence: Value =
    serde_json::from_str(body).map_err(|e| Refusal::MalformedRequest(e.to_string()))?;
  let request = match fence.get("type").and_then(Value::as_str) {
    Some("evaluate") => evaluate(&fence)?,
    Some("respond") => respond(&fence, view.ok_or(Refusal::NoPrecedingView)?)?,
    _ => {
      return Err(Refusal::MalformedRequest(
        "`type` is neither \"evaluate\" nor \"respond\"".to_owned(),
      ));
    }
  };
  let host = sent(&request);
  if host == fence {
    Ok(())
  } else {
    Err(Refusal::FramingDiffers { host })
  }
}

/// The request as the host writes it.
fn sent(request: &Request) -> Value {
  serde_json::to_value(request).expect("a request serializes")
}

fn string<'a>(fence: &'a Value, pointer: &str) -> Result<&'a str, Refusal> {
  fence
    .pointer(pointer)
    .and_then(Value::as_str)
    .ok_or_else(|| Refusal::MalformedRequest(format!("no string at {pointer}")))
}

/// Parsed as stratum 1 parses an instant.
fn instant(fence: &Value, pointer: &str) -> Result<Timestamp, Refusal> {
  string(fence, pointer)?
    .parse()
    .map(Timestamp::new)
    .map_err(|e| Refusal::MalformedRequest(format!("{pointer}: {e}")))
}

fn evaluate(fence: &Value) -> Result<Request, Refusal> {
  Ok(Request::Evaluate(Evaluate {
    now: instant(fence, "/now")?,
    event: Event {
      source: string(fence, "/event/source")?.to_owned(),
      kind: string(fence, "/event/kind")?.to_owned(),
      timestamp: instant(fence, "/event/timestamp")?,
      data: fence.pointer("/event/data").cloned().unwrap_or_default(),
    },
  }))
}

/// A respond answers `view`: the option is one it offers, the values are
/// exactly that option's fields (R-58), and each is of the JSON type its field
/// submits untouched — `Submitted::as_drawn`'s, so no kind is mapped to a type
/// here — and a `choice` value is one of its alternatives.
fn respond(fence: &Value, view: &View) -> Result<Request, Refusal> {
  let View::Choice(choice) = view;
  let selected = string(fence, "/response/option")?;
  let option = choice
    .options()
    .as_slice()
    .iter()
    .find(|option| option.id().as_str() == selected)
    .ok_or_else(|| Refusal::UnknownOption(selected.to_owned()))?;
  let values = fence
    .pointer("/response/values")
    .and_then(Value::as_object)
    .ok_or_else(|| Refusal::MalformedRequest("no object at /response/values".to_owned()))?;
  let drawn = option.fields().as_slice();
  let missing: Vec<String> = drawn
    .iter()
    .map(|field| field.id().as_str())
    .filter(|id| !values.contains_key(*id))
    .map(str::to_owned)
    .collect();
  let undrawn: Vec<String> = values
    .keys()
    .filter(|key| {
      !drawn
        .iter()
        .any(|field| field.id().as_str() == key.as_str())
    })
    .cloned()
    .collect();
  if !(missing.is_empty() && undrawn.is_empty()) {
    return Err(Refusal::FieldSet { missing, undrawn });
  }
  let mut answered = BTreeMap::new();
  for field in drawn {
    let (id, value) = (field.id(), &values[field.id().as_str()]);
    if discriminant(value) != discriminant(&Submitted::as_drawn(field.kind()).to_json()) {
      return Err(Refusal::WrongJsonType(id.as_str().to_owned()));
    }
    if let FieldKind::Choice { alternatives } = field.kind()
      && !alternatives
        .as_slice()
        .iter()
        .any(|alternative| value.as_str() == Some(alternative.id().as_str()))
    {
      return Err(Refusal::NotAnAlternative(id.as_str().to_owned()));
    }
    answered.insert(id.clone(), value.clone());
  }
  Ok(Request::Respond(Respond {
    view_id: ViewId::new(string(fence, "/view_id")?),
    now: instant(fence, "/now")?,
    response: UserResponse {
      option: option.id().clone(),
      values: answered,
    },
  }))
}

/// The one refusal `markdown` earns, whatever it is.
fn refused(markdown: &str) -> Refusal {
  let Verdict {
    checked, refusals, ..
  } = check(markdown);
  let mut refusals = refusals.into_iter();
  match (checked, refusals.next(), refusals.next()) {
    (1, Some((_, refusal)), None) => refusal,
    (checked, first, second) => {
      panic!("one checked fence, one refusal; got {checked}: {first:?}, {second:?}\n{markdown}")
    }
  }
}

/// `markdown` is checked once and refused nothing.
fn accepted(markdown: &str) {
  let verdict = check(markdown);
  assert!(
    verdict.checked == 1 && verdict.refusals.is_empty(),
    "one checked fence, accepted; got {}: {:?}\n{markdown}",
    verdict.checked,
    verdict.refusals
  );
}

/// A whole envelope: valid for its role, so the tag is its only possible fault.
const ENVELOPE: &str = r#"{"source": "watcher", "kind": "file-arrived", "timestamp": "2026-08-23T04:12:00Z", "data": {}}"#;

fn fenced(opener: &str, info: &str, body: &str) -> String {
  format!("Prose.\n\n{opener}{info}\n{body}\n{opener}\n\nMore prose.\n")
}

#[test]
fn an_untagged_json_fence_is_refused() {
  for opener in ["```", "~~~"] {
    assert!(
      matches!(
        refused(&fenced(opener, "json", ENVELOPE)),
        Refusal::Untagged
      ),
      "{opener}json"
    );
    accepted(&fenced(opener, "json goad:envelope", ENVELOPE));
  }
  // An untagged fence shown inside a tilde fence is its content, not a fence:
  // a closer is of the opener's own character.
  accepted(&format!(
    "~~~text\n```json\n{ENVELOPE}\n```\n~~~\n{}",
    fenced("```", "json goad:envelope", ENVELOPE)
  ));
}

#[test]
fn a_jsonc_fence_is_refused() {
  for info in ["jsonc", "json5", "JSON goad:envelope"] {
    assert!(
      matches!(refused(&fenced("```", info, ENVELOPE)), Refusal::Misspelt),
      "{info}"
    );
  }
  accepted(&fenced("```", "json goad:envelope", ENVELOPE));
}

#[test]
#[expect(
  clippy::print_stdout,
  reason = "the checked count is printed for a person to tally by hand (VA-1)"
)]
fn every_json_and_toml_fence_in_the_kit_is_tagged_and_checked() {
  let documents: Vec<PathBuf> = tree::files(&tree::root())
    .into_iter()
    .filter(|file| file.extension().is_some_and(|extension| extension == "md"))
    .collect();
  assert!(!documents.is_empty(), "the kit holds no Markdown to check");
  let mut checked = 0;
  let mut breaches = Vec::new();
  for document in &documents {
    let verdict = check(&tree::text(document));
    checked += verdict.checked;
    breaches.extend(
      verdict
        .refusals
        .into_iter()
        .map(|(line, refusal)| format!("{}:{line}: {refusal}", document.display())),
    );
  }
  println!("{checked} json and toml fences checked");
  assert!(
    checked > 0,
    "no json or toml fence found in {documents:?}: is the scanner seeing any?"
  );
  assert!(
    breaches.is_empty(),
    "fences refused:\n{}",
    breaches.join("\n")
  );
}

/// A response whose view's first option draws a `boolean` and a `choice`.
const VIEW: &str = r#"{"view": {"kind": "choice", "title": "Done?", "options": [
  {"id": "save", "label": "Save", "fields": [
    {"id": "done", "kind": "boolean", "label": "Finished?"},
    {"id": "mood", "kind": "choice", "label": "Mood",
     "options": [{"id": "up", "label": "Good"}, {"id": "down", "label": "Bad"}]}]},
  {"id": "skip", "label": "Skip"}]}}"#;

/// A respond to `VIEW`'s `save`, carrying `values`.
fn answer(values: &str) -> String {
  format!(
    r#"{{"protocol": 1, "type": "respond", "now": "2026-08-23T04:14:00Z", "view_id": "v#1",
  "response": {{"option": "save", "values": {values}}}}}"#
  )
}

/// `VIEW`, then a respond fence answering it: the respond is the second
/// fence checked.
fn answering(respond: &str) -> String {
  format!(
    "{}{}",
    fenced("```", "json goad:response", VIEW),
    fenced("```", "json goad:request", respond)
  )
}

/// The refusal of the respond fence in `answering(respond)`, or `None`.
fn respond_refusal(respond: &str) -> Option<Refusal> {
  let verdict = check(&answering(respond));
  assert_eq!(verdict.checked, 2, "{respond}");
  let mut refusals = verdict.refusals.into_iter();
  let refusal = refusals.next().map(|(_, refusal)| refusal);
  assert!(refusals.next().is_none(), "one refusal at most: {respond}");
  refusal
}

#[test]
fn a_respond_fence_with_a_value_of_the_wrong_json_type_is_refused() {
  assert!(
    matches!(
      respond_refusal(&answer(r#"{"done": "yes", "mood": "up"}"#)),
      Some(Refusal::WrongJsonType(field)) if field == "done"
    ),
    "a boolean field answered with a string"
  );
  assert!(respond_refusal(&answer(r#"{"done": false, "mood": "up"}"#)).is_none());
}

/// One negative control of `each_role_refuses_what_its_door_refuses`: a
/// document refused once, of the kind `refusal` accepts, and its twin, the
/// same fault mended, accepted.
struct Row {
  case: &'static str,
  refused: String,
  refusal: fn(&Refusal) -> bool,
  twin: String,
}

/// Why `row` does not hold, if it does not.
fn complaint(row: &Row) -> Option<String> {
  let refused = check(&row.refused);
  let twin = check(&row.twin);
  let held = matches!(refused.refusals.as_slice(), [(_, refusal)] if (row.refusal)(refusal))
    && twin.checked > 0
    && twin.refusals.is_empty();
  (!held).then(|| {
    let shown = |verdict: &Verdict| {
      verdict
        .refusals
        .iter()
        .map(|(line, refusal)| format!("{line}: {refusal}"))
        .collect::<Vec<_>>()
        .join("; ")
    };
    format!(
      "{}: refused [{}]; twin, {} checked, refused [{}]",
      row.case,
      shown(&refused),
      twin.checked,
      shown(&twin)
    )
  })
}

/// A choice with no option: `read_response` refuses it (R-13).
const NO_OPTION: &str = r#"{"view": {"kind": "choice", "title": "Empty", "options": []}}"#;

/// A `next_check` that is not a string: discarded (R-25), the rest accepted.
const NUMBER_NEXT_CHECK: &str = r#"{"view": null, "next_check": 45}"#;

const NOTHING_TO_SHOW: &str = r#"{"view": null}"#;

/// A whole config whose backend `command` is `argv`.
fn config(argv: &str) -> String {
  format!("[backend]\ncommand = {argv}\ntimeout = \"30s\"\n\n[schedule]\ndefault_poll = \"30m\"\n")
}

const COMMAND: &str = r#"["python3", "backend.py"]"#;

fn evaluate_at(now: &str) -> String {
  format!(
    r#"{{"protocol": 1, "type": "evaluate", "now": "{now}",
  "event": {{"source": "host", "kind": "scheduled", "timestamp": "2026-08-23T04:12:00Z", "data": {{}}}}}}"#
  )
}

#[test]
fn each_role_refuses_what_its_door_refuses() {
  let r13 = Requirement::R13.to_string();
  let r25 = Requirement::R25.to_string();
  let json = |info: &str, body: &str| fenced("```", info, body);
  let rows = [
    Row {
      case: "an unknown role",
      refused: json("json goad:reply", ENVELOPE),
      refusal: |r| matches!(r, Refusal::UnknownRole),
      twin: json("json goad:envelope", ENVELOPE),
    },
    Row {
      case: "a json role on a toml fence",
      refused: json("toml goad:envelope", ENVELOPE),
      refusal: |r| matches!(r, Refusal::WrongLanguage),
      twin: json("json goad:envelope", ENVELOPE),
    },
    Row {
      case: "a toml role on a json fence",
      refused: json("json goad:config", &config(COMMAND)),
      refusal: |r| matches!(r, Refusal::WrongLanguage),
      twin: json("toml goad:config", &config(COMMAND)),
    },
    Row {
      case: "a response tagged accepted that is refused",
      refused: json("json goad:response", NO_OPTION),
      refusal: |r| matches!(r, Refusal::ResponseRefused(_)),
      twin: json(&format!("json goad:response refused {r13}"), NO_OPTION),
    },
    Row {
      case: "a response tagged refused that is accepted",
      refused: json(
        &format!("json goad:response refused {r13}"),
        NOTHING_TO_SHOW,
      ),
      refusal: |r| matches!(r, Refusal::ResponseAccepted),
      twin: json("json goad:response", NOTHING_TO_SHOW),
    },
    Row {
      case: "refused R-N with the wrong id",
      refused: json("json goad:response refused R-14", NO_OPTION),
      refusal: |r| matches!(r, Refusal::WrongRequirement { .. }),
      twin: json(&format!("json goad:response refused {r13}"), NO_OPTION),
    },
    Row {
      case: "discarded R-N with the wrong id",
      refused: json("json goad:response discarded R-21", NUMBER_NEXT_CHECK),
      refusal: |r| matches!(r, Refusal::WrongRequirement { .. }),
      twin: json(
        &format!("json goad:response discarded {r25}"),
        NUMBER_NEXT_CHECK,
      ),
    },
    Row {
      case: "discarded R-N with no discard",
      refused: json(
        &format!("json goad:response discarded {r25}"),
        NOTHING_TO_SHOW,
      ),
      refusal: |r| {
        matches!(
          r,
          Refusal::WrongDiscards {
            tagged: 1,
            found: 0
          }
        )
      },
      twin: json("json goad:response", NOTHING_TO_SHOW),
    },
    Row {
      case: "a plain response carrying a discard",
      refused: json("json goad:response", NUMBER_NEXT_CHECK),
      refusal: |r| {
        matches!(
          r,
          Refusal::WrongDiscards {
            tagged: 0,
            found: 1
          }
        )
      },
      twin: json(
        &format!("json goad:response discarded {r25}"),
        NUMBER_NEXT_CHECK,
      ),
    },
    Row {
      case: "an evaluate whose framing differs: an instant not spelt as the host spells it",
      refused: json(
        "json goad:request",
        &evaluate_at("2026-08-23T14:12:00+10:00"),
      ),
      refusal: |r| matches!(r, Refusal::FramingDiffers { .. }),
      twin: json("json goad:request", &evaluate_at("2026-08-23T04:12:00Z")),
    },
    Row {
      case: "a request of neither type",
      refused: json("json goad:request", ENVELOPE),
      refusal: |r| matches!(r, Refusal::MalformedRequest(_)),
      twin: json("json goad:request", &evaluate_at("2026-08-23T04:12:00Z")),
    },
    Row {
      case: "a respond whose framing differs: another protocol",
      refused: answering(
        &answer(r#"{"done": false, "mood": "up"}"#).replace(r#""protocol": 1"#, r#""protocol": 2"#),
      ),
      refusal: |r| matches!(r, Refusal::FramingDiffers { .. }),
      twin: answering(&answer(r#"{"done": false, "mood": "up"}"#)),
    },
    Row {
      case: "a respond with no preceding view",
      refused: json(
        "json goad:request",
        &answer(r#"{"done": false, "mood": "up"}"#),
      ),
      refusal: |r| matches!(r, Refusal::NoPrecedingView),
      twin: answering(&answer(r#"{"done": false, "mood": "up"}"#)),
    },
    Row {
      case: "a respond selecting an option the view does not offer",
      refused: answering(
        &answer(r#"{"done": false, "mood": "up"}"#).replace(r#""save""#, r#""keep""#),
      ),
      refusal: |r| matches!(r, Refusal::UnknownOption(_)),
      twin: answering(&answer(r#"{"done": false, "mood": "up"}"#)),
    },
    Row {
      case: "a respond missing a drawn field (R-58)",
      refused: answering(&answer(r#"{"done": false}"#)),
      refusal: |r| matches!(r, Refusal::FieldSet { missing, undrawn } if missing == &["mood"] && undrawn.is_empty()),
      twin: answering(&answer(r#"{"done": false, "mood": "up"}"#)),
    },
    Row {
      case: "a respond carrying an undrawn field (R-58)",
      refused: answering(&answer(r#"{"done": false, "mood": "up", "note": ""}"#)),
      refusal: |r| matches!(r, Refusal::FieldSet { missing, undrawn } if missing.is_empty() && undrawn == &["note"]),
      twin: answering(&answer(r#"{"done": false, "mood": "up"}"#)),
    },
    Row {
      case: "a choice value outside its alternatives",
      refused: answering(&answer(r#"{"done": false, "mood": "save"}"#)),
      refusal: |r| matches!(r, Refusal::NotAnAlternative(field) if field == "mood"),
      twin: answering(&answer(r#"{"done": false, "mood": "down"}"#)),
    },
    Row {
      case: "an envelope its door refuses: no source",
      refused: json(
        "json goad:envelope",
        &ENVELOPE.replace(r#""source": "watcher", "#, ""),
      ),
      refusal: |r| matches!(r, Refusal::EnvelopeRefused(_)),
      twin: json("json goad:envelope", ENVELOPE),
    },
    Row {
      case: "a config its door refuses: an empty command",
      refused: json("toml goad:config", &config("[]")),
      refusal: |r| matches!(r, Refusal::ConfigRefused(_)),
      twin: json("toml goad:config", &config(COMMAND)),
    },
  ];
  let complaints: Vec<String> = rows.iter().filter_map(complaint).collect();
  assert!(
    complaints.is_empty(),
    "rows not held:\n{}",
    complaints.join("\n")
  );
}
