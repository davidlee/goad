# Research — Slice 002

**Producers:** slice-002-research workflow (5 survey threads, 3 spikes, 2 adversarial verifiers)
**As of:** 2026-09-04 · a6ae617

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — verified by execution, or by reading the cited site. Spike results carry ✓
  only where an adversarial verifier reproduced them.
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

## Citation forms

Canon claims cite the document id (`SPEC-001 R-35`, `ADR-002`). Code claims cite
`path:line`. Registry claims cite the crate source path and version. An uncited
claim is unverifiable by definition.

---

## Thread 1 — governing canon

### Binding

`docs/specs/001-host-backend-protocol.md`, `docs/adr/001-one-way-strata.md`,
`docs/adr/002-single-crate-until-triggered.md`. `docs/policy/` is empty ✓
(`ls docs/policy` — no files), so those three are the whole of canon.

**SPEC-001 says almost nothing about drawing.** §2 puts "how a view is drawn"
explicitly out of scope, and states the renderer's entire contract in one
sentence: the renderer abuts the spec at the canonical view — it consumes one,
produces a user response, and may not see wire types ✓. No requirement
R-1..R-54 describes drawing. The slice is therefore mostly on unwritten ground,
and `docs/AGENTS.md:29-41` requires that ground be drafted in the slice folder
(`draft-spec.md` / `canon-delta.md`), not held in an agent's head ✓.

What does bind, because a renderer can violate it by accident:

- **R-18** — only the renderer MAY branch on a hint key. This is the one
  requirement that grants the renderer a power; it is also the trap, because a
  hint that decides anything but presentation has become protocol ✓
  (`src/semantics/protocol/canonical.rs:127`, `Hints::as_map`).
- **R-30/R-32/R-33/R-34** — the host mints every `view_id`, at most one
  interaction is outstanding, a stale `respond` is rejected without contacting
  the backend, and rejecting one does not clear the outstanding interaction ✓
  (`src/shell/host.rs:152-176`, `src/shell/state.rs:100-115`).
- **R-35 / R-9** — the host MUST NOT validate a submitted answer beyond its
  `view_id`, and MUST NOT interpret a field value. This is the renderer's most
  accidentally-violable requirement: a required-field check, a min/max clamp, a
  disabled Submit button are each the host adjudicating what the spec assigns to
  the backend. `NumberRange` bounds are affordances, never gates ✓.
- **R-45/R-46** — no backend failure terminates the host; no panic on any value
  derived from a backend. R-46 is held by lints, not tests ✓ (`Cargo.toml:133`
  and the surrounding block: `unwrap_used`, `expect_used`, `panic`,
  `indexing_slicing`, `unreachable` all `deny`).
- **R-20 / R-47** — a view part the host cannot read MUST NOT be silently
  dropped; every refusal of a backend-supplied value MUST be reported. Both are
  stated about normalization. Whether they reach the glass is *not settled* —
  see Amendment candidates.
- **R-12** fires only on an *unrecognised* `kind`. A field kind the protocol
  models but the renderer does not implement (`number`, `datetime`, `choice`)
  is admitted by normalization and arrives fully valid ✓
  (`src/semantics/protocol/canonical.rs:246`).

**ADR-001** puts the renderer at stratum 3, the leaf: it may name
`crate::semantics` and `crate::shell`; nothing there may name it ✓. Its own
Verification section concedes that only the dependency-graph half is a build
gate; the direction half is a review gate.

**ADR-002** requires the trigger check to be recorded in this slice's design,
and is superseded — not amended — whichever way the check lands ✓
(`docs/adr/002-single-crate-until-triggered.md`, Verification). Its three
triggers, verbatim at `:33-42` ✓: T1 a dependency stratum 1 must not need in
order to build (with an explicit carve-out for optional dependencies); T2 a
second binary; T3 headless test wall-clock dominated by renderer build time.

**docs/memory/ is binding in practice**, and two notes are directly load-bearing:
`cite-requirements-not-finding-ids.md` (in force *from slice 002 by name*: code
comments cite `SPEC-001/R-N`, a section, an ADR or the brief — never a slice-local
`F-N`) and `a-bound-is-not-tested-at-the-bound.md` (name the two implementations
a limit-test must separate, and assert the observable that differs) ✓.

### Checked, not applicable

- `docs/policy/*` — empty ✓.
- SPEC-001 §6.4 (process transport), §6.1–§6.3 (wire formats), R-36..R-44
  (transport and scheduling) — closed by slice 001; the renderer sits above the
  transport and never sees it.
- SPEC-001 OQ-1 (capability declaration) and OQ-2 (validation feedback) — both
  renderer-shaped, both explicitly deferred past 002 by `slice-001.md`
  Follow-ups and `docs/roadmap.md`.
- ADR-002 T2 — slice 002 creates the *first* binary; there is no `[[bin]]`
  today ✓ (`src/lib.rs:1-12`, `Cargo.toml` has no `[[bin]]`). T2 says *second*.
  Say so explicitly rather than leave it to inference.
- `docs/roadmap.md` — advisory by its own words, binds nobody. It states the
  ADR-002 split as more settled than ADR-002 itself does; where they differ,
  canon wins.

### Amendment candidates

1. **ADR-002's stated reason for expecting T1 is measurably false.** It says a
   build-dependency with a conditional `build.rs` "cannot be gated as cleanly".
   It can, exactly. Three independent measurements ✓: an optional
   `[build-dependencies]` entry plus `#[cfg(feature = "ui")]` inside `fn main()`
   resolves the `--no-default-features` normal+build graph to **one node — the
   crate itself** (537 with the feature on), and `cargo build
   --no-default-features` prints `build.rs ran; ui = false` and compiles no
   Slint crate. T1 still fires, but on different ground (see Thread 6).
   Reconciling this is an ADR-002 amendment, and ADR-002 is superseded anyway.
2. **What a renderer does with a protocol capability it does not implement.**
   R-20 forbids silently dropping a view part *at normalization* and gives the
   reason ("dropping it would render a view the backend did not author"), but §2
   puts drawing out of scope, so R-20 does not reach the glass. This is exactly
   CLAUDE.md invariant 3's territory and needs `draft-spec.md` or a
   `canon-delta.md` entry against SPEC-001 §2.
3. **`tests/protocol/boundary.rs` scans `.rs` only**, root `src` ✓
   (`tests/protocol/boundary.rs:119`). A `.slint` file — labels, placeholder
   text, component names — is invisible to the domain-vocabulary scan. CLAUDE.md
   invariant 1 asserts "a boundary test greps for it"; the day slice 002 lands,
   that is false for the markup where the temptation is highest. The file's own
   header prescribes the fix shape: extend the configuration, not the walk.
4. **The gate's canonical command block lives in `docs/slices/001/design.md`
   §9** — a closed slice's design ✓ (`CLAUDE.md`, `justfile`). Slice 002 changes
   the gate. AGENTS.md otherwise forbids retro-fitting a closed design. Promoting
   §9 into canon is a canon creation requiring endorsement.
5. **SPEC-001 §7 names the fixture directory path normatively.** If the crate
   splits, moving `tests/protocol/fixtures/` is a canon change, not a file move.

---

## Thread 2 — code map

### Hotspots

Every file slice 002 touches is new except four:

| path | why |
|---|---|
| `Cargo.toml` | slint + slint-build + the testing dev-dependency; two new tokio features (`rt-multi-thread`, `sync`); possibly `[[bin]]`, a new `[[test]]`, or a workspace rewrite |
| `justfile` | the gate changes shape (Thread 6) |
| `docs/slices/001/design.md` §9 | canonical source for the gate |
| `tests/protocol/boundary.rs` | `.slint` invisibility; and if the crate splits, its `tokio` scan is retired (Thread 6) |

New: `build.rs`, `ui/*.slint`, a renderer module under `src/`, one binary, one
test target, and a small assertion-helper module.

### Cited facts

**The renderer's entire input is `Outcome`** ✓ (`src/shell/host.rs:70-100`):
`view: Option<Presented>`, `next_check`, `discarded: Vec<Discarded>`,
`stderr: Captured`, `failure: Option<Failure>`, `cleanup: Option<CleanupFailure>`.
`Presented { view_id, view }` is owned and pairs the view with the id that
answers it ✓ (`src/shell/host.rs:35-38`, invariant I14). Three screen states
fall straight out: `view: Some(_)` is the choice; `view: None, failure: None` is
empty; `view: None, failure: Some(_)` is diagnostic.

**The renderer's entire output is** `Host::respond(&mut self, now, view_id,
UserResponse)` ✓ (`src/shell/host.rs:152-176`). `OptionId::new` and
`FieldId::new` are `pub(super)` ✓ (`src/semantics/protocol/canonical.rs:60,89`),
so the renderer *cannot mint an id*. An answer names an id cloned out of the view
being answered, which means the renderer must carry the presented options (or
their ids) alongside the `view_id`. A design reaching for a `String` option id
hits a privacy error, and that error is the invariant working.

**The canonical walk** ✓ (`canonical.rs:162-256, 321-403`):
`View::Choice(Choice)` — one variant, **not** `#[non_exhaustive]`, so a renderer
`match` is exhaustive and a future view kind is a compile error at every site;
`Choice::{title() -> &str, body() -> Option<&Content>, options()}`;
`Options::as_slice()` non-empty by construction (no zero-option branch needed);
`Opt::{id, label() -> &str, fields}`; `Fields::as_slice()` MAY be empty;
`FieldKind::{Text, Boolean, DateTime, Number(NumberRange), Choice{alternatives}}`;
`Content::{Text, Markdown, Html, Uri}`.

**Diagnostics render themselves; stderr does not.** Every error type implements
`Display` ✓ (`src/shell/error.rs:23,58,84,107`; `src/semantics/error.rs:32,102,117,141`).
`Captured` does not — it is `{ bytes: Vec<u8>, truncated: bool }` ✓
(`src/shell/backend/transport.rs:74-78`), and up to `STDERR_LIMIT = 256 * 1024`
arbitrary, possibly non-UTF-8 bytes reach the renderer on **every** outcome
(R-42) ✓ (`src/shell/backend/process.rs:26`).

**The F-42/F-47 inheritance is three obligations, not one.** `slice-001.md`
Follow-ups: `Outcome` is per-call and forgotten (retention); a discarded
`next_check`'s `raw` renders verbatim and unbounded, newlines included
(bounding); and `ConfigError::Duration` both embeds its fault in `Display` and
returns it from `source()`, so a chain-walking logger prints it twice ✓
(`src/shell/error.rs:155-160, 183-190`). The same convention holds for
`BackendError::Protocol` and `ProtocolError::{Json,Bounds,Schedule}`. Round 5's
probe recorded a discard rendering as three physical lines from a `raw` of
`"\n18:00:00\n"`. The standing rule the repair established: **each fact rendered
exactly once** — re-prefixing the raw reopens F-42, dropping it reopens F-47.

