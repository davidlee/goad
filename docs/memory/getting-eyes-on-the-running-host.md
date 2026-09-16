# Getting eyes on the running host, from an agent session

Learned at slice 008, the first slice that worked visually against `just demo`
with a screenshot per change.

## The facts

**Launching it needs a pipe.** Running the host through the Bash tool exits
**144 with no output**, even with every stream redirected inside the launch
script. The redirection alone is not enough. Piping the invocation is:

```sh
./run.sh 2>&1 | tail
```

**Screenshot the window by id, not by geometry.** Under niri:

```sh
niri msg action screenshot-window --id "$id"
```

works wherever the window is. `grim -g` with computed geometry does **not**: a
tiled window reports no `tile_pos_in_workspace_view`, and the one capture taken
that way photographed a browser instead.

**Two surfaces cannot be reached this way at all.** The tray menu and the
diagnostic pane are opened by a person, not by the loop — the pane is reached
only through the tray, and `Command::OpenDiagnostics` is the only thing that
raises it, so a backend failure fills `diagnostics` without putting them on
screen. The diagnostic pane had never been looked at by any agent before slice
008's last session, and one look produced five defects, four of which were on
no list.

**To get something worth looking at into the pane**, a scratch config plus a
backend that exits non-zero after writing a ~220-character stderr line. The
demo's own backend never emits a line long enough to show a wrapping defect.

**To compare two treatments honestly, render them side by side and sample the
screenshot** — `ffmpeg` to read the actual pixel values — rather than judging by
eye. 008 chose a tonal ramp and a heading treatment this way, and in the first
case the ratio that had been asked for measured three levels out of 255 and read
as one flat sheet.

## How to apply

- Build the loop before the visual work, not during it: a restart script and a
  shot script in the scratchpad.
- **Anything the loop cannot reach is a surface nobody has seen.** Enumerate
  those explicitly and hand them to a person; do not let the screenshot cadence
  imply coverage.
- A human report *is* the evidence for those surfaces, and is the honest
  instrument rather than a fallback — see the zoom-survives-a-present case in
  `window-zoom-is-the-scale-factor-event.md`.

Related: `wayland-window-placement-is-the-compositors.md`,
`a-fixtures-size-is-not-the-products.md`,
`path-flake-ref-breaks-on-demo-socket.md`.
