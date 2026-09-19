//! `crates/goad/src/glass.rs` — stratum 3, and the only file in the crate
//! that names a generated type outside `generated.rs` itself (design.md
//! §5.3).

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
  FieldBlock, FieldRow, FieldValue, Kind, OptionRow, PromptWindow, Tray, WindowMode,
};
use crate::reception::Prepared;
use crate::view_model::{Body, DrawnKind};

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
  #[must_use]
  pub fn new(window: PromptWindow, tray: Tray, options: Rc<VecModel<OptionRow>>) -> Self {
    tray.set_image(tray_icon(TrayState::Idle));
    tray.set_hover_text(tooltip(&Diagnostics::default(), false).into());
    Self {
      window,
      tray,
      options,
      shown: None,
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
        let (rows, values) = option_models(prepared);
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
/// **`FieldBlock` names two types**, `design.md` §5.2's Slint struct and the
/// mapper's, and this file holds both. Only the generated one is named here
/// now — the mapper's is reached through `option.blocks` rather than written
/// down — so the bare name is unambiguous in this file and a phase that has to
/// name the other again should path-qualify it.
fn option_models(prepared: &Prepared) -> (Vec<OptionRow>, Vec<FieldValue>) {
  let mut values: Vec<FieldValue> = Vec::new();
  let mut rows: Vec<OptionRow> = Vec::new();

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
        values.push(field_value(
          prepared.draft.state_of(&option.id, &field.id).as_ref(),
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

/// One field's state channel: what the draft holds, in the slot the field's
/// kind makes meaningful.
///
/// A **lookup**, never stored in the row model as truth: the draft is the
/// authority and this is its projection for one present.
///
/// **`None` is *untouched*, and the glass reads it directly rather than
/// through `view_model::as_drawn`.** For four of the five kinds the two
/// coincide by construction, so a slot left at its default is what the field
/// was drawn showing; for `datetime` they do not, and the button reads *not
/// set* while the wire carries the epoch. That divergence is D-6's, and
/// routing the glass through `as_drawn` is exactly what would erase it
/// (design.md §5.2).
///
/// **Only `checked` is written**, because `boolean` is the only kind
/// `undrawn_form` draws and no control reads another slot yet. The match is
/// total over `Edited` all the same — that is what makes a value the draft can
/// hold and the screen cannot show a compile error rather than a default. Each
/// remaining arm is filled by the phase that draws its control and owns this
/// value arm in its Surfaces: `text` PHASE-05, `datetime` PHASE-07 (with the
/// `date` and `time` seed slots), `number` PHASE-08 (whose `number` slot is the
/// `Slider`'s alone), `choice` PHASE-09 (whose `index` needs the drawn
/// alternatives to locate the held id).
fn field_value(state: Option<&Edited>) -> FieldValue {
  match state {
    Some(Edited::Checked(checked)) => FieldValue {
      checked: *checked,
      ..FieldValue::default()
    },
    // Untouched, and the four kinds no control reads a slot for yet. Two
    // different statements with the same answer today, and they are one arm
    // because `clippy::match_same_arms` is `deny` and splitting them is an
    // error while the answers agree. PHASE-07 is where they part — an
    // unpicked `datetime` reads *not set* — and splitting the arm is that
    // phase's first move.
    None
    | Some(
      Edited::Typed(_) | Edited::Adjusted { .. } | Edited::Chosen(_) | Edited::Picked { .. },
    ) => FieldValue::default(),
  }
}

/// A `Vec` as the model the generated array member takes. Named once because
/// the row builders above nest two levels of it.
fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
  ModelRc::new(VecModel::from(items))
}