**`cargo test --no-default-features` is ADR-001's compiler** ✓: `cargo tree
--no-default-features` resolves goad + jiff, serde, serde_json — 16 unique
crates, no tokio, no toml; the default column is 31. It runs at all only because
`tests/integration` declares `required-features = ["shell"]`.

**Lints that will bite the renderer** ✓ (`Cargo.toml`): `unwrap_used`,
`expect_used`, `panic`, `indexing_slicing`, `print_stderr`, `as_conversions`,
`cast_possible_truncation`, `cast_precision_loss`, `clone_on_ref_ptr`,
`allow_attributes` (:129), `allow_attributes_without_reason` (:130),
`tests_outside_test_module` (:201), `future_not_send` (:197),
`await_holding_refcell_ref`. The existing suite has **zero** `.unwrap()` across
all twelve test files. `[lints]` applies to build scripts and to integration
test targets.

### Precedents

- Test tiers: tier 1 `tests/protocol/main.rs` runs in **both** feature columns —
  naming `tokio` there is a compile error, so a Slint test cannot live in it.
  Tier 2 `tests/integration/main.rs` gates on `required-features = ["shell"]`.
  Tier 3 is 14 bash scripts under `tests/backends/`.
- Every test module is declared `#[cfg(test)] mod x;` solely to satisfy
  `tests_outside_test_module` ✓ (`tests/protocol/main.rs:5-20`).
- `tests/integration/fake.rs` — a `Backend` answering from a scripted
  `VecDeque<Exchange>` with a shared `Calls` counter. This is the prior art for
  driving a renderer without a process; it is `pub(crate)`, so reuse means
  relocating or duplicating it.
- `tests/protocol/transport_shape.rs` — when a structural property needs a source
  scan, the house style is a **new purpose-built scanner**, not a generalised
  one; its header says so explicitly ✓ (`:14-17`).
- `docs/memory/slint-build-mechanics.md` settles build wiring: `slint` +
  `slint-build` at the same version, `build.rs` calls `slint_build::compile`,
  `slint::include_modules!()` at crate root, stock widgets need
  `import { Button } from "std-widgets.slint";` or the *build script* fails.

---

## Thread 3 — headless testing of a Slint UI

The question this research existed to settle. **Yes, decisively.**

`i-slint-backend-testing = "=1.17.1"`'s `init_no_event_loop()` installs a
windowless platform, and a `.slint` UI is instantiated, queried, driven and
asserted on inside an ordinary `cargo test` with `WAYLAND_DISPLAY`, `DISPLAY`
and `XDG_RUNTIME_DIR` unset ✓ — reproduced by three independent producers, 14–16
tests green per run at 0.14–0.20 s per binary.

**It is not merely absent of a compositor; it is structurally unreachable.** ✓
An adversarial verifier ran `strace -f -e trace=socket,connect,bind` over the
full suite with a **live niri compositor and the complete display environment
present**: **0 `socket()`, 0 `connect()`, 0 `bind()`** across the whole run, and
14/14 pass. `SLINT_BACKEND=winit` forced, compositor live: still 0 sockets.
`init_no_event_loop()` calls `i_slint_core::platform::set_platform()` directly,
before the backend selector ever runs
(`i-slint-backend-testing-1.17.1/lib.rs:37-45`), and it wins over the env
selector. winit, wayland-client, x11rb, glutin and femtovg *are* linked into the
test binary, so a silent fallback was structurally possible — and does not occur.

Negative control ✓: the same `ChoiceView::new()` without `init_no_event_loop()`
errors with `neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set` under
a stripped env, and creates a window under a full one. The stripping does real
work.

### What the tier can do

- **Assert** ✓: element existence and count; `accessible_role`, `accessible_label`
  (a `Text`'s label is its rendered string), `accessible_value`,
  `accessible_description`, `accessible_checked/checkable`,
  `accessible_item_index/count`, `accessible_enabled`, `id()`, `type_name()`,
  `bases()`, `layout_kind()`, `is_valid()`, `size()`, `absolute_position()`
  (`i-slint-backend-testing-1.17.1/search_api.rs:454-891`).
- **Drive** ✓: `invoke_accessible_default_action()`,
  `set_accessible_value(...)`; keyboard via the public
  `window().dispatch_event(WindowEvent::KeyPressed { text })` — Tab then Space
  activated a button and fired the Rust callback with no event loop.
- **Conditional states** ✓: an `if cond: elem := ...` branch enters and leaves
  the element tree as the condition moves; a `VecModel` push is seen by the
  repeater immediately. No event-loop tick required. This is exactly slice 002's
  empty and diagnostic states.
- **`slint::Timer` fires** under `init_no_event_loop()` + `mock_elapsed_time()` ✓,
  **contradicting the crate's own doc comment** (`lib.rs:29-33` says Timers
  "won't work"). Verified twice independently: `mock_elapsed_time` explicitly
  calls `TimerList::maybe_activate_timers` (`testing_backend.rs:44-54`).
- **`mock_single_click(PointerEventButton)` works in this cheap tier** ✓ — a real
  pointer sequence (PointerMoved + PointerPressed + `mock_elapsed_time(50)` +
  PointerMoved + PointerReleased at `absolute_center()`,
  `search_api.rs:952-973`), no `init_integration_test_*`, no `spawn_local`, no
  `run_event_loop`. Its own doc comment says it exists for tests without an event
  loop. **This deletes the "one `[[test]]` target per pointer test" budget line
  — but only for pointers. See the contradiction under §Cross-thread.**

### The traps, all reproduced

**T-A. Debug info, and the vacuous empty-state test.** The whole
ElementHandle/ElementQuery API returns **empty** with only a warning when the
Slint compiler emitted no debug info — `find_by_accessible_label` included
(`search_api.rs:61-65, 329-335`). `build.rs` must call
`slint_build::compile_with_config(path,
CompilerConfiguration::new().with_debug_info(true))`; the method is
`#[doc(hidden)]` (`slint-build-1.17.1/lib.rs:223-232`).

The consequence slice 002 cannot avoid ✓: **the empty-state test is the one test
that cannot detect its own harness being broken.** Written naturally it asserts
*absence*, and absence is what a broken harness returns. Demonstrated with a
deliberately wrong setup — three options in the model, assert zero showing:
debug info on → `left: 3, right: 0`, FAILED; debug info off → `ok`. Reproduced
verbatim by a verifier. Removing `with_debug_info` from `build.rs` turns 13 of 14
tests red — every one except the test that queries no elements.

Two mitigations, both needed: a guard test asserting a known element *is* found,
and writing the empty-state test as "the empty-state placeholder **is** present"
alongside "no options are present".

Partial correction to the "silent" framing ✓: `warn_missing_debug_info` routes
through `writeln!(std::io::stderr(), ...)` (`i-slint-core-1.17.1/debug_log.rs:87`),
a direct FD write libtest does not capture. **Loud in the log, invisible in the
result line.**

**T-B. `init_no_event_loop()` at the top of every `#[test]` fn, never behind a
`std::sync::Once`** ✓. The platform is a thread-local (`context.rs:51-54`), so a
`Once` guard makes later tests fall through to winit. Three tests behind a
`Once`, `--test-threads=3`: `a: true, b: false, c: false`. Per-test-fn init is
parallel-safe; `--test-threads=1` also passes, because libtest still spawns a
thread per test.

**T-C. Never `.match_descendants()` directly after `ElementQuery::from_root()`** ✓.
`from_root` is defined as `root_element().query_descendants()`, so the extra call
descends a second level and silently excludes depth-1 elements: `from_root().
match_type_name("VerticalLayout")` = 1, with `.match_descendants()` = 0.

**T-D. `accessible-role: button` does not confer focusability** ✓. A hand-rolled
`Rectangle` + `TouchArea` with full `accessible-*` declarations responds to
`invoke_accessible_default_action()` and to `mock_single_click`, but Tab+Space
never reaches it. Deleting the `FocusScope` from the spike's component turns
*only* the keyboard test red; deleting the `TouchArea` turns *only*
`mock_single_click` red. If the choice view is custom-drawn and keyboard-operable,
each option needs its own `FocusScope`. Stock `std-widgets` `Button` already
wraps one (`i-slint-compiler-1.17.1/widgets/fluent/button.slint:105-118`), and
activates on `" "` or `"\n"`, rejecting everything else so Escape bubbles to an
ancestor `FocusScope`.

**T-E. Selector ambiguity, and its fix is also wrong.** ✓ A component whose
`accessible-label` is also drawn by an inner `Text` yields **two** handles for
that label. The spike's house rule — select on `match_type_name("<Component>")`
and filter the label — fixes that. A verifier then refuted it for the real
protocol ✓: SPEC-001 R-14 and `Options::new`
(`src/semantics/protocol/canonical.rs:332-342`) constrain option **ids** only,
never labels, so two options may legally share a label, and the type+label
selector returns 2 for `[{snooze_5,"Snooze"},{snooze_15,"Snooze"}]`. **Select on
`accessible_description` carrying the `OptionId`** — verified unambiguous.
Separately, `match_id` is unusable for repeater contents: `id()` is `""` for
anonymous and repeated elements.

**T-F. `match_type_name` couples every test to a `.slint` identifier** ✓. A pure
rename `OptionButton` → `OptionCtl` turns 7 of 14 tests red and leaves the
empty-state test **green**, because a selector matching nothing satisfies a
count-of-zero. Same mitigation as T-A: assert presence, not only absence.

**T-G. Virtualisation breaks `find_all()`** ✓, and this is the trap that reaches
furthest into the design. SPEC-001 R-13 admits any number of options and a real
renderer will scroll. 100 options in a 300×120 window: `ListView` →
`find_all().len() == 5`; `ScrollView` → **6**. Raise the window to 3000 px and
both return 100. It is viewport-driven lazy instantiation, and `ScrollView` is a
`Flickable` too, so avoiding `ListView` does not avoid it. "Three options are
showing" becomes viewport- **and font**-dependent, and the absence-shaped
assertion hazard returns in a form the debug-info guard cannot catch.
Mitigations exist and are unmentioned in every earlier producer:
`ElementHandle::scroll(dx, dy)` (`search_api.rs:1068`) and
`accessible-item-index` / `accessible-item-count` (`search_api.rs:769-780`),
which let a visible row declare the true total.

**T-H. The headless tier hard-requires a system font.** ✓ With an empty
fontconfig (`FONTCONFIG_FILE` → `<fontconfig></fontconfig>`), **all 14 tests
fail at `ChoiceView::new()`** with a third-party panic:
`fontique-0.10.0/src/backend/fontconfig.rs:685: called Result::unwrap() on an
Err value: NoMatch`. `flake.nix:30-34` ships the fontconfig *library* on
`LD_LIBRARY_PATH` and no fonts; the face actually resolved on this machine comes
from the host's `/etc/fonts/fonts.conf` (266 font-file opens per run). A CI
container running `nix develop` with no fonts gets a hard panic, not a test
failure. **This is a flake question, and the flake is the user's.**

