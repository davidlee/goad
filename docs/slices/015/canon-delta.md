# Canon delta — Slice 015

One entry per affected document (`docs/AGENTS.md` §Canon that does not exist
yet, or must change): the document, the section, the change **as it will be
stated**, and why. Nothing here is applied mid-slice. It is applied at audit,
with the person's explicit endorsement, and recorded in `audit.md`'s
Reconciliation table.

**Status:** wording endorsed by the person before design review
(`design-log.md` 2026-10-19, *canon wording*). Review may still change it;
any change returns to the person. Amended after code review round 1 — R-60's
and R-17's join, and R-60's §7 row — in the person's words (`design-log.md`
2026-10-19, *code review round 1 dispositions*; `review-code.md` F-1, F-3).

**Promotion order: after slice 012, by preference, not as a gate.** R-60
skips R-59, which 012's unpromoted delta holds. If 015 closes first, its audit
promotes this delta anyway — a slice does not close holding a draft — and
SPEC-001 carries R-60 with no R-59 until 012's audit fills it (`design-log.md`
2026-10-19, *design review round 1 dispositions*, F-10).

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

> | R-60 | **A backend runs in its configuration's directory.** A command loaded from a configuration file MUST be started with that file's directory as its working directory, the file's path first resolved through every symbolic link in it, so the directory is the target's and never a link's. The host MUST NOT rewrite any argument: each is passed as written, whether or not it looks like a path. The program is the one element the host locates. A program that is a relative path — it contains a `/` and does not begin with one — MUST be executed as that path joined onto the same directory, each `.` component dropped and each `..` kept as written, by the host's own resolution and not by whatever a platform does with a relative program under a changed working directory, which is not uniform; the joined path is the name the program receives, a script and a native program alike. A program with no `/` is looked up on `PATH`. A program that cannot be started is R-44's *command not spawnable*; for a command loaded from a configuration file, its report MUST name the path the host tried to execute and the working directory it tried to execute it in. | §7 |

**Why.** R-36 says what the command is and nothing about where it runs, so a
relative path meant the directory the host was started in — a different
directory for a service, a launcher, or a config reached through a link.
Resolving arguments would mean guessing which are paths, which the
permissive-parsing invariant forbids; the working directory fixes the common
case (`["python3", "backend.py"]`) without a guess (`design-log.md`
2026-10-19, OQ-1). Links resolve to the target because the live config is a
link (OQ-2). The program clause exists because Rust's `Command::current_dir`
leaves a relative program platform-specific (`research.md` T2-S1). The
program receives the joined path, not its name as written, because a script
cannot receive anything else: its interpreter is handed the path `exec` was
given, whatever the first argument says (`review-design.md` F-1). The report's
paths are required because the place a program is looked for may now be a
directory the person never typed (F-11).

### Change 2 — §6.4 *Process transport*: one paragraph, after the first

> **Where the backend runs** (R-60). A configuration file and its backend can
> move together as one directory. The backend's working directory is the
> file's own, through any links, so `["python3", "backend.py"]` names the
> `backend.py` beside the file however the host was started. No argument is
> reinterpreted to get there — the host cannot tell an argument that is a path
> from one that only resembles one — so the working directory is the
> mechanism. The program is the exception because executing it is the host's
> own act: a relative one with a `/` is executed by its path beside the file
> and receives that path as its name. A script's interpreter is handed the
> path it was executed by, whatever the host passes as the first argument, so
> no other name could be promised to a script and a native program alike. A
> bare name is looked up on `PATH` after the change of directory, so a
> relative entry in `PATH` names a directory under the backend's working
> directory; that is the environment's choice. A failure to start a
> configured command names both the path and the directory, because neither
> need be one the person typed.

**Why.** §6.4 is the transport's prose; R-60 needs its reason stated where the
transport is described, as R-41's budgets are.

### Change 3 — §7 *Verification*: R-60's row, placed after R-36's

