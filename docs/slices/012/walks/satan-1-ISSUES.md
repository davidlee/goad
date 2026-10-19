# W-1 — SATAN's handback

SATAN's own report at the end of the router walk, as pasted by the person
(2026-10-19). Verbatim.

````text
Built and checked. `router/` is ready for the swap.

## What it is

```
router/backend.py     routing, the floor, next_check merging
router/children.toml  the table — adding a child is a config edit
router/config.toml    ready to be ~/.config/goad/config.toml
router/state/         which child's view is outstanding; when each is due
```

**Policy.** Host evaluates go to the children in table order; the first with a view shows it. An unanswered view keeps its child's floor, so a second child never replaces a question mid-answer; when that child returns null the floor is released and the table is consulted again. A respond returns to the child whose view it answers — the host mints `view_id` and gives the backend nothing back, so the router remembers `pending` in its state file. `next_check` is resolved per child (instant or span) and the earliest is sent; a child that names no instant isn't waited for. A child that exits non-zero, times out, prints nonsense or omits `view` is skipped, its stderr forwarded prefixed, and its next check dropped so the next poll retries.

**Table.** `checklist` first (it carries SATAN's ask queue, whose entries expire), `unwatched` second (weekly, tolerant of a few seconds — the router re-asks it the moment the checklist's view closes). Order is the policy; the file is re-read every exchange.

**The queue stayed with the checklist.** Its schema, option-id convention and filing rules are specified in that backend and SATAN's reader depends on them; a second implementation in the router would be two homes for one contract. The router's job is routing.

**Poke.**

```sh
goad-emit --socket /run/user/1000/satan/goad.sock --source satan --kind unwatched
goad-emit --socket … --source satan --kind checklist --data '{"why":"the review is due"}'
```

`--kind` is the child's name; `--data` reaches it untouched; the child still decides (unwatched treats it as "Check now" and keeps its one-hour quiet window). Exit 0 accepted, 1 refused (`too_soon` inside 3 s, `engaged`), 2 no host listening. After the swap the `--socket` flag is unnecessary — `goad-emit` reads the default config's ingress path.

## Verified

| | tests | `goad-check` |
|---|---|---|
| router | 32 ok | accepted |
| unwatched | 30 ok | accepted |
| checklist | 77 ok | accepted |

All with scratch state (`$ROUTER_STATE`, `$UNWATCHED_STATE`, `$GOAD_DATA`, `$GOAD_SATAN_QUEUE`); the real `unwatched/state/` still holds your three answers untouched, and no `router/state/` was created. The check exercised the poke path with `events/unwatched.json` and an unknown child kind. A broken child was also driven live: `/nonexistent/backend` first in the table, `unwatched` still answered, one stderr line.

## Swap

Stop the goad you're running first — a second instance loses the ingress socket and exits 2. (Your 11:44–11:45 answers are in `data/2026-10-19.json`, so one is live on the checklist.)

```sh
mkdir -p ~/.config/goad
ln -sfn ~/satan-corpus/goad/router/config.toml ~/.config/goad/config.toml
just run          # in ~/satan-corpus/goad/router
```

## Friction

- The goad-backend skill at the path I'm given is byte-identical to the first session's — same nix store hash, same files. If an update was meant to reach me, it isn't in that mount.
- `goad-check` still can't set `now`, so the wager's 20 h and the sitting's 7 d never run under it: `accepted` means "nothing refused", not "behaviour checked".
- It still answers only the first option, with defaults — every decline path stays untested.
- No state redirection: "scratch state" is each backend's private env convention, so a router must know all of its children's. `goad-check --env K=V`, or a protocol-level `$GOAD_STATE`, would make it a property of the protocol.
- The backend never learns the `view_id` its view was given, so a router can only remember *which child* it last showed. That is correct solely because R-31 keeps one view outstanding; relax that and routers break silently.
- Any `next_check` merger must reimplement the span grammar (units, no months, `1d` = 24 h). Two implementations of one grammar now exist, in the host and here.
- Events have no routing field: `kind` doubles as the child name, so a child's identity is copied into every emitter and a rename is two edits. A `target` key would be additive.
- `goad-emit` has no `--config`; without `--socket` it reads only the default config path, which is how an event quietly lands in a different instance's socket.
- goad refuses unknown config keys, so the routing table must be a second file — "the config" is now three files, and `goad-check --config` sees one.
- The chain bound of 8 is hit routinely once a router fronts a many-item child. It's a note, not a refusal, and the next request continues where the check stopped — but a single host request can never walk a long child.
- Carried from before: state under `$HOME` is invisible to a jailed consumer, and the reference recommends XDG dirs — exactly wrong when the reader is the sandboxed one.
````
