//! `crates/goad/src/glass.rs` — stratum 3, and the file that writes every
//! property the window declares (design.md §5.3).
//!
//! It is **not** the only file in the crate that names a generated type, and
//! saying so was a claim that outlived the code: `install.rs:15` names four of
//! them and `instant.rs:23` two more. What is true of this file is narrower and
//! is what the design rests on — every *write* to a window property happens
//! here, in `present`, so totality is checkable by reading one function.

use std::fmt;
use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, StyledText, VecModel};

use goad_semantics::protocol::canonical::ViewId;

use crate::controller::{Frame, Surface};
use crate::diagnostics::{
  BUSY_NOTICE, Diagnostics, TrayState, next_check_line, report_platform, tooltip, tray_icon,
};
use crate::draft::Edited;
use crate::generated::{
  Date, FieldBlock, FieldRow, FieldValue, Kind, OptionRow, PromptWindow, Time, Tray, WindowMode,
};
use crate::instant;
use crate::pending::Debounce;
use crate::reception::Prepared;
use crate::view_model::{Body, DrawnKind, PresentationField, interpret};
use crate::wire::PendingEdit;

/// Total, and the only method: writing every property, every call, is the
/// design's answer to a display server that fails partway through an
/// update (design.md §5.3, *Ownership*).
pub trait Glass {
  /// Write **every** property the frame carries a value for, then show the
  /// window in the frame's mode or hide it. Total and idempotent. `notice` is
  /// one of those properties: it is written from `frame.notice` on every call,
  /// and this is its only writer anywhere in the renderer (design.md §5.3).
  ///
  /// One declared property is deliberately not written: `Tray::shown`
  /// (`ui/app.slint`). It exists so that `visible` is a binding rather than a
  /// literal the compiler can fold into a constant — E-4's panic trap (F-28) —
  /// and the frame carries no value for it, because the tray is present for
  /// the life of the process. A writer for it would be a new frame field with
  /// nothing to put in it.
  ///
  /// **The row model is the second deliberate exception, and the argument for
  /// it is not the first one's** (design.md §5.3). It is retained state whose
  /// only writer is `present`, written on exactly the frames that can change
  /// it — a new `view_id`, or nothing shown. A display server that fails
  /// partway cannot leave it stale, because the only frame that would need to
  /// correct it is the frame that rebuilds it outright. Totality's purpose is
  /// that no property can be left holding a value no frame chose; that still
  /// holds, and it now holds for two different reasons rather than one. The
  /// *value* channel keeps the first reason unchanged: `values` and `epoch`
  /// are written on every call.
  ///
  /// **Two presents showing the same `view_id` are showing the same
  /// structure**, and that is a precondition on the caller rather than a
  /// property of the glass's types (§5.5 I-A). What holds it in production is
  /// `State::issue` minting a fresh id per view together with `Presentation`
  /// never being mutated after it is received; but `Prepared`'s fields are
  /// public and the test tiers build one by hand, so nothing here prevents one
  /// id from carrying two structures.
  fn present(&mut self, frame: Frame<'_>);
}

/// The production glass. Fields are private; the three handles are strong
/// clones taken in `start` (PHASE-10) and live for the process.
pub struct SlintGlass {
  window: PromptWindow,
  tray: Tray,
  options: Rc<VecModel<OptionRow>>,
  /// The view whose structure the row model currently holds, or `None` for a
  /// glass that has shown nothing. Read to decide whether this present has to
  /// rebuild the rows at all (design.md §5.3, §7 D8).
  shown: Option<ViewId>,
  /// **The same handle `install`'s callbacks hold**, never a second one
  /// (design.md §5.3, §8 R10). The caller creates it once and clones it into
  /// both, because two `Debounce` values would leave every case asserting the
  /// overlay green while measuring nothing — there is no compile error to
  /// catch it and no failure to read.
  ///
  /// Read here and written nowhere: `present` looks up what a control has
  /// raised and the host has not recorded yet. Nothing in this file clears it;
  /// the map empties on the enqueue of the send that carries each entry
  /// (`pending.rs`, design.md §5.4).
  pending: Rc<Debounce>,
}

/// Hand-written because the generated component handles carry no `Debug`,
/// by derive or by impl, and `missing_debug_implementations` is `deny`
/// (`Cargo.toml`, measured).
impl fmt::Debug for SlintGlass {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.debug_struct("SlintGlass").finish_non_exhaustive()
  }
}

