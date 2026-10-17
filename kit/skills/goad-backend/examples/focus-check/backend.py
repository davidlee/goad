"""Focus check: a goad backend in Python's standard library.

It keeps a time-boxed focus. Asked ("Check now"), it shows the current focus
and offers to continue, switch or break; with no focus set it asks for one.
Once the block has ended, the next scheduled check asks the same question.

One process per exchange: one JSON request on stdin, one JSON response on
stdout, nothing else on stdout. Diagnostics go to stderr.

State is under $XDG_STATE_HOME/focus-check/ (default ~/.local/state):
  state.json  the current focus and when its block ends
  log.jsonl   one line per decision, appended

An untouched text field arrives as "" (SPEC-001/R-57, R-58), so "" is treated
as no answer: no focus is set and no progress note is logged.
"""
import json
import os
import re
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path

EXAMPLE = "focus-check"
NEW_BLOCK = timedelta(minutes=25)
CONTINUE_FOR = timedelta(minutes=10)

INSTANT = re.compile(
    r"^(\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d)(?:\.(\d+))?(Z|[+-]\d\d:\d\d)$"
)


def parse_instant(text):
    """An RFC 3339 instant, spelled as datetime.fromisoformat accepts it."""
    match = INSTANT.match(text)
    if match is None:
        raise ValueError(f"not an RFC 3339 instant: {text!r}")
    whole, fraction, offset = match.groups()
    fraction = (fraction or "").ljust(6, "0")[:6]
    offset = "+00:00" if offset == "Z" else offset
    return datetime.fromisoformat(f"{whole}.{fraction}{offset}")


def state_dir():
    base = os.environ.get("XDG_STATE_HOME") or str(Path.home() / ".local" / "state")
    return Path(base) / EXAMPLE


def load_state():
    try:
        state = json.loads((state_dir() / "state.json").read_text())
    except FileNotFoundError:
        return {}
    except (OSError, ValueError) as error:
        print(f"{EXAMPLE}: ignoring unreadable state: {error}", file=sys.stderr)
        return {}
    return state if isinstance(state, dict) else {}


def save_state(state):
    directory = state_dir()
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "state.json").write_text(json.dumps(state))


def log(now, **entry):
    directory = state_dir()
    directory.mkdir(parents=True, exist_ok=True)
    with (directory / "log.jsonl").open("a") as lines:
        lines.write(json.dumps({"at": now.isoformat(), **entry}) + "\n")


def block_end(state):
    """When the current block ends, or None with no focus or an unreadable end."""
    if not state.get("focus"):
        return None
    try:
        return parse_instant(state["until"])
    except (KeyError, TypeError, ValueError):
        return None


def start_block(state, focus, now):
    state.update(focus=focus, until=(now + NEW_BLOCK).isoformat(timespec="seconds"))


def next_check(state):
    end = block_end(state)
    return {"next_check": end.isoformat(timespec="seconds")} if end else {}


def ask_for_focus():
    return {
        "kind": "choice",
        "title": "What are you focusing on?",
        "options": [
            {
                "id": "set",
                "label": "Start a block",
                "fields": [{"id": "focus", "kind": "text", "label": "Focus"}],
            },
            {"id": "later", "label": "Not now"},
        ],
    }


def check_in(focus):
    return {
        "kind": "choice",
        "title": focus,
        "body": "Still on it?",
        "options": [
            {
                "id": "continue",
                "label": "Continue for 10 min",
                "fields": [{"id": "note", "kind": "text", "label": "Progress note"}],
            },
            {
                "id": "switch",
                "label": "Switch focus to",
                "fields": [{"id": "focus", "kind": "text", "label": "New focus"}],
            },
            {"id": "break", "label": "Take a quick break"},
        ],
    }


def evaluate(request, now):
    state = load_state()
    event = request["event"]
    if event["source"] != "host":
        # A forwarded event is its writer's word, and nothing here wants one.
        return {"view": None, **next_check(state)}
    if event["kind"] == "startup":
        return {"view": None, **next_check(state)}
    end = block_end(state)
    if event["kind"] == "requested":
        view = check_in(state["focus"]) if end else ask_for_focus()
        return {"view": view}
    # "scheduled", and any kind this backend does not know (SPEC-001/R-56).
    if end is not None and now >= end:
        return {"view": check_in(state["focus"])}
    return {"view": None, **next_check(state)}


def respond(request, now):
    state = load_state()
    answer = request["response"]
    values = answer.get("values") or {}
    option = answer["option"]
    focus = str(values.get("focus", "")).strip()
    if option in ("set", "switch") and focus:
        start_block(state, focus, now)
        log(now, decision=option, focus=focus)
    elif option == "continue" and block_end(state) is not None:
        note = str(values.get("note", "")).strip()
        state["until"] = (
            max(block_end(state), now) + CONTINUE_FOR
        ).isoformat(timespec="seconds")
        log(now, decision="continue", focus=state["focus"], **({"note": note} if note else {}))
    elif option == "break":
        log(now, decision="break", focus=state.get("focus"))
        state = {}
    save_state(state)
    return {"view": None, **next_check(state)}


def main():
    request = json.load(sys.stdin)
    now = parse_instant(request["now"]).astimezone(timezone.utc)
    if request["type"] == "respond":
        response = respond(request, now)
    else:
        response = evaluate(request, now)
    json.dump(response, sys.stdout)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