**T-I. Geometry is readable but not portable, and the documented fix is
half-available.** ✓ Layout runs headless; `size()` and `absolute_position()`
return real values (`LogicalSize { width: 384.0, height: 24.0 }`, y 151.33 then
179.33 — reproduced to the digit). Determinism requires the literal font family
string `"FixedTestFont"` (`testing_backend.rs:135-139`), which short-circuits
`text_size` to `chars × pixel_size`; setting it moved the same positions to y
148.0 / 176.0 ✓. It does **not** rescue T-H. The other route,
`configure_test_fonts()` with embedded NotoSans, is `#[cfg(feature = "internal")]`
(`lib.rs:92-93`) and unreachable from outside Slint's own repo ✓ — so this
mitigation is verified *unavailable*.

**T-J. A test that asserts a failure to construct is coupled to the absence of a
display, and breaks the gate.** ✓ The spike's `oncetrap.rs` asserts a sibling
thread must fail to create a view. Under a plain `cargo test` in the dev shell —
which is exactly `just test` (`justfile:23-24`), with no env stripping — it
FAILS: `15 passed, 1 failed`. Any negative-construction assertion must not enter
the gate in that form.

### The lint collision — structural, and the sharpest ADR-002 evidence

`slint::include_modules!()` splices the compiler's generated Rust into the
crate's module tree, so the crate's `[lints]` apply to it. With goad's `[lints]`
copied **verbatim** (diffed byte-identical), clippy over unsuppressed generated
code errors in the thousands ✓.

Three independent derivations, three different totals, **the same twelve
lints**: 4786, 3205/target, 3619. The number scales with the size of the
`.slint` file — which is the point, because slice 002's real UI is larger than
any of the three probes. The twelve:
`as_conversions`, `unwrap_used`, `shadow_unrelated`, `same_name_method`, `panic`,
`indexing_slicing`, `let_underscore_must_use`, `clone_on_ref_ptr`, `todo`,
`pub_use`, plus rustc's `unreachable_pub` and `missing_debug_implementations`.

Why the generated code's own suppression does not help ✓: it opens with
`#![allow(clippy::all, clippy::pedantic, clippy::nursery)]` (verified verbatim at
generated `OUT_DIR/main.rs` line 2). **Every lint above is a restriction lint or a
rustc lint, and restriction lints are in none of those three groups.** goad's
lint table is unusually restriction-heavy, so the collision is near-total. There
is no `unsafe` in the generated code, and `cargo fmt` does not see it (it lives
in `OUT_DIR`).

Mitigation, verified to exit 0 ✓: one quarantine module carrying a blanket
module-level suppression around the include. **A verifier found the mechanism the
spike could not explain**: `clippy::allow_attributes` fires only on an **outer**
`#[allow]`, never on an **inner** `#![allow]` module attribute — demonstrated
with two hand-written probe modules in one crate, only the outer erroring
(clippy 0.1.99, cbae9b4cae 2026-08-28). That is not Slint-specific; **it is a
standing hole in goad's lint posture that any module can use**, which is itself
relevant to whether the wrapper is an acceptable design position.

`#![expect(...)]` also exits 0 and self-cleans, but is brittle: a lint the
codegen stops emitting becomes `unfulfilled_lint_expectations`, which
`-D warnings` turns into a build failure on a Slint patch bump ✓ (reproduced with
`clippy::mem_forget`, `clippy::exit`, `clippy::dbg_macro`). It survived a
substantial `.slint` expansion unchanged in one probe. **The spike's prose
recommends `allow`; the spike's own shipped code uses `expect`. The design must
choose deliberately rather than copy whichever it reads first.**

Also ✓: `[lints]` applies to `build.rs`. `slint_build::compile(...).unwrap()` —
the form `docs/memory/slint-build-mechanics.md` records — fails the gate on
`unwrap_used`. The build script must return `Result` and use `?`.

### Mechanics that cost a phase if discovered late

- **One `compile*` call, one root `.slint` file.** ✓ `include_modules!()` expands
  to `include!(env!("SLINT_INCLUDE_GENERATED"))` — one file — and each
  `compile_with_config` overwrites the variable. Two calls silently include only
  the second; the first file's components vanish with no error at that step.
  Multiple files go through one root that `import`s and re-exports them.
- **Ban the inline `slint::slint!{}` macro.** ✓ `with_debug_info` does not reach
  it; it reads `SLINT_EMIT_DEBUG_INFO=1` at proc-macro expansion time, and cargo
  does not track env changes for proc-macro expansion, so a stale artefact keeps
  failing until a file is touched. One mechanism, no env vars at test time.
- **`Window` already defines `title`** ✓, so `in-out property <string> title`
  fails in the build script with `Cannot override property 'title'`.
- **Handles are weak and fail loudly** ✓. Across `VecModel::remove(0)`, the
  removed row's handle goes `is_valid() == false` and every accessor returns
  `None`; a later `insert` does not revive it. It never silently re-points at a
  different row. Re-query after any mutation; a stale handle is a visible
  failure, not a wrong pass.
- **Default widget style is `fluent`** when unset
  (`i-slint-compiler-1.17.1/typeloader.rs:937`), overridable by `SLINT_STYLE`.
- **Debug info costs nothing shippable** ✓: release binary 30,881,096 bytes with,
  30,865,272 without — **+0.05%**. Leave it on unconditionally; a second build
  configuration would break any test running against a release build.

---

## Thread 4 — the Slint event loop and the tokio runtime

The integration `docs/memory/slint-build-mechanics.md` names as "unproven here —
it is slice 002's work" is now proven, end to end, against the real slice-001
`Host<ProcessBackend>` ✓.

**The shape that works** — call it the hybrid: a multi-threaded tokio `Runtime`
built in `main`, its `EnterGuard` held for the life of the Slint event loop, and
exactly **one** `slint::spawn_local` task that owns `Host<ProcessBackend>` and
consumes commands from a `tokio::sync::mpsc` channel. Slint callbacks only
enqueue. Verified ✓: nine exchanges in one run — a choice rendered, answered and
closed; non-zero exit, garbage stdout and two timeout classes each reported as
`Outcome.failure` with the host alive and re-invocable; the last exchange
succeeded and repainted. Identical results on live Wayland and headless with
`LD_LIBRARY_PATH` empty.

**The `EnterGuard` is safety-critical, not a convenience** ✓. Without it the
first poll of a `tokio::process` future on the Slint thread panics — `there is
no reactor running` (`tokio-1.53.1/src/process/unix/mod.rs:373`) — the panic
unwinds out of the event loop and the process exits **101**. That is CLAUDE.md
invariant 4 failing. A panic anywhere inside a `spawn_local` future kills the UI
the same way; a panic inside a `tokio::spawn` task awaited from `spawn_local` is
contained as `Err(JoinError::Panic(..))` and the UI keeps running ✓.

**A slow backend does not freeze the UI**, measured two ways ✓. A 50 ms repeating
`slint::Timer`: worst inter-tick gap **50.98 ms** headless across a run
containing a 1.0 s exchange and two ~2 s timeouts; 87.5 ms on Wayland, and that
one occurred at the first tick during GL init, not during any backend call.
Interactivity, not just repaint: button callbacks fired at t+1000.67 ms and
t+1599.98 ms — within 0.7 ms and 0.02 ms of schedule — **while** a 1.8 s exchange
was in flight, and the two enqueued answers were then correctly refused by host
state (`... is superseded; the outstanding interaction is ...#1`). Serialisation
of exchanges (I6) is preserved without blocking the UI.

**Shutdown is the sharp edge, and the naive version leaks a backend process** ✓.
After `run_event_loop` returns, nothing polls a `spawn_local` future again, so
`JoinHandle::abort()` cannot take effect and dropping the handle does not drop
the future — `kill_on_drop` never fires. Measured with a 30 s child in flight:

- **With** an `on_close_requested` → `Notify` → `KeepWindowShown` drain: window
  closed **17 ms** after the request (not after the 2 s backend timeout, because
  cancelling *drops* the future rather than waiting), child killed with the
  runtime still alive, and reaped. Exit 0, no survivors.
- **Without** it: exit 0, and `sleep 30` **survived the process**, reparented to
  pid 1440. `[task] ending` never printed. Reproduced headlessly too.

This is worse than SPEC-001's documented cancellation gap: AC-5 concedes disposal
is "attempted and not observed"; here nothing is attempted at all. The drain hook
is what turns it back into the documented case. The alternative — exchange in
`tokio::spawn`, `abort()` after the loop returns — also cancels, but splits the
`Host`'s `&mut self` exclusivity across a task boundary.

**Other verified mechanics:**

- The Slint platform is thread-local; the event-loop proxy is a process-global
  `OnceCell` ✓ (`i-slint-core-1.17.1/context.rs:51-54`, `platform.rs:218`).
  Component creation and `spawn_local` are bound to one thread;
  `invoke_from_event_loop` and `quit_event_loop` are callable from any.
  Touching Slint from a second thread is a **hard error**
  (`PlatformError::SetPlatformError(AlreadySet)`), not undefined behaviour ✓.
- On Linux winit sets `with_any_thread(true)`, so the loop *can* run off the main
  thread here ✓. macOS/iOS cannot. Treat "main thread" as the portable rule.
- `MainWindow::new()` with no display returns `Err(PlatformError::Other(...
  neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set))` ✓. There is no
  silent degradation and **no automatic headless fallback**. This is a first-class
  error path the entry point must report in the host's own voice.
- `run_event_loop()` **returns as soon as the last window is hidden** ✓ — measured,
  it exited immediately on `window().hide()`. A daemon-shaped host needs
  `run_event_loop_until_quit()`, verified running 1.2 s with zero windows and
  exiting cleanly on `quit_event_loop()`.
- goad's async lints accept the shape ✓: with `await_holding_lock`,
  `await_holding_refcell_ref`, `future_not_send`, `async_yields_async`,
  `large_futures` all `deny`, `cargo clippy --all-targets -- -D warnings` exits 0.
  Positive control: an `async fn` holding an `Rc` across an await produced
  `future cannot be sent between threads safely`, exit 101 — the lint is live and
  simply does not reach the inline `async {}` handed to `spawn_local`.
- `async-compat` is **not** needed and is not in this machine's registry cache ✓.
  Slint's docs recommend `Compat::new()` (`slint-1.17.1/lib.rs:301-354`); the
  persistent `EnterGuard` needs nothing beyond the tokio already in the manifest.
  `#[tokio::main]` + `block_in_place` is rejected: it builds a tokio loop on the
  main thread and then tells tokio not to use it.
- A tokio **current-thread** runtime entered on the Slint thread does not work ✓ —
  a 200 ms sleep never completed in 3 s, exactly as documented.
- New feature delta on the existing tokio entry: `rt-multi-thread` and `sync`,
  neither present today ✓ (`Cargo.toml:26`).
