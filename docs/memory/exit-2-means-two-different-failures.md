# `goad` exit 2 means *never started* **and** *stopped running*, and the unit cannot tell them apart

Found at slice 006's audit, from the journal.

## The fact

`main` has one `match` over `run()`'s `Result` and maps every `StartupError` to
exit **2**. That includes `Platform`, and `start` ends:

```rust
slint::run_event_loop_until_quit().map_err(StartupError::Platform)?;
```

So a compositor that goes away under a host which has been running for ten hours
exits 2 **exactly as** a host that could not read its configuration does.

`nix/module.nix` carries `RestartPreventExitStatus=2`, argued from three
variants that genuinely do not succeed on a retry — a bad configuration, an
unreadable clock, a held ingress socket. `Platform` is the fourth case and it
*does* succeed on a retry. It is also the only exit-2 that has ever occurred on
the daily-driver machine.

## Why the enumeration was convincing and still wrong

Three variants were named and the conclusion drawn about ten. Every named one
supported it. This is the standing shape of
`verify-the-enumeration-not-the-conclusion.md`: a correct conclusion about the
members you listed, applied to a set you did not walk.

## How to apply

- Until the taxonomy is split (a standing follow-up from 006), treat exit 2 as
  **ambiguous**, and do not add a second consumer that keys on it.
- Any new `StartupError` variant inherits exit 2 and therefore inherits the
  restart directive. Ask, at the variant, whether it would succeed on a retry.
- The general rule for a supervisor: a directive keyed on an exit code inherits
  that code's **whole** taxonomy, including variants added after the directive
  was written.
