# Review — implementation — Slice 005

**Subject:** implementation — `git diff 03b0286..c785f13`, the whole of slice
005 from the accepted design forward: `crates/goad-emit/` (new), the client half
of SPEC-003 in `crates/goad-shell/src/ingress/`, PHASE-01's four lifts and the
call sites they left in `crates/goad/`, and the `examples/` + `justfile` edits.
**Reviewer:** fresh agent, Claude Opus 5, adversarial
**Opened:** 2026-09-14
**State:** open

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-14 — the whole diff, written before reading any of it.

This slice is *thin by construction* and that is exactly what makes it worth
attacking carefully: a thin slice's defects are not in its architecture, they
are in its edges. Six lines of attack, in the order I expect them to pay.

**1. The wire contract, in the direction nobody tested before.** Slice 004 wrote
the reply; 005 is the first code that *reads* one. The permissive-wire invariant
says emit must admit what SPEC-003 admits and refuse only what it must, and the
project's stated raison d'être is not narrowing a contract to what today's
producer happens to emit. So: is `protocol` optional for the *stated* reason or
for a convenient one; does an unknown `reason` token survive verbatim; does an
unknown *field* survive; is `retry_after_ms` read where §6.3 puts it and nowhere
else; and — the mirror question the design does not ask — is anything *widened*
that should have been refused? Permissiveness about fields the host does not
model is the invariant; permissiveness about the *meaning* of `accepted` or
`reason` is a bug wearing the invariant's coat.

**2. The failure taxonomy, which is the whole product of this slice.** Emit's
entire output is one integer. 0 accepted, 1 the host refused, 2 no usable
answer. Every path through `main` must land on the right one, and the boundary
that will be wrong if any is: a §6.3 breach reported as a refusal (exit 1 with
no reason token to print), or a refusal reported as a breach. I will enumerate
the paths rather than read for them, because an exit code is not visible in the
shape of the code that produces it. Related: "every refusal is reported and says
which side was wrong" is a CLAUDE.md invariant, so a fault that exits 2 silently,
or exits 2 with a line that does not say whose fault it was, is a finding.

**3. The lifts — the worst thing in this diff, if it is here.** PHASE-01 claims
four moves and no behaviour change. A move that quietly changed a condition, an
error variant, a default, or an ordering is worse than a bug written openly,
because the phase's own exit criterion (EX-6: no assertion changed) reads green
over it. I will diff each lifted item against its pre-lift form with `git show`
rather than reading the new form and judging it plausible. `wire::Reply` is the
one that is *not* a pure move — `protocol`/`accepted` became `Option`, `reason`
became owned — so the host's emitted bytes are the thing to pin.

**4. Tests that assert proxies.** The project's named failure mode
(`docs/memory/a-green-test-can-assert-a-proxy.md`) and slice 004 shipped four of
them. PHASE-04's cases were mutation-checked against six injected defects;
PHASE-01/02/03's were not all checked that way, so that is where I will look
hardest. The question for each case is not "does it pass" but "name the
regression it guards, and would that regression actually red it". Specific
suspects before looking: any case asserting `send` succeeded rather than
asserting what arrived; any `read_reply` case whose expected value is also
producible by the defect it guards; any `args::parse` case that would survive a
flag being dropped on the floor; and AC-6's case, whose design-stated risk (R-3)
is precisely this.

**5. Strata and the crate edge.** `goad-emit` is stratum 3: it may name both
strata below and nothing may name it. The clock lift moved a `jiff` user *down*
into stratum 2 — the question is whether that moved a feature, a dependency edge
or an impurity into a stratum that forbids it, and whether `goad-semantics`
still builds pure and alone under its own feature set. AC-7's claim is that the
crate edge holds Slint-freedom; OQ-3 decided no allowlist row, so the *only*
thing holding AC-7 is the manifest — I will check the manifest says what AC-7
says it says, and that the vocabulary scan's walk actually reaches the new
member rather than being assumed to.

**6. The residue the design already knows about, and whether the code widened
it.** Three places state the configuration-path rule and one is executable.
`#[expect]`s claiming infallible serialization. `unwrap_in_result` is live.
A `--help` text that must state the unbounded wait (D-10). Each is a place where
prose and code can disagree silently, and prose is what was reviewed twice
already.

**What I am not reviewing.** Comment density and prose style are deliberate
here and settled; I will not file them. Nor will I relitigate D-1..D-11 or
OQ-1..OQ-7 as design questions — they are accepted. A finding about one of them
must be that the *code* does not do what the decision says, not that the
decision was wrong.

**Round 2** — 2026-09-14 — the repairs, written before reading them.

Ten findings, ten `fix-now`s, two fresh agents, 498 cases where there were 450.
The thing a second round is for is **not** re-confirming that the reported fix
happened; it is finding what the fix broke. Four lines of attack.

**1. The class fix is the dangerous one.** F-1/F-4/F-5 were repaired together as
a re-typing of the wire and a re-shaping of the reply path. A repair that moves
a type from `u8` to `Value` moves *every* decision that type was making, and the
decisions it was making badly are easier to see than the ones it was making
well. So: what did `u8` refuse that should still be refused, and where does the
adjudication now happen instead? `whole_millis` is the new single point of
judgement over a number a caller acts on, and it is the place to be hardest —
huge floats, negatives, `-0.0`, integers past `u64`, values JSON can spell but
`f64` cannot hold. And the repair states a *rule*, not just a change: each step
owns exactly one fault. A rule stated as an absolute is a claim, and SPEC-003's
own P-D says a clause that cannot name its exception has not been checked. I
will look for the exception rather than take the claim.

**2. The bound, in both directions.** `REPLY_LIMIT + 1` then `read_until` is an
off-by-one that is wrong in *two* ways, not one: too tight refuses a conforming
reply, too loose leaves the defect open. The at-bound case is the one that is
easy to write green and vacuous, and the response already concedes it had to be
EOF-framed to mean anything — which is a good sign about the author and a
reason to check the reasoning rather than trust it. Separately: the host writes
through the same `Reply` struct, so *"the host's bytes are unchanged"* is a
claim about a type that was edited underneath it.

**3. The parser rewrite, which is the biggest behavioural surface.** F-2's fix
replaced a scan with a two-pass split. A rewrite that fixes one row of a table
is the classic place to lose another, so I will walk the old table row by row
against the new code rather than reading the new table for plausibility. The
ruling — a flag-looking token in value position is a value — is stated as
following from *"emit does not narrow the values the protocol admits"*, and it
is the kind of ruling that is either principled or a rationalisation; the test
is whether the same principle is applied where it costs something.

**4. The proxy repairs, which could be proxies.** F-7's fix is (variant, phrase)
tables. A phrase pin is only as good as the phrase: one that appears in two
arms' output, or one lifted from a *payload's* `Display` rather than from the
arm under test, pins less than it looks. Fourteen injections are claimed; I
will check whether the pins would catch the swaps *in the current text*, which
is a different question from whether they did on the day.

**What I will say plainly.** A round that only lists doubts is not a review. For
each of the ten I will say whether it is closed, and on what evidence — measured
on the built binary where the finding was measured on the built binary, so the
two rounds are comparable.

**Round 3** — 2026-09-14 — the completed class fix, written before reading it.

The user ruled *finish the class* rather than *weaken the clause*, which is the
harder of the two and the one that can go wrong. Round 2's finding was an
over-claim; the repair to it is a clause that claims to say exactly where it
stops. So the single most valuable thing this round can do is **test the stated
boundary against the true one** — not agree that a boundary was stated.

Four lines of attack, and the first is the round.

**1. Is the exception list exactly right?** The clause names two shapes
`serde_json` yields no value for. I will go looking for a third rather than
check the two, because a list of exceptions is only worth its completeness, and
P-D's standard is *where would the exception be*, not *here are some*. Candidate
classes I will pose before reading the code: anything that fails in the *parser*
rather than in the typing — depth, trailing input, encoding, and any duplicate
rule that does not apply uniformly to modelled and unmodelled names. And the
sharper version of the same question: is any stated exception stated **too
broadly**? A clause that over-warns is as unchecked as one that under-warns.

**2. Did the shape guard narrow anything?** `one_json_object` is a second parse
of the same bytes, and a second parse can refuse what the first accepted. If it
is stricter than the field parse anywhere — an *unmodelled* field above all —
then a repair aimed at precision has bought it with wire compatibility, which is
the trade this project exists to refuse. I will diff the two parsers'
acceptance, not read their intent.

**3. Is `bytes_that_are_not_one_json_object_are_unreadable` load-bearing, or
does it look load-bearing?** The claim is that it was a passenger before and is
not now. That is a claim about what the *old* code did with `[1,2]`, and it is
checkable by reconstructing the old struct — which I will do, because "it passes
now" and "it would red without the guard" are different facts and only the
second is worth anything.

**4. The write side's completeness claim.** Two things are said to hold it.
Two is a count, and a count is a claim. `pub` fields on a `pub` struct with no
`#[non_exhaustive]` is a third path or it is not; the question is answerable by
reading the type, and it is answerable in the repository's own test code.

**Not in scope, and I will not disposition it.** `arbitrary_precision` is a
manifest decision with reach into `goad-semantics`' protocol types; the repair
agent raised it under S-5 rather than taking it, which is what S-5 is for. It is
pending, not a defect.

**Round 4** — 2026-09-14 — the fourth statement of one clause, written before
reading it.

The clause about where `Unreadable` stops has now been written four times and
been wrong three. That history is the brief: **the failure mode is not the
clause, it is the method used to fill it in.** Rounds 2 and 3 each found a
missing exception by probing a class the author had not thought of, and the
repair each time was to add that class. A list filled in that way is complete
only up to the imagination of whoever last probed it, which is not a property
anything can rely on.

So this round does one thing first and everything else after: **enumerate
`serde_json`'s failure surface against the declared fields directly, rather than
check the three rows written down.** The hypotheses I will pose before reading
the list — depth, number range, duplicate names, string escapes, surrogate
pairs, encoding, trailing input, and the leading-byte forms — come from what a
JSON parser can refuse, not from what this module has previously refused. If a
fourth shape exists it will be in the gap between "what the author enumerated"
and "what the parser can do", and that gap is where I will look.

Three further things, each a claim the repair makes that is checkable rather
than arguable.

**The division of labour.** The byte answers only *yes*; every *no* falls to the
`Map` read. That is either true of the code or not, and it is the load-bearing
claim of the whole design, because a byte that could refuse would be a heuristic
deciding a caller's exit code. I will also ask the question behind it: given
that `serde_json` skips exactly four whitespace bytes and this function skips
exactly the same four, **what is left for the `Map` read to do**, and is its
`Ok` arm a live path or insurance? A claim of reachability is as checkable as a
claim of correctness, and this review has found the difference to matter.

**The unmodelled boundary, which is the half that is behaviour rather than
prose.** F-15(iii) was the one finding in three rounds that cost a caller an
exit code. The fix is a reorder; a reorder is exactly the kind of change that
fixes the measured case and leaves the class open. I will pose the *class* — an
unmodelled field carrying each pathology, at depths past anything a test would
use — not the case the ledger named.

**F-16's count.** `#[non_exhaustive]` either closes the cross-crate literal or
it does not, and the enumeration either matches the paths that exist or it does
not. Both are read off the type and a repository-wide grep.

**Declined, not open.** `arbitrary_precision` is the user's decision and is
recorded as declined. I will check the clause says *declined* rather than
*pending*, and nothing further.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | major | fix-now | verified |
| F-3 | major | fix-now | verified |
| F-4 | minor | fix-now | verified |
| F-5 | minor | fix-now | verified |
| F-6 | minor | fix-now | verified |
| F-7 | minor | fix-now | verified |
| F-8 | minor | fix-now | verified |
| F-9 | nit | fix-now | verified |
| F-10 | nit | fix-now | verified |
| F-11 | minor | fix-now | verified |
| F-12 | nit | fix-now | verified |
| F-13 | nit | fix-now | verified |
| F-14 | nit | fix-now | verified |
| F-15 | minor | fix-now | verified |
| F-16 | nit | fix-now | verified |
| F-17 | minor | fix-now | repaired, closed by disposition |
| F-18 | nit | fix-now | repaired, closed by disposition |

### F-1 — `wire::Reply` narrows the wire on the two fields whose values it does not model

**Severity:** major
**Location:** `crates/goad-shell/src/ingress/wire.rs:32` and `:37`

**Expected:** CLAUDE.md invariant 2 and 005/D-11: *"emit must not narrow what it
accepts for a field it does not use"*, and `client.rs:154` states it in terms —
*"`protocol` is not read at all. Requiring it would narrow what a client accepts
from a conforming-enough host for a field it does not use."* SPEC-003 §6.3
places no range and no encoding constraint on `protocol`; it says only that it
is *"this contract's version"*. R-14 says `retry_after_ms` is *"a whole number
of milliseconds"* and does not say how a host spells one.

**Observed:** D-11 was implemented as *presence*-permissive only. The declared
Rust types are `Option<u8>` and `Option<u64>`, and serde enforces both on the
way in. A reply the client understands completely is rejected at the parse step
because of a field it has just said it does not read: the whole reply becomes
`SendFault::Unreadable`, and `main` exits **2**. A `too_soon` refusal that a
caller could have acted on is reported as *no usable answer*, and the rendered
line (`render.rs:96`) then says *"the reply from … is not one JSON object"* —
which is false of the bytes, because it **is** one JSON object.

**Evidence:** measured against the workspace's own `serde_json` 1.0.151, using
`Reply`'s exact field types, at
`/tmp/claude-1000/-home-david-dev-goad/4c13d9cb-2371-49f9-a904-b99e3cad6f57/scratchpad/probe2.rs`:

```
Option<u8>  <-    255  =>  Ok(Some(255))
Option<u8>  <-    256  =>  Err(invalid value: integer `256`, expected u8)
Option<u8>  <-   1000  =>  Err(invalid value: integer `1000`, expected u8)
Option<u8>  <-    1.0  =>  Err(invalid type: floating point `1.0`, expected u8)
Option<u8>  <-    "1"  =>  Err(invalid type: string "1", expected u8)
Option<u64> <- 1800.0  =>  Err(invalid type: floating point `1800.0`, expected u64)
```

