# Canon delta — Slice 011

One entry per affected document (`docs/AGENTS.md` §Canon that does not exist
yet, or must change). Each entry names the document and the section, gives the
change **as it will be stated**, and says why. Nothing here is applied
mid-slice. It is applied at audit, with the user's explicit endorsement, and
recorded in `audit.md`'s Reconciliation table.

**Vocabulary.** Canon text below says **"update of the surface"** where code
and `design.md` say *present*. SPEC-001's `Presentation` is the backend's view,
so a canon sentence about "presentations" would read as being about that view.

**The count rule** (`CLAUDE.md`, name never count). Every count in a passage
this delta touches is either replaced or justified as exempt. "Eight" as the
size of the reason set is exempt. It is a closed list that a test holds:
`ingress::the_reason_token_set_is_closed_at_eight`, R-14's row.

---

## SPEC-003 (host event ingress)

The entries below touch the R-15 requirement, §6.3, and §7's R-15 and R-12
cells. No requirement id is added, removed or renumbered.

### Change 1 — §4, R-15's requirement

**Why.** R-15 says an idle refusal reaches the diagnostics surface, but not
when. Today every idle refusal reaches it at once, because each one costs a
full update, and the writer sets the rate. On the running host that pins the UI
thread (`research.md` Thread 3). The user chose to coalesce on both edges, with
the interval living in code (`design-log.md`, OQ-1, OQ-2).

The user also chose to word R-15 around what the code guarantees: an update of
the surface within the interval, showing the latest refusal decided by then
(`design-log.md`, F-1/F-2). The rule states that guarantee, and P-D requires
each exception to be named:

- **Overwrite.** `Controller::refuse` replaces the whole slot. A refusal that
  is overwritten before the update is never shown. What the slot retains is
  FU-3's question.
- **Loop end.** An update still due when the loop ends is never made.
- **Clock overflow.** In that fallback, every refusal updates the surface, so
  the per-interval limit does not hold.

The host decides all of them. The overwrite is the host's even though the
refusal that overwrites may have been decided elsewhere, for example a shape
refusal decided by whatever accepts connections: the surface holds one fold
because the host chose that retention (FU-3).

The rule is not an exception to R-12. R-12 forbids holding back **envelopes**,
and this rule holds back only updates of the surface.

**As it will be stated** (the whole cell; everything from the colon in the
first sentence through the sentence naming §6.4 is new, as is the final clause):

> A refusal the host decides **while no exchange is in flight** MUST also be
> reported on the host's own diagnostics surface, so that it is visible to a
> person who is not the writer: the surface MUST be updated **within a fixed
> interval** of that refusal being decided, and the update shows the latest
> refusal decided by then. Its exceptions are the host's own. A refusal
> overwritten on the surface before that update is never shown, whichever side
> decided the later refusal, because the surface holds what the host chooses to
> retain, and that is not this requirement's. An update still due when the
> host's loop ends is not made. A writer the host
> does not control sets the rate of refused envelopes, so updates of the
> surface caused only by refused envelopes MUST NOT exceed one per interval: a
> refused envelope decided when no such update has been made within the last
> interval is shown without waiting for the interval, and one decided inside an
> interval is shown when that interval ends, unless another update has come
> sooner. That limit yields to the host's own arithmetic: where the end of an
> interval cannot be represented on the host's clock, the interval is not held
> and every refusal updates the surface. The interval is the host's own —
> fixed, not configurable, and not visible to a writer, which is why §6.4 does
> not list it. It spaces updates of the surface and nothing else: no reply
> waits for it (R-8, R-12), and it begins no evaluation (SPEC-002/R-12). A
> refusal decided while an exchange *is* in flight, and one decided after the
> host's loop has ended, are reported to the writer only. This is a bound on
> what the surface can hold, not a licence to be silent: **every envelope's**
> refusal reaches its writer in the reply R-8 requires. The one refusal that
> reaches no writer is the one that answers no envelope — the ingress-stopped
> `unavailable` of §6.3, for which this surface is the only report there is;
> it is shown without waiting and is never held to the interval.

The number is not in canon. It is `REFUSAL_PRESENT_INTERVAL` in
`crates/goad/src/controller.rs`, the same way ADR-004's floor is
`MINIMUM_SPACING`.

