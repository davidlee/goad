# Audit & reconciliation — Slice 006: packaging and the startup surface

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `4f9fb9d..732f0dc` — the whole slice, twenty-three commits, five
phases and two code-review rounds. `4f9fb9d` is the joint design-and-plan
ledger resolving; `732f0dc` is the handover this audit picks up from.

**Question.** For this slice to be finished, five things have to be true, and
this audit intends to check four of them itself.

1. **The binary a person now runs is the one the slice built, and it works.**
   Nine acceptance criteria, of which three (AC-1's second half, AC-2's, AC-8's)
   cannot be discharged by any test and were discharged by a person watching
   the software run. `docs/AGENTS.md` §Tiers is blunt about why that clause
   exists: slices 001–003 all closed green on a binary that could not open a
   window. So the question is not whether the gate is green — it is whether
   `notes.md` names **what was observed**, specifically enough that a reader
   who was not there can tell the observation from the intention.

2. **Every verification criterion in `plan.md` was actually discharged, and
   still is.** Twenty-five VT/VA/VH across five phases. Two populations are
   suspect and the audit separates them: criteria held by a **standing test**,
   which the gate re-checks on every run, and criteria discharged by a
   **one-time observation** in the phase that wrote them — a store path read, a
   generated unit printed, an `env -i` run. The second population can stop
   being true without anything going red. Three exit criteria are additionally
   known to have been falsified by repairs landed at audit (`review-code.md`
   F-8), and the phases they belong to stay `done`; that is a reconciliation
   row, not a defect, but an audit walking `plan.md` from the top would read it
   as three unmet criteria and this document has to say otherwise.

3. **Nothing was touched that no phase declared, and nothing declared was
   dropped.** No stage of this slice has diffed the paths actually changed
   against the surfaces each phase declared. `docs/AGENTS.md` calls undeclared
   paths the strongest lead available to an audit, and this is the audit's one
   genuinely unexamined surface. Both directions are checked: undeclared-and-
   touched (scope creep, or a design change taken quietly), and declared-and-
   untouched (dropped work, or a stale design).

4. **The record is true about the code, and canon is true about both.** Five
   reconciliation rows are already owed and carried in `notes.md` §Open, three
   of them amendments to canon. The class they belong to is the one this
   repository has watched rot repeatedly: **a claim nothing re-reads.** A spec's
   count of siblings, a spec's citation by line number, a doc comment's
   cardinality — each true when written and falsified by an edit somewhere
   else. The audit's own contribution here is to check the class rather than
   accept the list: are those five all of them?

5. **The ledger is closed.** `review-code.md`'s Protocol defines *Done* as every
   finding `verified` or `withdrawn`. Ten findings, no blockers, none contested;
   F-7..F-10 carry dispositions and Responses and **empty `Outcome:` lines**.
   Those four are a raiser's call on repairs the responder made, and they are
   owed before anything else in this document can claim the review is resolved.

**Invariants the slice is held to.** `CLAUDE.md`'s five, of which three can bear
on this work and are checked rather than assumed: **no domain vocabulary in
host code** — and the boundary scan does not read `.nix`, so `nix/module.nix`
and `flake.nix` are held by a human read (PHASE-02/VA-2) and by nothing else,
which makes them worth re-reading here; **a backend failure never takes the
host down** — untouched, no protocol surface moved; **strata run one way**
(ADR-001) — OQ-3 answered *stratum 3*, so `goad-shell` should be untouched and
that is checkable rather than assertable. Two more bind from outside
`CLAUDE.md`: **POL-001's phase gate**, which OQ-2 promised not to amend and
whose six commands must still be present, in order, unweakened and
unconditional; and **SPEC-003 R-3 / R-4**, which require ingress startup
failures to name their path and which AC-6 extends rather than narrows.

**Tier.** 1 (thin), unraised. The one deviation — `design.md` over the 300-line
cap — was an explicit user decision recorded at the design's head and in
`design-log.md`, taken as a deviation rather than as a tier change. An audit may
not lower a tier to make a closing argument easier, and does not here.

**Where the bodies are likely buried.** In descending order of expected yield:
the surface delta, because nobody has looked; the one-time observations, because
nothing re-checks them; the *person ran it* evidence, because that is the clause
this repository has failed before; and the completeness of the reconciliation
list, because every row on it was found by someone looking at something else.

<!-- This is the audit's scope — evidence, criteria, canon. The code review's
     own lines of attack belong in `review-code.md`'s Brief, not here. -->

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

### Tests and checks

`just check` — **exit 0**, run at `732f0dc` on 2026-09-22. `just -n check`
prints POL-001 §Compliance's six commands and was compared against the policy's
fenced block line for line: **identical, in order**. Nothing is weakened,
removed or made conditional, which is what OQ-2 promised when it declined to
put `nix build` on the gate.

Three further checks were run by this audit rather than inherited:

- **ADR-001, measured not asserted.** `git diff --name-only 4f9fb9d..HEAD`
  names no file under `crates/goad-semantics` or `crates/goad-shell`. OQ-3's
  *stratum 3* answer therefore holds by construction: no stratum was crossed in
  a new direction because no lower stratum was touched at all.
- **The vocabulary invariant where the scan cannot reach.** `domain_scan` reads
  `rs` and `slint` only, so `flake.nix`, `nix/module.nix` and the `justfile` are
  held by a human read and by nothing else. The same seven words were applied
  to all three by hand. Two hits, neither domain vocabulary: `journalctl` in the
  module header (one word, and `mentions` matches whole words singular or
  plural, so a widened scan would not flag it) and `§Compliance` in the
  `justfile` header, which is POL-001's own section name and **would** flag —
  noted because it is the one token that would make a widened scan red for a
  reason that has nothing to do with the invariant.
- **The citation-rot class across all of canon, not only the three sites
  `notes.md` had found.** `docs/specs/`, `docs/policy/` and `docs/adr/` were
  swept for `path:line` in both spellings. Exactly three, all in SPEC-003's R-3
  and R-4 verification rows, all three wrong — the enumeration `notes.md`
  carried was complete. `CLAUDE.md:98` is the rule's own illustration, not a
  citation.

### Acceptance criteria

| | verdict | evidence |
|---|---|---|
| AC-1 | **met** | PHASE-01/EX-2 builds; VA-7 a window with text headless; PHASE-05/VH-1 the person's own words, *"diagnostics window shows: nothing to report"*, from the tray's Diagnostics pane — which draws unconditionally, so the backend having nothing to say is orthogonal to the criterion |
| AC-2 | **met** | PHASE-01/EX-3 builds; PHASE-05/VH-3 — the nix-built `goad-emit`, no `--socket`, reading the ingress path out of the configuration, exit 0 into the running host |
| AC-3 | **met** | PHASE-01/VA-4 under `env -i`; VA-6 the wrapper. **Re-read at audit** from the store path the unit actually runs: five `LD_LIBRARY_PATH` prefixes (gcc-lib, fontconfig-lib, libglvnd, libxkbcommon, wayland) and `export FONTCONFIG_FILE=${FONTCONFIG_FILE-…}` — deferential for fontconfig, prefixing for libraries, which is the split `review-design.md` F-1 and F-9 argued |
| AC-4 | **met** | PHASE-03/VT-1, VT-2, VT-3 — standing tests, green at audit. **Re-run at audit:** the store binary prints `0.1.0 (22f412c)` on stdout at exit 0 |
| AC-5 | **met** | PHASE-05/VA-1. **Re-run at audit**, both at exit 0: `<store>/bin/goad --version` → `0.1.0 (22f412c)`; `~/.cargo/bin/goad --version` → `0.1.0`. The parenthetical is the only difference |
| AC-6 | **met** | PHASE-04/VT-1 and VT-2 — standing, and strengthened after `review-code.md` F-7, whose mutation this audit re-ran: routing the read failure into the parse arm reds one case and only it |
| AC-7 | **met as written; its stated reason is false** | The unit systemd holds was **re-read at audit**: `Description`, `Restart=on-failure`, `RestartPreventExitStatus=2`, `RestartSec=2`, `ExecStart` the store path character-for-character, no `EnvironmentFile`, `After`/`PartOf`/`WantedBy` all `graphical-session.target`. The mechanical half is discharged. The criterion's *because* — *exit 2 is every `StartupError` and none of them succeeds on a retry* — is false for `Platform`, and the journal shows that is the only exit-2 that has ever occurred. See §Verdict |
| AC-8 | **met** | `just check` exit 0 at audit; VH-1 above is the person's evidence, which `docs/AGENTS.md` §Tiers requires and a green gate does not supply |
| AC-9 | **met** | PHASE-05/VA-2, `just install` green. **Re-run at audit:** `~/.cargo/bin/goad --version` → `0.1.0` at exit **0**, where before this slice it exited 2 reading `--version` as a configuration path |

### Verification criteria

Twenty-five across five phases: six VT, sixteen VA, three VH. **All twenty-five
discharged.** The audit separates them by what holds them *now*, because that is
the question a criterion's discharge does not answer.

**Held by a standing test** — the gate re-checks these on every run, and did at
audit: PHASE-01/VT-1 (vacuous: the phase writes no Rust), PHASE-03/VT-1, VT-2,
VT-3, PHASE-04/VT-1, VT-2.

**Re-verified by this audit**, though nothing obliged them to still be true:
PHASE-01/VA-5 (the gate block, compared to POL-001 line for line), VA-6 (the
wrapper, read from the store), PHASE-02/VA-2 (the vocabulary scan's words
applied by hand to `.nix`), PHASE-04/VA-1 (`StartupError` has ten variants —
counted), PHASE-05/VH-2 (the loaded unit), VA-1 and VA-2 (both binaries' version
lines).

**One-time observations, accepted as recorded and re-checkable only by
repeating the phase:** PHASE-01/VA-1, VA-2, VA-3, VA-4, VA-7; PHASE-02/VA-1,
VA-3; PHASE-03/VA-1; PHASE-04/VA-2; PHASE-05/VH-1, VH-3, VA-3. Each is written
in `notes.md` with its verbatim output — store paths, the generated attrset, the
`env -i` run — which is what makes them evidence rather than assertion. **VH-1
is the one that cannot be re-run by anything**: it is a person's observation of
a window, and `docs/AGENTS.md` §Tiers exists because slices 001–003 closed green
without it.

Three exit criteria and two design statements were falsified by repairs landed
at audit. They are not unmet criteria and the phases stay `done`; see §Design
drift not reconciled.

### Surface delta

`git diff --stat 4f9fb9d..HEAD` — twenty-three files. **No declared surface went
untouched**, and every declared path appears in the diff.

**Two paths were touched that no phase declared**, and both are in one commit:

| path | phase surface? | fate |
|---|---|---|
| `.gitignore` | declared by no phase, and not in `slice-006.md` §Scope | **accounted for.** Raised as a PHASE-01 finding — the repository had no rule for `nix build` out-links and only the user's global file covered the bare name, not `result-*`. Put to the user; endorsed |
| `docs/slices/006/design.md` | a slice artefact, not a phase surface | **accounted for.** PHASE-02 declined to carry the hand-written unit's `Description` across, correctly, since §5.2(d) gave `Unit` as `After` + `PartOf` only — and **raised it rather than deciding it**. Put to the user as a design amendment; endorsed; §5.2(d) amended **forward**, then implemented. Not a retro-fit |

Commit `c67262e` says so in terms: *"Two user-endorsed calls taken between
PHASE-03 and PHASE-04, neither belonging to a phase's declared surface."* The
strongest lead available to an audit was followed and found the STOP-and-consult
protocol working, with the decisions recorded in `design-log.md` and the
superseded PHASE-02 decision marked as superseded in its own sheet.

**One declared-but-untouched path remains, by design:** `slice-006.md` §Scope
names `docs/roadmap.md` *"at close"*. It is written at close, in this document's
Closure checklist.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** resolved · outstanding blockers: **none**

Three rounds. Ten findings — one major, four minor, five nits — every one
`verified`, none contested, no `blocker` raised in any round. Round 3 was a
verification pass rather than a fresh sweep, on the trend both prior syntheses
measured and stated: round 1 found one code defect, round 2 one, round 3 none.
Its reasoning and its declared residue are in the ledger's own Synthesis.

## Verdict

**The slice does what it set out to do.** Everything between *built* and
*running daily* now exists and is in use: a crane build produces a wrapped
binary that is self-contained anywhere, a home-manager module in this repository
builds the unit from a store path, both binaries answer `--version`, and every
`StartupError` that has a path in hand names it. The hand-written unit outside
the repository is retired. All nine acceptance criteria are met, all twenty-five
verification criteria discharged, the gate is green, and a person has watched
the packaged binary draw a window with text in it — which is the clause this
project has failed before and the reason it is written down.

**Three things are being accepted knowingly.**

**1. `RestartPreventExitStatus=2` suppresses the one restart that would
succeed.** This is the audit's own finding and the most serious thing in this
document. `nix/module.nix` argues the directive from three `StartupError`
variants and concludes *none of those succeeds on a retry* about all ten. The
omitted one is `Platform`: `start` ends
`run_event_loop_until_quit().map_err(StartupError::Platform)`, so a compositor
that goes away under a host which has been running for hours exits 2 exactly as
a host that never started does — and it is the **only** exit-2 that has ever
occurred on this machine. Measured in the journal: four such exits on 2026-09-21
and 2026-09-22, two of them followed by no restart at all, leaving the user's
intervention shell absent for 2h12m and 1h59m. The three restarts that did
follow came 13–20s later, which is not `RestartSec=2` — the session
re-triggering `WantedBy`, not systemd retrying.

The directive behaves exactly as written. What is wrong is the argument for
writing it, and AC-7 states that argument in terms. The mapping is inherited
(PHASE-08 of an earlier slice) and the directive came from the hand-written
unit, but 006 is where it became this repository's contract — OQ-1's whole case
for the module living here. Dispositioned by the user as a **follow-up slice**:
separating *never started* from *stopped running* is an exit-code taxonomy
change reaching the startup surface and SPEC-003's failure vocabulary, not an
edit to a unit file. What landed in-slice is the honest comment —
`nix/module.nix` now names `Platform` as the known exception and says the
directive is knowingly wrong in that one case, so the next reader cannot
re-derive the same conclusion from the same three variants.

It is a clean instance of *verify the enumeration, not the conclusion*: three
named, ten in the enum, and the one omitted is the one that fires.

**2. The two binary tiers still hold four helpers twice.** Transcribed
deliberately (PHASE-03/VT-2: do not invent a second convention), cheap at two
copies, and nothing at stratum 3 is shared because neither crate may depend on
the other. A third binary tier is where this needs an answer rather than a third
copy. Carried, not settled.

**3. `extraConfig`'s type admits shapes home-manager rejects** —
`review-code.md` F-5, dispositioned `follow-up` in round 1 and already a
follow-up in `slice-006.md`. Which shapes are legitimate is a judgement about
the option surface rather than a defect in it, and the wrong narrowing costs a
consumer a directive they were entitled to.

**What the audit checked and found nothing in.** The surface delta, in both
directions — the two undeclared paths were each raised as a finding and endorsed
before being taken, and no declared surface was dropped. ADR-001, by
construction: the slice touches neither lower stratum. POL-001, compared line
for line. The domain-vocabulary invariant, applied by hand where the scan cannot
reach. The citation-rot class across the whole of canon rather than the three
sites already known. And the ledger's own claims: F-7's mutation was re-run in
the working tree rather than read from its Response, and reproduced to the case.

**One correction to the record, made at audit.** PHASE-05 recorded that the
packaged binary had repaired a tray-icon defect the cargo binary had. Its own
journal, read at audit with eight post-cutover starts available instead of one,
falsifies it: five carry `Slint: Failed to create system tray icon: 0` and three
do not, on the packaged binary throughout. The failure is intermittent and
independent of the install path, which retires both candidates `notes.md` §Open
was carrying — including the one that would have been a defect in what this
slice shipped. The observation stands; the inference drawn from a sample of one
did not.

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/003-host-event-ingress.md` R-4 verification row | *"rendered beside its **eight** siblings"* → *"rendered beside **every other `StartupError` variant**"* | document stale, code right — PHASE-04's split made it nine, and the count is the thing that goes stale. **Canon: endorsed by the user, 2026-09-22**, who chose removing the count over correcting it | [x] |
| `specs/003-host-event-ingress.md` R-3 and R-4 verification rows | three `path:line` citations replaced by symbols — `main`'s single `match run()`; `BindFault::LivenessUnknown`; the `TryLockError::Error` arm of `hold` | document stale, code right. All three were wrong; **two were wrong before this slice opened**. Nothing re-reads canon on the commit that moves a line. **Canon: endorsed 2026-09-22** | [x] |
| `CLAUDE.md` §Working here | *cite by symbol, never by line number* becomes *name, never count — and cite by symbol*: one rule, two halves, one reason, binding canon explicitly | neither cleanly stale nor a defect — a rule decided inside this slice at `review-code.md` F-4, applied to two doc comments, and then found unswept eleven lines away at F-10. **Canon: endorsed 2026-09-22** | [x] |
| `nix/module.nix`, the `Restart` comment | names `Platform` as the exception the three-variant enumeration omits, and says the directive is knowingly wrong in that one case | code right, comment wrong — the enumeration draws a conclusion about ten variants from three. **Endorsed 2026-09-22**; the real repair is a follow-up | [x] |
| `notes.md` §Findings, §What was observed, §Open | the tray claim corrected against the journal; the §Open entry closed as **settled** | the record was wrong, the code is not. **Endorsed 2026-09-22** | [x] |
| `docs/roadmap.md` §006 | a closing line | `slice-006.md` §Scope names it *"at close"* | [x] |

**Nothing in `plan.md` or `design.md` was retro-fitted.** `design.md` §5.2(d)
was amended **forward** during execution, before the code it describes was
written, with user endorsement (`design-log.md`, `c67262e`) — which is a
different act from making a design agree with code that already departed from
it.

**Design drift not reconciled:** two repairs taken at audit reversed statements
the plan and the design make, and both repairs were right. `design.md` is a
record of intent at a point in time and stays as written; the departure is
recorded here instead.

| statement | says | the tree, after the repair |
|---|---|---|
| `plan.md` PHASE-03/EX-3 | the **caller** passes `option_env!(…).filter(…)` | callers pass `option_env!("GOAD_REVISION")` bare |
| `plan.md` PHASE-03/EX-5 | `goad-emit` takes the same `option_env!` filter | same — the filter is inside `render::version_line` |
| `design.md` §5.2(g) | *"Callers pass `option_env!(…).filter(…)`"* | same |
| `plan.md` PHASE-04/EX-3 | `StartupError`'s doc comment says **ten** variants | it states no count, by rule |
| `design.md` §5.2(f) | *"The doc comment's 'eight variants' becomes ten"* | same |

Both repairs also supersede an argument `notes.md` records as a decision —
PHASE-03's *"`option_env!` sits in each `main`, not inside `version_line`"*,
whose second half `review-code.md` F-2 measured false, and PHASE-04's *"the
enum's doc says **ten** because the plan requires a count there"*, which F-4
replaced with the rule now in `CLAUDE.md`. The decisions stand as the record of
what was decided then; this table is what says they were superseded.

A third statement is superseded by the audit itself and is not drift: AC-7's
*because* clause, which §Verdict item 1 falsifies. It is recorded there rather
than here because it is a finding about the code's behaviour, not a divergence
between the code and a document describing it.

**No drafts to promote.** This slice opened no `draft-spec.md` and no
`canon-delta.md`: the canon changes above are amendments found at audit, not
rules the slice was running on. `slice-006.md` §Governing canon predicted this
correctly — nothing here writes canon, which is why the tier stayed 1.

## Closure

- [x] All findings dispositioned; no blockers outstanding — ten in
      `review-code.md`, every one `verified`, none contested, no `blocker`
      raised in any of three rounds
- [x] All acceptance criteria met, or explicitly waived by the user — all nine
      met. **AC-7's *because* clause is false** and is annotated as such at the
      criterion; the criterion itself is discharged and the falsified reasoning
      is §Verdict item 1 and the first follow-up
- [x] Each verification criterion in `plan.md` walked against the code, or the
      gap measured and carried — twenty-five, all discharged, separated in
      §Evidence by what holds each one *now*
- [x] Tests and checks green — `just check` exit 0 at `732f0dc` and again after
      the audit's own edits; `just -n check` compared to POL-001 line for line
- [x] Specs / policy / ADRs reconciled, with user endorsement where amended —
      SPEC-003 twice, `CLAUDE.md` once, all endorsed 2026-09-22
- [x] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason
      written down — **none existed.** The slice ran on canon as it stood; the
      amendments above are rot found at audit, not rules the slice needed
- [x] `notes.md` §Open swept against `slice-006.md` §Follow-ups; every entry
      dispositioned — ten entries: five **settled** (three canon amendments, the
      design-drift table, the tray question closed by measurement), five
      **carried** as follow-ups, one of which is this audit's own finding
- [x] `slice-006.md` Summary and Follow-ups written
- [x] `notes.md` Harvest current; durable facts lifted to `docs/memory/` — seven
      new entries, one amended, and what deliberately stayed behind is named
- [x] `slice-006.md` stage set to `done` — it had read `planned` since PHASE-01
- [x] `docs/roadmap.md` §006 closed — the dated entry, the heading, the diagram
      node and the sequence table