impl SlintGlass {
  /// Writes the tray's `image` and `hover-text` **before returning**,
  /// because the tray registers nothing until a non-empty image is
  /// assigned (`builtins.slint:3241-3244`) and the loop's first `present`
  /// happens only after the event loop has started.
  ///
  /// `pending` is **a clone of the handle `install` was given**, and the caller
  /// creates it before either (`main.rs:95-107`). A glass given a `Debounce` of
  /// its own overlays nothing and says nothing about it, which is §8 R10.
  #[must_use]
  pub fn new(
    window: PromptWindow,
    tray: Tray,
    options: Rc<VecModel<OptionRow>>,
    pending: Rc<Debounce>,
  ) -> Self {
    tray.set_image(tray_icon(TrayState::Idle));
    tray.set_hover_text(tooltip(&Diagnostics::default(), false).into());
    Self {
      window,
      tray,
      options,
      shown: None,
      pending,
    }
  }
}

impl Glass for SlintGlass {
  /// Infallible: every property setter returns `()`. A `show()` or
  /// `hide()` failure is reported on stderr through
  /// `diagnostics::report_platform` and `present` returns; the process
  /// keeps running, with no de-duplication (design.md §5.3).
  fn present(&mut self, frame: Frame<'_>) {
    self
      .window
      .set_mode(if matches!(frame.surface, Surface::Diagnostics) {
        WindowMode::Diagnostic
      } else {
        WindowMode::Prompt
      });

    let (heading, body, degraded, rows, values) = match frame.shown {
      Some(prepared) => {
        let (rows, values) = option_models(prepared, &self.pending);
        (
          prepared.presentation.title.clone(),
          styled(&prepared.presentation.body),
          prepared.presentation.body_is_degraded(),
          rows,
          values,
        )
      }
      None => (
        String::new(),
        StyledText::default(),
        false,
        Vec::new(),
        Vec::new(),
      ),
    };
    self.window.set_heading(heading.into());
    self.window.set_body(body);
    self.window.set_body_degraded(degraded);
    self.window.set_busy(frame.busy);

    // **The order of these three writes is §5.5 I-F, and it is load-bearing in
    // a way nothing reports when it is wrong.**
    //
    // `values` first, because `set_vec` instantiates the rows and a row
    // evaluates `root.values[field.slot]` *while* it is being instantiated.
    // With the rows written first, a new view's rows index the previous view's
    // shorter array — which Slint answers with a default-initialised
    // `FieldValue` rather than an error: a zero that looks like a value.
    // Writing the new view's values while the old rows still index them costs
    // nothing, because the next statement destroys those rows.
    //
    // The rows second, and **only where the view changed**: repeating over
    // them destroys every element beneath, so a present that rebuilt them
    // unconditionally would destroy the widget a person is working in on every
    // tray check (§7 D8, D9).
    //
    // The epoch last, because the guard reads `root.values[field.slot]` when
    // the epoch changes; bumping it first would run every guard against the
    // previous present's values.
    self.window.set_values(model(values));
    let showing = frame.shown.map(|prepared| prepared.view_id.clone());
    if self.shown != showing {
      self.options.set_vec(rows);
      self
        .window
        .set_options(ModelRc::from(Rc::clone(&self.options)));
      self.shown = showing;
    }
    // Wrapping, because the epoch is a change signal and not a count: an `i32`
    // that saturated would stop firing the guard, and one that overflowed
    // would panic in a debug build. Every wrap is still a change, which is all
    // a `changed` handler reads.
    self
      .window
      .set_epoch(self.window.get_epoch().wrapping_add(1));

    let lines: Vec<SharedString> = frame
      .diagnostics
      .lines()
      .iter()
      .map(SharedString::from)
      .collect();
    self
      .window
      .set_diagnostic_lines(ModelRc::new(VecModel::from(lines)));

    let next_check = frame.next_check.map(next_check_line).unwrap_or_default();
    self.window.set_next_check(next_check.into());

    let notice = if frame.notice { BUSY_NOTICE } else { "" };
    self.window.set_notice(notice.into());

    self.tray.set_image(tray_icon(frame.diagnostics.state()));
    self
      .tray
      .set_hover_text(tooltip(frame.diagnostics, frame.shown.is_some()).into());

    let shown = match frame.surface {
      Surface::Hidden => self.window.hide(),
      Surface::Prompt | Surface::Diagnostics => self.window.show(),
    };
    if let Err(error) = shown {
      report_platform(&error.to_string());
    }
  }
}