**Why not §6.4.** §6.4 lists the bounds a watcher's author has to know. This
interval is invisible to every writer and changes no reply, so it does not
belong there. The cell says so, so that nobody later reads its absence from
§6.4 as an omission.

### Change 2 — §6.3, *Which refusals a person sees*, and the `unavailable` paragraph

**Why.** Two statements become false under Change 1 (review F-3). Both lists
are grouped by the side that decides the refusal, and that grouping stands.
The new exceptions are the host's, as Change 1 states: its retention, whatever
side decided the refusal that overwrites, and its stop.

**As it will be stated.** Each of the following replaces the quoted text
exactly. The rest of §6.3 is unchanged.

1. The lead-in, "Only those the host decides while no exchange is in flight
   also reach the diagnostics surface a person reads (R-15). **Which of the
   eight that is turns on which side decided the refusal**", becomes:

   > Only those the host decides while no exchange is in flight are also
   > reported on the diagnostics surface a person reads, on R-15's terms.
   > **Which of the eight those are turns on which side decided the refusal**

   "Eight" is kept; the count rule above explains why.

2. In the first bullet, "so the loop's state on arrival is what decides their
   fate: they reach the surface when it happened to be idle, and **not
   otherwise**." becomes:

   > so the loop's state on arrival is what decides whether they may reach the
   > surface: only when it happened to be idle, and **not otherwise** — and
   > then on R-15's terms, so one overwritten by a later refusal before the
   > surface is next updated is not shown.

3. In the second bullet, "reachable only when it is idle, so for these two
   *always* is exact." becomes:

   > reachable only when it is idle, so the loop's state never withholds them.
   > What can withhold one is R-15's overwrite and loop-end exceptions: a
   > later refusal overwriting it before the surface is next updated, or the
   > loop ending first.

4. In the `unavailable` paragraph, the counts of its causes, which no test
   holds, are replaced:
   - "**`unavailable` covers four causes, and one of them does not pass.**"
     becomes "**`unavailable` covers the causes below, and one of them does not
     pass.**";
   - "The fourth cause is" becomes "The last cause is";
   - "The reason set remains closed at the eight above — what this admits is a
     fourth cause of one of them, not a ninth token." becomes "The reason set
     remains closed at the eight above — what this admits is one more cause of
     one of them, not a new token."

### Change 3 — §7, R-15's verification cell

*Amended by `review-code.md` F-1:* the T2 bullet's count is "the whole
intervals their span holds" (rounded down), not "the number of intervals their
span covers" (rounded up), which admitted one extra update on every run.

**Why.** The positive case reads `Served.controller`'s retained `Diagnostics`
after the loop has stopped, so it stays green even if nothing is ever shown
(FU-2; memory `a-refusal-is-recorded-not-shown`). R-15 now carries a bound and
a rule, and the timing of an update is observable only where the update
happens.

**As it will be stated** (the whole cell). It is laid out here in paragraphs
and bullets for reading. At promotion it becomes one table cell, with the
breaks joined by `;` as the current cells are.

> renderer, all `crates/goad/tests/renderer/ingress.rs`. Every case below
> reads the **window**. At each update inside `serve`, the test glass first
> delegates to the real glass, then records the instant together with the
> window's own mode and diagnostic lines. Timed claims are measured from that
> record. They are not read from the retained `Diagnostics`, which cannot show
> whether anything was displayed, nor from a poller, whose lag would count
> against the bound.
>
> **The rule and the bound.**
>
> - `ingress::a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once`
>   — a refusal decided after a quiet interval is shown within a fraction of
>   the interval. So is a later one decided just after an update a person
>   caused, once the interval has passed: the interval runs from updates that
>   refusals caused, and from nothing else.
> - `ingress::a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest`
>   — a writer sends distinct refused envelopes flat out across several
>   intervals. The deadline that ends each interval fires at most once per
>   interval, and a stall moves an update without adding one. So the flood's
>   updates are counted against one more than the whole intervals their span
>   holds, with a stated allowance for the first update's lag. An update
>   per refusal fails that count by orders of magnitude, and so does an interval
>   much shorter than the host's. While the flood runs
>   they are never further apart than twice the interval, which a debounce
>   fails. The last update names the last envelope's refusal, within
>   twice the interval of its reply.
> - `ingress::a_command_during_a_coalesced_interval_presents_at_once_and_carries_the_refusal`
>   — while a refusal is still unshown, a person's command updates the surface
>   within a fraction of the interval, and that update shows the refusal.
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
> `ingress::ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`
> — the ingress-stopped `unavailable` is the one refusal that answers no
> envelope. It reaches the surface whether the loop was idle or mid-exchange
> when ingress died, and when the loop was idle it does so well before any
> scheduled firing, so it is not held to the interval.
>
> **Review, not a test.**
>
> - A flood does not take the person's window. `serve` yields to the event
>   loop once per arrival because ingress holds one arrival at a time and
>   produces the next only after replying to the last (§6.4). No renderer case
>   observes the event loop's share of the thread; the witness is a person's
>   run.
> - The loop checks the deadline that ends an interval ahead of arrivals, so
>   the bound rests on the timer alone and not on that one-at-a-time shape.
> - An update still due when the loop ends is not made; the stop path shows
>   nothing to anyone.

