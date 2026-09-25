# Canon delta — Slice 011

One entry per affected document (`docs/AGENTS.md` §Canon that does not exist
yet, or must change). Each names the document, the section, the change **as it
will be stated**, and why. Nothing here is applied mid-slice. It is applied at
audit, with the user's explicit endorsement, and recorded in `audit.md`'s
Reconciliation table.

**Vocabulary.** Canon text below says **"update of the surface"** where code
and `design.md` say *present*. SPEC-001's `Presentation` is the backend's view,
and a canon sentence about "presentations" would read as being about that.
SPEC-003 §7's current R-12 row does say "presentations", and that row is
rewritten here anyway.

---

## SPEC-003 (host event ingress)

The changes: the text of the R-15 requirement, and the R-15 and R-12 cells in
§7. No requirement id is added, removed or renumbered.

### Change 1 — §4, R-15's requirement

**Why.** R-15 requires an idle refusal to reach the diagnostics surface, but
says nothing about when. Today it reaches the surface at once, because each
refusal costs a full update of the surface, and a writer sets the rate. On the
running host that pins the UI thread (`research.md` Thread 3). The user chose to
coalesce on both edges at an interval that lives in code (`design-log.md`,
OQ-1, OQ-2).

This amendment states the rule that repair follows. It is careful to show that
the rule is not an exception to R-12: R-12 forbids holding back **envelopes**,
and nothing here holds one back. P-D requires any new absolute to name its
exception. For the new *within* clause that exception is the loop ending, and
the deciding side is the host.

**As it will be stated** (the whole cell). New: the *within* clause that ends
the first sentence, the three sentences after it, and the last clause of the
final sentence. The rest is the current text.

> A refusal the host decides **while no exchange is in flight** MUST also be
> reported on the host's own diagnostics surface, so that it is visible to a
> person who is not the writer, and MUST reach that surface **within a fixed
> interval** of being decided, unless the host's loop ends first. A writer the
> host does not control sets the rate of refused envelopes, so the updates of
> that surface they alone cause MUST NOT exceed one per interval: an envelope
> refused when no such update has been made within the last interval is shown
> at once, and one refused inside an interval is shown when that interval ends
> — as the latest refusal decided by then — unless another update of the
> surface has carried it sooner. The interval is the host's own: fixed, not
> configurable, and not visible to a writer. It spaces updates of the surface
> and nothing else — no reply waits for it (R-8, R-12), and it begins no
> evaluation (SPEC-002/R-12). A refusal decided while an exchange *is* in
> flight, and one decided after the host's loop has ended, are reported to the
> writer only. This is a bound on what the surface can hold, not a licence to
> be silent: **every envelope's** refusal reaches its writer in the reply R-8
> requires. The one refusal that reaches no writer is the one that answers no
> envelope — the ingress-stopped `unavailable` of §6.3, for which this surface
> is the only report there is; it is shown at once and never held to the
> interval.

The number is not in canon. It is `REFUSAL_PRESENT_INTERVAL` in
`crates/goad/src/controller.rs`, the way ADR-004's floor is `MINIMUM_SPACING`.
The last clause records `design.md` D6: that refusal is never coalesced.

### Change 2 — §7, R-15's verification cell

**Why.** The positive case reads `Served.controller`'s retained `Diagnostics`
after the loop has stopped. That read stays green even if nothing is ever shown
(FU-2; memory `a-refusal-is-recorded-not-shown`). R-15 now also carries a
bound and a rule, and neither can be observed anywhere but the window.

**As it will be stated** (the whole cell). It is laid out in paragraphs and
bullets here so it can be read. At promotion it goes into §7's table as one
cell, with the breaks joined by `;`, the way the current cells are written.

