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
use crate::pending::Pending;
use crate::reception::Prepared;
use crate::view_model;
use crate::view_model::{Body, DrawnKind, resolve};

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
  /// holds, and it now holds for two different reasons rather than one.
  ///
  /// **Two presents showing the same `view_id` are showing the same
  /// structure**, and that is a precondition on the caller rather than a
  /// property of the glass's types (§5.5 I-A). `Prepared`'s fields are public
  /// and the test tiers build one by hand, so nothing here prevents one id
  /// from carrying two structures; what holds it in production is
  /// `State::issue` minting a fresh id per view.
  fn present(&mut self, frame: Frame<'_>);
}

/// The production glass. Fields are private; the handles are strong clones
/// taken in `start` (PHASE-10) and live for the process.
pub struct SlintGlass {
  window: PromptWindow,
  tray: Tray,
  options: Rc<VecModel<OptionRow>>,
  /// The view whose structure the row model currently holds, or `None` for a
  /// glass that has shown nothing. Read to decide whether this present has to
  /// rebuild the rows at all (§5.3).
  shown: Option<ViewId>,
  /// **The same `Rc` the callbacks hold.** Two `Pending` values give an
  /// overlay that never overlays anything, with every case still green and
  /// nothing measured (§8 R10) — which is why `main.rs` creates it once and
  /// clones it into both halves.
  pending: Rc<Pending>,
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
  pub fn new(
    window: PromptWindow,
    tray: Tray,
    options: Rc<VecModel<OptionRow>>,
    pending: Rc<Pending>,
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

    // **The order of the three writes is I-F, and it is load-bearing.**
    //
    // `values` first, because `set_vec` instantiates the rows and a row
    // evaluates `root.values[field.slot]` *while* it is being instantiated.
    // With the rows written first, a new view's rows index the previous
    // view's shorter array — which Slint answers with a default-initialised
    // `FieldValue` rather than an error, a zero that looks like a value.
    // Writing the new view's values while the old rows still index them costs
    // nothing, because the next statement destroys those rows.
    //
    // The rows second, and **only where the view changed**: repeating over
    // them destroys every element beneath, so a present that rebuilt them
    // unconditionally would destroy the widget a person is typing into on
    // every tray check.
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
    // Wrapping, because the epoch is a change signal and not a count: an
    // `i32` that saturated would stop firing the guard, and one that
    // overflowed would panic in a debug build. Every wrap is still a change.
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
/// by construction rather than by two numberings kept in step (§5.5 I-B).
///
/// One `OptionRow` per retained option, carrying the presentation's
/// `PresentationOption`, the option's drawn fields, and the view token the
/// markup hands back on `chosen` and on `edited` (design.md §5.3, R-14, D10,
/// D19); one `FieldValue` per drawn field, in the order the rows number them.
///
/// Everything here is rebuilt from scratch on every present: nothing is
/// cached, so nothing has an invalidation rule to get wrong. The value channel
/// now has a second source — `pending.rs` — and that survives the property,
/// because `pending.rs` is not a cache: it is live state with one writer and a
/// stated lifetime, read at the instant it is needed and never copied anywhere
/// that could go stale.
fn option_models(prepared: &Prepared, pending: &Pending) -> (Vec<OptionRow>, Vec<FieldValue>) {
  let view = prepared.view_id.as_str();
  let mut values: Vec<FieldValue> = Vec::new();
  let mut rows: Vec<OptionRow> = Vec::new();

  for option in &prepared.presentation.options {
    let mut blocks: Vec<FieldBlock> = Vec::new();
    for block in &option.blocks {
      let mut fields: Vec<FieldRow> = Vec::new();
      for field in &block.fields {
        // The slot is read *before* the value is pushed, which is the whole of
        // I-B: there is no second counter to fall out of step with the
        // vector's own length.
        let slot = i32::try_from(values.len()).unwrap_or(0);
        values.push(field_value(
          drawn(prepared, pending, option, field).as_ref(),
        ));
        fields.push(FieldRow {
          id: field.id.as_str().into(),
          label: field.label.as_str().into(),
          kind: kind_of(&field.kind),
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
      view: view.into(),
      blocks: model(blocks),
    });
  }

  (rows, values)
}

/// **A field's value is the draft's, overlaid** (design.md §5.3).
///
/// The draft is what the host has recorded; where `pending.rs` holds an entry
/// for this (option, field) *made on the view being presented*, that entry is
/// what the person is looking at and it wins. Where `resolve` refuses the
/// entry — a renderer bug, §5.2 — the draft's value stands, and the refusal is
/// reported by the command that carries the entry rather than by this present.
///
/// The overlay goes through `resolve` rather than through a second mapping,
/// and that is the point of routing it this way: `pending.rs` holds
/// `Reported`, the display is written from `Edited`, and a
/// `Reported` -> `FieldValue` mapping beside the existing
/// `Edited` -> `FieldValue` one is exactly the duplication this design has
/// been avoiding everywhere else.
///
/// Without it, any present landing inside the 150 ms window — and `serve`
/// presents before every command it handles — writes the draft's older value
/// into the slot, the guard sees a difference, and the widget is corrected to
/// a value the person replaced 40 ms ago.
fn drawn(
  prepared: &Prepared,
  pending: &Pending,
  option: &view_model::PresentationOption,
  field: &view_model::PresentationField,
) -> Option<Edited> {
  let held = prepared.draft.state_of(&option.id, &field.id);
  pending
    .shown(
      prepared.view_id.as_str(),
      option.id.as_str(),
      field.id.as_str(),
    )
    .and_then(|reported| resolve(&reported, held.as_ref(), &field.kind))
    .or(held)
}

/// One field's state channel.
///
/// `None` is *untouched*, and it is read directly rather than run through
/// `as_drawn` — which is what keeps D-6's epoch a fact about the wire rather
/// than a fact about the screen. For the two kinds this renderer draws so far
/// the two coincide by construction: an untouched `boolean` is unticked and an
/// untouched `text` is empty, which is exactly what `as_drawn` answers. The
/// kind that will not coincide is `datetime`, whose button reads *not set*
/// while the wire answers the epoch.
///
/// A value of the wrong kind for the field leaves the slot at its default. It
/// is not reachable from the callbacks — `controller::edit` resolves against
/// the field's own `DrawnKind` — and it is a display, not a decision, so there
/// is nothing here to refuse to.
fn field_value(state: Option<&Edited>) -> FieldValue {
  let mut value = FieldValue::default();
  match state {
    Some(Edited::Checked(checked)) => value.checked = *checked,
    Some(Edited::Typed(text) | Edited::Adjusted { text, .. }) => value.text = text.as_str().into(),
    Some(Edited::Chosen(_) | Edited::Picked { .. }) | None => (),
  }
  value
}

/// The markup's kind discriminant, which selects which control is drawn and
/// which slot of the value struct is meaningful.
///
/// Exhaustive with no `_` arm, like every other match over a host-local kind:
/// a sixth kind is a compile error here rather than a field that draws
/// nothing. Three of the five name a control this phase does not draw; the
/// markup's `if` chain answers them with nothing until the phase that adds it.
fn kind_of(kind: &DrawnKind) -> Kind {
  match kind {
    DrawnKind::Boolean => Kind::Boolean,
    DrawnKind::Text => Kind::Text,
    DrawnKind::Number(_) => Kind::Number,
    DrawnKind::Choice { .. } => Kind::Choice,
    DrawnKind::DateTime => Kind::Datetime,
  }
}

/// A `Vec` as the model the generated array member takes. Named once because
/// the row builders above nest two levels of it.
fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
  ModelRc::new(VecModel::from(items))
}