/// The body a frame carries, mapped to the type `PromptWindow::set_body`
/// takes. `Body::Rich` is already parsed and is cloned rather than
/// re-parsed — the parse is retained, not repeated (design.md §5.2).
fn styled(body: &Body) -> StyledText {
  match body {
    Body::None => StyledText::default(),
    Body::Plain(text) => StyledText::from_plain_text(text),
    Body::Rich(rich) => rich.clone(),
  }
}

/// **Both models in one pass**, so a field's slot is its index into `values`
/// by construction rather than by two numberings kept in step (design.md §5.5
/// I-B). `values` is flat across the whole presentation, so the counter runs
/// across options and blocks alike — which is why there is one loop nest here
/// rather than one function per level.
///
/// One `OptionRow` per retained option, carrying the presentation's
/// `PresentationOption`, the option's drawn fields, and the view token the
/// markup hands back on `chosen` and on `edited` (design.md §5.3, R-14, D10,
/// D19); one `FieldValue` per drawn field, in the order the rows number them.
///
/// Everything here is rebuilt from scratch on every present: nothing is
/// cached, so nothing has an invalidation rule to get wrong. What changed is
/// which of the two results the caller *writes* — the values always, the rows
/// only for a view it has not shown — and that is `present`'s decision, not
/// this function's.
///
/// **The value channel has two sources, and no cache either way.** A field's
/// value is what the draft holds, *overlaid* with what `pending.rs` is holding
/// for that (option, field) — see [`overlaid`]. That does not weaken the
/// sentence above: `pending.rs` is live state with one writer and a stated
/// lifetime, read at the instant it is needed and copied nowhere that could go
/// stale, so there is still nothing to invalidate (design.md §5.3).
///
/// `carried()` is read **once** for the whole presentation rather than per
/// field: it holds only what a person has touched inside the last 150 ms, so
/// the scan per field is over ones and twos, and the alternative is rebuilding
/// a map out of a map.
///
/// **`FieldBlock` names two types**, `design.md` §5.2's Slint struct and the
/// mapper's, and this file holds both. Only the generated one is named here
/// now — the mapper's is reached through `option.blocks` rather than written
/// down — so the bare name is unambiguous in this file and a phase that has to
/// name the other again should path-qualify it.
fn option_models(prepared: &Prepared, pending: &Debounce) -> (Vec<OptionRow>, Vec<FieldValue>) {
  let mut values: Vec<FieldValue> = Vec::new();
  let mut rows: Vec<OptionRow> = Vec::new();
  let held = pending.carried();
  let today = instant::today_local();

  for option in &prepared.presentation.options {
    let mut blocks: Vec<FieldBlock> = Vec::new();
    for block in &option.blocks {
      let mut fields: Vec<FieldRow> = Vec::new();
      for field in &block.fields {
        // The slot is read **before** the value is pushed, which is the whole
        // of I-B: there is no second counter that could fall out of step with
        // the vector's own length. `try_from` cannot fail for any presentation
        // a backend can send — `i32::MAX` fields would not fit in memory — and
        // `as` is denied crate-wide, so the saturating fallback is the
        // spelling rather than a judgement about the bound.
        let slot = i32::try_from(values.len()).unwrap_or(i32::MAX);
        // The draft, then the overlay over it. `or` and not `unwrap_or`: where
        // there is no entry, or where `interpret` refuses the one there is,
        // the draft's value stands (design.md §5.3).
        let drafted = prepared.draft.state_of(&option.id, &field.id);
        let overlay = overlaid(
          &held,
          prepared.view_id.as_str(),
          option.id.as_str(),
          field,
          drafted.as_ref(),
        );
        values.push(field_value(
          overlay.as_ref().or(drafted.as_ref()),
          &field.kind,
          &today,
        ));
        fields.push(FieldRow {
          kind: markup_kind(&field.kind),
          id: field.id.as_str().into(),
          label: field.label.as_str().into(),
          slot,
        });
      }
      blocks.push(FieldBlock {
        // `heading: None` renders `""`, which the markup reads as an
        // **untitled** block rather than a missing one — an ungrouped run is
        // not drawn under a heading that does not claim it.
        heading: block.heading.as_deref().unwrap_or_default().into(),
        fields: model(fields),
      });
    }
    rows.push(OptionRow {
      id: option.id.as_str().into(),
      label: option.label.as_str().into(),
      view: prepared.view_id.as_str().into(),
      blocks: model(blocks),
    });
  }

  (rows, values)
}