- The grandchild orphan is **canon, not a defect** ✓: a backend that spawns a
  grandchild inheriting the pipes costs the 2 s timeout plus the full 500 ms
  cleanup budget and reports `cleanup=Some(...)`; `exec`ing costs exactly 2.001 s
  with `cleanup=None`. `docs/slices/001/design.md:1437` explicitly refuses
  process-group killing per brief §14. Slice 002 must not "fix" this.

---

## Thread 5 — canonical types, content, and window behaviour

### The data model imposes no obstacle; the type system does

A `.slint` `struct` becomes a plain Rust struct with `#[derive(Default,
PartialEq, Debug, Clone)]`, kebab-case → snake_case, `string` → `SharedString`,
`float` → `f32`, and — load-bearing — **an array-typed member becomes
`ModelRc<T>`** ✓. Fields are emitted alphabetically, so construct by name;
`Default` is derived, so a later-added column is purely additive.

The ownership idiom that compiles and works ✓: Rust holds
`Rc<VecModel<OptionRow>>` for the process lifetime, calls `set_options(ModelRc::
from(Rc::clone(&options)))` **once**, and thereafter `set_vec`s wholesale (the
view-replacement path) or mutates incrementally. Both propagate to the repeater —
asserted by counting live elements, not by reading the model back.
`clone_on_ref_ptr` forces `Rc::clone(&options)`, never `options.clone()` ✓.

**Option-scoped fields are not dead-ended** ✓, which was the question worth
asking. A nested `for` over a nested model renders per-option fields, and — the
non-obvious part — assignment to nested model data *from `.slint`* writes back
through the `ModelRc` into the Rust-owned `VecModel` (`row_data(0).text_value ==
"20"` after driving a `LineEdit`). Verified independently a second time inside a
repeated **custom** element, covering R-15/R-16.

The two v0 choices that **would** dead-end it, both avoidable at zero cost ✓:
parallel scalar arrays (`in property <[string]> option_labels` — a nested model
cannot be attached to a `[string]` without restructuring every binding), and
making the callback payload the option **label** rather than the `OptionId`.
`OptionId` and `AlternativeId` are deliberately separate namespaces
(`canonical.rs:55-96`). A row struct carrying `id` + `label` from v0 makes adding
`fields: [FieldRow]` a purely additive `.slint` edit.

**Slint's type system has no map, no payload-carrying sum type, no `f64`, no
`Option`** ✓ (`i-slint-compiler-1.17.1/langtype.rs:19-75`; `.slint` enums
generate C-style Rust enums). So `Hints`, `FieldKind::Number(NumberRange)`,
`Choice{alternatives}`, `Option<Content>` and `Option<f64>` must flatten into
declared columns by a hand-written mapper. There is no dictionary lookup and no
fold in the `.slint` expression language, so hint *interpretation* must live in
Rust. None of it reaches the wire, so the mapper is the correct place to absorb
it — provided the design says so, because the tempting alternative is to shrink
the protocol's vocabulary to what the schema declares.

**`float` is f32 and SPEC-001's bounds are f64, and goad's own gate refuses the
bridge** ✓: `range.min().unwrap_or_default() as f32` fails on
`cast_possible_truncation` and `as_conversions`. The form that passes carries the
bound as rendered text. Beyond presentation, this is R-9's territory: answer
values must be held in Rust, never round-tripped through Slint numerics.

The mapper compiles against the real goad types and stays exhaustive by
construction ✓ — `match view { View::Choice(c) => ... }`, so a new `View` variant
is a compile error. None of the canonical inbound types derives `Serialize`, so
it is necessarily hand-written.

### Content: markdown is in, HTML and URI are not

