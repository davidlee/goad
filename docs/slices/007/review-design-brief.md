# Review brief — Slice 007 design

Hand this to a fresh agent. It is the statement of what was asked; it is not
canon and it is not a finding. Delete it or keep it once `review-design.md`
resolves.

---

You are the **adversarial design reviewer** for slice 007, "the renderer grows a
form", in the goad repository. Your job is to attack `docs/slices/007/design.md`
and the draft canon it rests on, and to record what you find in
`docs/slices/007/review-design.md` — a ledger that already exists, with its
header and Brief filled in.

The repository is at `/home/david/dev/goad` unjailed and `/workspace/goad` inside
the bwrap jail; `git rev-parse --show-toplevel` settles it. There is no display in
the jail, so nothing visual is observable from there by anyone.

## Read first, in this order

1. `CLAUDE.md`, then `docs/AGENTS.md`. The methodology and the five invariants.
   Not optional.
2. `docs/slices/007/review-design.md` — **the ledger's Protocol section is
   self-contained and binding on you**: roles, append-only ids, the severity and
   disposition tables, and the guardrails. Read its Brief too; it states what the
   design's author expects to be wrong.
3. `docs/slices/007/design.md` — the subject. All of it.
4. `docs/slices/007/canon-delta.md` — CD-1, the proposed **R-57** and **R-58**.
   This is a **draft**: nothing outside slice 007 may cite it, and attacking it
   is explicitly part of your job.
5. `docs/slices/007/slice-007.md` — scope, tier, AC-1..AC-10.
6. `docs/slices/007/research.md` — the verified research the design cites. Its
   rows are **unmarked**, meaning "a claim with its site named", not "checked".
   Where the design load-bears on one, check it.
7. `docs/specs/001-host-backend-protocol.md`, all of it. `docs/adr/001`, `003`,
   `004`. `docs/policy/001`. `docs/specs/002-host-scheduling-behaviour.md` §8.

`docs/slices/007/design-log.md` is the conversation that produced the design. Read
it **after** you have formed your own view, not before — it is persuasive by
construction, and reading it first is how a reviewer talks themselves into
`aligned`.

## What to attack, in priority order

**1. A-2, and with it §5.4 and AC-5.** The design assumes that `set_vec` on each
present re-establishes `FieldRow.checked` from the draft — that a `CheckBox` which
assigned its own `checked` on click does not stay permanently detached from the
model. Slint drops a declarative binding when the property is assigned
imperatively; whether the model reset rebuilds the repeater's elements or updates
them in place is the open half.

This may be settleable **on paper** from the pinned sources, and if you can settle
it you save the first implementation phase a session. Read pinned sources only:
`~/.cargo/registry/src/*/i-slint-{core,compiler}-1.17.1/` unjailed, or in the jail
the checkout bound at `/workspace/slint`, which carries the **`v1.17.1` tag** —
`git show v1.17.1:<path>`. Its working tree has drifted to 1.18 and is good for
ideas, never for API facts. The repeater and `ModelNotify`/`Model::set_vec` paths
in `i-slint-core-1.17.1/internal/core/{model,item_tree}.rs` are where the answer
is.

**2. D6, the design's load-bearing structural claim.** §5.1 and I-3 assert that
because `answer()` walks the presentation's **declared fields** and `Draft`
exposes no key enumeration, SPEC-001/R-58 holds in both directions "with no check
to forget". Try to break it. Is there any path by which an undrawn field enters a
`FieldBlock`, or a draft key reaches the wire, or a field is drawn that the walk
would miss? If there is, the property degrades to a convention and the design
says otherwise.

**3. R-57 and R-58 as canon.** Are they falsifiable as worded? Is a backend author
able to write against them without reading the design? Does §7's convention really
verify an **outbound** requirement by unit tests in `canonical.rs` — check what
the R-8, R-2 and R-6 rows actually do — or has the design invented a precedent?
Is the split into two requirements right, or has it produced two rules that
cannot be tested independently? Is `datetime` having "no defined submitted form"
a defined state or a hole a conforming host could fall into?

**4. The claim that no ADR is needed.** §10 rests it entirely on SPEC-001 §1
already carrying D1's rationale. `docs/AGENTS.md` says "any decision that shaped
the draft and could later be reversed by accident gets its own ADR". Is D1 such a
decision? Is D5 or D11?

**5. `boolean`-only, argued against its author.** The design says drawing one kind
of five is a legitimate renderer subset under R-55, and that R-57 typing all four
is what keeps it from being a narrowing. Argue the other side properly: is there a
reading under which shipping a renderer that draws 20% of the field kinds
*produces the effect of* a narrowing, whatever the spec says? SPEC-001 §1 and §2's
boundary sentence are the texts.

**6. The remaining surface**, at whatever depth is left: P-1..P-3 as stated versus
as applied; the grouping rule against every edge case in §5.5; whether AC-10's
bound ("legibility") is actually statable or will collapse into taste; whether
§9's validation table has a row that asserts a proxy rather than the behaviour
(`docs/memory/tests-asserting-proxies.md` is the precedent, and the design claims
to have learned from it).

## Specific claims the design makes that you should check rather than accept

- That `crates/goad-semantics/` needs **no change**. Every accessor named in §5.2
  should exist and be `pub`.
- That exactly **one** test asserts `Undrawn::OptionFields` and exactly **one**
  match arm consumes it.
- That `material`'s `Button` and `CheckBox` expose the same accessible surface as
  `fluent`'s, so the style change does not disturb the headless tier.
- That `structure.rs:308-318` bans the identifier `resolve` in
  `crates/goad/src` production lines, and that nothing in the design proposes one.
- That the vocabulary scan does **not** cover `group`, and that `FieldBlock` and
  `heading` are not host domain concepts wearing layout names.
- That an array-typed struct member (`blocks: [FieldBlock]`) is supported —
  §5.5/A-1 rests on a parse-error test and on `[[StandardListViewItem]]`
  properties, which are related but not the same thing.

## Rules

- **Raise findings against the artefact, not against your preferences.** Each
  finding needs Expected / Observed / **Evidence**, where evidence is a citation
  that makes the claim checkable by someone who disagrees with you.
- **You are the raiser, not the responder.** Do not dispose of your own findings.
  Leave `disposition`, `response` and `outcome` blank; the author disposes with
  the user, per the ledger's Protocol.
- Set severity at raise time and do not negotiate it with yourself.
- A ledger with no findings is **not done** — it means the review has not run. If
  you genuinely find nothing at a given depth, say at what depth you looked.
- Change no code, amend no canon, edit no document but `review-design.md`. In
  particular **do not edit `design.md`** — a design defect is a finding, not a
  repair.
- `just check` is not yours to run.
- If the work heads past roughly 200k tokens, write what you have into the
  ledger, say which threads are unexamined, and stop. A second agent continues
  from the file; a truncated final report is not a substitute for a complete
  ledger on disk.
- If the environment contradicts this brief — a path that does not exist, a tool
  that is absent — say so in the ledger and work around it.