/// The drawn kind, as the markup's discriminant. Total over `DrawnKind`, so a
/// sixth kind added there is a compile error here rather than a field drawn as
/// something else.
///
/// This is where PHASE-01's honest constant went: the kind is now read off the
/// field the mapper drew (`view_model::PresentationField`) instead of being
/// asserted here.
fn markup_kind(kind: &DrawnKind) -> Kind {
  match kind {
    DrawnKind::Boolean => Kind::Boolean,
    DrawnKind::Text => Kind::Text,
    DrawnKind::Number(_) => Kind::Number,
    DrawnKind::Choice { .. } => Kind::Choice,
    DrawnKind::DateTime => Kind::Datetime,
  }
}

/// **What `pending.rs` holds for one field, where it holds anything for it** —
/// the third of I-H's three sites, and the whole of this phase (design.md §5.3
/// *A field's value is the draft's, overlaid*, §7 D26, D27).
///
/// `None` means *show the draft's value*, and it means that for both of the
/// reasons it can: no entry for this field, or an entry `interpret` refuses.
/// A refused entry is a renderer bug and is reported by the command that
/// carries it, never by a present — so the draft's value stands and this
/// function says nothing (design.md §5.2).
///
/// **It goes through `interpret` and not through a mapping of its own.**
/// `pending.rs` holds a `Reported`; the display is written from an `Edited`; a
/// second `Reported` → `FieldValue` mapping beside the existing `Edited` one
/// is the duplication the design avoids everywhere else, and it would be a
/// second place for the two to disagree about what a report means.
///
/// **The view test is I-H.** An entry made on a view that has since been
/// replaced is not shown, because the ids it is keyed by are strings the new
/// view is free to reuse — without this, a stale entry would be written into a
/// new view's widget. Nothing here removes it: the timer sends it under its own
/// view, the controller refuses it `SupersededView`, and it leaves the map
/// because the send was enqueued (design.md §5.4, *A new view*).
fn overlaid(
  held: &[PendingEdit],
  view: &str,
  option: &str,
  field: &PresentationField,
  drafted: Option<&Edited>,
) -> Option<Edited> {
  held
    .iter()
    .find(|entry| entry.view == view && entry.option == option && entry.field == field.id.as_str())
    .and_then(|entry| interpret(&entry.value, drafted, &field.kind))
}

/// What a `datetime` field's button reads before anybody has picked one.
///
/// It is the screen's half of D-6: the wire carries the epoch for the same
/// field, because `R-58` forbids omitting a value for a drawn field, and a
/// button reading `1970-01-01T00:00:00+00:00` would be the host showing a
/// person an answer nobody gave (design.md §5.2, §7 D1, D2).
const NOT_SET: &str = "not set";