### Change 4 — §7, R-12's verification cell

**Why.** The cell cites
`ingress::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`
and records *one presentation per refusal* as a measured fact ("845/845, 1.000
per refusal, ~1690/s — F-15's settlement"). Change 1 makes that ratio wrong by
design. The figure was also measured headless and does not describe the running
host (`research.md`, *Cross-thread findings*). It is dropped, not replaced
(`design.md` D13). The cell's count is also replaced.

**As it will be stated.** Two clauses change; the rest of the cell is verbatim.

1. The clause

   > `ingress::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`
   > — a writer emitting flat out produces a bounded number of evaluations over
   > a window far shorter than the spacing, the excess replies name the bound,
   > and the same test **records** the number of presentations the host makes
   > over that window (measured 845/845, 1.000 per refusal, ~1690/s — F-15's
   > settlement) rather than merely detecting a rate;

   becomes

   > `ingress::a_flat_out_writer_raises_no_evaluation_rate` — a writer emitting
   > flat out produces a bounded number of evaluations over a window far
   > shorter than the spacing, and the excess replies name the bound. What the
   > same flood costs a person's surface is R-15's, and is verified there;

2. "The anchor's independence in **three** directions:" becomes

   > The anchor's independence, in each direction a case below names:

---

## SPEC-002 (host scheduling behaviour)

### Change 5 — §7, R-12's verification cell (citation, and its counts)

**Why.** The cell cites the same case by its old name. **Its claim does not
change.** The words it attaches to that case are already exactly the R-12 half
the renamed case keeps. The cell also counts its independence cases twice
("three"), and no test holds that count; the user extended this change to
replace both counts, as Change 4 does for SPEC-003's R-12 cell
(`design-log.md`, *audit: dispositions and canon endorsement*). The cases
themselves are already named in the cell, so the rewrite quantifies by rule.

**As it will be stated.** Each of the following replaces the quoted text
exactly. The rest of the cell is unchanged.

1. > `crates/goad/tests/renderer/ingress.rs::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`

   becomes

   > `crates/goad/tests/renderer/ingress.rs::a_flat_out_writer_raises_no_evaluation_rate`

2. > **The independence of the two anchors** (§3 P-E), in all three directions rather than one:

   becomes

   > **The independence of the two anchors** (§3 P-E), in each direction a case below names, not in one alone:

3. > Each of the three was shown to fail when the anchor it holds is broken, rather than merely observed to pass

   becomes

   > Each of those cases was shown to fail when the anchor it holds is broken, rather than merely observed to pass

"Two anchors" is kept: it names R-12's own closed pair (R-4's scheduled
anchor and this one), not a list that can grow.

---

## Checked, and not changed

- **SPEC-003 §5**, *When ingress stops*: the ingress-stopped fold is still
  reported, and it is not coalesced (Change 1's final clause).
- **SPEC-003 §6.4**: the interval does not belong there (see *Why not §6.4*
  under Change 1).
- **SPEC-003 P-C**: nothing an envelope receives is delayed.
- **ADR-004, SPEC-002/R-4**: the new deadline writes neither anchor and begins
  no evaluation (`design.md` I-4).
- **No new ADR**: the rule lives in R-15. `design.md` §9's controls M3
  (debounce) and M8 (interval from any update) guard the reversals most likely
  to happen by accident.
