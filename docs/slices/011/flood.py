#!/usr/bin/env python3
"""flood.py — slice 011 AC-6/VH-1's flood.

Drives the host's ingress socket flat out with refused envelopes, so a
person can watch the window and tray stay responsive while it runs
(`plan.md` EX-6, PHASE-03's hand-over steps). This script is not part of
`just check`; nothing in the gate runs it.

Usage: flood.py SOCKET_PATH [WRITER_COUNT]

Each of WRITER_COUNT (default 4) threads loops: connect to SOCKET_PATH,
send one envelope carrying a key none of SPEC-003 §6.2's four admits,
numbered `flood-<n>` off one shared counter, read the reply, close. Once a
second the main thread prints the refusal count so far and the last key
sent. Ctrl-C prints the last key once more and exits 0.
"""

from __future__ import annotations

import json
import socket
import sys
import threading
import time

DEFAULT_WRITER_COUNT = 4


def envelope(n: int) -> str:
  """[`flood-<n>`]'s envelope: unknown key, refused before any other field
  is read (A-2)."""
  body = {
    "source": "flood",
    "kind": "flood",
    "timestamp": "2026-01-01T00:00:00Z",
    "data": {},
    f"flood-{n}": 0,
  }
  return json.dumps(body, separators=(",", ":"))


class Tally:
  """Shared state across writers: the next index to send, the count of
  replies read, and the last key sent — one lock, so "the last key" and
  "refusals so far" the main thread prints always agree with each other."""

  def __init__(self) -> None:
    self.lock = threading.Lock()
    self.next_index = 0
    self.refusals = 0
    self.last_key = None

  def claim(self) -> int:
    with self.lock:
      index = self.next_index
      self.next_index += 1
      return index

  def record(self, index: int) -> None:
    with self.lock:
      self.refusals += 1
      self.last_key = f"flood-{index}"

  def snapshot(self):
    with self.lock:
      return self.refusals, self.last_key


def write_one(path: str, index: int) -> None:
  """One connection: connect, send one envelope and a newline, shut the
  write half (the wire protocol's own framing, `ingress/client.rs`'s
  `send`), read the one reply line, close."""
  with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as sock:
    sock.connect(path)
    sock.sendall(envelope(index).encode("utf-8") + b"\n")
    sock.shutdown(socket.SHUT_WR)
    reply = b""
    while not reply.endswith(b"\n"):
      chunk = sock.recv(4096)
      if not chunk:
        break
      reply += chunk


def writer_loop(path: str, tally: Tally, stop: threading.Event) -> None:
  while not stop.is_set():
    index = tally.claim()
    write_one(path, index)
    tally.record(index)


def main(argv: list[str]) -> int:
  if len(argv) < 2 or len(argv) > 3:
    print(f"usage: {argv[0]} SOCKET_PATH [WRITER_COUNT]", file=sys.stderr)
    return 2
  path = argv[1]
  writer_count = int(argv[2]) if len(argv) == 3 else DEFAULT_WRITER_COUNT

  tally = Tally()
  stop = threading.Event()
  threads = [
    threading.Thread(target=writer_loop, args=(path, tally, stop), daemon=True)
    for _ in range(writer_count)
  ]
  for thread in threads:
    thread.start()

  try:
    while True:
      time.sleep(1)
      refusals, last_key = tally.snapshot()
      print(f"refusals so far: {refusals}, last key: {last_key}")
  except KeyboardInterrupt:
    _, last_key = tally.snapshot()
    print(f"last key: {last_key}")
    stop.set()
    return 0


if __name__ == "__main__":
  sys.exit(main(sys.argv))