/// One field's state channel: what the draft holds — overlaid with what a
/// control has raised and the host has not recorded yet — in the slot the
/// field's kind makes meaningful.
///
/// A **lookup**, never stored in the row model as truth: the draft is the
/// authority — and, for as long as an edit is in flight, `pending.rs` is what
/// stands between the draft's older value and the widget. This is their
/// projection for one present; the join is [`overlaid`]'s.
///
/// **`None` is *untouched*, and the glass reads it directly rather than
/// through `view_model::as_drawn`.** For four of the five kinds the two
/// coincide by construction, so a slot left at its default is what the field
/// was drawn showing; for `datetime` they do not, and the button reads *not
/// set* while the wire carries the epoch. That divergence is D-6's, and
/// routing the glass through `as_drawn` is exactly what would erase it
/// (design.md §5.2).
///
/// **The kind is a parameter because *untouched* is not one value.** An
/// untouched `text` field shows the empty string and an untouched `datetime`
/// shows [`NOT_SET`] and opens its picker on today — so the `None` arm cannot
/// be answered from the state alone, which is exactly the divergence above
/// stated as a signature.
///
/// **Only `checked`, `text`, `date` and `time` are written**, because
/// `boolean`, `text` and `datetime` are the only kinds `drawn_form` draws and
/// no control reads another slot yet. The match is total over `Edited` all the
/// same — that is what makes a value the draft can hold and the screen cannot
/// show a compile error rather than a default. Each remaining arm is filled by
/// the phase that draws its control and owns this value arm in its Surfaces:
/// `number` PHASE-08 (whose `number` slot is the `Slider`'s alone), `choice`
/// PHASE-09 (whose `index` needs the drawn alternatives to locate the held id).
fn field_value(state: Option<&Edited>, kind: &DrawnKind, today: &(Date, Time)) -> FieldValue {
  match state {
    Some(Edited::Checked(checked)) => FieldValue {
      checked: *checked,
      ..FieldValue::default()
    },
    // The text a person typed, verbatim. It is what the guard compares itself
    // against — string against string, so the comparison is an identity — and
    // what the `LineEdit`'s binding reads (design.md §5.2's comparand table).
    Some(Edited::Typed(text)) => FieldValue {
      text: text.as_str().into(),
      ..FieldValue::default()
    },
    // **What the button shows, and what its picker reopens on** — one pick,
    // read two ways. The text is the same RFC 3339 rendering `draft.rs`
    // submits, so a person sees the value that will leave the host rather than
    // a prettier one; that agreement is asserted at the screen and at the wire
    // in one case rather than held by a shared formatter, because the two
    // spellings answer to different rules — `R-57` governs the wire and
    // nothing governs the screen (design.md §5.2, §7 D3).
    //
    // `decompose` is pure: the offset the pick was resolved in travels in the
    // draft beside the instant, so reopening the picker reads no clock and no
    // zone (design.md §5.4).
    Some(Edited::Picked { instant, offset }) => {
      let (date, time) = instant::decompose(*instant, *offset);
      FieldValue {
        text: instant
          .instant()
          .display_with_offset(*offset)
          .to_string()
          .into(),
        date,
        time,
        ..FieldValue::default()
      }
    }
    // **Untouched, and `datetime` is the one kind that can say so.** The
    // button reads [`NOT_SET`] while the wire carries the epoch, and the
    // picker opens on today rather than on 1970 — seeding from `as_drawn`
    // would put D-6's sentinel on the screen in the one place D-6 chose it to
    // keep out of (design.md §7 D21).
    None if matches!(kind, DrawnKind::DateTime) => FieldValue {
      text: NOT_SET.into(),
      date: today.0.clone(),
      time: today.1.clone(),
      ..FieldValue::default()
    },
    // Untouched for a kind whose default slot *is* what it was drawn showing,
    // and the two kinds no control reads a slot for yet. One arm because
    // `clippy::match_same_arms` is `deny` and splitting them is an error while
    // the answers agree; PHASE-08 and PHASE-09 part them.
    None | Some(Edited::Adjusted { .. } | Edited::Chosen(_)) => FieldValue::default(),
  }
}

/// A `Vec` as the model the generated array member takes. Named once because
/// the row builders above nest two levels of it.
fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
  ModelRc::new(VecModel::from(items))
}