> renderer, all `crates/goad/tests/renderer/ingress.rs`, and every case below
> reads the **window**, not the retained `Diagnostics`, because a refusal that
> is folded but never shown is exactly what a read of the retained value cannot
> see.
>
> **The bound and the rule.**
>
> - `ingress::a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once`:
>   a lone refusal after a quiet interval is on the window at once. So is a
>   second one after a further quiet interval, so the edge re-arms and does not
>   hold only at startup.
> - `ingress::a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest`:
>   a writer sends distinct refused envelopes flat out across several
>   intervals. It costs the surface at most one update per interval beyond the
>   first. While the flood is still running, the window's line never stays
>   unchanged for longer than the interval plus a stated slack; a debounce,
>   which shows nothing until the writer pauses, fails that. When the flood
>   ends, the window names the last envelope's refusal.
> - `ingress::a_command_during_a_coalesced_interval_presents_at_once_and_carries_the_refusal`:
>   an update the interval is holding back does not hold back one a person
>   asked for, and that update carries the refusal.
>
> **The negative.**
> `ingress::a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`
> is what makes the bound a claim rather than an excuse. It reads the **live**
> window while the exchange is still running. `absorb` replaces the retained
> `Diagnostics` on the way out, so a read at the end cannot tell a branch that
> shows nothing from one that shows the refusal and is wiped afterwards
> (`docs/slices/004/review-code.md` F-23).
>
> **The last clause, from both sides.**
> `ingress::a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
> and
> `ingress::ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`:
> the ingress-stopped `unavailable` is the one refusal that answers no
> envelope, and it reaches the surface whether the loop was idle or
> mid-exchange when ingress died.
>
> **Review, not a test.**
>
> - `serve`'s `select!` polls the deadline that ends an interval ahead of
>   arrivals, so a writer that keeps arrivals always ready cannot starve it. In
>   this tier the accept task shares the loop's thread, so no flood keeps
>   arrivals ready on every poll, and no case can tell the two orders apart.
> - A refusal still not shown when the loop ends is never shown. The stop path
>   shows nothing to anyone.

### Change 3 — §7, R-12's verification cell

**Why.** The cell cites
`ingress::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`
and records *one presentation per refusal* as a measured fact ("845/845, 1.000
per refusal, ~1690/s — F-15's settlement"). Change 1 makes that ratio wrong by
design. The figure also came from the headless tier and does not describe the
running host (`research.md`, *Cross-thread findings*). So the figure is dropped,
not replaced. The case keeps its R-12 half under a name that says only that, and
the presentation claim moves to R-15's cell (`design.md` D13).

**As it will be stated.** One clause of the cell is replaced, and the rest
stays verbatim. The old clause:

> `ingress::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`
> — a writer emitting flat out produces a bounded number of evaluations over a
> window far shorter than the spacing, the excess replies name the bound, and
> the same test **records** the number of presentations the host makes over
> that window (measured 845/845, 1.000 per refusal, ~1690/s — F-15's
> settlement) rather than merely detecting a rate;

The new clause:

> `ingress::a_flat_out_writer_raises_no_evaluation_rate` — a writer emitting
> flat out produces a bounded number of evaluations over a window far shorter
> than the spacing, and the excess replies name the bound. What the same flood
> costs a person's surface is R-15's, and is verified there;

---

## SPEC-002 (host scheduling behaviour)

### Change 4 — §7, R-12's verification cell (citation only)

**Why.** The cell cites the same case by its old name. **Its claim does not
change**: the words it attaches to that case are already exactly the R-12 half
the renamed case keeps. This is the one entry outside SPEC-003, and it is
mechanical.

**As it will be stated.** In the cell's first sentence,

> `crates/goad/tests/renderer/ingress.rs::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`

becomes

> `crates/goad/tests/renderer/ingress.rs::a_flat_out_writer_raises_no_evaluation_rate`

and nothing else in the cell changes.

---

## Checked, and not changed

- **SPEC-003 §6.3** (*Which refusals a person sees*) and **§5** (*When ingress
  stops*). Where they say a refusal *reaches* the surface, it still does, and
  R-15 now bounds when. The grouping by which side decides stands.
- **SPEC-003 P-C.** Nothing an envelope receives is delayed; what is held back
  is the surface update.
- **ADR-004, SPEC-002/R-4.** The new deadline writes neither anchor, and it
  begins no evaluation (`design.md` I-4).
- **No new ADR.** The rule lives in R-15. The reversal most likely to happen
  by accident is a throttle turning into a debounce, and T2's M3 guards
  against it (`design.md` §9).
