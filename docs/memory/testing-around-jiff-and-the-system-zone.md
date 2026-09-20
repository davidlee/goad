# Testing around jiff and the system zone

Measured in slice 009, PHASE-04.

## `jiff::Timestamp::MAX` is a whole day below `DateTime::MAX`

`Timestamp::MAX` is `9999-12-30T22:00:00.999999999Z`. So
`DateTime::to_zoned` refuses the top of the civil range in **every** zone,
`+14:00` included.

That makes `9999-12-31T23:59:59` a **zone-independent** witness for the fallible
conversion — which matters, because a function that reads the *system* zone
cannot be tested with a zone-dependent one.

The offset-dependence is real one notch lower: `9999-12-31T12:00:00` resolves at
`Pacific/Kiritimati` and is refused at every smaller offset. Do not build a case
on that one.

## Zone-parameterise the pure half before asserting anything about a zone

Neither behaviour worth asserting — DST disambiguation, and a local date
differing from UTC's — can be reached through the machine's own zone: a CI box
set to UTC has no DST and no disagreement, and `std::env::set_var` is in
`clippy.toml`'s `disallowed-methods`.

Split the pure function out — `composed_in(zone, …)`, `local_midnight(zone, …)`
— and the units say something true rather than something that happened to hold
on this machine. What is left over is one token per function, *that the zone is
the system's*, and review is the only thing that holds it. Say so in the design
rather than implying the unit covers it.

## An assertion about a machine property should be a predicate, not a value

A verification criterion asked for *an offset that is not `+00:00`*. The offset
was assertable here (`Australia/Melbourne`, `+10:00`) and asserting it would
have encoded this machine into the suite.

`!TimeZone::system().is_unknown()` separates exactly *the feature is on* from
*the feature is off* — featureless `jiff` falls back to `TimeZone::unknown()` —
and separates nothing else. A UTC box passes it and it still means something.

## Removing a feature is a louder instrument than adding one

No gate command rejects a feature switched **on** in a shared dependency; that
residue stands. But an injection that **deletes** two `jiff` features turns five
units red.

The asymmetry is worth knowing when arguing about a residue: the argument is
owed on the way in, and once the units exist the way out is checkable.

## Rounding, and a wire format that cannot express the difference

RFC 3339's `time-numoffset` is `("+" / "-") time-hour ":" time-minute` — it
cannot express a sub-minute offset at all, so rounding it is the only conforming
behaviour, not a lossy shortcut.

Related: `tokio-time-runs-under-slints-executor.md`,
`a-bound-is-not-tested-at-the-bound.md`.