A second host implementation — which SPEC-003 §1 exists to admit, *"a second
host implementation can be held to the same contract"* — written in a language
whose JSON encoder emits a version or a millisecond count as `1.0` / `1800.0`
(Python's `json.dumps(1.0)`, any encoder handed a float) produces a reply this
client cannot read at all. So does any future host past `protocol: 255`.

This is the specific narrowing CLAUDE.md names as *"the failure this project
exists to avoid"*, arrived at from the read side rather than the render side.
Nothing in the slice's four tiers reaches it: every `protocol` value any case
uses is `1` or `2` (`wire.rs:69,74`, `client.rs:204,227,234,246`), all inside
`u8`, so the type is invisible to the suite.

Two separable halves, and the responder may split them: `protocol` is the
clear-cut one, because the code's own doc says the field is not read;
`retry_after_ms` is the one with a live consequence today, because it is the
field a `too_soon` caller acts on.

**Disposition:** fix-now
**Response:** Confirmed, and the repair is the class rather than the two
instances. The rule the code now holds: **a field's Rust type may narrow the
wire only as far as JSON's own types do.** `accepted` is a JSON boolean and
`bool` is exactly that; `reason` and `detail` are JSON strings. `protocol` and
`retry_after_ms` are JSON *numbers*, and JSON has one number type — `1`, `1.0`
and `1e0` are one value — so `u8` and `u64` were narrower than the wire and
refused a conforming host over how its encoder spelled a number. Both now carry
the number as written (`Option<serde_json::Value>`), and the **meaning** is read
in `read_reply`: `protocol` is not read at all, neither presence nor value; and
`retry_after_ms` is the whole number of milliseconds R-14 says it is, read the
same from `1800`, `1800.0` and `1.8e3`.

Permissiveness stops at the spelling. A fractional, negative, out-of-range or
non-numeric `retry_after_ms` is not rounded, clamped or guessed at — it is a
named §6.3 breach, because that field is one this client *models* and invariant
2's permissiveness covers only fields it does not. The conversion is cast-free
(`as_conversions` is denied workspace-wide, and for this reason: `as` would
saturate a huge float and truncate a fractional one, silently, into a
millisecond count a caller would act on).

The general form of that rule is what F-3, F-4 and F-5 fall out of, and it is
recorded in the module doc: **a field whose value this client adjudicates must
be typed so that its parse cannot fail**, or its wrongness is reported one step
too early — as bytes, in a sentence that is false of them. The reply path is now
three steps with exactly one fault each (`the connection → one line of bytes → a
Reply → an Answered`), so `Unreadable`'s sentence *"not one JSON object"* is
true whenever it is said, and everything past that door is a named breach of a
numbered rule.

The host's side is untouched: it still writes `"protocol":1`, and
`the_reply_s_bytes_are_exactly_these` is green unchanged.

Cases: `protocol_is_not_read_whatever_it_says` (including `1.0`, `300`, `"1"`
and `null`), `a_whole_retry_after_ms_is_read_however_it_is_spelled`, and
`a_retry_after_ms_that_is_not_a_whole_number_of_milliseconds_is_a_named_breach`.
Each was measured red before the fix and red again under the defect injected
back into the source.

**Outcome:** verified

Round 2. The narrowing is gone and the repair is the class, not the two
instances. Re-measured on the built binary against a fake listener, the two
command lines round 1 measured:

```
{"protocol":1.0,"accepted":true}                                → exit 0, silent
{"accepted":false,"reason":"too_soon","retry_after_ms":1800.0}  → exit 1, "refused: too_soon retry_after_ms=1800"
```

`whole_millis` (`client.rs:297-327`) is correct at every edge I could pose, and
correct for the stated reason rather than by luck. Checked against the
workspace's own `serde_json` 1.0.151: `1800`/`1800.0`/`1.8e3` → `1800`; `1800.5`
→ breach on `fract`; `-1` and `-0.0` pass the `fract` guard (`-0.0 == 0.0`) and
are caught one step later by `u64::from_str` refusing the `-` sign; `2^64` and
`1e300` render in full through `{:.0}` — Rust's `Display` for `f64` never uses
scientific notation — and are refused by `parse` rather than saturated; and
`18446744073709549568.0`, the largest `f64` below `2^64`, is accepted exactly.
The cast-free route is not fastidiousness: `as` would have turned each of the
last three into a plausible millisecond count a caller would have slept on.

The host's write side is genuinely untouched — `reply()` now builds
`Value::from(1)` and `Value::from(u64)`, and `the_reply_s_bytes_are_exactly_these`
is unchanged and green, which is the assertion that would have caught it.

What the repair over-claimed is [[F-13]] on the write side and [[F-11]] on the
fault separation; neither is a defect in this finding's fix.

### F-2 — `-h`, `--help` and `--version` are matched in value position, so a line that sent nothing exits 0

**Severity:** major
**Location:** `crates/goad-emit/src/args.rs:99-107`

**Expected:** 005/D-4 and `slice-005.md` AC-1: exit codes are about *who was
wrong*, and **0** means the host accepted the envelope. `render::USAGE` tells the
caller the same thing in the one place a caller reads: *"exit 0 if the host
accepted it"* (`render.rs:29-30`). The prior art the plan named
(`crates/goad/src/startup.rs:115`) matches help only as `[only]` — the whole of
the command line.

**Observed:** `parse` scans the entire argument vector for `-h`, `--help` and
`--version` **before** it knows which arguments are flags and which are values,
so any of the three appearing as a flag's *value* is consumed as a request for
help. The event is never built, nothing is connected to, and the process exits
**0** — which the caller's wrapper reads as *the host accepted it*.

**Evidence:** measured against the built binary at `c785f13`:

```
$ ./target/debug/goad-emit --source w --kind k --data --version
0.1.0
$ echo $?
0

$ ./target/debug/goad-emit --source w --kind -h
usage: goad-emit --source S --kind K [--data JSON] [--socket PATH]
… (the whole usage block)
$ echo $?
0
```

The reachable shape is a caller interpolating a value it did not author —
`goad-emit --source hook --kind "$event"` from a shell hook, or `just emit`
(`justfile:72-73`), which passes both words through unquoted from the command
line. A `kind` of `-h` is a silent no-op reported as success, which is the one
outcome D-4 says an exit code must never produce.

The behaviour is documented (`args.rs:80-81`, *"anywhere"*) and a case pins it
(`args.rs:249-257`), so this is a deliberate choice rather than an oversight —
but the choice was made about *ergonomics* (*"so a mistyped line can still ask
for help"*) and its cost lands on the exit-code contract, which no part of the
design weighed. The narrow fix is to resolve flag values first and match the
three only in flag position; the narrower one is `startup::arguments`' rule,
help only when it is the whole line.

**Disposition:** fix-now
**Response:** Confirmed, and the repair is the position rule rather than the
three tokens. `parse` now resolves **value positions first**: `split` walks the
argument vector once, pairing each of the four value-taking flags with the token
after it, and the scan for `-h`/`--help`/`--version` looks only at tokens in
**flag position**. A token consumed as some flag's value is never a flag.

The ergonomic property the table claims is kept whole, and is in fact wider than
before: `--help` is still honoured *before any value is inspected*, so
`goad-emit --bogus --help` and `goad-emit --source w --kind k --help` both print
the usage block. What it no longer does is read a *value* as a request for help.

**The ruling on `goad-emit --source --help`: `--help` is `--source`'s value.**
The line then has no `--kind`, and that is what the caller is told (exit 2). The
alternative — a value that looks like a flag is itself a usage error — was
rejected on this slice's own principle, twice stated: emit does not narrow the
values the protocol admits. `-h` is a `kind` SPEC-003 §6.2 admits, and a caller
interpolating `--kind "$event"` is entitled to send it; a client that refused it
would be the renderer mistake pointed at a command line, and this parser has no
`--` escape, so the refusal would make that `kind` unsendable at all. It is also
the line [[F-6]] draws in the same repair — emit refuses *malformed arguments*,
not well-formed values whose meaning is the host's to judge.

Shape: a `Flag` enum, and `split` returning `Argument::Flagged(Flag, Option<_>)`
or `Argument::Bare`. One place decides which tokens take a value, so the scan and
the fold cannot disagree about it — which is what the defect was. The fold
matches `Flag` exhaustively, so a fifth flag cannot be added without the compiler
naming every place that must learn about it. `value()` is gone: its job moved
into `split`.

The doc table is rewritten to match, with a row per case and no row without one.
The two that had no case before now do: `--help` beside `--version` in **both**
orders (the ordering is the whole content of that claim), and a value position
holding each of the three tokens.

Cases: `a_help_or_version_token_in_value_position_is_a_value` (the finding's own
measured command line, plus `--kind -h`),
`a_flag_in_value_position_is_consumed_by_the_flag_before_it`, and
`help_wins_over_version_in_either_order`. The first two were measured red before
the fix — `--data --version` returned `Invocation::Version`, exactly as the
finding measured on the binary. Measured again on the built binary after:
`goad-emit --source w --kind k --data --version` exits **2** naming `--data`.

**Outcome:** verified

Round 2. Measured on the built binary, the finding's own command line and its
sibling:

```
$ goad-emit --source w --kind k --data --version
goad-emit: --data is not JSON: invalid number at line 1 column 2, in --version
$ echo $?
2
$ goad-emit --source w --kind -h --socket ./nope.sock
goad-emit: nothing is listening at ./nope.sock: No such file or directory (os error 2)
$ echo $?                       # the envelope was built and the send attempted
2
$ goad-emit --bogus --help      # the ergonomic row the old table promised
usage: goad-emit …
$ echo $?
0
```

Exit 0 is now reachable only from a real acceptance or from a help/version token
**in flag position**, which is what the finding asked for.

The rewrite loses no row. I walked the old table against the new code rather
than reading the new table: `--socket=x` is still `Unknown`; a bare token is
still `Positional`; a repeated flag is still `Repeated` and not last-wins; a
flag with nothing after it is still `NoValue`; `--source ""` is still `Empty`
and `--socket ""` still `Empty`; a non-UTF-8 value is still `NotUtf8` while
`--socket` stays exempt; error precedence within a line is unchanged, because
the fold walks in the same order the old loop did. `split` pairing a flag with
the following token reproduces the old `value()` exactly — including the one
case that reads oddly and always has, `--source --kind k` giving `source` the
string `--kind` and then reporting `Missing("--kind")`. That is now a table row
with a case rather than an accident.

**The ruling is right, and it is right for the reason given rather than for
convenience.** SPEC-003 §6.2 admits any non-empty string as `kind`; this parser
has no `--` escape; so a client that refused `-h` in value position would make
that `kind` unsendable at all, which is narrowing the values emit will carry —
the same move the project exists to refuse, pointed at a command line instead of
a wire. The cost is real and is paid in the right place: a caller who forgets a
value gets a message about the *next* flag. `Flag` as an enum with an
exhaustive match in the fold is what makes the scan and the fold unable to
disagree again, which was the defect rather than the symptom.

### F-3 — the client's read of the reply is unbounded in bytes

**Severity:** major
**Location:** `crates/goad-shell/src/ingress/client.rs:114-117`

**Expected:** SPEC-003 §6.4 bounds a read at **64 KiB** and states why in terms:
*"an unbounded read from an untrusted writer is the defect SPEC-001/R-43 names
on the other socket"*. R-7 makes both bounds obligatory on the host's side and
requires they be *stated rather than implied*. The host implements it
(`ingress/mod.rs`, `ENVELOPE_LIMIT`). This slice writes the mirror of that read
and states no bound at all.

**Observed:**

```rust
let mut line = String::new();
let read = BufReader::new(&stream)
  .read_line(&mut line)
  .map_err(SendFault::Faulted)?;
```

`read_line` grows `line` until it meets a `\n` or EOF. A peer that writes
without ever terminating makes `goad-emit` allocate without limit until the
allocator fails or the OOM killer takes the process — at which point the caller
gets a signal rather than one of the three exit codes, and `exchange.rs:57-62`'s
own `expect("the binary must exit rather than be signalled")` names that
outcome as the thing the tier assumes cannot happen.

**Evidence:** the asymmetry is visible in one grep — `ENVELOPE_LIMIT` and
`ENVELOPE_DEADLINE` are imported into the client's own test file
(`crates/goad-shell/tests/integration/ingress.rs:22`) to script the host's
refusals, and neither appears in `client.rs`. The design reasoned carefully
about the unbounded **wait** and concluded it is the contract (§5.4, D-10,
citing §6.4) — and that reasoning is correct and is not what this finding is
about. It never reached the unbounded **read**, which §6.4 bounds in the same
paragraph and for a different reason.

The realistic trigger is a host bug — a reply loop that never terminates a line
— rather than an attacker; SPEC-003 §2 puts multi-user hardening out of scope
and §6.1 makes the containing directory the user's responsibility, which is why
this is not filed higher. It is still the case that the slice implements one
half of a bounded exchange and leaves the bound off. A `.take(ENVELOPE_LIMIT)`
before `read_line`, and a fault variant or a reuse of `Unreadable`, is the whole
fix; recording it as knowingly-accepted residue with a written rationale is the
other honest disposition.

**Disposition:** fix-now
**Response:** The bound is implemented, symmetric, at 64 KiB.
`client::REPLY_LIMIT` **aliases** `ENVELOPE_LIMIT` rather than restating the
number, and its doc cites §6.4's `bytes per read | 64 KiB` row and that row's
own reason — §6.4 names the host's direction, but the defect it names is the
*read*, not the direction. `reply_line` wraps the stream in
`take(REPLY_LIMIT + 1)` before `read_until`, which is `read_envelope`'s own
`+ 1` idiom reused because this is the mirror of that read, and an overrun is
`SendFault::Oversized { limit }` — the host's breach, named as the host's, the
same step-owns-one-fault shape as [[F-1]].

Two cases, and the second is not decoration.
`a_reply_one_byte_past_the_byte_bound_is_refused_as_the_host_s_breach` says the
bound exists; `a_reply_exactly_at_the_byte_bound_is_read` says it is *this*
bound. The at-bound probe is deliberately **EOF-framed**: a newline-terminated
reply at the bound never reaches the length check at all, because the terminator
is found first, so a case built that way would assert nothing about the bound
and would stay green under a `>=`. Injections confirm all three edges — removing
the bound, capping at `REPLY_LIMIT` instead of `REPLY_LIMIT + 1`, and `>=` for
`>` — each red exactly one of the two cases.

Stated residue, outside the repair's surface: `render.rs`'s
`every_send_fault_names_the_path_and_which_fault_it_was` builds its fault list
by hand rather than by an exhaustive match, so the new `Oversized` arm's line is
rendered by no case. One array entry completes it.

**Outcome:** verified

Round 2. The bound is right in both directions, and `reply_line`
(`client.rs:185-203`) is a faithful mirror of `read_envelope`
(`ingress/mod.rs:768-790`) — same `+ 1` idiom, same `> LIMIT` test, same
trailing-newline pop, same `limit` carried into the fault. Worked through by
hand at `REPLY_LIMIT = 65536`:

| reply | read | verdict |
|---|---|---|
| 65536 bytes + `\n` | 65537 = cap, ends in `\n` | popped, accepted |
| 65536 bytes, EOF-framed | 65536, no `\n` | `65536 > 65536` false → accepted |
| 65537 bytes + `\n` | 65537 = cap, `\n` truncated away | `65537 > 65536` → `Oversized` |
| unterminated stream | 65537 = cap | `Oversized`, allocation capped |

So a payload up to and including the limit is read and one past it is refused,
which is the same semantic `more_than_the_byte_limit_is_refused_too_large_and_the_limit_itself_is_accepted`
already holds on the host's side. The two cases pin both edges: with the cap at
`REPLY_LIMIT` instead of `REPLY_LIMIT + 1` the over-bound reply truncates into
broken JSON and reds as `Unreadable`; with `>=` for `>` the at-bound reply reds
as `Oversized`. The EOF framing of the at-bound probe is necessary exactly as
the response says — a newline-terminated reply at the bound never reaches the
length test.

The host's write side is unaffected: nothing in `ingress/mod.rs`'s reply path
changed but the two `Value::from` constructions, and the byte-exact case holds
them.

The residue this response named — `SendFault::Oversized` rendered by no case —
is **closed**, by [[F-7]]'s repair rather than this one: it is the sixth entry
in `every_send_fault_names_the_path_and_which_fault_it_was`, pinned on *"did not
end within 65536 bytes"*. The two repairs met without colliding.

### F-4 — a reply that is `accepted: true` *and* carries a `reason` is guessed at rather than refused

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/client.rs:172`

**Expected:** SPEC-003 §6.3: *"`reason` is present exactly when `accepted` is
`false`"*. A reply carrying both breaches that sentence as squarely as the two
breaches `design.md` §5.5 does enumerate. CLAUDE.md invariant 2 draws the line
the code should be on: *"an ambiguous message fails rather than being guessed at
— permissiveness is about fields the host does not model, never about the
meaning of fields it does"*. `reason` is a field this client models; it is in
`Answered::Refused` and it is what a wrapper branches on.

**Observed:**

```rust
(Some(true), _reason_is_not_read_on_an_acceptance) => Ok(Answered::Accepted),
```

The binding's name shows the choice was made deliberately, and it is recorded
nowhere — not in `design.md` §5.5's list of §6.3 breaches, not in D-11, not in
`notes.md`'s PHASE-02 decisions. The result is that the one reply shape whose
two modelled fields disagree about the verdict is resolved silently in favour of
**exit 0**. A host whose refusal path sets `reason` but fails to clear
`accepted` — an ordinary second-implementation bug, and the only one of this
class that is not caught by the two breaches emit does name — gets reported to
every caller as *the host took it*.

**Evidence:** `read_reply`'s four match arms at `client.rs:168-181` are total
over `(Option<bool>, Option<String>)` and there is no arm for this pair; no case
in `client.rs`'s `mod tests` or in the integration tier constructs it. Contrast
the sibling shape `{"protocol":1,"accepted":false}`, which *is* named a breach
and *is* pinned by two cases (`client.rs:226-229`,
`exchange.rs:183-202`) — the two shapes are the same kind of §6.3 violation and
are treated oppositely.

A third `NonConforming` arm is the symmetric answer. `aligned` is a defensible
disposition — `accepted` is the authoritative field and reading it as such is a
reasonable rule — but it needs the written reason, because right now the code
carries the decision and no document carries the argument.

**Disposition:** fix-now
**Response:** Confirmed. `(Some(true), Some(_reason))` is now a fourth
`NonConforming` arm naming the rule it breaches, and `read_reply`'s doc sentence
about `reason` *"not being read on an acceptance"* is gone — it was the record
of the guess, and it was wrong.

The asymmetry this creates is the right one and is worth stating plainly: an
*unmodelled* field (a sixth key, a `protocol` this client cannot hold) is
ignored, while a *modelled* field that contradicts another modelled field is
refused. That is invariant 2's own line — permissiveness is about fields the
host does not model, never about the meaning of fields it does — and it is the
same line [[F-1]] draws through `retry_after_ms`.

Case: `an_acceptance_that_also_carries_a_reason_is_a_named_breach`, measured red
before the fix and red again under the old `(Some(true), _reason)` arm injected
back.

**Outcome:** verified

Round 2. Measured on the built binary:

```
$ … reply {"protocol":1,"accepted":true,"reason":"too_soon"}
goad-emit: the reply from ./q.sock breaches SPEC-003 6.3: `accepted` is true and
a `reason` is present; SPEC-003 6.3 admits `reason` exactly when `accepted` is false
$ echo $?
2
```

The fourth arm is there (`client.rs:267-270`), it names the rule it breaches
rather than gesturing at one, and the doc sentence that recorded the guess is
gone. The `read_reply` match remains total over `(Option<bool>, Option<String>)`
with no `_` arm, so a fifth shape cannot be added without the compiler naming
this function.

The asymmetry the response states — an *unmodelled* field ignored, a *modelled*
field contradicting another refused — is the right line and is invariant 2's
own. It is not yet drawn all the way through the other two modelled-field
contradictions §6.3 admits no more than this one; that is [[F-14]], and it is a
nit rather than a defect in this fix.

### F-5 — a reply that is not UTF-8 is reported as a connection fault

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/client.rs:117`, rendered at
`crates/goad-emit/src/render.rs:89-91`

**Expected:** CLAUDE.md: *"Every refusal is reported and says which side was
wrong."* `SendFault`'s own doc states the same obligation at `client.rs:52-53`:
*"Every variant is the client's account of what went wrong, and each names which
side was at fault."* AC-3 requires the exit-2 message to name *which of those
happened*.

**Observed:** `read_line` returns `io::ErrorKind::InvalidData` when the bytes
before the newline are not valid UTF-8, and every `io::Error` from that call —
transport faults and encoding faults alike — is mapped to `SendFault::Faulted`.
The caller is then told *"goad-emit: the connection to /run/goad.sock faulted:
stream did not contain valid UTF-8"*, which points at the transport. The
transport did its job; the host's serializer is what was wrong, and the caller's
remedy is in the host, not the socket.

**Evidence:** `client.rs:114-117` has one `map_err(SendFault::Faulted)` covering
the read, and `render.rs:89-91` renders it as *"the connection to {path}
faulted"*. The exit code is 2 either way, so nothing about the wrapper contract
moves — this is about the sentence, which is the part AC-3 makes a requirement.
Compare the adjacent `SendFault::Unreadable`, which exists precisely to say *the
host's bytes were wrong* and would carry this case correctly.

Lowest-severity siblings of this: no case reaches `Faulted` at all
(`notes.md` PHASE-02 Findings records it as stated residue), so the mislabel is
also unobserved.

**Disposition:** fix-now
**Response:** Confirmed, and it fixes itself once the read stops demanding text.
`reply_line` reads **bytes** (`read_until`, `Vec<u8>`) and `answer` hands them to
`serde_json::from_slice`, so the encoding is `serde_json`'s question and is
answered one step later, where the answer names the host. JSON text *is* UTF-8,
so bytes that are not UTF-8 are not one JSON object, and they land on
`Unreadable` beside `[1,2]` and `not json` — which is the variant that exists to
say *the host's serializer was wrong*. `Faulted` is now the transport's alone,
which is what its doc had always promised.

Both halves of the finding's sibling note are closed too: the case the finding
records as not existing is
`a_reply_that_is_not_utf_8_is_the_host_s_bytes_not_a_faulted_connection` (one
line of fake listener, `b"\xff\xfe\n"`), and the unit tier's
`bytes_that_are_not_one_json_object_are_unreadable` now poses the same bytes to
`answer` directly. Injecting the old `read_line`-into-a-`String` read back into
`reply_line` reds the integration case with the finding's own measured
`Faulted(… stream did not contain valid UTF-8)`.

**Outcome:** verified

Round 2, and this is the cleanest of the ten: the fix removes the fault rather
than re-routing it. `reply_line` returns `Vec<u8>` and `answer` calls
`serde_json::from_slice`, so nothing in the client demands UTF-8 and
`io::ErrorKind::InvalidData` can no longer arise from an encoding — `Faulted` is
the transport's alone, which is what its doc always claimed.
`a_reply_that_is_not_utf_8_is_the_host_s_bytes_not_a_faulted_connection` poses
`b"\xff\xfe\n"` from a real fake listener and lands on `Unreadable`; the unit
tier poses the same bytes to `answer` directly. The `fake_listener` helper being
re-typed from `&'static str` to `Vec<u8>` is what made the case possible at all,
and it is the same change that let F-3's oversized probe exist — one helper
edit, two cases that could not have been written without it.

The sibling residue round 1 noted — no case reaches `Faulted` — stands, and is
now narrower and more honest than it was: `Faulted` has one meaning instead of
two, so the untested variant is at least unambiguous. Carried into the
synthesis rather than re-raised.

### F-6 — emit pre-empts one host rule while the slice's stated principle is not to

**Severity:** minor
**Location:** `crates/goad-emit/src/args.rs:154-160` (`text`), documented at
`args.rs:46-49`

**Expected:** the slice states the principle three times and applies it twice.
AC-5: *"The CLI does not pre-empt the check: SPEC-003/R-13 is the host's
requirement, and a client that duplicated it would make the host's own refusal
untested from this side."* `design.md` §5.5, on the byte bound: *"Emit does not
second-guess the byte bound, for the reason it does not pre-empt R-13."*

**Observed:** emit *does* pre-empt the other half of SPEC-003/R-9 — *"an empty
`source` or `kind` … MUST each be refused"* — and does so client-side, as a
usage error. `args.rs:46-49`'s doc comment cites §6.2 as the justification,
which is the host's rule being restated in the client.

The observable consequence is an exit code that disagrees with the wire for the
same envelope. `goad-emit --source "" --kind k` exits **2** (*no usable answer*);
the identical envelope written with `socat` draws `accepted: false`,
`reason: "invalid_envelope"` from the host, which through emit would be exit
**1** with a token to branch on. A wrapper that distinguishes *refused* from
*could not get an answer* — the whole justification for D-4's three codes — is
told the wrong one.

**Evidence:** `args.rs:154-160` refuses empty `--source`/`--kind` before any
connection; `crates/goad-shell/src/ingress/envelope.rs`'s
`an_empty_source_or_kind_is_refused_naming_it` is the host-side rule the client
now shadows; `args.rs:284-297` pins the client-side refusal.

This may well be the right call — an empty flag value genuinely *is* a usage
error, and unlike R-13 the host's own rule keeps its unit case either way. What
is missing is the argument: the slice wrote down why it declines to pre-empt
twice and never wrote down why it pre-empts here. `--socket ""` is a different
matter and is emit's own (there is no host rule about it).

**Disposition:** fix-now
**Response:** The check stays; the argument it was missing is now written where
a reader of the code meets it. Behaviour does not move: emit still refuses an
empty `--source`/`--kind`/`--socket` locally, at exit 2, before any connection.

The boundary, and it is not the one the finding assumed was blurred: emit does
not pre-empt host rules about the **meaning of well-formed values** — `"host"`
is a value the caller was entitled to type and the host declines, so it goes on
the wire and comes back as exit 1 with a token to branch on. Emit does refuse
**malformed arguments**, which is a different thing: `--source ""` is the same
mistake as `--source` with nothing after it, spelled differently, and the
argument vector alone says so. The host's own empty-value rule
(`goad_shell::ingress::envelope`, `EnvelopeFault::Empty`) is real, tested
host-side by `an_empty_source_or_kind_is_refused_naming_it`, and deliberately
never reached from here.

`slice-005.md` AC-5 carries that paragraph already and is not duplicated. What
changed is the code: `UsageError::Empty`'s doc comment now makes the malformed /
well-formed distinction and cites the host rule it does not shadow, and the §6.2
citation is gone — that citation *was* the pre-empt reading, and it was the
wrong justification for a right check. `parse`'s doc-table row says the same in
one clause.

`--socket ""` is the same class with no host-side counterpart at all, and the
comment says so.

**Outcome:** verified

Round 2, and the response's re-framing is better than the finding's. The
finding said *emit pre-empts a host rule*; the right cut is **malformed
argument** against **well-formed value whose meaning is the host's**, and
`--source ""` is on the first side of it — the same mistake as `--source` with
nothing after it, legible from the argument vector alone. On that reading emit
pre-empts nothing: `"host"` is a well-formed value and goes on the wire, and the
host's `EnvelopeFault::Empty` is simply never reached rather than duplicated.

Checked rather than accepted: `EnvelopeFault::Empty { key }` exists
(`ingress/envelope.rs:44`) and
`an_empty_source_or_kind_is_refused_naming_it` is its case, so the host-side
rule the code now cites is real and is tested where the comment says. The §6.2
citation that *was* the pre-empt reading is gone from `UsageError::Empty`'s doc
— removing a wrong justification for a right check is the substance here, and it
is the part that stops the next reader re-deriving the finding. `slice-005.md`
AC-5 carries the same paragraph, which is card reconciliation and is factually
accurate.

Behaviour is unmoved, which is correct: the finding was about an argument, not
an exit code.

### F-7 — three `render` cases assert a property of the set, not which arm produced which line

**Severity:** minor
**Location:** `crates/goad-emit/src/render.rs:210-235`, `:241-274`, `:278-303`

**Expected:** `docs/memory/a-green-test-can-assert-a-proxy.md`, and the
question `design.md` R-3 asks of a case: name the regression it guards, and
check the regression reds it. PHASE-04's cases were put through exactly that
(`notes.md` F-04-2, six injections); PHASE-03's were not — its VA-1 is the
manifest check alone (`plan.md` PHASE-03/VA-1, `notes.md:428`).

**Observed:** each of the three loops over a set of variants and asserts a
property that **every member of the set already satisfies by construction**, so
the mapping from variant to line — the only thing these functions decide — is
unheld:

- `every_usage_error_names_the_flag_and_reprints_nothing` asserts each line
  contains the flag. Swap the bodies of `UsageError::Missing` and
  `UsageError::Empty` in `usage_error_line` and both lines still contain their
  flag: **green**, and a caller is now told `--kind was given an empty value`
  when `--kind` was not given at all.
- `every_startup_fault_names_the_path_and_what_was_wrong` asserts three of the
  five lines contain `config.toml`. Swap `Unreadable` and `Unparseable`: both
  still name the path, **green**, and a user with an absent file is told it is
  unparseable.
- `every_send_fault_names_the_path_and_which_fault_it_was` asserts each line
  names the path and that the five are pairwise distinct. Swap `Unreachable`
  and `Faulted`: still five distinct lines each naming the path, **green** —
  and a caller whose host is not running is told the connection faulted.

**Evidence:** the three assertion bodies are `line.contains(named)`,
`line.contains("config.toml")` and `line.contains("/run/goad.sock")` plus a
`dedup` on the collected lines; no case in the file pairs a variant with its own
text. Contrast `a_refusal_shows_its_reason_token` (`:174`) and
`retry_after_ms_is_shown_exactly_when_it_is_there` (`:181`), which do assert
per-value content and would red on the corresponding swap.

The cheap repair is the shape those two already use: pair each variant with a
distinctive substring of its own line in the case's table, so the loop asserts a
mapping rather than a property.

**Disposition:** fix-now
**Response:** Confirmed by measurement, not by reading. Each of the three cases
was restored to its pre-repair body, the swap the finding names was injected,
and each stayed **green** — all three, exactly as predicted. They are now tables
of (variant, a phrase only that variant's arm says), so the loop asserts a
mapping: `Missing` → *is required*, `Empty` → *an empty value*, `Unreadable` →
*could not be read*, `Unparseable` → the `ConfigError`'s own text, `Unreachable`
→ *nothing is listening at*, `Faulted` → *faulted*, and so on. The distinctness
check in the send-fault case stays rather than being implied away: two arms could
say each other's phrase *as well as* their own, and the per-arm pins would not
see it.

`SendFault::Oversized` is now the sixth entry, which closes [[F-3]]'s stated
residue — its line was rendered by no case.

**The injection pass PHASE-03 never had**, over every case in `render.rs`, not
only the three. Fourteen defects injected one at a time, each lint-clean (no
`warning:` in any build), each reverted:

| injected | red |
|---|---|
| `reason` dropped from the refusal line | `a_refusal_shows_its_reason_token`, `retry_after_ms_is_shown_exactly_when_it_is_there` |
| `retry_after_ms` never shown | `retry_after_ms_is_shown_exactly_when_it_is_there` |
| `retry_after_ms` always shown | `a_refusal_shows_its_reason_token` |
| `detail` dropped | `detail_is_carried_into_the_line` |
| an acceptance renders a line | `an_acceptance_renders_nothing` |
| `Missing`/`Empty` swapped | `every_usage_error_names_the_flag_and_reprints_nothing` |
| a usage error reprints the usage block | `every_usage_error_names_the_flag_and_reprints_nothing` |
| `Unreadable`/`Unparseable` swapped | `every_startup_fault_names_the_path_and_what_was_wrong` |
| `NoIngress` stops naming the section | `every_startup_fault_names_the_path_and_what_was_wrong` |
| `NoPath` stops naming `--socket` | `every_startup_fault_names_the_path_and_what_was_wrong` |
| `ClockUnreadable` stops naming the timestamp | `every_startup_fault_names_the_path_and_what_was_wrong` |
| `Unreachable`/`Faulted` swapped | `every_send_fault_names_the_path_and_which_fault_it_was` |
| `Oversized` stops naming the path | `every_send_fault_names_the_path_and_which_fault_it_was` |
| `Unreadable`/`NonConforming` swapped | `every_send_fault_names_the_path_and_which_fault_it_was` |

Nothing survived. The four cases that were already load-bearing —
`a_refusal_shows_its_reason_token`, `retry_after_ms_is_shown_exactly_when_it_is_there`,
`detail_is_carried_into_the_line`, `an_acceptance_renders_nothing` — were so
before this repair; the three the finding names became so in it.

**Outcome:** verified

Round 2, and this is the repair I tried hardest to break, because a phrase pin
can be a proxy for a proxy. It is not one. I re-derived each swap against the
**current** text rather than trusting the injection log:

- `Missing`/`Empty` — *"is required"* against *"an empty value"*; neither line
  contains the other's phrase. Reds.
- `Unreachable`/`Faulted` — *"nothing is listening at"* against *"faulted"*;
  the `Unreachable` line does not contain *faulted*. Reds.
- `Unreadable`/`NonConforming` — *"is not one JSON object"* against *"breaches
  SPEC-003 6.3"*. Reds.
- `Unreadable`/`Unparseable` on the startup side — these two carry different
  payload types so the arms cannot literally swap; the realistic mutation is
  swapping the format strings, and *"could not be read"* disappearing from the
  `Unreadable` line reds it. Worth noting for the next reader: the
  `Unparseable` pin, *"names no program"*, is `ConfigError::EmptyCommand`'s own
  `Display` (`error.rs:170-173`) rather than anything `startup_error_line`
  writes, so that arm's own wording is pinned only by its partner. It is enough,
  and it is thinner than the others.

Keeping the distinctness check beside the per-arm pins is right and the stated
reason is the real one: a mutation that made two arms say each other's phrase
*in addition to* their own would pass every `contains` and fail `dedup`. The
two instruments hold different things.

`Oversized` as the sixth entry closes [[F-3]]'s stated residue. And the
fourteen-injection pass is the pass PHASE-03 never ran — the finding's real
subject was the missing pass as much as the three cases, and both halves are
answered.

### F-8 — the configuration-path rule is now stated in five places, one of them executable

**Severity:** minor
**Location:** `crates/goad-emit/src/render.rs:37-41` and `:133-135` (both new in
this slice), against `crates/goad-shell/src/config.rs:21-26`

**Expected:** CLAUDE.md: DRY, and *"no parallel implementation"*. PHASE-01/EX-1
made the executable rule singular — *"the XDG and `HOME` logic appears once in
the workspace"* — and the lift delivers that.

**Observed:** the *logic* is singular; the *statement of the rule a user reads*
went from two copies to four, and this slice wrote both new ones:

| where | executable? |
|---|---|
| `goad_shell::config::default_path`'s doc table (`config.rs:21-26`) | yes — five cases, `config.rs:493-535` |
| `goad::startup::StartupError::NoConfigPath`'s `Display` (`startup.rs:52-55`) | no |
| `goad::diagnostics::USAGE` (`diagnostics.rs:313-317`) | no |
| **`goad_emit::render::USAGE` (`render.rs:37-41`)** | no |
| **`goad_emit::render::startup_error_line`'s `NoPath` arm (`render.rs:133-135`)** | no |

**Evidence:** a change to `default_path` reds `config.rs`'s five cases and
nothing else; all four prose statements go stale in silence, and the two new
ones are in the binary whose users have no other route to the rule. The drift is
already latent rather than hypothetical: `default_path` returns `None` when
`HOME` is empty, and neither `USAGE` block says so — both read as though
`$HOME/.config/goad/config.toml` is always the fallback. Nothing ties any of the
four to the function, and the emit-side pair was written by transcription.

Not obviously worth a mechanism; worth a decision, recorded. The minimum is that
`render.rs:37-41` cite `default_path` as the rule's home the way
`startup.rs:96` now does, so the next reader knows which copy is authoritative.

**Disposition:** fix-now
**Response:** The minimum the finding names, both halves, and no mechanism. A
fifth copy is not worth a macro or a generated constant; knowing which copy is
the authority is worth three lines.

`render::USAGE`'s doc comment now says the discovery paragraph is prose *about*
[`goad_shell::config::default_path`], which is where the rule is executable and
where its own doc table states it — the form `startup.rs:96` already uses — and
that a change to that function is what must drive a change here.

The latent drift the finding measured is corrected rather than recorded: the
`USAGE` text said `$HOME/.config/goad/config.toml` was the fallback and stopped,
which reads as though there is always one. It now states the row the function has
always held — when `HOME` is unset or empty as well, there is no default path at
all and emit says so rather than guessing one, so pass `--socket`. That sentence
and `startup_error_line`'s `NoPath` arm now agree, which they did not.

The rule's fifth statement — `render.rs`'s `NoPath` arm — was already true and is
untouched. `goad::diagnostics::USAGE` is outside this repair's surface; whether
it carries the same correction is `crates/goad`'s to answer.

One case added on the executable side, which is the class [[F-7]] belongs to and
is the only reason this repair opens `config.rs`: `default_path`'s table says
`XDG_CONFIG_HOME` *"unset, empty or relative"* and the **empty** spelling had no
case. `an_empty_xdg_config_home_is_ignored_and_home_answers` gives it one.

**Outcome:** verified

Round 2. The repair does the smaller, better thing: it names the authority
rather than building a mechanism, and it *corrects* the drift the finding
measured rather than recording it. `render::USAGE` now states the `HOME`-empty
row — *"When HOME is unset or empty as well, there is no default path at all
and emit says so rather than guessing one: pass --socket"* — so the help text
and `startup_error_line`'s `NoPath` arm now say the same thing, which they did
not. The doc comment above `USAGE` says the paragraph is prose *about*
`goad_shell::config::default_path` and that a change there must drive a change
here, which is the form `startup.rs:96` already uses.

`an_empty_xdg_config_home_is_ignored_and_home_answers` (`config.rs:515-524`)
gives the executable side the row it was missing — the table said *"unset, empty
or relative"* and only two of the three had a case.

The count is unchanged at five, which is the right outcome for a finding whose
claim was about *which copy is authoritative*, not about how many exist. One
copy is now knowingly stale and is named as such: `goad::diagnostics::USAGE`
still reads as though `$HOME/.config/goad/config.toml` is always the fallback.
Declining it as outside this repair's surface is legitimate; it is carried into
the synthesis so it is not lost.

### F-9 — PHASE-01's insertion re-attached an existing doc comment to a different case

**Severity:** nit
**Location:** `crates/goad-shell/src/ingress/mod.rs:926-937`

**Expected:** a lift moves things; PHASE-01/EX-6 asks for no assertion and no
fixture changed, and the same care applies to what documents them.

**Observed:** the new case was inserted immediately after the doc comment
belonging to `a_reply_is_one_newline_terminated_line`, so that comment now
documents `the_reply_s_bytes_are_exactly_these` and the case it was written for
has none:

```rust
/// §6.3's reply is one **line**. Asserted here as well as on the wire
/// (`tests/integration/ingress.rs`) because this is the function that owes
/// the byte, and the wire case cannot say which of the two writers produced
/// it.
/// 005/PHASE-01/VT-3. The bytes, not a parse of them: …
#[test]
fn the_reply_s_bytes_are_exactly_these() { … }
…
#[test]
fn a_reply_is_one_newline_terminated_line() { … }
```

**Evidence:** `git show 03b0286:crates/goad-shell/src/ingress/mod.rs` carries
that paragraph directly above `a_reply_is_one_newline_terminated_line`; the
diff hunk at `mod.rs:927` adds the new text after it rather than before. The
first sentence — *"§6.3's reply is one line"* — is now false of the case it
sits on, which asserts two exact byte strings and says nothing about framing.

**Disposition:** fix-now
**Response:** Re-attached. The §6.3-newline paragraph sits above
`a_reply_is_one_newline_terminated_line` again, and
`the_reply_s_bytes_are_exactly_these` keeps the 005/PHASE-01/VT-3 paragraph that
was written for it. No assertion moved.

**Outcome:** verified

Round 2. The §6.3-newline paragraph is back above
`a_reply_is_one_newline_terminated_line` (`ingress/mod.rs:970-973`) and
`the_reply_s_bytes_are_exactly_these` keeps the VT-3 paragraph written for it.
`git diff` over that hunk moves two comment blocks and no assertion.

### F-10 — `an_accepted_envelope_exits_0_and_says_nothing` holds half its own name

**Severity:** nit
**Location:** `crates/goad-emit/tests/binary/exchange.rs:71-88`

**Expected:** D-7 is about the whole of what a cron job sees, and the case's
name claims it: *says nothing*.

**Observed:** the case asserts `output.stdout.is_empty()` and never looks at
`output.stderr`. A regression that wrote a line to stderr on the acceptance path
— the one outlet `main` uses for every other outcome — leaves it green.

**Evidence:** `exchange.rs:84-85` is the whole assertion pair; `stderr_of` is
used only as an `assert_eq!` message on the same lines. `assert!(output.stderr
.is_empty())` is the one-line completion. Filed as a nit because the sibling
`refused_line(&Answered::Accepted) == ""` case (`render.rs:203-205`) covers the
same ground from the pure side and `main` does not call it on acceptance at all.

**Disposition:** fix-now
**Response:** `assert!(output.stderr.is_empty())` added, so the case holds the
whole of its own name. The doc comment says why both streams: stderr is the
outlet `main` uses for every other outcome, so it is where a regression would
land. `stdout_of` replaces the `{:?}` byte dump in the existing stdout assertion,
so a failure reads as text.

Two cases added beside it, the same class of gap — a stated behaviour with no
case at this tier. PHASE-03/EX-6 states `--help` and `--version`, AC-8 verified
them by hand at audit, and the binary tier had neither:
`help_prints_the_usage_block_on_stdout_and_exits_0` (the usage block on
**stdout**, exit 0, nothing on stderr) and
`version_prints_the_package_version_on_stdout_and_exits_0` (the package version
and nothing else, so a caller can read it as a value). `render::USAGE` being
right says nothing about `main` reaching it, which is what these hold.

**Outcome:** verified

Round 2. `assert!(output.stderr.is_empty())` is there, and `stdout_of` replaces
the `{:?}` byte dump so a failure reads as text rather than as a decimal array.

The two cases added beside it are the better half of this repair and are not
what the finding asked for: `help_prints_the_usage_block_on_stdout_and_exits_0`
and `version_prints_the_package_version_on_stdout_and_exits_0`. PHASE-03/EX-6
states both behaviours, AC-8 checked them by hand, and no case at any tier held
that `main` *reaches* `render::USAGE` — `render.rs` being right says nothing
about the wiring. Both assert the empty stderr too, so the class the finding
named is closed across all three exit-0 paths rather than at the one it
measured.


### F-11 — the reply path's "one fault per step" is stated as an absolute and is not one

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/client.rs:21-42` (the module's
rule), `crates/goad-shell/src/ingress/wire.rs:26-35` (the same rule on `Reply`)

**Expected:** the repair states two claims in terms, and the second is an
absolute:

> The reply travels three steps, and **each step owns exactly one fault** …
> `serde_json` decides whether they are one JSON object, and that is the *only*
> place [`SendFault::Unreadable`] can arise — so its sentence, *"not one JSON
> object"*, is true whenever it is said.

and the rule that is supposed to make it hold:

> a field whose *value* this client adjudicates must be typed so that it cannot
> fail to parse, or its wrongness is reported one step too early, as bytes, in a
> sentence that is false of them.

SPEC-003 §3 P-D is the standard the project already holds such a sentence to:
*"A sentence here that says a mechanism always holds or never fails is making a
claim about the mechanism … a clause that cannot say where its exception would
be is a clause that has not been checked."*

**Observed:** the clause has exceptions and names none. Five reply shapes that
are, in plain fact, one JSON object are reported to the caller as not being one.
Measured on the built binary at the repaired tree, against a fake listener:

```
{"protocol":1,…,"retry_after_ms":1e999}  exit 2
  goad-emit: the reply from ./q.sock is not one JSON object: number out of range
{"accepted":"yes"}                       exit 2
  goad-emit: the reply from ./q.sock is not one JSON object: invalid type: string "yes", expected a boolean
{"accepted":false,"reason":5}            exit 2
  … is not one JSON object: invalid type: integer `5`, expected a string
{"accepted":false,"reason":"x","detail":7}  exit 2
  … is not one JSON object: invalid type: integer `7`, expected a string
{"accepted":true,"accepted":false}       exit 2
  … is not one JSON object: duplicate field `accepted`
```

**The first one is the part that matters**, because it is the repair's own rule
failing on the one field the rule was written for. `retry_after_ms` was re-typed
to `Option<Value>` precisely so that its parse could not fail and its wrongness
could be *named* — and `Value` itself refuses a number outside `f64`'s range, so
`1e999` never reaches `whole_millis` at all. Measured directly against
`serde_json` 1.0.151: `serde_json::from_str::<Value>("1e999")` is
`Err(number out of range)`. The other four are the ordinary consequence of
`accepted: Option<bool>`, `reason: Option<String>` and `detail: Option<String>`
— correct types under `Reply`'s *other* rule (narrow only as far as JSON's own
types do) and incompatible with this one, which asks for types that cannot fail.

The two rules on the same struct pull against each other, and the module doc
asserts both without noticing.

**Evidence:** the measurements above, reproducible with any listener that writes
those bytes. Nothing about the **exit code** is wrong in any of the five —
`Unreadable` and `NonConforming` are both exit 2, and in every case the host is
genuinely the side at fault, which is the obligation CLAUDE.md actually imposes.
What is wrong is the sentence the caller reads, which is round 1 [[F-5]]'s class
exactly: a message that points at the wrong thing. The repair fixed that class
for the encoding case and then claimed it universally.

Two honest dispositions. Either the clause names its exception — *"the only
place `Unreadable` arises from the bytes' **shape**; a value the wire admits and
`Reply`'s types do not lands here too"* — and `Unreadable`'s rendered sentence
softens to something true of all of them (*"is not a reply this client can
read"*). Or the three string/bool fields move to `Value` as well and every
wrong-typed field becomes a named breach, which is the rule applied all the way
and is a larger change than this round should take. `tolerated` with the
exception written down is also defensible; silently leaving an absolute that
P-D would reject in a spec is not.

**Disposition:** fix-now
**Response:** The finding is right and the clause was the overclaim, but the
ruling went the other way: the rule is applied all the way rather than softened.
**Every `Reply` field is now a `Value`**, and `read_reply` adjudicates all five
against §6.3 — `accepted` must be a JSON boolean, `reason` and `detail` JSON
strings, `retry_after_ms` a whole number of milliseconds — so a wrong type is a
**named breach** wherever it sits. Four of the five measured shapes move from
`Unreadable` to `NonConforming` with the rule they break in the sentence.

Two things the widening exposed that the finding could not have seen, both now
closed:

- **serde's derive reads a struct from a JSON *sequence* as well as a map.**
  Once no field could refuse a value, `[1,2]` deserialized into a `Reply` of
  `protocol: 1, accepted: 2` and an *array* was reported as a §6.3 breach.
  `answer` now asks the shape question first (`one_json_object`) and the field
  question second — two parses of a line already bounded at `REPLY_LIMIT`,
  because they are two questions.
  `bytes_that_are_not_one_json_object_are_unreadable` is what holds it, and is
  load-bearing for the first time.
- **The duplicate-name case is why the two parses are not one.** A
  `Map<String, Value>` resolves `{"accepted":true,"accepted":false}` silently to
  its last value; the derived struct refuses it. Reading the shape through the
  map and the fields through the struct keeps the refusal.

**The clause now says where it stops**, as §3 P-D requires, and the exceptions
are pinned rather than described
(`the_two_shapes_serde_json_will_not_yield_a_value_for_are_unreadable`). Two
remain, and both are documents `serde_json` yields **no value at all** for, so no
field typing can reach them:

- **`1e999`** — a number outside the range `serde_json` represents, in any
  field, including one this client does not model. RFC 8259 §6 explicitly
  permits a parser to set such a limit, so this is that parser's limit and not a
  defect of this module. Closing it needs the `arbitrary_precision` feature,
  which changes `Number`'s representation and `PartialEq` for **every crate in
  the workspace** — `goad-semantics`'s `NumberRange` and protocol wire types
  included. That is a manifest decision and outside this repair's surface; it is
  raised rather than taken, and it is a poor trade on its face: a global change
  to how the workspace holds numbers, to improve one sentence about one
  pathological literal.
- **a duplicated name** — ambiguous about the field the verdict turns on, so
  failing is right; only the sentence is coarse.

Both are exit 2 either way, and the host is the side at fault in both, so what
the exceptions cost is the precision of one sentence and not a caller's
decision. Cases: `a_modelled_field_of_the_wrong_type_is_a_named_breach` (five
shapes across three fields) and the exception pin above. Injections: reverting
`accepted` to `Option<bool>`, reverting `reason` to `Option<String>`, dropping
`detail`'s adjudication, and dropping the shape guard each red exactly one case.

**Outcome:** verified

Round 3, and the class is finished rather than the clause weakened, which is the
harder of the two answers and the right one. All five `Reply` fields are
`Option<Value>` and `read_reply` adjudicates each against §6.3 through `flag`,
`text` and `whole_millis`, so a wrong JSON type is a named breach wherever it
sits. Three of the five shapes this finding measured now say something true:

```
{"accepted":"yes"}                     → breaches SPEC-003 6.3: `accepted` is not a JSON boolean
{"accepted":false,"reason":5}          → breaches SPEC-003 6.3: `reason` is not a JSON string
{"accepted":false,"reason":"x","detail":7}
                                       → breaches SPEC-003 6.3: `detail` is not a JSON string
```

The other two are the two the module doc now names as exceptions, with the
reason each is one, per §3 P-D — and `the_two_shapes_serde_json_will_not_yield_a_value_for_are_unreadable`
pins them rather than describing them, which is the difference between a clause
that has been checked and one that has been written.

The two-parse design is necessary and I checked that rather than accepting it.
Neither parse alone keeps both refusals: `[1,2]` **deserializes into a `Reply`**
once every field is a `Value` (measured: `Reply { protocol: Some(Number(1)),
accepted: Some(Number(2)), … }`), so the derived struct cannot answer the shape
question; and `serde_json::Map<String, Value>` resolves
`{"accepted":true,"accepted":false}` silently to its last value (measured: `Ok`),
so the `Map` cannot answer the duplicate question. Two walks of a line already
bounded at `REPLY_LIMIT`, once per process, is the right price.

**The passenger claim is exactly right**, and it is the kind of claim worth
checking rather than believing. Reconstructing round 2's struct and posing
`[1,2]` to it: `Err(invalid type: integer 2, expected a boolean)` — the case was
green on a *type* error, not a shape one, so it held nothing about shape. With
the current types and no guard, `[1,2]` reaches `flag()` and becomes a §6.3
breach, which reds the case. Load-bearing now, passenger before, precisely as
stated.

[[F-15]] is raised against the precision of the new boundary, not against this
fix.

### F-12 — `protocol` is now unread in value as well as in presence, and nothing records what that concedes

**Severity:** nit
**Location:** `crates/goad-shell/src/ingress/client.rs:236-240`

**Expected:** D-11 settled that `protocol` is *optional on read* — *"emit must
not narrow what it accepts for a field it does not use"*. The repair went
further, and says so: *"`protocol` is not read at all — not its presence and not
its value."* SPEC-003 §6.3's own sentence about the field is
*"`protocol` is this contract's version, not SPEC-001's; no client speaks
both."*

**Observed:** the position is right for today and is the direct answer to *did
making it unread lose anything a client should keep?* — **no.** There is one
version of this contract, `protocol` has no other use, and reading it could only
have produced a refusal of a reply the client understands. The repair is
correct.

What is unrecorded is the consequence that goes with it: a host that one day
writes `protocol: 2` gets §6.3 **version 1** semantics applied to its reply
silently, with no signal at either end. `protocol_is_not_read_whatever_it_says`
pins `300` as *accepted*, which is the right assertion for the rule and is also
the assertion that will be load-bearing in the wrong direction if a v2 ever
changes what `accepted` means.

**Evidence:** `client.rs:236-240` and `read_reply`'s match, which never
mentions `reply.protocol`; `protocol_is_not_read_whatever_it_says`
(`client.rs:367-381`). The residue is a fact about a contract with one version
and no negotiation step, so there is nothing to *do* about it in this slice —
the disposition this expects is `aligned` or `tolerated` with one sentence added
beside the rule, saying that a version this client cannot understand is read as
one it can, and that closing it is SPEC-003's job rather than a client's.

Filed because a nit costs nothing to note and this is the kind of thing found
twice if it is found at all.

**Disposition:** fix-now
**Response:** Agreed on all of it, including that there is nothing to *do* — so
what this took is the sentence the finding asked for, in `read_reply`'s doc
beside the rule it qualifies: a host that one day writes `protocol: 2` gets §6.3
**version 1** semantics applied to its reply, silently, at both ends. That is
the right trade today — this contract has one version and no negotiation step,
so reading the field could only ever refuse a reply this client understands —
and closing it is `SPEC-003`'s job, not a client's.

Two things now make the concession hard to lose. `read_reply` destructures its
argument and names the field `protocol: _unread`, so *"not read at all"* is
visible at the one place it could stop being true; and `wire::VERSION` is the
one value the host can write, because `Reply::written` takes no `protocol`
argument.

**Outcome:** verified

Round 3, and it is the shape a nit of this kind should take: no behaviour moved,
and the concession is now written where it could stop being true.
`read_reply` destructures `protocol: _unread`, so the one place the field could
start being read names it, and the doc paragraph states the trade in the terms
the finding asked for — a host writing `protocol: 2` gets §6.3 v1 semantics
silently at both ends; there is nothing to do about it in a contract with one
version and no negotiation step; closing it is SPEC-003's job.

`protocol_is_not_read_whatever_it_says` continues to pin `1`, `1.0`, `300`,
`"1"` and `null` as all accepted, which is the assertion that makes *not read at
all* a claim about the value rather than about the presence.

### F-13 — the `Value` widening is justified from the read side only, on a type whose whole point is that it serves both

**Severity:** nit
**Location:** `crates/goad-shell/src/ingress/wire.rs:26-35`, against
`crates/goad-shell/src/ingress/mod.rs:583-591`

**Expected:** D-2's reason for one `Reply` is that *"two definitions of one wire
object drift, and nothing fails when they do"* — the type is deliberately
bidirectional, and `wire.rs`'s own module doc opens on that: *"One definition,
read by both halves of the exchange: the listener writes it and a client reads
it."*

**Observed:** the paragraph the repair added argues the `Value` change entirely
from the reading side — *"a field whose parse cannot fail is a field whose
wrongness can be named"* — and says nothing about what it costs the writing
side, which is the same fields on the same struct. The cost is small and real: a
host edit spelling its own version wrongly used to be `error[E0308]` and is now
green code.

```rust
// before: error[E0308], expected `u8`
// after:  compiles, and writes {"protocol":"1",…}
let wire = wire::Reply { protocol: Some(serde_json::json!("1")), … };
```

`Reply` is `pub` with `pub` fields, so the constraint that left the type did not
move anywhere else in the type system; what holds it now is one assertion,
`the_reply_s_bytes_are_exactly_these` (`ingress/mod.rs:938-951`).

**Evidence:** `reply()` is the only production constructor and does build
`Value::from(1)` and `Value::from(u64)`, so nothing is wrong today, and the
byte-exact case would red on any change to what is written. That is why this is
a nit and not a finding about behaviour. The gap is in the record: the one
paragraph a future editor reads before changing these field types explains the
read side's requirement and leaves them to rediscover that the write side paid
for it. Two clauses — *what the host must still write, and what now holds it* —
is the whole repair.

**Disposition:** fix-now
**Response:** Correct, and round 2's widening made it sharper rather than
softer: `accepted`, `reason` and `detail` gave up their types too, so the write
side is now `Value` throughout. The answer is not separate read and write types
— that is exactly the drift D-2 exists to prevent, and it is not mine to decide
alone in any case — but **one type with one typed constructor**.

`Reply::written(accepted: bool, reason: Option<&str>, retry_after_ms:
Option<u64>, detail: Option<&str>)` is the only thing that builds a `Reply` in
production, and `ingress::reply` goes through it. The write side ends up
**better** typed than before this slice: `protocol` is not an argument at all,
it is `wire::VERSION`, so the misspelling the finding demonstrates
(`protocol: Some(json!("1"))`) is not expressible through the constructor rather
than merely unlikely.

The two clauses the finding asked for are in `Reply`'s doc, stated as what holds
the write side and in what order: (1) `Reply::written` is the only production
constructor and takes typed parts; (2) `the_reply_s_bytes_are_exactly_these`
asserts the bytes those parts become, byte for byte. Plus the sentence for the
next editor: the fields are `pub` because the **reader** needs them, and anyone
changing their types changes `written`'s signature in the same breath.
`wire::writing_and_reading_are_the_same_shape` now builds its subject through
`written` rather than a literal, so the round trip under test is the host's own.

**Outcome:** verified

Round 3. The repair answers the finding's actual request — *what the host must
still write, and what now holds it* — and answers it with a type rather than
only with prose, which is more than was asked. `Reply::written(bool,
Option<&str>, Option<u64>, Option<&str>)` is the sole production constructor,
`ingress::reply` goes through it, and `protocol` is not a parameter at all: it
is `wire::VERSION`, so the host cannot write a version by hand even correctly.
That is strictly better than the pre-slice private `Wire`, which took `protocol`
as a field and could have been given `2`.

`writing_and_reading_are_the_same_shape` now round-trips through `written`
rather than a literal, so the case under test is the host's own path, and
`the_reply_s_bytes_are_exactly_these` still holds the bytes those arguments
become.

The doc paragraph names both holders and says why the fields stay `pub`. What it
does not name is a third path, which is [[F-16]] — a new finding about the type,
not a contest of this fix.

### F-14 — the newly drawn "modelled fields that contradict are refused" line is drawn through one pair of three

**Severity:** nit
**Location:** `crates/goad-shell/src/ingress/client.rs:261-280`

**Expected:** [[F-4]]'s repair states the rule it now holds: *"an **unmodelled**
field … is ignored, while a **modelled** field that contradicts another modelled
field is refused. That is invariant 2's own line."* SPEC-003 §6.3 makes three
statements of that shape, not one: `reason` is present *exactly* when `accepted`
is `false`; `retry_after_ms` *"is present exactly when `reason` is `too_soon`"*;
and *"No other reason carries that field"* (R-14).

**Observed:** the rule is applied to the first and not to the other two. Both
`retry_after_ms` and `reason` are fields this client models. Measured on the
built binary:

```
{"protocol":1,"accepted":true,"reason":"too_soon"}                 exit 2  named breach   ← refused
{"protocol":1,"accepted":true,"retry_after_ms":"banana"}           exit 0  silent          ← admitted
{"protocol":1,"accepted":false,"reason":"engaged","retry_after_ms":500}
                                     exit 1  "refused: engaged retry_after_ms=500"         ← admitted and printed
```

The second is the sharper of the two: `retry_after_ms` is never looked at on an
acceptance, so a value that is not even a JSON number passes through a client
that refuses the same field for being `1800.5` two lines away.

**Evidence:** `read_reply`'s `(Some(true), None)` arm returns `Accepted` without
consulting `reply.retry_after_ms`, and `whole_millis` is called only from the
`(Some(false), Some(reason))` arm (`client.rs:266`, `:276`).

The behaviour is defensible and may well be right — neither admitted shape can
mislead a caller about the verdict, which was [[F-4]]'s actual harm, and
`design.md` §5.5 says `retry_after_ms` is *"reported, never obeyed"*, so carrying
a stray one is consistent with the design. `aligned` is the likely disposition
and it needs one sentence: the line is not *modelled field contradicts modelled
field*, it is **a contradiction about the verdict**, which is narrower and is
what actually justifies refusing one and carrying the others. As stated, the
rule reads as though it were applied everywhere it holds, and it is not.

**Disposition:** fix-now
**Response:** Both halves, and they part company — which is the finding's own
point, and the reason the rule as I first stated it was wrong.

**The type half is fixed.** `retry_after_ms` is adjudicated for its *type*
wherever it appears, including on an acceptance, so
`{"accepted":true,"retry_after_ms":"banana"}` is a named breach instead of exit
0. That is the same rule as [[F-11]]'s and it is now applied to all five fields
without exception.

**The co-occurrence half is `aligned`, and takes the sentence the finding
drafted.** The line is not *a modelled field contradicting a modelled field* —
that phrasing was too wide and I wrote it. It is **a contradiction about the
verdict**, and only that, because only that can make a caller wrong about
whether the envelope was taken. A well-typed `retry_after_ms` beside `engaged`
is surplus advice a conforming writer may ignore (§6.3 says so in terms), and a
ninth reason token from a newer host may legitimately carry it — refusing it
would narrow the wire over a field that cannot mislead anyone, which is the
failure this project exists to avoid. The same argument covers `too_soon`
arriving without it: a host in breach of R-14's *"MUST additionally carry"*, but
not a host whose verdict is in doubt.

Both halves are pinned:
`a_wrong_typed_retry_after_ms_is_a_named_breach_even_on_an_acceptance` and
`surplus_but_well_typed_advice_is_carried_rather_than_refused`. The second is a
characterization pin of a stated line, so its injection is the *opposite*
defect — refusing a well-typed `retry_after_ms` beside a non-`too_soon` reason
reds it.

**Outcome:** verified

Round 3, and the re-drawn line is better than the one the finding asked for. The
finding said the rule was applied to one pair of three; the repair's answer is
that there were two rules collapsed into one, and separating them resolves all
three cleanly:

- **Types first**, everywhere, including where the value is useless. Measured:
  `{"accepted":true,"retry_after_ms":"banana"}` was exit 0 and silent at round 2
  and is now *"breaches SPEC-003 6.3: `retry_after_ms` is not a JSON number"*.
- **Then the verdict**, and only a contradiction *about the verdict* is refused.
  Measured: `{"accepted":false,"reason":"engaged","retry_after_ms":500}` →
  exit 1, *"refused: engaged retry_after_ms=500"*.

That is the narrower rule the finding said was the real one, stated in the code
and justified: surplus well-typed advice cannot make a caller wrong about
whether the envelope was taken, and a ninth reason token from a newer host may
legitimately carry it, so refusing it would narrow the wire for nothing. Both
sides are pinned — `a_wrong_typed_retry_after_ms_is_a_named_breach_even_on_an_acceptance`
and `surplus_but_well_typed_advice_is_carried_rather_than_refused` — and the
second is the one that stops the rule from being over-applied later.

Checked against §6.3's three co-occurrences rather than against the two the
finding named: `reason`-exactly-when-`accepted`-is-false is enforced in both
directions; `retry_after_ms`-exactly-when-`too_soon` and *"no other reason
carries that field"* are both carried. Consistent with the stated line, in all
three.

### F-15 — the exception clause's stated boundary is not the true one: one exception missing, one stated too broadly, and one of them narrows the wire

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/client.rs:42-65` (the *Where that
stops* clause), `client.rs:266-278` (`one_json_object` and its doc)

**Expected:** the clause exists to meet SPEC-003 §3 P-D — *"a clause that cannot
say where its exception would be is a clause that has not been checked"* — and
it names two shapes: a number outside `serde_json`'s range, and a duplicated
name. The standard P-D sets is completeness, not the presence of a list. And
`wire.rs:12-15` states the constraint the second parse must not break: *"A sixth
field from a newer host must not turn a perfectly good verdict into a fault."*

**Observed:** three discrepancies, each measured. The first two cost precision;
the third costs behaviour.

**(i) A third exception exists and is not named: `serde_json`'s recursion
limit.** Nesting past 128 levels anywhere in the reply fails in the parser, so
a document a person would certainly call one JSON object is reported as not
being one. Measured on the built binary:

```
{"accepted":true,"extra":[[[…130 deep…]]]}
  → exit 2
  goad-emit: the reply from ./s.sock is not one JSON object: recursion limit
             exceeded at line 1 column 152
```

It belongs on the list for the same reason the other two do, and it has the same
shape: `serde_json` yields no value, so no field typing can reach it. It is also
the one an author would not think of, which is exactly what a checked list is
for.

**(ii) The duplicated-name exception is stated more broadly than it holds.** The
clause says *"a duplicated name"*, and `one_json_object`'s doc says the second
parse *"is what rejects a duplicated name"*. Both are true only of the **five
modelled** names: serde's derive tracks duplicates for the fields it declares
and skips unknown ones with `IgnoredAny`, which tracks nothing. Measured:

```
{"accepted":true,"severity":1,"severity":2}   → exit 0, silent
{"accepted":true,"accepted":false}            → exit 2, duplicate field `accepted`
```

The behaviour is defensible — a duplicated *unmodelled* name is a field this
client does not model, and invariant 2 says ignore it — but the clause does not
say that, and a reader checking the clause against the code finds it says more
than the code does. The pinning case poses only the modelled shape, so nothing
holds the broader claim either.

**(iii) The shape guard is stricter than the field parse on an unmodelled field,
and that is a narrowing this repair introduced.** `one_json_object` parses every
value in the object, including values in fields `Reply` would have skipped. So
for a deeply nested *unmodelled* field the guard refuses what the field parse
accepts — and what the previous revision accepted. Measured against round 2's
struct, reconstructed exactly:

| bytes | round 2 | round 3 |
|---|---|---|
| `{"accepted":true,"extra":[[[…130 deep…]]]}` | `Ok(accepted=Some(true))` → **exit 0** | `Unreadable` → **exit 2** |

A caller whose envelope the host *accepted* is now told no usable answer could
be had, on account of the shape of a field this client does not model. That is
`wire.rs:12-15`'s own sentence failing, in the one direction it was written to
prevent.

**Evidence:** the three measurements above, plus the derive's behaviour
established directly against `serde` 1.0.229 / `serde_json` 1.0.151 in a probe
carrying `Reply`'s exact field types and round 2's alongside it. The two
citations the clause already makes are accurate and I checked them: RFC 8259 §6
does permit a parser to limit number range, and §4 does make a duplicated name's
meaning unpredictable.

**Severity, and what it is not.** 128 levels of nesting in a reply whose five
specified fields are all scalars is absurd, and I have not filed this higher for
that reason. But (iii) is a behavioural narrowing rather than a wording problem,
it was introduced by a repair, and it is on the axis the project's second
invariant protects — so the exposure belongs in this paragraph and not in the
severity.

Three dispositions, and they are independent. (i) and (ii) are clause edits: add
depth to the list, and say *a duplicated name among the five*. (iii) is a
decision — accept the narrowing with the depth written down, or make the shape
question cheaper than a full value parse so it cannot refuse more than the field
parse does. `tolerated` with the reasoning recorded is defensible for all three;
leaving the list at two while a third is reachable is not, because that is the
state P-D describes.

**Disposition:** fix-now
**Response:** All three, and (iii) is fixed rather than tolerated — a repair
that narrows the wire on an unmodelled field is the one thing this project
exists not to ship.

**(iii) — reordered, and the order is now the load-bearing thing.** The field
parse runs **first** and the shape question second. `serde_json` skips an
unknown field without building a value for it, so running that parse first makes
it the strictest thing in the path and nothing after it can refuse more than it
does. The shape question is then settled from the **first non-whitespace byte**:
after a parse that succeeded the document can only be an object or the sequence
form, so `{` decides it, and nothing is parsed twice on the accepting path. The
130-deep unmodelled field is exit 0 again.

Measured in both directions, as asked, against the real types rather than
reasoned about:

| bytes | before | after |
|---|---|---|
| `{"accepted":true,"extra":[[[…130…]]]}` | `Unreadable` | **`Accepted`** |
| `[1,2]` | `Unreadable` | `Unreadable` |
| `{"accepted":true,"accepted":false}` | `Unreadable` (duplicate field) | `Unreadable` (duplicate field) |

Both refusals held. The `Map` read survives on the **refusal path only**, because
`SendFault::Unreadable` carries a `serde_json::Error` and there is no other way
to obtain one; it runs for a reply already being refused. Its `Ok` arm is not
dead code — if it succeeds the document was an object after all and the byte was
wrong, and the parse is authoritative over the byte.

**(i) and (ii) — taken, and the list is now enumerated from the cases rather
than from the paragraph.** That is the third time this clause has been written
the other way round and been wrong, so it was written from measurement this
time: all five declared fields were posed against all three failure shapes, and
an unmodelled field against the same three. The result is sharper than the
finding predicted — **the reorder scopes every exception to the five declared
fields**, including the two that were previously unscoped:

| in a **declared** field | in an **unmodelled** field |
|---|---|
| nesting past 128 levels → `Unreadable` | `Accepted` |
| `1e999` → `Unreadable` | `Accepted` |
| duplicated name → `Unreadable` | `Accepted` |

So (i)'s recursion limit is on the list, (ii)'s *"a duplicated name"* is now *a
duplicated **declared** name*, and the range exception the earlier clause called
*"in any field, including one this client does not model"* was corrected to the
opposite by the same fix. `wire.rs`'s *"a sixth field from a newer host must not
turn a perfectly good verdict into a fault"* is true at every depth, range and
repetition.

`arbitrary_precision` was raised and is **declined** — recorded in the clause as
declined, not as open.

Cases: `an_unmodelled_field_cannot_turn_a_good_verdict_into_a_fault` (three
shapes) pins the boundary;
`the_three_shapes_serde_json_will_not_yield_a_value_for_are_unreadable` pins the
list; `bytes_that_are_not_one_json_object_are_unreadable` pins the shape guard.
Four injections, each reds exactly one: restoring the old order, dropping the
shape question, accepting any first byte, and making the byte check never match.

One clause of my own repair was corrected before it shipped, since it was a
fourth instance of the same pattern: the doc first claimed the whitespace set's
exactness was *"what lets one byte stand in for a parse"*. It is not — the `Map`
read is, because the byte answers only *yes*. The doc now says which half is
load-bearing and which is a fast path.

**Outcome:** verified

Round 4, all three parts, and the third is the one that mattered.

**(iii), the behavioural half, is fixed at the class rather than the case.** The
repair reorders `answer` so the `Reply` parse runs first, which makes it the
strictest thing in the path — `serde_json` skips an unknown field with
`ignore_value()`, an explicit-stack scan that builds no value, so nothing after
that parse can refuse more than it did. I posed the class rather than the case
the finding named, on the built binary:

```
unmodelled field, 200 levels deep    → exit 0
unmodelled field, 1e999              → exit 0
unmodelled field, duplicated name    → exit 0
unmodelled field, "\ud800"           → exit 0
```

Two hundred levels, not the 130 the case uses, and a fourth pathology the
ledger never named. An unmodelled field now costs nothing at all, which is
`wire.rs:12-15`'s own constraint holding rather than being restated.

**(i) and (ii) are done.** Depth is on the list with the limit named, and the
duplicate row now reads *a duplicated **declared** name*, which is the true
scope — measured:
`{"accepted":true,"extra":1,"extra":2}` → exit 0,
`{"accepted":true,"accepted":false}` → exit 2.

**The division of labour is exactly as claimed, and I checked the code rather
than the sentence.** `one_json_object` short-circuits to `Ok` only, on
`first == Some(&b'{')`; every other path goes to the `Map` read, so the byte
cannot produce a refusal at all. The self-corrected sentence is the right one:
exactness of the whitespace set is a property of the *fast path's hit rate*, not
of correctness, because an unrecognised leading byte falls through to a parse
that accepts the reply anyway. That replacement is true.

The `Ok` arm's reachability is [[F-18]] — a note about a claim, not about the
code, which is right.

**`arbitrary_precision` is recorded as declined**, in those words, with the
reason: a manifest-wide change to how numbers are held, to improve one sentence
about one pathological literal. Not dispositioned here.

[[F-17]] is a fourth shape of the same kind the list already holds three of. It
is a new finding, not a contest of this fix — every part of what F-15 asked for
landed.

### F-16 — "two things hold the write side" is a count, and there is a third path it does not name

**Severity:** nit
**Location:** `crates/goad-shell/src/ingress/wire.rs:43-58`, `:66-76`

**Expected:** the repair's answer to [[F-13]] is an enumeration, and an
enumeration is a completeness claim: *"Two things hold the write side now, and
neither is a field's type: 1. [`Reply::written`] is the only production
constructor … 2. `ingress::the_reply_s_bytes_are_exactly_these` asserts the
bytes."*

**Observed:** `Reply` is `pub` with five `pub` fields and no `#[non_exhaustive]`,
so the struct literal is still a construction path, and it is a path out of
`goad-shell`, `goad` and `goad-emit` alike. What holds the write side is
therefore neither of the two things named but a third: **that `ingress::reply`
is the only code which writes a reply, and nothing enforces that.** Item 2 is a
guard on `reply()`'s output, not on the type — a second writer built by literal
would be asserted by nothing.

```rust
// compiles today, from any of the three crates:
let wire = Reply {
  protocol: Some(serde_json::json!("1")),
  accepted: Some(serde_json::json!("yes")),
  reason: None, retry_after_ms: None, detail: None,
};
```

**Evidence:** the path is not hypothetical — it is exercised in the repository
already, at `crates/goad-shell/src/ingress/mod.rs:958-966`, where
`what_the_listener_writes_parses_back_to_what_it_built` builds its expected
value as a struct literal. That is a correct use in a test and it is also the
proof that the literal is available.

Worth noting what this does **not** say: the claim that the write side is better
than it was before this slice is true, and [[F-13]]'s outcome says so — `written`
types everything `Wire` typed and removes `protocol` from the caller's reach
entirely. The residue is that `Wire` was *module-private*, so it had no escape
hatch at all, and `Reply` must be `pub` because the reader needs it. The
enumeration is what overstates, not the design.

`#[non_exhaustive]` on `Reply` closes the cross-crate half at zero cost: it
blocks literal construction from `goad` and `goad-emit` and leaves `goad-shell`'s
own code — including that test — untouched, because the attribute only binds
outside the defining crate. Inside `goad-shell` the guarantee stays what it
honestly is, one writer by convention and review. `aligned` with the third item
added to the list is equally defensible; a count that reads as complete and is
not is the only thing being raised.

**Disposition:** fix-now
**Response:** Correct: an enumeration is a completeness claim and that one had a
third path. `#[non_exhaustive]` is on `Reply`, and it costs nothing — every
literal in the repository is inside `goad-shell` (`ingress/mod.rs:960`,
`wire.rs:123`) and the attribute binds only outside the defining crate.

**Verified rather than assumed**, by building the literal from a separate crate
— `goad-shell`'s own integration target, which faces the same barrier `goad` and
`goad-emit` do:

```
error[E0639]: cannot create non-exhaustive struct using struct expression
    --> crates/goad-shell/tests/integration/ingress.rs:1602:12
```

The enumeration is now three, and the third is the honest one the finding asked
for: (1) `Reply::written` is the only production constructor and takes typed
parts; (2) `#[non_exhaustive]` makes the literal unavailable outside
`goad-shell`, so from the other two crates `written` is not merely the
constructor used but the only one there is; (3) **inside `goad-shell` the
literal remains**, so in-crate this is one writer by convention and review, not
by the type — and what `the_reply_s_bytes_are_exactly_these` asserts is that one
writer's *output*, not the type. A second in-crate writer built by literal would
be asserted by nothing, which is why there is not one.

The finding's own note is kept in the doc rather than dropped: the write side is
better than pre-slice — `written` types everything the old private `Wire` typed
and removes `protocol` from a caller's reach entirely — and it was only the
count that overstated.

**Outcome:** verified

Round 4, and the count is now a true one. `#[non_exhaustive]` is on `Reply`
(`wire.rs:77`), so the struct literal is unavailable from `goad` and
`goad-emit`, and `Reply::written` is not merely the constructor used there but
the only one that exists. The enumeration is three items, and the third is the
honest one the finding asked for: *"Inside `goad-shell` the literal is still
available … so in-crate this is **one writer by convention and review**, not by
the type."*

Checked rather than accepted: every `Reply` struct literal in the repository is
inside `goad-shell` — `wire.rs:134` and `ingress/mod.rs:960`, both test
fixtures, and `client.rs:387` is a destructuring pattern, which
`#[non_exhaustive]` also permits in-crate. `goad-emit` names `Reply` nowhere at
all; it reaches the reply through `client::send`. So the attribute costs
nothing and closes the half it can close.

The finding asked for a true count and named `#[non_exhaustive]` as the cheap
fix. Both landed, and the third item says what the type does not hold rather
than implying the type holds everything.

### F-17 — the exception list is three and the true number is four: malformed string content in a declared field

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/client.rs:43-81` (the *Where that
stops* clause), and `the_three_shapes_serde_json_will_not_yield_a_value_for_are_unreadable`

**Expected:** the clause's own criterion, stated twice and pinned by two cases:
a **document a person would call one JSON object that `serde_json` yields no
value for**, and *"every one of the three is a fault in one of the five fields
`Reply` declares, and that scope is the whole boundary."* SPEC-003 §3 P-D is
what the clause exists to satisfy.

**Observed:** a fourth shape meets that criterion exactly and is not on the
list — **malformed string content in a declared field**. Measured on the built
binary:

```
{"accepted":true,"detail":"\ud800"}    → exit 2
  goad-emit: … is not one JSON object: unexpected end of hex escape at line 1 column 34
{"accepted":true,"extra":"\ud800"}     → exit 0
```

It qualifies on every clause of the criterion. A lone surrogate is
**grammatically a JSON object**: RFC 8259 §7 admits `\u` followed by four hex
digits without requiring a pair, and §8.2 says only that such a string is *"not
interoperable"* and leaves the behaviour to the implementation — which is the
identical standing to `1e999`, where §6 lets a parser limit number range and the
clause already lists it for that reason. `serde_json` yields no value.
And it is declared-scoped, like the other three: the same string in an
unmodelled field is accepted, because `ignore_value()` does not validate escape
sequences.

Its sibling is worth recording beside it, because it is the same code path with
a different standing: a raw invalid UTF-8 byte inside a **declared** field's
string is also `Unreadable`
(`Err(invalid unicode code point)`) and inside an unmodelled field is
accepted. That one is *not* an exception by the clause's criterion — RFC 8259
§8.1 requires JSON text to be UTF-8, so those bytes are not a JSON object at all
and the sentence is true of them — but the asymmetry is real and measured, and a
reader tracing the boundary will meet it.

**Evidence:** the two measurements above, plus a direct enumeration against
`serde` 1.0.229 / `serde_json` 1.0.151 over `Reply`'s exact field types, each
pathology posed in a declared field and in an unmodelled one:

| shape | declared | unmodelled |
|---|---|---|
| nesting past the limit | `Err(recursion limit exceeded)` | `Ok` — to 5000 levels |
| `1e999`, and a 400-digit integer | `Err(number out of range)` | `Ok` |
| duplicated name | `Err(duplicate field)` | `Ok` |
| **`"\ud800"`** | **`Err(unexpected end of hex escape)`** | **`Ok`** |
| invalid UTF-8 in a string | `Err(invalid unicode code point)` | `Ok` |
| `"\q"` | `Err(invalid escape)` | `Err(invalid escape)` — not declared-scoped |
| `1e-999` | `Ok` | `Ok` |
| duplicate nested inside a value | `Ok` | `Ok` |

**The finding is the method, not the row.** The repair states how the list was
built — *all five declared fields against all three failure shapes* — and that
is a cross-product over **hypothesised** shapes, so it can only confirm the
shapes already thought of. It is the third time this clause has been completed
that way and the third time a shape outside the hypothesis set has turned up.
Adding a fourth row makes the list right today and leaves the method unchanged.

So the repair worth taking is not a row. The clause has **two claims in it, and
only one of them is fragile**:

- *the scope* — every such fault is in one of the five declared fields, and an
  unmodelled field cannot cost a verdict. This is **generative, exact, and
  already pinned** by `an_unmodelled_field_cannot_turn_a_good_verdict_into_a_fault`.
  It follows from a property of the code — the `Reply` parse runs first and
  `ignore_value()` builds nothing — rather than from an enumeration, so it
  cannot go stale by someone failing to imagine a shape.
- *the count and the list* — fragile by construction, and wrong now.

State the boundary as the scope, demote the list to *the shapes measured so
far*, and drop the number. The clause then says something true that stays true,
and the cases keep pinning both halves. Behaviour need not move at all: all four
shapes are exit 2 with the host at fault, and the caller's decision is unchanged
— what is wrong is a sentence that counts.

**Disposition:** fix-now
**Response:** The method, taken. The clause is now a **scope** and the list is
demoted to *shapes measured so far*, explicitly non-exhaustive, with no count.

**The suggested wording is not exactly true, and this response is where I say
so rather than ship it.** *"An unmodelled field can never cost a verdict"* is
disproven by this finding's own `\q` row, and by two more I measured before
writing the sentence: an unbalanced bracket and a bare control byte in a string
are each refused from an unmodelled field too. All three are correct refusals —
none of those documents is JSON — so the sentence they earn, *"not one JSON
object"*, is **true** of them. The claim that is exactly true is therefore about
where the sentence is *false*, not about where a verdict is lost:

> Every document that is **grammatically one JSON object** and is nevertheless
> refused as not being one is refused over a fault in one of the five fields
> `Reply` declares. An unmodelled field's value is skipped rather than built, so
> every limit that comes of building one — a number's range, a `\u` escape's
> code point, nesting depth, a name seen twice — cannot arise there. What is
> left for a skipped field is finding where its value ends, which is the grammar
> itself: so an unmodelled field can cost a verdict only when the bytes are not
> JSON at all, and there the sentence is true of them.

That is generative in the way the finding asks for — it follows from
`ignore_value()` constructing nothing, not from an enumeration — and it is
exact against every row measured, the finding's eight and five more posed after
it (a 400-digit integer, `1e-999`, a nested duplicate, 5000 levels of nesting,
an unbalanced bracket).

The scope's second half had nothing pinning it, which the edit revealed, so
`bytes_that_are_not_one_json_object_are_unreadable` gained `"\q"` **in an
unmodelled field** — the boundary is now held from both sides. The two scope
cases are renamed to stop carrying the claim they no longer make:
`the_three_…` → `a_declared_field_serde_json_will_not_build_a_value_for_is_unreadable`
(the count is gone from the name as well as the prose, and the lone surrogate
joins its shapes), and `an_unmodelled_field_cannot_turn_a_good_verdict_into_a_fault`
→ `an_unmodelled_field_that_is_still_json_cannot_cost_a_verdict`, whose doc says
that the qualification is not hedging and where the excluded case is pinned.

The sibling the finding asked to be recorded is recorded: raw invalid UTF-8 is
`Unreadable` in a declared field and accepted in an unmodelled one, so the skip
is *more* lenient than the grammar there (§8.1). It widens acceptance and never
narrows it, so the scope holds — and a reader tracing the boundary now meets the
asymmetry in the doc rather than in a debugger.

**Outcome:**

### F-18 — `one_json_object`'s `Map`-read `Ok` arm is unreachable, which is correct and is not what the doc says

**Severity:** nit
**Location:** `crates/goad-shell/src/ingress/client.rs:303-315` (the doc),
`:328-331` (the arm)

**Expected:** the doc explains the arm as a live fallback: *"at worst an
unrecognised leading byte costs one needless parse of a reply that is then
accepted anyway"*, and *"If it succeeds, the document was an object after all and
the byte was wrong about it — the parse is authoritative, so that answer
stands."*

**Observed:** nothing can reach that `Ok` arm today, because the two whitespace
sets agree exactly. Measured against `serde_json` 1.0.151: it accepts leading
`0x20`, `0x09`, `0x0A`, `0x0D` before a value and rejects `0x0B`, `0x0C`, `0x00`
and `0xA0` with *expected value at line 1 column 1*. `one_json_object` skips
exactly those same four. So:

- `from_slice::<Reply>` succeeding implies the top-level value was `{…}` or
  `[…]` — every other JSON value is an `invalid type` error against a struct,
  which is the doc's own first sentence and is correct;
- the first non-whitespace byte is therefore `{` for every object and `[` for
  every array, with nothing else possible;
- `{` takes the fast path, so the `Map` read runs **only** for arrays, where it
  always errors.

Measured: `{"accepted":true}` and `   {"accepted":true}` → first byte `{`, exit
0; `[1,2]` and `  [1,2,3,4,5]` → first byte `[`, exit 2.

**Evidence:** the byte-set comparison above, and the four control measurements.
Nothing in the suite exercises the arm, and nothing can — which is the part
worth writing down, because a future reader with a coverage report, or one
simplifying the function, will find an `Ok` arm no case reaches and no comment
defending it *as unreachable*.

**The arm should stay.** It is insurance against exactly the drift the author
already self-corrected for once: if `serde_json`'s whitespace set ever widens,
or this one narrows, the byte stops being exact and the `Map` read is what keeps
the answer right. That is a good reason to keep dead code, and it is a different
reason from the one the doc gives. One clause fixes it — *the byte is exact
today, because both skip RFC 8259 §2's four bytes; the `Map` read is what makes
a future divergence cost a parse instead of a wrong answer, and nothing reaches
it while they agree.*

**Disposition:** fix-now
**Response:** Correct on all three counts — unreachable, correct, and not what
the doc said. The arm stays; only the doc moved. It now reads:

> **Its `Ok` arm is unreachable today, deliberately kept, and untestable.**
> `serde_json` accepts exactly `0x20`, `0x09`, `0x0A` and `0x0D` before a value
> and this function skips exactly those four, so the two whitespace sets
> coincide: a parse that has already succeeded leaves `{` for every object and
> `[` for every array and nothing else, `{` takes the fast path, and the read
> therefore runs only for arrays — where it always errors. No case reaches that
> arm and none can write one.
>
> It stays because it insures against the two sets **diverging**: if
> `serde_json`'s whitespace ever widens or this one narrows, the byte stops
> being exact, and the `Map` read is what makes that cost one extra parse
> instead of one wrong refusal. Read it as a bound on a future defect rather
> than as a path, and do not delete it for want of coverage.

The two sentences the finding quotes are gone — *"at worst an unrecognised
leading byte costs one needless parse"* and *"if it succeeds, the document was
an object after all"* both described a live fallback, and neither was true. The
last clause is aimed squarely at the reader the finding names: the one arriving
with a coverage report.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

This is the account of the implementation review of slice 005 — four adversarial
rounds over `git diff 03b0286..HEAD`, eighteen findings, no blockers at any
point. A reader who trusts this section does not need the findings.

**State of the ledger.** Sixteen findings `verified`, two open (F-17 `minor`,
F-18 `nit`), none `contested`, none `withdrawn`. It therefore does not meet the
Protocol's `Done` by the letter, and it should be closed **by disposition
rather than by a fifth repair round**: neither open finding is a defect in
behaviour, both are about sentences, and both are closable in the ledger with
an `aligned` or `tolerated` and a clause. The recommendation is in F-17 and is
not "add a row" — it is to state the boundary as the **scope** claim, which is
generative, exact and already pinned, and to demote the enumeration to *the
shapes measured so far*. That is what stops this clause needing a fifth
statement.

`just check` exits 0 at **503 cases**, from 450 at the review's start.

---

## What the review changed

**Three defects that would have shipped.**

- **The client narrowed the wire on the two fields whose values it does not
  read** (F-1). `protocol: Option<u8>` and `retry_after_ms: Option<u64>` refused
  a conforming host over how its encoder spelled a number — `1.0`, `1800.0`, any
  version past 255 — and reported the whole reply as *not one JSON object*, of a
  document that was one. This is the failure CLAUDE.md names as the one the
  project exists to avoid, reached from the read side that slice 004 never had.
- **A value of `-h` exited 0 having sent nothing** (F-2). `goad-emit --source w
  --kind k --data --version` printed the version and exited **0** — the code D-4
  reserves for *the host accepted it* — because the help scan ran over the whole
  argument vector before values were resolved.
- **The reply was read unbounded** (F-3). SPEC-003 §6.4 bounds a read at 64 KiB
  and cites SPEC-001/R-43 for why; the host implements it and the client did
  not, so a host that never terminated a reply allocated until the OOM killer
  turned a caller's exit 2 into a signal.

**And the substantive smaller ones.** A §6.3-ambiguous reply guessed at rather
than refused, where an `accepted: true` carrying a `reason` was reported to every
caller as *the host took it* (F-4). A non-UTF-8 reply blamed on the transport
(F-5). Three `render` cases asserting a property of a set instead of a mapping
(F-7) — the project's own named failure mode, sitting in the one phase whose VA-1
was a manifest check rather than an injection pass. A wrong-typed
`retry_after_ms` on an acceptance passing silently at exit 0 (F-14). And the
one narrowing a **repair** introduced: a shape guard stricter than the field
parse, turning a reply carrying a deeply nested unmodelled field from exit 0 into
exit 2 (F-15(iii)).

**How the repairs were made matters as much as what they fixed.** Each of the
three majors was repaired at the level of the **rule**, not the instance, and
each fix closed things no finding had asked about: `Oversized` reaching the
render table; a `fake_listener` re-typing that made two cases writable at all;
`--help` and `--version` cases for behaviour PHASE-03 stated, AC-8 checked by
hand, and no tier had ever reached; and a shape guard that turned an existing
passenger case into a load-bearing one. Every round verified the previous
round's repairs; none was contested, none withdrawn, and no repair had to be
repaired.

---

## What the review confirmed

**The four PHASE-01 lifts are lifts.** `clock.rs` is byte-identical to its
pre-lift form once doc comments are set aside; `default_path` reproduces
`arguments`' `[]` arm condition for condition; `line_to` moved with its body and
its judgement intact and gained the two cases it never had. The one change that
was *not* a move — the reply's wire type — is pinned by an exact-byte
characterization case written **before** the lift, which is the right instrument
in the right order.

**The strata hold, and AC-7 holds by the crate edge as claimed.** `cargo tree -p
goad-emit` contains zero occurrences of `slint`; the manifest is the three
dependencies and no dev-dependency; nothing in the workspace names the new
member; and the domain-vocabulary scan reaches it by reading `workspace.members`
for itself rather than by a list someone had to remember to edit.

**The contract is implemented in both directions and the client is now
permissive where it should be and strict where it must be.** `protocol` is
unread in value as well as presence; an unknown reason token and an unknown
field survive; `1800`, `1800.0` and `1.8e3` are one verdict; and every field
§6.3 types is adjudicated against that type, so a wrong one is a named breach
rather than a parse failure wearing a false sentence. `whole_millis` is correct
at every edge posed — `1800.5`, `-1`, `-0.0`, `2^64`, `1e300` all refused,
`18446744073709549568.0` accepted exactly — and correct *by construction*,
because the cast-free `fract`-then-`parse` route is what stops `as` silently
turning any of them into a plausible millisecond count someone would sleep on.

**The exchange is bounded symmetrically**: `reply_line` mirrors `read_envelope`'s
`take(LIMIT + 1)` idiom, its `> LIMIT` test and its pop, and both edges are
pinned with the at-bound probe EOF-framed for the reason that makes it mean
anything.

**AC-5 and AC-6 are held honestly rather than by proxy.** `source: "host"` is
really sent and the host's own refusal is really what comes back, with the
judge's record asserted so a client-side pre-empt would red the case; AC-6's
subject is the normalized `Event`, never the bytes.

**The canon and card reconciliation is factually accurate where it touches
code**: five members, the allowlist instrument covers exactly `goad-semantics`
and `goad-shell`, the vocabulary scan really is self-enumerating, and
`EnvelopeFault::Empty` exists as cited.

---

## What this knowingly leaves standing

**Three (measured) declared-field shapes report a coarse sentence.** A reply
that is one JSON object but which `serde_json` yields no value for is reported
as *"not one JSON object"*. The shapes measured are: nesting past the recursion
limit, a number outside the range `serde_json` represents, a duplicated
**declared** name, and — F-17, not yet on the list — malformed string content
such as a lone `\ud800`. All are exit 2 with the host at fault either way, so
**what they cost is the precision of one sentence, never a caller's decision.**
The boundary that matters is exact and pinned: every one of them is a fault in
one of the five fields `Reply` declares, and **an unmodelled field cannot cost a
verdict** — measured to 200 levels of nesting, `1e999`, a duplicated name and a
lone surrogate, all exit 0. That is the invariant, and it holds.

**`arbitrary_precision` is declined, not open.** Closing the `1e999` shape needs
that feature workspace-wide, which changes `Number`'s representation and
`PartialEq` for `goad-semantics`' protocol types. Raised under plan STOP
condition S-5 and declined by the user: a manifest-wide change to how numbers
are held, to improve one sentence about one pathological literal.

**AC-2's last clause is review-held by design, and review holds it.** *"Nothing
branches on `detail`"* (SPEC-003/R-14) is an absence of code, and a test that
renders a line containing `detail` cannot tell appending from branching —
`plan.md`'s Coverage table says so rather than claiming a case. Checked at every
round and true at the last: `render::refused_line` is the only function that
reads `detail`, it interpolates the string, and no arm anywhere matches on its
content.

**`SendFault::Faulted` is constructed in production and reached by no case at
any tier.** A connection that breaks mid-exchange has no deterministic trigger
at this tier. It is narrower and more honest than it was — after F-5 it has one
meaning instead of two — but it remains the one variant nothing exercises.

**`one_json_object`'s `Map`-read `Ok` arm is unreachable** while `serde_json`'s
whitespace set and this module's agree, which they do exactly. The arm is right
and should stay: it is insurance against that drift, and it is the reason the
leading-byte test cannot produce a wrong refusal. F-18 is that nothing says so.

**`goad::diagnostics::USAGE` still reads as though `$HOME/.config/goad/config.toml`
is always the fallback**, which `default_path` has never said. F-8 corrected
emit's copy and declined the host's as outside its surface. Two binaries state
one rule; one states it correctly. `crates/goad`'s to answer.

**The exchange is held end to end only by AC-8** — one person, once — because no
test target links both the CLI binary and a running host. That is the right call
for a tier-1 slice and it is unchanged by anything in four rounds.

---

## The durable lesson

After round 1, every finding this review raised was the same shape: **a correct
fix stated one notch stronger than it holds.** *"`Unreadable`'s sentence is true
whenever it is said"* had five exceptions. *"A modelled field contradicting
another is refused"* was applied to one of §6.3's three co-occurrences. *"Two
things hold the write side"* was three. *"Three exceptions"* is four.

SPEC-003 §3 P-D already legislates for this — *an absolute clause names its own
exception, and one that cannot say where its exception would be has not been
checked* — and the repairs reached for it by name, which was the right instinct.
What four rounds add is the part P-D does not say: **an enumerated list is not a
checked clause.** Three times the list was completed by cross-producting the
shapes its author had thought of, and three times a shape outside that set
turned up. The clause that survives is the one stated **generatively**, from a
property of the code — *the `Reply` parse runs first and `ignore_value()` builds
nothing, therefore an unmodelled field cannot cost a verdict* — because it
cannot go stale by someone failing to imagine an input. Where a list is
genuinely wanted, it belongs beside such a claim as *examples measured*, not as
a count.

The implementation was sound at round 1 and is materially better now. Nothing in
this ledger has been a reason to hold the slice at any point.

---

## Closing note — 2026-09-14, by the orchestrator

The Synthesis above is the raiser's, written when the ledger stood at sixteen
`verified` and two open. This note records what happened after it and does not
amend it.

**The user's disposition: make both edits, close without a fifth round.** F-17
and F-18 were repaired — two sentences, no code path changed — and the ledger
closes at **eighteen findings, no blockers, none contested, none withdrawn.**
Sixteen were verified by a review round; **F-17 and F-18 were repaired but not
themselves reviewed**, which is the one thing this closure accepts knowingly and
the reason it is recorded here rather than folded into the table.

What the two edits did, so the Synthesis's "not yet on the list" reads correctly
against the code:

- **F-17's list is gone**, not extended. The clause is now a scope claim, and it
  is generative rather than enumerated: an unmodelled field's value is *skipped
  rather than built*, so every limit that comes of building one — a number's
  range, a `\u` escape's code point, nesting depth, a name seen twice — cannot
  arise there. What remains for a skipped field is finding where its value ends,
  which is the grammar, so an unmodelled field can cost a verdict only when the
  bytes are not JSON at all — and there *"not one JSON object"* is true of them.
- The orchestrator's **proposed wording for that claim was itself over-stated**
  and was rejected by the repair agent with three counter-measurements (`\q`,
  an unbalanced bracket, a bare control byte, each refused from an unmodelled
  field and each correctly). That is the fifth instance in this slice of the
  pattern the review named, and the first raised against the orchestrator.
- The edit revealed the scope's second half was pinned by nothing;
  `bytes_that_are_not_one_json_object_are_unreadable` gained a `"\q"` row, so
  the boundary is now held from both sides. Both scope cases were renamed to
  stop carrying a count.
- **F-18's arm stays.** Unreachable today, deliberately kept, untestable — a
  bound on a future defect rather than a path, documented as such.

`just check` exits 0 at **503 cases**, 450 at the start of the audit.

