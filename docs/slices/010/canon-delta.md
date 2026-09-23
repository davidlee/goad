# Canon delta — Slice 010

One entry per affected document (`docs/AGENTS.md` §Canon that does not exist
yet, or must change). Each names the document, the section, the change **as it
will be stated**, and why. Nothing here is applied mid-slice: it is applied at
audit, with explicit user endorsement, and recorded in `audit.md`'s
Reconciliation table.

New canon is not here — the draft spec this slice writes is a whole document and
lives in `draft-spec.md`.

---

## SPEC-003 (host event ingress)

§7's R-4 cell (Change 1) and R-3 cell (Change 3), which land together, and §9
References (Change 2), which stands alone. **No requirement of SPEC-003
changes**, and no requirement id is added, removed or renumbered. The changes
are listed in the order they are applied, which is not their numbering: Change
3 falls due with Change 1 and follows it.

### Change 1 — §7, R-4's verification cell (AC-8)

**What is wrong.** One sentence in the cell, in the middle of an otherwise sound
row:

> **The non-zero exit is review, not a test**: no test target links the binary,
> and `main`'s single `match run()` (`crates/goad/src/main.rs`) maps every `Err`
> to exit 2.

It carries two defects of different kinds.

- *Stale.* **No test target links the binary** was a proxy for *no test can
  observe the exit status*, and that has been false since slice 006 added
  `crates/goad/tests/binary/`, whose cases spawn the built binary through
  `CARGO_BIN_EXE_goad` and assert the status it answers. The proxy is worth
  naming: that target does not *link* the binary either — nothing does — so
  replacing the sentence with a corrected version of the same claim would
  restate a distinction that was never the point. Canon stated a fact about the
  tree and nothing re-read it when the tree moved — the rot `CLAUDE.md`
  §Working here names, in the place it rots worst.
- *A defect recorded as a virtue.* **`main`'s single `match run()` maps every
  `Err` to exit 2** is not an argument that the requirement is verified. It is
  the conflation this slice exists to repair, written into a normative document
  as though it were the mechanism holding a requirement. Left as it stands, a
  reader would reasonably take SPEC-003 to be relying on it.

**What it will say.** The sentence above is replaced, in place, by:

> **The non-zero exit is held by a case on the built binary, by the shape of
> the classifier, and by review.**
> `exit_codes::an_unbindable_ingress_path_exits_2`
> (`crates/goad/tests/binary/exit_codes.rs`) spawns the built binary against a
> configuration naming an ingress path it cannot bind and asserts the status a
> caller reads. `startup::listener` (`crates/goad/src/startup.rs`) runs before
> the first Slint call, so this settles headlessly, as the other cases in that
> target do. For the variants no case reaches, what
> holds the status is the **shape** of `exit::status`
> (`crates/goad/src/exit.rs`): a single `Err` arm that reads no `StartupError`
> variant, so one number is answered for every variant and a variant cannot be
> filed elsewhere without that arm being edited. No test holds that universal.
> What `exit_status::every_startup_failure_is_2`
> (`crates/goad/tests/renderer/startup.rs`) holds is a sample — it names
> representative variants rather than counting them, and asserts the number for
> each; and review that `main` routes `run`'s `Err` there and nowhere else.
> **Which number that is, and what a reader may infer from it, is SPEC-00N's and
> not this document's.**

`SPEC-00N` is a placeholder: promotion substitutes the number the draft spec is
assigned, in this cell and in Change 2 below. A cell left holding the
placeholder is a promotion that did not finish.

**Everything else in the cell stands** — the integration case names, the
`display_text` and `stderr_outlets` renderings, and the *two halves, two cases*
paragraph with its F-21 citation. Only the sentence quoted above is touched.

**Why the case is written in this slice rather than deferred.** It was deferred
at round 1 on the ground that it verifies a requirement of SPEC-003 and not one
of this slice's, which was sound until round 3 showed the repair cannot land
without it (F-26). SPEC-003 §7's own preamble is the constraint: *"A row naming
no test is a row this spec may not be amended holding: where a clause **cannot**
be reached by a test, the row says so in terms."* The sentence being replaced
qualified for that escape by claiming unreachability, falsely. Removing the
false claim removes the escape, so an amended cell that still named no test
would breach the preamble it is written into. The case is the cheaper of the two
honest options; amending the preamble is the other, and it was not taken.

### Change 3 — §7, R-3's verification cell (falls due with Change 1)

**Why.** R-3's cell says its `LivenessUnknown` arm has *"the same position as
R-4's exit code below and R-5's process exit"*. After Change 1 that is false:
`LivenessUnknown` is a value no cooperating test **can** produce, while R-4's
exit is now held by a named case. This was always the event the narrowing fell
due on (`slice-010.md` §Follow-ups, before it was struck), and Change 1 is that
event.

**What it will say.** One phrase in the cell, in the sentence beginning
*"`BindFault::LivenessUnknown`"*:

> the same position as R-4's exit code below and R-5's process exit

is replaced, in place, by:

> the same position as R-5's process exit

The rest of the sentence and of the cell stands. R-4 leaves the analogy rather
than being contrasted inside it: after Change 1 its exit is held by a case, and
a clause saying so would be R-4's cell's business restated in R-3's. R-5's half
of the analogy is left as it stands and is **not** vouched for here: whether a
cooperating test can reach a killed host's process exit is R-5's cell's
question, and this slice does not settle it (`review-design.md` F-59).

**Not separable from Change 1.** Applying Change 1 without this one leaves
SPEC-003 asserting an equivalence its own amended cell contradicts.

### Change 2 — §9 References (separable)

**Why.** After Change 1, R-4's cell defers the meaning of the status to another
document. SPEC-003's §9 lists every document it abuts, and a deferral with no
entry there is a seam a reader has to find by grepping.

**What will be added**, as a bullet in the existing list:

> - **SPEC-00N** (process exit status) — the numbers a startup failure exits
>   with, and what a consumer may infer from each. R-4's failure is one of the
>   causes filed under *never started*; this document says it is a startup
>   failure and that one says what that costs the process.

**Separable from Change 1.** If the user endorses only the verification-cell
repair, Change 1 stands alone and is complete without this; the cell names the
other document in prose either way.
