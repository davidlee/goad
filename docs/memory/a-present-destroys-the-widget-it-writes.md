# A present destroys the widget it writes, and that is load-bearing

Measured by the throwaway spike at commit `a698217`, which the next commit
deleted. Read that commit for the code; this note is the durable part.

## The fact

`SlintGlass::present` ends in `self.options.set_vec(rows)` — and `set_vec`
calls `ModelNotify::reset()`, whose `RepeaterTracker::reset` does
`instances.clear()` (`i-slint-core/model/repeater.rs`). **Every row element in
the form is dropped and rebuilt on every present.** The loop presents at the top
of every iteration, including after an `Edit`, so ticking one checkbox rebuilds
the whole form.

`set_row_data` does not do this. It notifies `row_changed`, which calls
`comp.update(row, data)` on the **existing** instance. Nothing is destroyed.

Measured: five presents in place construct **zero** elements; one `set_vec`
constructs one per row.

## Why the destruction is load-bearing, and not an oversight

`fluent/checkbox.slint:27` is `root.checked = !root.checked`. A property
*assignment* in Slint removes whatever binding the use site declared, so after
the first click `checked: field.checked` has stopped tracking the model
**permanently**.

Measured, not inferred: click the box, then `set_row_data` a row saying
`false`, and the widget stays `true`. The rebuild is the only thing that gives
it a fresh binding — and a rebuilt element does converge, also measured.

So design §5.4's A-2 — *the draft is the authority; the present corrects a click
the host dropped* — is resting on the destruction specifically. Switching to
`set_row_data` for the layout benefit alone silently breaks it.

## The repair that keeps both

Re-assert imperatively instead of relying on the binding, guarded by a
comparison, triggered by an epoch the host bumps on every present:

```slint
property <int> tick: root.epoch;
changed tick => {
    if (self.checked != field.checked) { self.checked = field.checked; }
}
```

Measured under a real event loop: the handler fires once per row per present;
it writes **nothing** when widget and draft agree; and it converges a clicked
widget whose binding is gone, with the element preserved throughout.

The guard is the whole point. Writing nothing when nothing diverged is what
lets a caret, a text selection or a slider drag survive a present — there is no
mechanism by which an untouched widget could be disturbed.

## How to apply

- Treat `set_vec` in a present as a **rebuild**, and say so wherever it is
  written. It is not a cheaper `set_row_data`.
- Anything that re-establishes a widget's state on a present must say which of
  the two mechanisms it depends on — a live binding, or an imperative write.
  They fail in opposite conditions.
- Before concluding a widget tracks its model, click it first. An unclicked
  widget's binding is intact and will converge on its own, so a test that never
  interacts measures nothing (`a-green-test-can-assert-a-proxy.md`).

Related: `change-handlers-need-an-event-loop.md`,
`a-green-test-can-assert-a-proxy.md`,
`a-scroll-view-is-a-size-barrier-both-ways.md`.
