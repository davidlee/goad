# `nix build` from inside this checkout reads the **git tree**, not the working directory

Met three times in slice 006 — PHASE-02, PHASE-05, and once in `~/flakes`.

## The fact

This repository's flake is referenced in the bare/git form (a `path:` reference
is impossible here — the root holds a live unix socket nix refuses to copy). A
git input reads the git tree, which means:

- an **untracked** file is invisible to the build, and
- a **tracked-but-modified** file is not.

So `nix/module.nix` had to be `git add`ed before it would build at all, and a
dirty working tree changes what a consumer elsewhere installs.

## The second half, which costs more

`self.shortRev or self.dirtyShortRev or ""` means a dirty tree **stamps
`<rev>-dirty` rather than failing**. That is the designed-in witness and it
works — but only for someone reading the version line. A consuming flake records
it as `dirtyRev` in its lock, where nobody looks. In slice 006 an uncommitted
`nix flake update` in this repository silently redefined what a `home-switch`
one directory away was about to install.

## How to apply

- `git add` a new file before building. A build that does not fail is not
  evidence the file was seen.
- Commit before letting a consumer lock this repository, and check the version
  line for `-dirty` when a store path looks unfamiliar.
- `~/flakes` is **not its own git repository** — its root is `/home/david`, so
  the same rule applies there one directory up.

Related: `path-flake-ref-breaks-on-demo-socket.md`.