Slint 1.17.1 ships a built-in **`StyledText`** element and a first-class
`styled-text` property type, so `in property <styled-text> body` yields
`set_body(slint::StyledText)` ✓ (`i-slint-compiler-1.17.1/builtins.slint:689-786`;
`generator/rust.rs:101`). `slint::StyledText::from_markdown(&str) -> Result<_,
StyledTextFromMarkdownError>` is **public stable API** ✓
(`i-slint-core-1.17.1/styled_text.rs:43-63`, re-exported at
`slint-1.17.1/lib.rs:235-237`). **Zero extra dependencies** — the parser is
pulldown-cmark 0.13.4 already inside the Slint tree ✓ (412 unique crates for
slint + slint-build, consistent with slice 001's 411). Brief §11.1's condition
("Markdown if Slint integration is straightforward") is satisfied.

Rendering confirmed visually ✓: `**2h**` bold and `[docs](...)` as a blue
underlined link, from a snapshot taken *after* a hide/show cycle.

**The catch: unsupported markdown is an `Err`, not a degradation.** ✓ Measured
over a 32-case corpus, and independently reproduced. OK: plain, bold, italic,
strikethrough, inline code, inline links, autolinks, bare URLs, ul, ol, two
paragraphs, `<u>`, `<font color>`, escapes, empty. **ERR**: `# heading`
("Markdown headings are not supported"), setext heading, `> blockquote`,
`![image]`, `---`, fenced code block, `<b>`, `<a href>`, and every HTML block.
Tables and footnotes parse OK **as literal pipe text** (only
`ENABLE_STRIKETHROUGH` is set, `i-slint-common-1.17.1/styled_text.rs:320-323`).

So a backend sending a perfectly legal `{"kind":"markdown","value":"# Take a
break"}` makes the renderer's markdown path fail. **There is currently nowhere to
report it**: `Discarded` has exactly one variant, `Schedule`
(`src/semantics/protocol/normalize.rs:52-57`) ✓. Whether v0 falls back to
`from_plain_text` and reports, or refuses the view, is a decision the design must
take — and refusing is exactly the "narrow the protocol to what the renderer
draws" failure CLAUDE.md invariant 3 names.

Two further content facts ✓:

- **`Content::Text` and `Content::Markdown` need different handling** even though
  both end in a `StyledText`. The implicit string→styled-text coercion inside
  `.slint` calls `from_plain_text`, **not** `from_markdown`
  (`styled_text.rs:185-195`). Right for Text; silently wrong for Markdown.
- **`from_markdown` did not panic on any hostile input tried** — 200 000
  characters, 5 000 paragraphs, 2 000-deep emphasis nesting, NUL, U+FFFD, an RTL
  override, all `Ok`. One obscure trap: any string containing **U+E541**, Slint's
  private-use interpolation placeholder, errors with "Argument index 0 out of
  range" (`i-slint-common-1.17.1/styled_text.rs:138`).

**HTML has no renderer and no WebView in Slint at all** ✓. `Content::Html` and
`Content::Uri` have no element. Brief §11.2 defers WebView; nothing here changes
that.

**Links do not open themselves, and the scheme policy is goad's** ✓.
`StyledText` fires `link-clicked(url)` with the URL verbatim
(`i-slint-core-1.17.1/items/text.rs:305-317`), and `from_markdown` accepts
`javascript:`, `file:///etc/passwd` and `about:blank --remote-debugging-port=9222`
without complaint. `Platform.open-url` routes to `webbrowser::open`, and Slint
depends on webbrowser 1.2.4 **without** its `hardened` feature, so no scheme is
filtered. The Rust-side `open_url` is not stable API — only
`slint::private_unstable_api::re_exports::open_url` ✓. `xdg-open` exists on this
machine from the NixOS system profile, **not** from the goad flake ✓
(`flake.nix:30-53, 98-107`). SPEC-001 R-19 says the host MUST NOT dereference a
`uri`; whether clicking a link inside a body counts is a design decision.

**Text capabilities are three disjoint sets** ✓: `Text` wraps and elides but has
no links and no inline styling; `StyledText` has links and inline runs but **no
`wrap` and no `overflow`** properties (markdown text always wraps to fill width);
selectable, copyable text needs a read-only `TextInput`, which is plain-text
only. If a diagnostic state should let the user copy an error, that is a third
element.

### Window behaviour: Wayland refuses three things the design might assume

All verified against winit 0.30.13's Wayland backend ✓:

- **Placement.** `set_outer_position` is an empty body commented "Not possible on
  Wayland"; `outer_position()`/`inner_position()` return `NotSupportedError`.
  Measured: after `set_position(50,50)` the window reported (0,0)
  (`wayland/window/mod.rs:263-275`).
- **Always-on-top.** Slint's `always-on-top` maps to
  `WindowLevel::AlwaysOnTop`; winit's Wayland `set_window_level` is an empty
  no-op (`:430`). xdg-shell has no window-level concept, and wlr-layer-shell is
  not implemented in winit at all.
- **Focus stealing.** `focus_window` is an empty no-op (`:629`). Whether a newly
  mapped surface takes keyboard focus is entirely the compositor's decision. The
  only attention mechanism winit has is `request_user_attention` via
  xdg-activation-v1, and **Slint does not expose it**.

Also ✓: the Window `icon` property is silently dropped (the app icon comes from
the xdg app id, `slint::set_xdg_app_id`, which must be set before show); minimize
is one-way; and **the compositor dictates geometry** — under niri the spike
window was given 932×2120 physical pixels regardless of declared content.

**`hide()` destroys the window on Wayland** rather than unmapping it ✓ — Slint's
own comment is "Wayland doesn't support hiding a window, only destroying it
entirely"; it calls `suspend()`, dropping the winit window and its render
surface, and `show()` recreates it. It *is* reliable: 8 consecutive hide/show
cycles at 800 ms all succeeded, size preserved, content correct every re-show.
Budget for surface recreation per prompt, and keep the view model in Rust rather
than in Slint properties that die with the window.

**`SystemTrayIcon` is the only "unobtrusive but discoverable" affordance Slint
offers** ✓ — a top-level component with `icon`, `tooltip`, `title`, `visible`,
`clicked()` and one `Menu` child; StatusNotifierItem over D-Bus via ksni 0.3.6;
`system-tray` is a **default** feature, so it costs nothing over the 411 already
counted. It works on this machine (registered `org.kde.StatusNotifierItem-341410-1`,
listed by noctalia's watcher, gone on exit) ✓, and **a visible tray keeps
`run_event_loop()` alive with no window at all** — hiding it ended the loop in
0.3 ms. Two mechanical constraints if it is in scope ✓: it is a separate
top-level component that does **not** share globals with the Window (each gets
its own copy), and **`hide()` panics** ("Constant property being changed",
`i-slint-core-1.17.1/properties.rs:788`) unless `visible` carries a binding —
workaround verified. It also shows nothing until a non-empty icon image is
assigned.

Slint offers **no desktop-notification API and no urgency hint** ✓; ksni's
`NeedsAttention` is not exposed. The whole discoverable-diagnostic palette is:
tray icon image, tooltip, title, menu contents, `clicked()`, and the window
title.

---

## Thread 6 — ADR-002: does T1 fire, and what does the split cost?

**T1 fires — on two grounds, neither of which is the one ADR-002 gives.**

**Ground 1: Cargo forbids optional dev-dependencies.** ✓ Verbatim:
`dev-dependencies are not allowed to be optional: i-slint-backend-testing`.
Dev-dependencies are built for `cargo test` regardless of features and regardless
of whether the target using them is skipped by `required-features` — measured on
a goad-shaped probe with the UI test target gated OFF. Crate counts, reproduced
identically by two producers ✓:

| column | goad today | with the testing dev-dep |
|---|---|---|
| `cargo tree --no-default-features` | 16 | **223** |
| `--no-default-features --edges normal,build` | 16 | 16 |
| default | 31 | 548 |

And it is **compiled, not merely resolved** ✓: `cargo test --no-default-features`
from clean took 10.2 s wall / 57 s user and produced `libi_slint_backend_testing`,
`libi_slint_core`, `libi_slint_common`, `i_slint_core_macros` in
`target/debug/deps`. The one escape hatch nobody had tested is closed ✓:
`[target.'cfg(feature = "ui")'.dev-dependencies]` — cargo 1.99 accepts the
manifest but warns "this key is not supported... will not work as expected", and
the dep resolves in **neither** column.

**Nuance the design must state precisely** ✓: T1's literal wording is "must not
need in order to **build**", and `cargo build --no-default-features` is clean at
16 crates. It is `cargo test --no-default-features` that pulls 223 — and that is
the gate ADR-001 itself names (`docs/adr/001-one-way-strata.md`, Verification).
**Argue T1 from ADR-001's gate, not from the word "build".**

**Ground 2, and the sharper one: the lint collision (Thread 3).** A single-crate
goad must carry a twelve-lint blanket suppression over generated code *inside the
same crate whose lint discipline is the point* — and it works only via a hole in
`clippy::allow_attributes` that any module can use.

**ADR-002's stated reason is refuted** ✓, three times independently: an optional
build-dependency plus `#[cfg(feature)]` in `build.rs` gates as cleanly as the
tokio runtime gate.

**T3 is borderline, not decisive** ✓. Clean build of all test binaries 25.6–26.6 s
wall / ~3m20s user. A `cargo clippy --all-targets -- -D warnings` immediately
after a fully warm build still re-checks the whole Slint tree: **9.1 s wall / 62 s
user** — clippy keeps its own artifact cache. `just check` (`justfile:17`) runs
build, test, `test --no-default-features`, deno check, **two** clippy columns
(`:57-59`) and fmt, so from clean the Slint tree is compiled or checked in four
to five separate cache columns. Baseline for comparison: slice 001's audit
recorded the integration tier at 1.46 s wall, "nowhere near firing".

### The split, dry-run: a relocation, not a redesign

Executed in a worktree ✓. **111 files renamed; 91 of them (77 protocol fixtures +
14 fake-backend shell scripts) with zero content change.** Of the 20 renamed
files with content changes, 19 changed only import paths, path-string constants,
or a `mod` list. **One file changed substantively** — `boundary.rs`. 4 files
added, 1 deleted (`src/lib.rs`, whose 12 lines were entirely the feature-gated
module tree), 3 modified in place. `just check` exits 0. Clean gate 11.1 s
before → 6.9 s after. **~6 minutes from the first `git mv` to a green gate.**

**No production file needed a change beyond its import block.** No signature
changed; no type moved between strata.

**The cost ADR-002 flagged was zero** ✓. It predicted "splitting the error
taxonomy" as a negative consequence. It cost **two lines** — the taxonomy was
already split by stratum, and the stratum-2 file already wrapped the stratum-1
one across a named seam. Both files moved intact. This is the clearest available
evidence that ADR-001 was being honoured rather than merely asserted.

**What the split exposes** ✓, and it is the finding ADR-002 asked for:
`tests/protocol/transport_shape.rs` was in the **stratum-1** test target with a
**stratum-2 source file** as its subject, and `boundary.rs` was in the same target
scanning all of `src/`. Both are upward reaches the single crate hid. Neither can
stay where it was; both re-home into a new `shape` target in the stratum-2 crate.
`boundary.rs` has no correct home in a two-member workspace and must move again
when stratum 3 arrives.

**The one place the split is weaker than advertised** ✓: adding
`tokio.workspace = true` to the stratum-1 manifest leaves `cargo build
--workspace` at exit 0. The fact moved out of the source and into the manifest,
so `boundary.rs`'s `tokio` grep is now the wrong instrument. Two of its three
forbidden tokens become compile errors (verified by negative control:
`error[E0433]: cannot find module or crate goad_shell` / `tokio`); the third needs
~84 lines of new test reading `[dependencies]` out of stratum 1's manifest. The
`NO_DOMAIN_VOCABULARY` scan is **not** redundant — no compiler objects to a type
called `Habit` — and needs one `Scan` per member, with a named residual risk:
nothing forces a *new* member to get one.

**The gate goes from seven commands to five** ✓, because the split retires the
matrix §9 exists to cover:

```
cargo build --workspace
cargo test --workspace
deno check examples/typescript/backend.ts
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

There is no `shell` feature any more — tokio and toml become unconditional
dependencies of the stratum-2 crate and absent from stratum 1, so
`--no-default-features` stops being a distinct column. Its successor is
`cargo test -p goad-semantics` (27 tests, 2.6 s from an empty `target/`), kept
out of the gate as a diagnostic. The second clippy column and its
`-A dead_code -A unreachable_pub` carve-out go with it — and that carve-out was
already inert on today's code ✓ (the pre-split `--no-default-features` clippy
column is clean without it).

**Recommendations for the four things ADR-002 left open** (all measured):
`goad-semantics` / `goad-shell` under `crates/`, leaving the name `goad`
unclaimed for the stratum-3 application crate; `[workspace.dependencies]` with
members writing `serde.workspace = true`, so *which member inherits which entry*
becomes the stratum statement; the 137-line lint table moves once into
`[workspace.lints]`; `clippy.toml` stays as one copy at the workspace root
(positive control ✓: `too-many-arguments-threshold = 1` appended there made both
members error, proving members read it); fixture corpus at `tests/fixtures/` in
the workspace root, addressed as `"../../tests/fixtures/…"` — three string
constants, versus zero if it stays under the semantics crate, but the root keeps
a stratum-3 renderer out of another crate's test directory.

---

## Cross-thread findings

**1. The question this research existed to settle is answered, and it shrinks the
slice.** A Slint UI is driven and asserted headlessly in an ordinary `cargo test`
with a live compositor present and zero sockets opened. Slice 002 does **not**
need a test-strategy investigation phase, does not need `SLINT_BACKEND=headless`
(gated behind `slint/mcp`, which drags in prost/protox/pbjson/image and did not
resolve here), does not need `slint_interpreter` or `slint-viewer`, and does not
need the `renderer-software` snapshot tier. A test is ~15 lines at ~0.2 s per
binary. AC-5 ("a simple choice rendered correctly") and AC-13 ("do not crash the
GUI") both get real observables. **Size the slice on the renderer, not on the
harness.**

**2. What that costs instead is a decision about goad's lint posture, and it must
be taken in design.** Slice 002 cannot compile with goad's `[lints]` until a
quarantine module wraps `include_modules!()`. The wrapper is ~15 lines and works
only because `clippy::allow_attributes` does not fire on inner module attributes
— a standing hole, not a Slint-specific concession. The alternative is the
ADR-002 split, where the renderer crate owns `ui/`, `build.rs`, the Slint
dependency, the testing dev-dependency **and** a laxer lint set, so stratum 1
never sees any of it. This is the same decision as T1, arriving from a second
direction.

**3. Three producers agree T1 fires and all three refute ADR-002's reason for
it.** The design must record the trigger check ADR-002's Verification requires,
argue T1 from ADR-001's *test* gate rather than the word "build", and correct the
build-dependency clause. ADR-002 is superseded either way; a fired trigger with
no split needs its own ADR.

**4. The split is cheap and the code is the small half of it.** Six minutes,
111 renames, 91 byte-identical, one substantive file change. The expensive part
is documentary: ~60 lines of `design.md` §9 prose become false, and a superseding
ADR is owed. **Do the split *before* adding Slint, not alongside it** — every
measurement above was taken on a tree with no renderer, and folding a 411-crate
build-dependency into the same diff puts the renderer's problems and the split's
problems in one change with no way to tell them apart.

**5. Contradiction, unresolved between producers: how much tier-2 testing slice
002 needs.** The headless spike concludes "probably NO tier-2 targets at all",
because `mock_single_click` works in the cheap tier ✓. A verifier accepts that
for pointers and refutes the generalisation ✓: `slint::spawn_local` and
`slint::invoke_from_event_loop` both return `Err(NoEventLoopProvider)` under
`init_no_event_loop()`, and goad's `Host::evaluate`/`respond` are `async`. So the
conclusion holds **only if the design bridges async→UI with `block_on`** — which
a verifier proved works, awaiting a real `tokio::process::Command` and then
reading the element tree inside the same `block_on`, because a tokio
current-thread runtime and the testing platform share a thread. If the design
instead uses `spawn_local` (the idiomatic choice, and the one Thread 4's verified
architecture uses at runtime), every wiring test needs
`init_integration_test_*`, which is once-per-process, which is **one `[[test]]`
target per test**. Thread 4 verified exactly that tier working with
`init_integration_test_with_system_time()` at 0.15 s. **The design decides which
tier its wiring tests use, and the answer changes the Cargo.toml.**

**6. Contradiction, unresolved: which suppression form to use.** The headless
spike's prose recommends `#![allow(...)]`; its own shipped code uses
`#![expect(...)]`. Both exit 0 today ✓. `expect` self-cleans and turns a Slint
patch bump into a build failure; `allow` is stable and silently drifts. Neither
is future-proof, and the twelve-lint list is empirical against one `.slint` file
— a bigger UI may emit more (one verifier's larger UI emitted the same twelve;
another's produced a higher total of the same twelve).

**7. The house rule for element selection was wrong, and the protocol is why.**
Select on `accessible_description` carrying the `OptionId`, not on
`accessible_label`, and not on component-type-plus-label. SPEC-001 R-14 permits
two options to share a label ✓. This is a small fact with a large shape: **the
testing API's affordances and SPEC-001's guarantees do not line up, and where
they differ the spec wins.**

**8. Three absence-shaped hazards compound, and the empty-state test sits at
their intersection.** Missing debug info (T-A), a stale `match_type_name` after a
`.slint` rename (T-F), and viewport virtualisation (T-G) each make "N options are
showing" pass against a broken UI, and only the first is caught by the debug-info
guard. This is `docs/memory/a-bound-is-not-tested-at-the-bound.md` in a new
costume: name the two implementations the assertion must separate, and assert
presence, not only absence.

**9. The diagnostic state has no vocabulary and no report channel.** Brief §13
asks for "unobtrusive but discoverable"; Slint's entire palette is tray icon,
tooltip, title, menu, `clicked()`, and the window title — no notification, no
urgency hint ✓. Meanwhile the renderer must bound two arbitrary values on every
exchange (256 KiB of possibly-non-UTF-8 stderr; an unbounded verbatim `raw`), and
`Discarded` has one variant, so a rejected markdown body has nowhere to be
reported. Retention, bounding, and the report channel are three separate design
questions the F-42/F-47 inheritance opens at once.

**10. An intervention that must appear in front of the user cannot be built from a
plain Slint window on Wayland.** Placement, always-on-top and focus-stealing are
all verified no-ops ✓. What remains: mapping a new window (the compositor
decides), a tray icon, and — outside goad — compositor window rules keyed on the
xdg app id. That argues for calling `slint::set_xdg_app_id` early and documenting
a recommended niri rule, which pushes the policy to where it actually lives. It
also makes `run_event_loop_until_quit()` load-bearing rather than merely correct,
since `run_event_loop()` exits the moment the prompt window is hidden — which is
goad's steady state.

**11. ADR-001's direction discipline held under the split, and slice 002 is where
the bet is cashed.** `design.md` assumption A2 said that if the discipline
slipped, the split would be a redesign rather than a file move, "which ADR-002
says would itself be the finding". It did not slip in production code. It **did**
slip in the test layout, in exactly two places, and the split is what found them.

---

## Design-input deltas

Each is a consequence the design must account for, not a summary.

1. **Record the ADR-002 trigger check, and correct ADR-002's reason while doing
   it.** T1 fires on the un-gateable dev-dependency (16→223 crates in ADR-001's
   own gate column) and on the lint collision. It does *not* fire for the reason
   ADR-002 states. Either outcome supersedes ADR-002.
2. **If the split is taken, take it first, as its own change, before Slint lands.**
   Recommended shape, all measured: `crates/goad-semantics` + `crates/goad-shell`,
   name `goad` reserved for stratum 3, `[workspace.dependencies]`,
   `[workspace.lints]`, one root `clippy.toml`, fixtures at `tests/fixtures/`,
   a five-command gate, and an ~84-line manifest-reading replacement for
   `boundary.rs`'s retired `tokio` scan.
3. **`build.rs` must return `Result` and must pass
   `CompilerConfiguration::new().with_debug_info(true)`.** The gate rejects
   `.unwrap()`/`.expect()` in build scripts, and without debug info the entire
   query API returns empty. Debug info stays on in release (+0.05%).
4. **The design owes a guard test, not just a rule.** Assert a known element *is*
   found, so a `build.rs` edit fails loudly rather than turning the suite into
   vacuous passes. Write the empty-state test as presence-plus-absence.
5. **Decide the async→UI bridge for tests, in design, because it sizes
   Cargo.toml.** `block_on` keeps everything in one cheap `[[test]]` target;
   `spawn_local` needs `init_integration_test_*`, one target per test.
6. **Choose `allow` or `expect` for the generated-code quarantine, with the
   reason written down**, and record that the wrapper works via a hole in
   `clippy::allow_attributes` that is not Slint-specific.
7. **The row struct is the real data-model decision and must be a struct from
   v0**: `OptionRow { id, label }`, id being the `OptionId`, never the label,
   never conflated with `AlternativeId`. Parallel `[string]` arrays dead-end
   option-scoped fields; a struct row makes `fields: [FieldRow]` additive.
8. **Rust owns everything**: the `Rc<VecModel>` for the process lifetime,
   `set_options` once, `set_vec` per view; the `ViewId` paired with the view; every
   answer value; every interpretation of `Hints`. No value a response submits may
   be reconstructed from a Slint property (f32 vs f64, R-9).
9. **Write the mapper as one exhaustive `match` over `View` and `FieldKind`**, so
   a variant added to SPEC-001 is a compile error rather than a blank window. Do
   not add a `_ =>` arm. State the rule that the mapper is lossy toward the
   renderer and never toward the protocol or the response.
10. **Decide what a `Content::Markdown` value that `from_markdown` rejects does.**
    Headings, images, block quotes, code blocks and HTML are all legal SPEC-001
    and all `Err`. Lossy-render-and-report vs refuse-the-view is CLAUDE.md
    invariant 3's territory, and `Discarded` currently has nowhere to carry the
    report. Also: `Content::Text` must go through `from_plain_text` and
    `Content::Markdown` through `from_markdown`, because the implicit `.slint`
    coercion is `from_plain_text`.
11. **Decide whether the choice view scrolls before writing the first count
    assertion.** If it does, `find_all()` counts the viewport, not the model; the
    assertions need `accessible-item-count` or a driven `scroll()`.
12. **Select on `accessible_description` (the `OptionId`).** Two options may
    legally share a label.
13. **Testability is a `.slint` obligation.** Each interactive element declares
    `accessible-role`, `-label`, `-description`, `-action-default`, plus
    `-checked`/`-item-index`/`-item-count` where the tests assert state — and a
    `FocusScope` per option if keyboard operation is in scope. A bare `Rectangle`
    is invisible to every query. This is the accessibility story for free, not a
    tax.
14. **The runtime architecture is settled, and three parts of it must be stated
    explicitly**: where the `EnterGuard` is created and that it outlives the loop
    (without it, exit 101 on the first backend call); that `MainWindow::new()`
    returns `Err` with no display and is reported in the host's voice; and that
    the close path drains before the loop dies (without it, a backend child
    outlives the process, silently, with exit 0). Add `rt-multi-thread` and
    `sync` to the tokio features.
15. **Use `run_event_loop_until_quit()`, not `ComponentHandle::run()`**, if the
    empty state is a hidden window — `run_event_loop()` returns the moment the
    last window hides.
16. **Assert structure and semantics, not geometry or pixels.** Determinism needs
    the literal family `"FixedTestFont"`, which a production UI should not carry,
    and `configure_test_fonts()` is unreachable.
17. **Never write a test that asserts a failure to construct.** It is coupled to
    the absence of a display and breaks `just test` on a developer machine.
18. **Bound both arbitrary values at the glass, and test the bound rather than
    the outcome.** 256 KiB of possibly-non-UTF-8 stderr with a `truncated` flag
    already set, and an unbounded verbatim `raw` with newlines intact. Do not walk
    `source()` chains naively — `ConfigError::Duration` and friends render their
    fault twice. Keep the F-42/F-47 rule: each fact rendered exactly once.
19. **Extend `boundary.rs` to `.slint`, or state in writing that the renderer's
    strings are unguarded.** Extend the configuration, not the walk. `goal` and
    `reminder` are the realistic UI trips.
20. **Three one-line phase-sheet rules**, each worth a debugging round: init at
    the top of every `#[test]` fn (never a `Once`); never `.match_descendants()`
    after `from_root()`; one `slint_build::compile*` call and one root `.slint`
    that imports the rest. Plus: ban the inline `slint::slint!{}` macro, and
    remember `Window` already owns `title`.
21. **Build a small test-helper module before the first assertion**, mirroring
    `tests/integration/harness.rs`: tests return `Result<(), Box<dyn Error>>`,
    `?` on `Component::new()`, and an `only()` helper turning
    `Vec<ElementHandle>` into a `Result` with a count in the error. Zero
    `.unwrap()` is the existing house standard.

---

## What remains unknown

Honest gaps. Each names how it would be settled.

- **Whether the crate splits.** The mechanical question is answered from three
  directions; the judgement is the user's, and `design-log.md` 2026-09-04 rules
  it a canon event that stops and waits.
- **Whether the headless tier survives a fonts-less environment.** With an empty
  fontconfig every test panics inside `fontique` at construction ✓. The face
  resolved here comes from the host system profile, not the flake. Settled by a
  flake decision (add a font package) or by measuring in a container — **and the
  flake is not this research's to change.**
- **Whether goad's full lint table accepts the renderer.** Only the five async
  lints were proven against the runtime shape ✓, and only the twelve-lint
  generated-code list against three `.slint` files. `pedantic`,
  `needless_pass_by_value`, `shadow_unrelated` and ~75 others are unproven
  against hand-written renderer code. Settled by running the §9 clippy columns on
  the first renderer commit, before the design commits to the shape.
- **Whether `just check` stays tolerable.** Nobody ran it in the goad tree — a
  build writes `target/` and the tree was to stay clean. The spike's own gate was
  36 s wall / 4m28s user; clippy re-checks the Slint tree in a separate cache
  column. Settled by one run in a worktree after the dependency lands. This is
  T3's evidence and it is currently borderline.
- **Why `take_snapshot()` returned `Ok` with an all-zero buffer under femtovg in
  one mode and `Err("not supported")` in another.** Unchased. Under
  `SLINT_BACKEND=winit-software` and under the testing backend's software
  renderer it returns correct pixels ✓. Settled by pinning the software renderer
  if pixel assertions are ever wanted — or by not wanting them.
- **Whether `SLINT_BACKEND=headless` works here.** Unresolved: `slint/mcp` would
  not resolve because this sandbox holds a stale sparse-index entry for
  `pbjson-build` (dated 2025-08-29) while crates.io publishes 0.9.0. Almost
  certainly a sandbox artefact. Low priority — the in-test software renderer is
  verified and cheaper.
- **Whether `slint_build::CompilerConfiguration::with_debug_info` is safe to
  depend on**, being `#[doc(hidden)]`. The env-var fallback is *worse*, not
  safer (it needs the var on every build and has the proc-macro rebuild-tracking
  problem). Settled by asking upstream.
- **Whether the ksni tray survives its StatusNotifierWatcher restarting.** A
  long-lived intervention shell will outlive its host bar. Settled by restarting
  noctalia while a tray-bearing binary runs.
- **Whether other Slint constant-folding traps of the `SystemTrayIcon::hide()`
  shape exist** — properties whose default binding makes a Rust setter panic. One
  was found by hitting it; no audit was done.
- **Whether niri places a newly mapped goad window where the user will see it,
  and whether it takes focus.** Only the *inability to force either* was
  verified. Settled by mapping a window while another app has focus, and by
  reading niri's per-app-id window rules.
- **How very long unstructured text lays out in a `StyledText`** with no `wrap`
  and no `overflow` property, if the host falls back to `from_plain_text` on an
  `Err`. Settled by a rendering spike at a realistic width.
- **Non-Linux behaviour.** The non-main-thread latitude is winit's
  `with_any_thread(true)` on unix. macOS and Windows untested; if the project
  ever targets them, the main-thread rule becomes binding.
- **Whether `slint-build` tolerates being optional in practice.** Cargo's
  mechanism was proven ✓; Slint was never actually added behind that gate, and
  the split dry-run deliberately contained no Slint. Also untested inside a
  workspace member: whether `slint_build::compile("ui/main.slint")` resolves
  relative to the member (it uses `CARGO_MANIFEST_DIR`, so it should) and what
  411 crates do to workspace feature unification.

---

## Thread 7 — the `serve` loop, built and run (added at review round 3)

Added during round 3's repair of F-22, and it refuted an assumption two rounds
of review had left standing. A standalone crate reproducing the design's exact
loop shape — `Cancel` over `tokio::sync::watch::<bool>`, an `mpsc::Receiver`, the
`Pending` enum, an `Rc`-bearing glass presented across the loop, and both
`select!`s `biased` — compiled and run offline against tokio 1.

**The borrow holds** ✓. The `async` block's mutable borrow of `host` is released
by `break`, so `Served { ending, host, controller, glass }` after the loop
compiles. `select!` binding the exchange future in one arm while the other arm's
handler takes `&mut controller` is accepted: they are different locals.

**Cancellation drops the exchange** ✓. Tripping `Cancel` 1 ms into a 10 ms
exchange returns `Served { ending: Stopped, .. }` with nothing folded, in under
9 ms. The un-polled branch's future is dropped by `select!`.

**The level-held property holds** ✓. A `Cancel` tripped *before* `stopped()` is
first awaited still ends the loop on the first iteration, with zero backend
calls — the failure mode a bare `Notify` has.

**`busy` returns to `false`** ✓, with the clear in `absorb` (F-21).

### The lint result, which is the part that changed the design

Under `#![deny(clippy::all)]` + `#![deny(clippy::future_not_send)]`, matching
goad's table for these two:

| shape | result |
|---|---|
| `fn serve(..) -> impl Future`, future **is** `Send` | clean |
| `fn serve(..) -> impl Future`, future `!Send` | **2 errors** — `future_not_send` **and** `clippy::manual_async_fn` |
| `async fn serve(..) -> Served` + one `#[expect(clippy::future_not_send, reason = …)]` | clean, expectation **fulfilled** |

**The third row does not transfer to the design's signature, and Thread 11
measured that.** Every shape in this table is *concrete*: the spike's glass was
a concrete `Rc`-bearing type, and `clippy::future_not_send` drops `Send`
obligations that mention a type parameter at the top level. `serve` is generic
over `B: Backend` and `G: Glass`, so the lint does not fire on it at all, the
`#[expect]` this row justified is *unfulfilled*, and
`unfulfilled_lint_expectations` fails the gate. Rows 1 and 2 stand — they are
why `serve` is an `async fn` — and row 3's attribute is deleted (Thread 11,
`design.md` §5.5 A-5).

Two consequences:

1. **`future_not_send` does reach a `fn` returning an `async` block.** A-5
   assumed it did not. The earlier reading looked settled because the future
   under test was `Send` and the lint had nothing to fire on — the same
   vacuous-assertion shape `docs/memory/a-bound-is-not-tested-at-the-bound.md`
   describes, in a lint costume.
2. **The dodge costs a second denied lint.** `clippy::manual_async_fn` is in
   `clippy::all`, which goad denies, and it fires on a `fn` returning a single
   `async` block. So the shape chosen to avoid one deny-level lint trips two.

`Cancel::stopped()` keeps `-> impl Future<Output = ()> + use<>` and trips
nothing: `manual_async_fn` fires only when the body *is* a single `async` block,
and `stopped` clones its receiver first.

### One deadlock, found by hitting it

The first spike harness pre-loaded two commands into a capacity-1 channel with
`send().await` before running the loop, and hung on the second send. Production
never does this — `main` sends once and callbacks use `try_send` — but it pins a
requirement the design had only implied: **`Wire::send` must be `try_send`,
never `send().await`.** A Slint callback is synchronous and on the UI thread; an
awaiting send against a loop that is not reading would block the thread the loop
needs. `Full` is only reportable because the send does not wait.

---

## Thread 8 — §5.2's markup, compiled (added at review round 3)

Added to discharge A-7 rather than carry it, after F-27 showed what carrying a
cheap assumption costs. §5.2's markup block was extracted **verbatim** from
`design.md` into a crate with `slint = "=1.17.1"`, `slint-build = "=1.17.1"` and
a `build.rs` calling
`compile_with_config(.., CompilerConfiguration::new().with_debug_info(true))`.

**It compiles** ✓, and the check is not vacuous ✓: replacing `accessible-role:
list` with a nonsense role fails the *build script* with
`error: Unknown unqualified identifier` at the exact `.slint` line, plus
`The accessible-role property must be a constant expression`. Restoring it
builds clean.

**The generated API is what §5.3 and §5.4 assume** ✓ — `set_mode`,
`set_heading`, `set_body`, `set_options`, `set_body_degraded`, `set_busy`,
`set_notice`, `set_diagnostic_lines`, `on_chosen`, `on_close_diagnostics`, and
the tray's `on_check_now`, `on_show_diagnostics`, `on_quit`.

**A-6 is discharged** ✓: `title: root.mode == WindowMode.prompt ? "goad" :
"goad — diagnostics"` compiles. The fallback of moving the two literals into
`diagnostics.rs` is not needed.

### What it found — the tray had no way to be written (F-28)

The first version of the block *set* the tray's inherited `icon`, `tooltip` and
`visible` rather than declaring them, on the reasoning that a redeclaration is
an error. Both halves of that reasoning are right and the conclusion was wrong:

| what the markup does | result |
|---|---|
| `in property <image> icon;` on a `SystemTrayIcon`-inheriting component | `error: Cannot override property 'icon'` (same for `tooltip`, `visible`) |
| `visible: true;` — setting the inherited property to a literal | compiles, and the generated code calls `set_constant()` on `icon`, `title`, `tooltip` **and** `visible`; **no `set_icon`/`set_tooltip` accessor is generated at all** |
| declare `image` / `hover-text` / `shown`, then bind `icon: root.image;` etc. | compiles; generates `set_image`, `set_hover_text`, `set_shown`; **zero** `set_constant()` on `visible` |

Two consequences, neither reachable by reading `builtins.slint`:

1. **Inheriting a builtin property does not expose it to Rust.** A
   `SystemTrayIcon`-rooted component's `icon` and `tooltip` are settable from
   markup only. Anything the host must write at runtime needs a declared
   property of its own with the builtin bound to it.
2. **E-4's constant-folding trap is wider than recorded.** `visible: true` is
   *not* "carrying a binding" — a literal folds. Only a binding to something
   that can change leaves the property live. The earlier note that the tray
   "declares the binding anyway" would have shipped the panic it was written to
   avoid.

The corrected block is what `design.md` §5.2 now carries; it was re-extracted
from the design after the edit and rebuilt, and generates the eleven setters
above with `visible` unfolded.

---

## Thread 9 — F-9's failure-matrix schema, instantiated (added at review round 4)

Built to settle the two passages the round-3 handover named as *unbuilt and
unreviewed*, on the lesson round 3 drew about itself: an assumption a scratch
crate can reach should be reached before a phase starts. A crate carrying
`Cargo.toml`'s `[lints.rust]` and `[lints.clippy]` blocks **verbatim** (lines
53-218) plus the repo's `rustfmt.toml`, with the real `Display` impls vendored
from `src/semantics/error.rs`, `src/shell/error.rs`, `src/shell/host.rs`,
`src/semantics/protocol/normalize.rs` and `src/semantics/protocol/canonical.rs`,
and all 33 rows of §9 item 12.3 instantiated against them.

**Final state green** ✓: `cargo clippy --all-targets -- -D warnings` exits 0,
`cargo fmt --check` clean, 8/8 tests pass.

### The design's own strings, compiled against the code

Two defects, both in one row pair, and both of the kind reading cannot find ✓:

| row | design said | the code writes |
|---|---|---|
| T3 | `stderr: that answer is not to be trusted` | `stderr: that answer is not to be trusted\n` |
| P2 | `stderr: config is missing` | `stderr: config is missing, so this is all you get\n` |

`tests/backends/exits-zero-with-unparseable-stdout.sh:5` and the `@garbage` arm
of `answers-as-instructed.sh` both write `config is missing, so this is all you
get` ✓ — the design's string is not even a prefix the row could have matched
under `Exact`. And `echo … >&2` appends U+000A, which §5.4's escape step renders
as the two characters `\n` ✓, so an `Expect::Exact` on either design string
fails.

**All 15 protocol lines, T1–T4, the three cleanup lines and all six discard
lines were checked and are correct** ✓, including `invalid bounds: min 10 is
above max 1` (`f64` `10.0` prints as `10`), `backend wrote more than 8388608
bytes to stdout`, `backend did not respond within 500ms` and `backend was not
disposed of within 500ms`. The `at` paths in P6, P13 and P14 were checked
against their named fixtures; the six D-row raw values against
`tests/protocol/fixtures/schedule/`; P5's key against
`protocol-text/R-44-a-duplicate-key-on-the-envelope.json`.

### Five rows carry a stderr line the table did not mention

`hangs-past-the-timeout.sh:2`, `floods-stdout-past-the-cap.sh`,
`leaves-a-grandchild-holding-stderr.sh` and
`leaves-a-grandchild-holding-stdout-too.sh` each open with `echo "$$" >&2` ✓ —
R-41 bookkeeping. `process.rs:88-145` shows `seen` is a `&mut Captured` the
drain borrows ✓, so a drain that times out still yields what it captured. T2,
T4, C1, C2 and C3 therefore each produce `stderr: <pid>`, whose text no table
can pin. §12.3's "those three are the whole of `Prefixed`" was false, and
`Expect::Unpinned` is the third form.

### The lint table, twice, on code the design implies

The escape step cannot be a `String` accumulator ✓. Both accumulating spellings
are denied, and each is the other's suggested repair:

```
error: `format!(..)` appended to existing `String`
   = note: `-D clippy::format-push-string` implied by `-D clippy::pedantic`
error: non-binding `let` on an expression with `#[must_use]` type
   = note: requested on the command line with `-D clippy::let-underscore-must-use`
```

### Two rules for the test target itself

- `clippy::tests_outside_test_module` fires on every `#[test]` at the root of a
  `tests/…` target ✓ — nine diagnostics from one file. Fixed by
  `tests/table/main.rs` + `#[cfg(test)] mod f9;`, which is what
  `tests/integration/main.rs:6-24` already does and says why.
- `clippy::unnecessary_wraps` refuses a `#[test]` returning `Result` with no `?`
  in it ✓ — six diagnostics. Six of the eight tests had to return `()`.

### Schema corrections the instantiation forced

Fourteen, listed in the ledger's F-9 round-4 entry and carried into `design.md`
§9 item 12. The load-bearing ones: the fold is `Controller::absorb`, not
`receive` (an `Outcome` is not `Clone`, `host.rs:70-96`, so only one of the two
can run) ✓; `observed` is the **complete** per-channel line list rather than a
containment list, which is the only way "no failure" and "exactly one discard"
can be stated; `Channel` gains `Protocol`, because
`no action taken: backend response rejected: ` is `BackendError::Protocol`'s own
prefix and is not derivable from the channel T1–T4, S1 and S2 also use; `Case`
gains `id` and `schedule`; `Presentation` derives `Debug` alone, so assertion
3's "leaves the previous `Prepared` identical" is not expressible and is
restated on `ViewId`, which derives `PartialEq` (`canonical.rs:42`) ✓.

### Not executed

The scratch crate does not run a `Host`, so S1's `Schedule::Seed` was reasoned
rather than run. It is now corroborated by reading:
`schedule::resolve(Some(retained), None, poll, now)` returns the retained
instant while it is ahead of `now` (`schedule.rs:221-237`) ✓, `DEFAULT_POLL` is
30 minutes (`harness.rs:220`) ✓, and slice 001's `seeded_check()` is
`2026-08-23T04:42:00Z` (`failure_matrix.rs:44-46`) ✓ — the same value from the
same construction instant.

---

## Thread 10 — F-26's startup surface, compiled (added at review round 4)

562 lines across `src/{main,startup,diagnostics,clock,stubs}.rs` in a crate
whose `Cargo.toml` carries the `[lints.rust]` and `[lints.clippy]` blocks
byte-for-byte from `Cargo.toml:53-218`, plus the project's `rustfmt.toml` and
one dependency, `jiff 0.2 default-features = false`, matching the real manifest.
rustc/clippy 1.99.0-beta.3.

**Final state** ✓: `cargo clippy --offline --all-targets -- -D warnings` exit 0;
18 tests pass, none using `.unwrap()` or `.expect()`; `cargo fmt --check` clean.

### The outlet survives, and one of its three stated grounds was false

All nine candidate spellings were compiled simultaneously in one probe module
(independent lints, so each reports its own failure) ✓:

| spelling | lint |
|---|---|
| `eprintln!` / `println!` | `clippy::print_stderr` / `clippy::print_stdout` |
| `let _ = writeln!(..);` | `clippy::let_underscore_must_use` |
| `let _: io::Result<()> = writeln!(..);` | the same — the annotation does not help |
| `writeln!(..);` | `unused_must_use`, via `unused` |
| `writeln!(..).ok();` | **none** |
| `drop(writeln!(..));` | **none** |
| `match writeln!(..) { Ok(()) \| Err(_) => () }` | **none** — the design's choice |

`Result::ok` is not `#[must_use]` and `Option` is not a must-use type, so
nothing fires on `.ok();` ✓. The design claimed it trips `unused_must_use`; it
does not. Two of three grounds hold, the third is false, and the rejection of
`.ok()` is now an explicitness argument — otherwise the next reader shortens the
line believing the table forbade it, and is right.

Also measured: `use std::io::Write` must **not** be imported ✓. The
`impl std::io::Write` bound supplies `write_fmt`, and the import trips
`unused_imports`.

### `unreachable_pub` is gate-fatal for every `pub` item in a renderer module

Nine errors on the first compile — one per `pub` item §5.4 declares ✓:

```
error: unreachable `pub` item
  --> src/diagnostics.rs:24:1
   | help: consider restricting its visibility: `pub(crate)`
   = note: requested on the command line with `-W unreachable-pub`
```

The boundary was measured, because it decides the fix: a `pub` item at the
**crate root** of a bin does not fire, and does not trip `dead_code` either;
the lint fires only for `pub` inside a **private** module ✓.

This is a **class** defect, not an instance: it reaches every `pub` the design
writes anywhere under `crates/goad/src/` — §5.3's `Wire`, `SlintGlass`,
`Reported`, `Discarded`, §5.4's `tray_icon.rs` consts, all of it. It is confined
to `crates/goad`; `crates/goad-semantics` is a lib and unaffected.

*The prescription this thread drew from it — `pub(crate)`, not a lib target —
was not adopted, and the reason is recorded in `design-log.md` 2026-09-05,
"Round 4: the renderer crate is a library plus a thin binary". The scratch crate
had no `tests/` target, so it could choose private modules freely; `design.md`
§9 runs four of its validation items in `tests/…` targets of `crates/goad`,
which `pub(crate)` puts out of reach. The measurement stands; the crate shape it
implied does not.*

### Three more, each a code change

- `.map_err(|_| StartupError::Enqueue)?` is `clippy::map_err_ignore` ✓, whose
  own documented escape is a named-underscore binding. `wall_clock`'s
  `SystemTimeError` needs the same.
- `arguments` had **no callable call site**: §5.4's `run()` passed one argument
  to a two-argument signature ✓. The call site that compiles is
  `arguments(std::env::args_os(), &|name| std::env::var_os(name))?`, and the
  closure is load-bearing — `var_os` is generic over `K: AsRef<OsStr>`, so
  `&std::env::var_os` does not coerce to `&dyn Fn(&str) -> Option<OsString>` ✓
  (`redundant_closure` does not fire).
- **Nobody skipped `argv[0]`.** `std::env::args_os()` yields the program name
  first ✓, so every row of the four-row table was off by one. The skip now lives
  inside `arguments`, where the table's tests cover it.

### Confirmed rather than assumed

- `let _entered = …` and `let _task = …` survive
  `clippy::no_effect_underscore_binding` because their initialisers are calls ✓.
  The lint does fire on `let _x = some_place;`, so the shape matters and the
  design has the right one.
- `StartupError::source()` and `ClockError::source()` are `None` under the
  default `Error` impl ✓, and `ConfigError`'s own real `source()` chain
  (`src/shell/error.rs:177-186`) does not leak through.
- `missing_errors_doc` does not fire on `pub(crate)` items ✓ (public-API only) —
  which, under the lib shape actually adopted, means it **does** fire on all of
  them.
- `dead_code = "warn"` is gate-fatal via `-D warnings` ✓, so every one of the
  eight `StartupError` variants needs a construction site in the phase that
  lands the enum. This bit twice during the exercise.

### Not measured

`slint::PlatformError`'s and `slint::EventLoopError`'s `Display` text — slint is
not in the scratch crate, and the design carries those verbatim by policy, so
only the wrapper sentences were pinned. `ConfigError` was stood in for rather
than linked. `Path::is_absolute` was exercised on Linux only; goad is
Wayland-only, so that is a note, not a gap.

---

## Thread 11 — the design's own text under the real lint table (added at review round 4)

A-2 said the workspace lint table was unproven against ~75 lints and would be
settled "on the first renderer commit". This thread settles everything a scratch
crate can reach without Slint, which is most of it.

Two trees, both preserved. The first holds the design's own text for `install`,
`Wire`, `Cancel`, `Controller`, `Diagnostics`, the mapper, `receive`,
`arguments`, the two outlets, `wall_clock`, `serve` and the tray rasteriser,
transcribed **as written**, with `[lints.rust]` and `[lints.clippy]`
`diff`-confirmed byte-identical to the goad tree, plus goad's own `clippy.toml`
and `rustfmt.toml`.

```
$ cargo clippy --all-targets -- -D warnings
error: could not compile `a2-instances` (lib) due to 13 previous errors
```

**Thirteen errors across nine lints** ✓, and the rasteriser eight more:

| n | lint | where |
|---|---|---|
| 2 | `clippy::map_err_ignore` | §5.4's step-6 `Enqueue`, and `wall_clock` |
| 2 | `clippy::missing_errors_doc` | `arguments`, `Controller::answer` |
| 2 | `clippy::new_without_default` | `Controller::new`, `Cancel::new` |
| 2 | `clippy::shadow_unrelated` | `for undrawn in undrawn` inside `Diagnostics::of` — **not** `install()` |
| 1 | `clippy::needless_pass_by_value` | `Diagnostics::of(reported: Reported, …)` |
| 1 | `clippy::match_same_arms` | `Wire::send`'s `Ok(())` and `Err(Closed(_))` arms |
| 1 | `clippy::format_push_string` | the escape step |
| 1 | `unused_imports` | consequence of the above |
| 1 | `unfulfilled_lint_expectations` | the `#[expect(clippy::future_not_send)]` on `serve` |
| 8 | `clippy::integer_division`, `clippy::arithmetic_side_effects` | the rasteriser's sample grid; `alpha = u8::try_from(covered * 255 / 16)…` is two denied lints in one expression |

The second tree is the same code with the nine repairs applied: clippy clean,
`cargo fmt --check` clean, 6 tests pass, **zero `#[expect]` on renderer code** ✓.
The centre-pixel property §9 item 16 turns on survives the saturating rewrite —
asserted by test, not argued ✓.

### `serve` and `future_not_send`, with controls

The measurement that matters most, because it reverses round 3's:

| shape | `future_not_send` |
|---|---|
| `serve<B: Backend, G: Glass>` — the design's signature | does **not** fire |
| `serve_concrete(.., glass: RcGlass)` — non-generic, concrete `Rc` | **fires** |
| `serve_generic_with_concrete_rc<G>(..)` — generic + one concrete `Rc` local | **fires** |

Clippy drops `Send` obligations that mention a type parameter at the top level ✓,
so `B` and `G` being unbound is not enough on its own; the two controls prove the
negative is not vacuous. Everything else `serve` holds across an await is `Send`,
`slint::StyledText` included: it is `SharedVector`-backed and
`unsafe impl<T: Send + Sync> Send for SharedVector<T>`
(`i-slint-core-1.17.1/sharedvector.rs:97`) ✓, so `Controller` does not make the
future `!Send` either.

**Consequence:** the `#[expect]` F-27's repair added to `serve` is *unfulfilled*,
and `unfulfilled_lint_expectations` is an error under the gate's `-D warnings`.
The attribute would have failed the gate for the opposite reason. Thread 7's
third row measured a **concrete** shape and does not transfer.

### Two things the design had written as conditionals

- **The rejected rebinding shape is clean.** `let wire = wire.clone();` passes
  goad's table ✓; under `-W clippy::shadow_reuse` it is reported, and that is a
  restriction lint this table does not enable. It is never
  `shadow_unrelated` — that lint fires on a **loop** binding reusing the
  collection's name. The six distinct clone bindings in `install` stay on
  readability grounds; `design.md`'s stated lint reason was wrong.
- **`Wire`'s `Debug` is not conditional.** `slint::Weak<T>` implements no
  `Debug` in 1.17.1 — no derive, and no impl anywhere in `i-slint-core-1.17.1`
  or `slint-1.17.1` ✓ — so the hand-written impl is required.

### Corroboration

Thread 10, built independently in the same session, reached the same repair for
the outlet (`|_negative|`, `|_returned|` in place of `|_|`) and left `line_to`'s
`match writeln!(sink, "{line}") { Ok(()) | Err(_) => () }` unchanged. The two
runs agree.

### Still unproven, and it is only what needs Slint

`SlintGlass::present`, the `include_modules!()` quarantine and A-1's twelve-lint
list, the `slint::spawn_local` call site, `slint::Image` /
`SharedPixelBuffer` / `Rgba8Pixel`, and `build.rs`. The Slint API stand-ins here
carry the real signatures (`FnMut` callback setters, `on_close_requested`,
`as_weak`/`clone_strong`, no `Debug` on `Weak`) but not the real types. The
rasteriser was measured over a `Vec<u8>` rather than a
`SharedPixelBuffer<Rgba8Pixel>`; the arithmetic — where every lint fired — is the
design's own, the buffer type is not. `serve`'s future holds a real
`Host<ProcessBackend>` future in production and a stub here, which cannot change
the `future_not_send` result (a generic `B` is filtered out either way) but has
not been compiled. **A-4 is untouched** and still needs the real tree.
