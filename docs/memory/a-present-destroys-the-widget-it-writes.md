# A present destroys the widget it writes, and that is load-bearing

Measured by the throwaway spike at commit `a698217`, which the next commit
deleted. Read that commit for the code; this note is the durable part.

## The fact

`set_vec` calls `ModelNotify::reset()`, whose `RepeaterTracker::reset` does
`instances.clear()` (`i-slint-core/model/repeater.rs`). **Every row element
under a repeater is dropped and rebuilt.** `set_vec` in a present is a
*rebuild*, not a cheaper `set_row_data`.

`set_row_data` does not do this. It notifies `row_changed`, which calls
`comp.update(row, data)` on the **existing** instance. Nothing is destroyed.

Measured: five presents in place construct **zero** elements; one `set_vec`
constructs one per row.

**Where `set_vec` is called from is a separate decision, and it has moved.**
The spike measured a `present` that called it unconditionally, so every present
rebuilt the form and ticking one checkbox destroyed the widget that was ticked.
`SlintGlass::present` now calls it **only where the `view_id` it is showing has
changed**, and it is not the last thing the function does — the epoch is
(slice 009, design.md §7 D8, §5.5 I-F). So a present that changes nothing
destroys nothing. That does not retire this note: the repair below is what
makes the *rebuilding* present safe, and a new view still takes that path.

## Why the destruction is load-bearing, and not an oversight

`fluent/checkbox.slint:27` is `root.checked = !root.checked`. A property
*assignment* in Slint removes whatever binding the use site declared, so after
the first click `checked: field.checked` has stopped tracking the model
**permanently**.

Measured, not inferred: click the box, then `set_row_data` a row saying
`false`, and the widget stays `true`. The rebuild is the only thing that gives
it a fresh binding — and a rebuilt element does converge, also measured.

So *the draft is the authority; the present corrects a click the host dropped*
was resting on the destruction specifically. Switching to `set_row_data` for the
layout benefit alone would silently have broken it — and so would the
conditional `set_vec` the code now has, which is why the repair below landed in
the same slice as the condition.

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

## Two paths reset a repeater, and retaining the model closes only one

Found at slice 009's audit (`review-code.md` F-B9, then F-C6, which is F-B9's
repair being wrong about what it held).

Handing the window a **fresh** `ModelRc` on every present rebuilds everything,
because `ModelRc`'s `PartialEq` is `core::ptr::eq`. The obvious repair is to
retain one `Rc<VecModel<_>>` and `set_vec` into it. **That is not sufficient**,
and the reason is worth keeping:

```
VecModel::set_vec   =  *self.array.borrow_mut() = new.into();  self.notify.reset();
RepeaterTracker::reset =  self.is_dirty.set(true);  self.inner.borrow_mut().instances.clear();
```

`reset` clears every instance **without consulting the model pointer at all**.
So an unconditional `set_vec` of *identical* content still destroys and rebuilds
every element, on every present, for as long as the repeater is up. Retaining
the model closes the **pointer** path and leaves the **mutation** path wide
open. Measured with the `inits` counter below: 1 → 2 → 3 across three presents
with the write unguarded, 1 → 1 → 1 with it guarded.

**So every `set_vec` in a present needs a guard, and the guard's subject depends
on what the model tracks.** Slice 009 ships both and the asymmetry is
deliberate:

- the **form**'s rows change only when the view does, so the guard is on
  identity — `if self.shown != showing`;
- the **diagnostics**' lines change independently of the view, so the guard is
  on **content** — `row_count`, then element-wise, in `glass.rs::write_if_changed`.

An identity guard on the diagnostics would miss every change; a content guard on
the form would be a pointless pass over rows that only ever change together with
the view.

One cost, stated honestly because slice 009 first stated it wrongly: on the path
that **does** write, the content comparison is work **added**, not shared.
`set_vec` is a move plus `notify.reset()` — it allocates nothing and touches no
element, so it does none of the element-wise work a guard might be imagined to
be reusing.

## How to measure it

Element identity is not otherwise observable, so count construction in the
markup:

```slint
out property <int> inits;      // `out` suffices — assignable from inside a
…                              // repeater, and does not widen the input surface
CheckBox { init => { root.inits += 1; } }
```

For a **repeater** this is not merely convenient, it is the only instrument
there is: a repeater has no `ChangeTracker` behind it, so no `changed` handler
can reach it.

An element that is destroyed and recreated runs `init` again; one that is
updated in place does not. Add a second counter inside the re-assert's guard and
the pair separates three states that otherwise look alike: *not rebuilt*, *no
write needed*, and *written back*. This is what turns "does a present disturb
the widget" from an argument into a number.

Do the same for anything else whose identity matters. Reading the widget's
*value* cannot tell you — a rebuilt element and a preserved one both end up
holding the model's value, which is precisely why the defect survives ordinary
assertions.

## How to apply

- Treat `set_vec` in a present as a **rebuild**, and say so wherever it is
  written. It is not a cheaper `set_row_data`.
- Anything that re-establishes a widget's state on a present must say which of
  the two mechanisms it depends on — a live binding, or an imperative write.
  They fail in opposite conditions.
- Before concluding a widget tracks its model, click it first. An unclicked
  widget's binding is intact and will converge on its own, so a test that never
  interacts measures nothing (`a-green-test-can-assert-a-proxy.md`).
- A widget the host has been told about and one it has not are indistinguishable
  to a guard that reads only the model. If an edit can be in flight — debounced,
  queued, or refused — the value channel has to carry what is in flight as well
  as what is recorded, or the guard corrects a person mid-sentence
  (slice 009, design.md §7 D26).

Related: `change-handlers-need-an-event-loop.md`,
`a-green-test-can-assert-a-proxy.md`,
`a-scroll-view-is-a-size-barrier-both-ways.md`.
