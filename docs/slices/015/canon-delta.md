# Canon delta — Slice 015

One entry per affected document (`docs/AGENTS.md` §Canon that does not exist
yet, or must change): the document, the section, the change **as it will be
stated**, and why. Nothing here is applied mid-slice. It is applied at audit,
with the person's explicit endorsement, and recorded in `audit.md`'s
Reconciliation table.

**Status:** wording endorsed by the person before design review
(`design-log.md` 2026-10-19, *canon wording*). Review may still change it;
any change returns to the person.

**Promote after slice 012.** R-60 skips R-59, which 012's unpromoted delta
holds. Promoting this delta first would leave SPEC-001 with a gap until 012's
audit fills it. 015's audit waits for 012's promotion, or says why not.

Test names are `design.md` §9's; at audit each is re-pointed to the symbol that
shipped. Nothing here cites a line number, and nothing here counts.

Checked, not affected: SPEC-002, SPEC-004, POL-001, ADR-001, ADR-003, ADR-004,
ADR-005 (`design.md` §10).

---

## SPEC-001 (the host/backend interaction protocol)

No id is renumbered. One is added: **R-60**. R-59 is the next id by the text
of SPEC-001 today, but slice 012's unpromoted delta claims it and code already
cites it; R-60 does not collide whichever slice promotes first.

### Change 1 — §4 *Process transport*: new R-60, placed after R-36

> | R-60 | **A backend runs in its configuration's directory.** A command loaded from a configuration file MUST be started with that file's directory as its working directory, the file's path first resolved through every symbolic link in it, so the directory is the target's and never a link's. The host MUST NOT rewrite any element of the argument vector: each argument is passed as written whether or not it looks like a path, and the program receives its own name as written. The program is the one element the host locates. A program that is a relative path — it contains a `/` and does not begin with one — MUST be executed from that same directory, by the host's own resolution and not by whatever a platform does with a relative program under a changed working directory, which is not uniform. A program with no `/` is looked up on `PATH`. A program that cannot be started is R-44's *command not spawnable*. | §7 |

**Why.** R-36 says what the command is and nothing about where it runs, so a
relative path meant the directory the host was started in — a different
directory for a service, a launcher, or a config reached through a link.
Resolving arguments would mean guessing which are paths, which the
permissive-parsing invariant forbids; the working directory fixes the common
case (`["python3", "backend.py"]`) without a guess (`design-log.md`
2026-10-19, OQ-1). Links resolve to the target because the live config is a
link (OQ-2). The program clause exists because Rust's `Command::current_dir`
leaves a relative program platform-specific (`research.md` T2-S1).

### Change 2 — §6.4 *Process transport*: one paragraph, after the first

> **Where the backend runs** (R-60). A configuration file and its backend can
> move together as one directory. The backend's working directory is the
> file's own, through any links, so `["python3", "backend.py"]` names the
> `backend.py` beside the file however the host was started. Nothing in the
> argument vector is reinterpreted to get there — the host cannot tell an
> argument that is a path from one that only resembles one — so the working
> directory is the mechanism. The program is the exception because executing
> it is the host's own act: a relative one with a `/` is found beside the
> file, and a bare name on `PATH`, as a shell would find it.

**Why.** §6.4 is the transport's prose; R-60 needs its reason stated where the
transport is described, as R-41's budgets are.

### Change 3 — §7 *Verification*: R-60's row, placed after R-36's

> | R-60 | integration, every case loading its configuration with `Config::load` and driving a real backend from a working directory that is **not** the configuration's, so the behaviour R-60 replaces fails each one. All `crates/goad-shell/tests/integration/working_directory.rs`: `::a_relative_argument_resolves_beside_the_config_from_another_directory` — the working directory, which the backend reports with `pwd -P`, and the argument as written, which it reports as its `$0`; `::a_linked_config_runs_its_backend_beside_the_target` — through a link to the file and through a link in a directory component, each with a decoy backend beside the link that answers differently; `::a_relative_program_with_a_slash_resolves_beside_the_config`; `::a_program_with_no_slash_is_never_looked_for_beside_the_config` — the `PATH` clause, against an executable of that name beside the configuration; `::the_backend_receives_its_program_as_written` — the program's own name, observed as argv[0] of a native program. Unit, the resolution as a pure function: `crates/goad-shell/src/config.rs::tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash`. Through a built binary: `crates/goad-check/tests/kit/examples.rs::each_shipped_example_is_accepted_by_the_checker` starts the checker outside each kit example's directory, and the checker spawns as the host does |

**Why.** Every row names its tests (§7's convention). Each integration case is
red under today's spawn, which is what makes it evidence rather than a
description.

---

## SPEC-003 (host event ingress)

No id is renumbered. One is added: **R-17**, the next free id.

### Change 1 — §4 *Requirements*: new R-17, placed after R-16

> | R-17 | A relative socket path MUST be resolved against the directory of the configuration file that names it, found as SPEC-001/R-60 finds it — through every symbolic link in the file's path — and never against the directory the host was started in. The resolved path is the path R-1 to R-5 speak of, and the one every startup failure names. A resolved path too long for a socket address is a failure to bind, and R-4's. | §7 |

**Why.** §6.1 says the path comes from the configuration and not what a
relative one means, so it meant the directory the host was started in. The
kit's examples already write `./goad.sock` beside the config, and anything
that reads the same file to find the socket agrees with the host only under
one rule (`design-log.md` 2026-10-19, OQ-3). The length clause is stated
because the absolute form can exceed a limit the relative form did not.

*Count exemption:* "R-1 to R-5" names a closed range of existing ids, which
cannot grow.

### Change 2 — §6.1 *Configuration and the socket*: the opening paragraph

Replace its first sentence, *The socket's path comes from the host's
configuration and has no default.*, with:

> The socket's path comes from the host's configuration and has no default. A
> relative path is relative to **the configuration file's directory**, through
> any links (R-17), and never to the directory the host was started in, so a
> configuration and its socket move together. A program that reads the same
> file to find the socket names the same one only by the same rule; any such
> program's command line is outside this document. **A socket address holds a
> bounded path** — 107 bytes on Linux — and a relative path in a deep
> directory can resolve past it. That is a failure to bind (R-4); the remedy
> is an absolute path somewhere short, such as `$XDG_RUNTIME_DIR`.

The rest of the paragraph (*The mode is owner-only…*) is unchanged.

**Why.** §6.1 is where the path is described; the limit is the one new way a
configuration that worked can stop working, so it is stated beside the rule
that causes it.

### Change 3 — §7 *Verification*: R-17's row, placed after R-16's

> | R-17 | renderer and unit. `listener::a_relative_configured_path_binds_beside_the_config` (`crates/goad/tests/renderer/startup.rs`) loads a configuration naming `./goad.sock` with `Config::load`, from a working directory that is not the configuration's, and asserts a socket at the configuration's canonical directory and none in the working directory. The resolution itself, byte for byte so that no `./` survives into a length-limited address: `config::tests::a_relative_path_resolves_against_the_directory_given` (`crates/goad-shell/src/config.rs`). The length clause: `ingress::a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path` (`crates/goad-shell/tests/integration/ingress.rs`), with a path one byte shorter binding in the same directory as its control |

### Change 4 — §9 *References*: SPEC-001's entry

Append to the SPEC-001 bullet:

> ; R-60, whose anchor directory R-17 resolves a relative socket path against.

**Why.** R-17 depends on R-60's definition of the directory; §9 lists every
requirement this document leans on.