> | R-60 | integration, built-binary and unit. Every case that drives a real backend loads its configuration and does so from a working directory that is **not** the configuration's. Unless a full path is given, `crates/goad-shell/tests/integration/working_directory.rs`. Red under the spawn R-60 replaces: `::a_relative_argument_resolves_beside_the_config_from_another_directory` — the working directory, which the backend reports with `pwd -P`, and the argument as written, which it reports as its `$0`; `::a_linked_config_runs_its_backend_beside_the_target` — through a link to the file, with a decoy backend beside the link that answers differently; `::a_relative_program_with_a_slash_is_executed_as_the_anchored_path` — the script's `$0` is the joined path, which is also what tells the host's own resolution from the platform's: under a changed working directory the platform executes the same file, and the script's `$0` stays relative; `::a_native_program_receives_its_anchored_path_as_its_name` — a copy of `sh` beside the configuration, run as `./shell -c …`, reports the joined path as its argv[0], so a native program is held as well as a script and passing the name as written would turn it red; `::a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for` — the report names the joined path and the directory. A pin, green under the old spawn and red under an implementation that looks beside the configuration for a bare name, on the built-binary tier so that `PATH` is set on one child process and the inherited environment cannot decide it: `crates/goad-check/tests/binary/run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`, which runs `goad-check --config` with `PATH` naming one directory holding a decoy, a second decoy beside the configuration, and asserts that only the one on `PATH` ran. A link in a **directory** component of the configuration's path leaves the kernel's working directory the same either way — a directory reached through a link is the target directory — so it is held on the loaded value, from which `PWD`, a relative program's name and the socket path are all taken: `crates/goad-shell/tests/integration/working_directory.rs::a_config_reached_through_a_linked_directory_is_anchored_at_the_target` asserts the command's working directory is the target's. Unit, the resolution as a pure function: `crates/goad-shell/src/config.rs::tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash`. Through a built binary: `crates/goad-check/tests/kit/examples.rs::each_shipped_example_is_accepted_by_the_checker` starts the checker outside each kit example's directory, and the checker spawns as the host does |

**Why.** Every row names its tests (§7's convention). The row says which cases
are red under today's spawn and which is a pin, and why the directory-link
half is held on a value rather than a process (`review-design.md` F-2, F-3,
F-4, F-5).

---

## SPEC-003 (host event ingress)

No id is renumbered. One is added: **R-17**, the next free id.

### Change 1 — §4 *Requirements*: new R-17, placed after R-16

> | R-17 | A relative socket path MUST be resolved against the directory of the configuration file that names it, found as SPEC-001/R-60 finds it — through every symbolic link in the file's path — and never against the directory the host was started in. It is joined onto that directory as SPEC-001/R-60 joins a relative program: each `.` component dropped, each `..` kept. The resolved path is the path R-1 to R-5 speak of, and the one each of their startup failures names. A resolved path too long for a socket address is a failure to bind, and R-4's. | §7 |

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

> | R-17 | renderer, integration and unit. `listener::a_relative_configured_path_binds_beside_the_config` (`crates/goad/tests/renderer/startup.rs`) loads a configuration naming `./goad.sock` with `Config::load`, from a working directory that is not the configuration's, and asserts a socket at the configuration's canonical directory and none in the working directory. The resolution itself, byte for byte so that no `./` survives into a length-limited address: `config::tests::a_relative_path_resolves_against_the_directory_given` (`crates/goad-shell/src/config.rs`). The length clause: `ingress::a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path` (`crates/goad-shell/tests/integration/ingress.rs`) — one byte too long is R-4's failure to bind, naming the path, and one byte shorter binds in the same directory as its control. The platform's own wording of the refusal is not asserted |

### Change 4 — §9 *References*: SPEC-001's entry

Append to the SPEC-001 bullet:

> ; R-60, whose anchor directory R-17 resolves a relative socket path against.

**Why.** R-17 depends on R-60's definition of the directory; §9 lists every
requirement this document leans on.
