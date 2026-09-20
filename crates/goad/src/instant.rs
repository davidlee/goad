//! `crates/goad/src/instant.rs` — the two impure reads the draft may not make
//! (design.md §5.1, §5.2).
//!
//! Three functions. Two of them read the world — the **system time zone**, in
//! `compose` and again in `today_local`, and the **wall clock**, in
//! `today_local` alone — and that is the whole of what this module does that
//! `draft.rs` cannot. `draft.rs` declares itself pure, so these live here
//! rather than there; a module whose reason to exist is holding the impurity
//! should not undercount where it performs it, which is why the count is
//! **three sites of two kinds** and not two of two.
//!
//! Nothing here is drawn or wired yet. PHASE-07 is the only consumer:
//! `install.rs`'s `datetime` arm composes what the two pickers handed back,
//! and `glass.rs` fills a `datetime` field's seed slots from `decompose` where
//! the field has been picked and from `today_local` where it has not.

use jiff::civil::DateTime;
use jiff::tz::{Offset, TimeZone};

use goad_semantics::protocol::canonical::Timestamp;
use goad_shell::clock::wall_clock;

use crate::generated::{Date, Time};

/// A completed pick, in the zone the person picked in — or `None`, in which
/// case nothing is recorded and the button still shows what it showed, so the
/// person can see that the pick did not take (design.md §5.2, §5.4).
///
/// **Checked at every step, and using none of jiff's convenience
/// constructors.** `civil::date` and `Date::at` *panic* when their arguments
/// are out of range; `Date::new` and `Time::new` return a `Result` for the
/// same inputs. Slint's `Date` and `Time` carry `int` fields, which is `i32`,
/// while jiff wants `(i16, i8, i8)` and `(i8, i8, i8, i32)` — so every
/// conversion is a `try_from` and never a cast, because a cast truncates
/// rather than refuses and that is the same defect as an `f32` channel
/// (§8 R7).
///
/// **The `None` surface is all four fallible steps** (§5.5 A-4), each named
/// because a step left off the list is a step that becomes an `unwrap`:
///
/// 1. the integer conversions into `Date`'s fields;
/// 2. the integer conversions into `Time`'s fields;
/// 3. `Date::new` and `Time::new`, which refuse a civil value that does not
///    exist — `2024-02-30`, `24:00`;
/// 4. `DateTime::to_zoned`, which refuses a civil datetime that does not fit
///    the timestamp range. That is **not** the same boundary as `Date::new`'s:
///    `Timestamp::MAX` is `9999-12-30T22:00:00.999999999Z`, a whole day below
///    `DateTime::MAX`, and whether a given civil datetime fits depends on the
///    offset it is resolved in — `9999-12-31T12:00:00` resolves at `+14:00`
///    and is refused at every smaller offset. Both measured (PHASE-04/VT-2).
///
/// What `to_zoned` does **not** refuse is an ambiguous or nonexistent civil
/// time. jiff resolves those under `Disambiguation::Compatible`: a DST fold
/// takes the earlier occurrence and a gap shifts forward, both **succeeding**
/// (§7 D13, D19). The button then shows the composed value, so a person sees
/// the shift rather than being deceived by it, and the instant reaches the
/// backend carrying the offset it was resolved in — **rounded to the nearest
/// minute, which for a sub-minute offset is not the offset it was resolved
/// in** (F-P1).
///
/// A tzdb zone's pre-standardisation LMT offset carries seconds:
/// `Australia/Melbourne` is `+09:39:52` before 1895-02-01.
/// `Timestamp::display_with_offset` computes the civil datetime at the *exact*
/// offset and then prints the offset rounded —
/// `print_timestamp_with_offset_buf` calls `offset.to_datetime(*timestamp)` and
/// then `print_offset_rounded_buf`, whose doc reads *"If the given offset has
/// non-zero seconds, then they are rounded to the nearest minute"*
/// (`jiff-0.2.35/src/fmt/temporal/printer.rs:310-318`, `:772-775`). Measured:
/// `Europe/Amsterdam` −30 s, `Europe/Paris` +21 s, `Australia/Melbourne` −8 s,
/// composing 1880-06-15T12:00:00 in each.
///
/// **`R-57` is not breached, and there is no repair available.** RFC 3339's
/// `time-numoffset` is `("+" / "-") time-hour ":" time-minute` — it cannot
/// express a sub-minute offset at all, so rounding is the only conforming
/// behaviour. Nothing a person sees disagrees either: the button carries the
/// same rounded string (`glass.rs::field_value`) and `decompose` reopens the
/// pickers on the exact civil value, so the round trip through the host is
/// lossless.
/// What was wrong was this sentence claiming a fidelity the wire format has no
/// room for. PHASE-04/VT-1 cannot see it: it composes a 2024 date, where every
/// zone's offset is a whole number of minutes.
///
/// Takes references rather than values: the caller is `install.rs`'s mapper,
/// which holds a `&FieldEdit`, and every field read out of these is an `i32`.
/// The same reasoning as PHASE-03's `Controller::edit(&Reported)`.
pub fn compose(date: &Date, time: &Time) -> Option<(Timestamp, Offset)> {
  composed_in(&TimeZone::system(), date, time)
}

/// What this field's pickers open on, for a field already picked. The inverse
/// of `compose`, and **pure**: the offset is the one the pick was resolved in
/// and is carried in the draft beside the instant, so no zone is read here
/// (design.md §5.2, §5.4).
///
/// Sub-second precision is dropped, which costs nothing: the pickers this
/// feeds resolve to the minute.
pub fn decompose(instant: Timestamp, offset: Offset) -> (Date, Time) {
  split(offset.to_datetime(instant.instant()))
}

/// What the pickers open on for a field nobody has picked: **today's date at
/// 00:00, in the person's own zone**. Both impure reads happen here — the
/// clock, and the zone, which the clock alone cannot substitute for, because
/// an instant is not a local date.
///
/// Total, and deliberately so. `glass.rs` calls this once per present for the
/// seed slots of every unpicked `datetime` field, and a present has no way to
/// report a failure — which is exactly why threading the instant through
/// `Frame` was rejected (design.md §5.3). The clock's own failure path is
/// therefore absorbed here: `wall_clock` refuses a system clock reading before
/// 1970 or outside jiff's range, and on that reading there is no better date
/// to open a picker on than the epoch's. It is a pathological branch, not the
/// untouched-field path — seeding an untouched field from `as_drawn` would put
/// 1970 on the screen in the ordinary case, and D-6 chose the epoch precisely
/// to keep it out of there.
pub fn today_local() -> (Date, Time) {
  local_midnight(
    &TimeZone::system(),
    wall_clock().unwrap_or(Timestamp::new(jiff::Timestamp::UNIX_EPOCH)),
  )
}

/// Midnight on the date an instant falls on **in a named zone**, which is the
/// whole of what `today_local` computes once the two reads are made. Pure, and
/// zone-parameterised for the same reason `composed_in` is: the claim worth
/// asserting — that a local date cannot be answered from the clock alone — is
/// a claim about two zones disagreeing, and a test cannot change the zone the
/// machine is in.
fn local_midnight(zone: &TimeZone, now: Timestamp) -> (Date, Time) {
  let (date, _) = split(now.instant().to_zoned(zone.clone()).datetime());
  (
    date,
    Time {
      hour: 0,
      minute: 0,
      second: 0,
    },
  )
}

/// `compose` with the zone named rather than read, so a unit can drive a zone
/// this machine is not in. `compose` is this applied to `TimeZone::system()`
/// and is the only caller outside the tests; the split exists because the DST
/// behaviour worth asserting belongs to a zone that *has* DST, and a CI box
/// set to UTC has none.
fn composed_in(zone: &TimeZone, date: &Date, time: &Time) -> Option<(Timestamp, Offset)> {
  let civil_date = jiff::civil::Date::new(
    i16::try_from(date.year).ok()?,
    i8::try_from(date.month).ok()?,
    i8::try_from(date.day).ok()?,
  )
  .ok()?;
  let civil_time = jiff::civil::Time::new(
    i8::try_from(time.hour).ok()?,
    i8::try_from(time.minute).ok()?,
    i8::try_from(time.second).ok()?,
    0,
  )
  .ok()?;
  let zoned = DateTime::from_parts(civil_date, civil_time)
    .to_zoned(zone.clone())
    .ok()?;
  Some((Timestamp::new(zoned.timestamp()), zoned.offset()))
}

/// The one place a jiff civil datetime becomes the pair Slint's pickers speak.
/// `i32::from` throughout — every jiff field is narrower than an `int`, so the
/// widening is infallible and no `try_from` is owed in this direction.
fn split(datetime: DateTime) -> (Date, Time) {
  (
    Date {
      year: i32::from(datetime.year()),
      month: i32::from(datetime.month()),
      day: i32::from(datetime.day()),
    },
    Time {
      hour: i32::from(datetime.hour()),
      minute: i32::from(datetime.minute()),
      second: i32::from(datetime.second()),
    },
  )
}

// PHASE-04's units. The `VT-n` here are **PHASE-04's** — slice 008 left a
// different `VT-n` sequence in `tests/renderer/wiring.rs`, and `design.md`
// §7's `Dn`, `design-log.md`'s `D-n` and `prototype-notes.md`'s `P-n` collide
// three ways besides. Always the file with the id.
#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::Timestamp;
  use goad_shell::clock::wall_clock;
  use jiff::tz::TimeZone;

  use super::{compose, composed_in, decompose, local_midnight, today_local};
  use crate::generated::{Date, Time};

  fn date(year: i32, month: i32, day: i32) -> Date {
    Date { year, month, day }
  }

  fn time(hour: i32, minute: i32, second: i32) -> Time {
    Time {
      hour,
      minute,
      second,
    }
  }

  /// The zone every DST case below is driven through. Named rather than the
  /// system's, because the behaviour under test belongs to a zone that has
  /// DST and the machine's might not.
  fn new_york() -> TimeZone {
    TimeZone::get("America/New_York").expect("the tzdb carries America/New_York")
  }

  /// PHASE-04/VT-1 — an ordinary pick composes to the instant and the offset
  /// the system zone gives it.
  ///
  /// Neither half names a number this machine chose. The offset is compared
  /// against the system zone's own answer *at the composed instant*, which is
  /// a different route to it than `compose` takes; the instant is compared by
  /// reading it back, which is the inversion VT-1's second half asks for.
  #[test]
  fn an_ordinary_date_and_time_compose_to_the_instant_the_system_zone_gives() {
    let (picked_date, picked_time) = (date(2024, 6, 15), time(14, 30, 0));

    let (instant, offset) =
      compose(&picked_date, &picked_time).expect("an ordinary civil value composes");

    assert_eq!(
      offset,
      TimeZone::system().to_offset(instant.instant()),
      "the offset is the system zone's, at the instant composed"
    );
    assert_eq!(
      decompose(instant, offset),
      (picked_date, picked_time),
      "reading the instant back at that offset gives the civil value handed in"
    );
  }

  /// PHASE-04/VT-1, the other direction — `decompose` is pure and reads no
  /// zone, so a fixed offset gives a fixed answer on every machine.
  #[test]
  fn decompose_reads_the_instant_at_the_offset_it_is_given_and_nowhere_else() {
    let epoch = Timestamp::new(jiff::Timestamp::UNIX_EPOCH);

    assert_eq!(
      decompose(epoch, jiff::tz::Offset::UTC),
      (date(1970, 1, 1), time(0, 0, 0))
    );
    assert_eq!(
      decompose(epoch, jiff::tz::Offset::constant(-5)),
      (date(1969, 12, 31), time(19, 0, 0)),
      "the same instant, five hours west"
    );
  }

  /// PHASE-04/VT-2 — each of `compose`'s four fallible steps answers `None`
  /// rather than panicking. One case per step, and the step each one reaches
  /// is named, because a step that no case reaches is a step that can become
  /// an `unwrap` without anything going red.
  #[test]
  fn each_of_composes_four_fallible_steps_answers_none_rather_than_panicking() {
    let zone = new_york();
    let noon = time(12, 0, 0);
    let ordinary = date(2024, 6, 15);

    assert_eq!(
      composed_in(&zone, &date(40_000, 1, 1), &noon),
      None,
      "step 1 — a year no i16 holds, refused by the conversion and not by jiff"
    );
    assert_eq!(
      composed_in(&zone, &ordinary, &time(1_000, 0, 0)),
      None,
      "step 2 — an hour no i8 holds, refused by the conversion"
    );
    assert_eq!(
      composed_in(&zone, &date(2024, 2, 30), &noon),
      None,
      "step 3 — every integer fits, and `Date::new` refuses the civil date"
    );
    assert_eq!(
      composed_in(&zone, &date(9999, 12, 31), &time(23, 59, 59)),
      None,
      "step 4 — a civil datetime `Date::new` accepts and `to_zoned` cannot fit"
    );
  }

  /// PHASE-04/VT-2, the fourth step's reason — whether a civil datetime fits
  /// the timestamp range depends on the **offset** it is resolved in, which is
  /// why `to_zoned` is a step of its own rather than a consequence of step 3.
  /// One civil value, two answers, and the zone is the only difference.
  #[test]
  fn whether_a_civil_datetime_fits_the_timestamp_range_depends_on_its_offset() {
    let (edge_date, edge_time) = (date(9999, 12, 31), time(12, 0, 0));
    let far_east =
      TimeZone::get("Pacific/Kiritimati").expect("the tzdb carries Pacific/Kiritimati");

    assert!(
      composed_in(&far_east, &edge_date, &edge_time).is_some(),
      "at +14:00 it is the last instant the range holds"
    );
    assert_eq!(
      composed_in(&new_york(), &edge_date, &edge_time),
      None,
      "at -05:00 the same civil value is past the end"
    );
  }

  /// PHASE-04/VT-3 — a civil time inside a DST **gap** succeeds, shifted
  /// forward. `2024-03-10 02:30` does not exist in New York: the clocks go
  /// 02:00 → 03:00.
  #[test]
  fn a_civil_time_inside_a_dst_gap_succeeds_by_shifting_forward() {
    let (instant, offset) = composed_in(&new_york(), &date(2024, 3, 10), &time(2, 30, 0))
      .expect("a gap is resolved, not refused");

    assert_eq!(
      decompose(instant, offset),
      (date(2024, 3, 10), time(3, 30, 0)),
      "02:30 shifts forward to 03:30"
    );
    assert_eq!(offset, jiff::tz::Offset::constant(-4), "and lands in EDT");
  }

  /// PHASE-04/VT-3 — a civil time inside a DST **fold** succeeds, taking the
  /// earlier of the two occurrences. `2024-11-03 01:30` happens twice in New
  /// York: the clocks go 02:00 → 01:00.
  ///
  /// The offset is what says *which* occurrence was taken. `-04:00` is EDT,
  /// before the fall back; the two candidate instants are `05:30:00Z` and
  /// `06:30:00Z`, and asserting the instant is what stops this passing on the
  /// later one.
  #[test]
  fn a_civil_time_inside_a_dst_fold_succeeds_by_taking_the_earlier_occurrence() {
    let (instant, offset) = composed_in(&new_york(), &date(2024, 11, 3), &time(1, 30, 0))
      .expect("a fold is resolved, not refused");

    assert_eq!(
      offset,
      jiff::tz::Offset::constant(-4),
      "the earlier occurrence is the one still in EDT"
    );
    assert_eq!(
      instant.instant().to_string(),
      "2024-11-03T05:30:00Z",
      "and not 06:30:00Z, which is the same civil time an hour later"
    );
    assert_eq!(
      decompose(instant, offset),
      (date(2024, 11, 3), time(1, 30, 0)),
      "the civil value is the one handed in — the fold moves the instant, not the clock face"
    );
  }

  /// PHASE-04/VA-2 — `TimeZone::system` no longer falls back to
  /// `Etc/Unknown`, which is what the `tz-system` / `tzdb-zoneinfo` features
  /// in `crates/goad/Cargo.toml` are for.
  ///
  /// **The predicate, not the offset.** Without the features
  /// `TimeZone::try_system` compiles to an unconditional `Err` and
  /// `TimeZone::system` swallows it into `TimeZone::unknown()`, which behaves
  /// as UTC — so the host would submit `+00:00` for every pick, everywhere,
  /// silently (design.md §10). `is_unknown` separates exactly that from a zone
  /// that was read, and separates nothing else: a machine set to UTC still
  /// answers a *known* zone and still passes. Asserting an offset instead
  /// would assert a property of whichever machine ran the test — this one is
  /// `Australia/Melbourne` at `+10:00`, which was checked by hand beside this.
  #[test]
  fn the_system_time_zone_is_read_rather_than_fallen_back_from() {
    let zone = TimeZone::system();

    assert!(
      !zone.is_unknown(),
      "the manifest's `tz-system` feature is what stops this being `Etc/Unknown`"
    );
  }

  /// PHASE-04 — the clock alone cannot answer a *local* date, which is why
  /// `today_local` makes **two** reads and not one (design.md §5.1).
  ///
  /// One instant, two zones, two different dates. `2024-06-15T06:00:00Z` is
  /// already the 15th's evening at `+14:00` and still the 14th at `-11:00`, so
  /// a `today_local` that resolved at UTC — or at any single fixed offset —
  /// would be wrong for somebody. This is the deterministic half of that
  /// claim; the half that says the zone read is the **system's** is one token
  /// in `today_local` and is held by review, because a test cannot change the
  /// zone the machine is in.
  #[test]
  fn one_instant_is_two_different_local_dates_in_two_different_zones() {
    let instant = Timestamp::new(
      "2024-06-15T06:00:00Z"
        .parse()
        .expect("the fixture must be an instant"),
    );
    let far_east =
      TimeZone::get("Pacific/Kiritimati").expect("the tzdb carries Pacific/Kiritimati");
    let far_west = TimeZone::get("Pacific/Midway").expect("the tzdb carries Pacific/Midway");

    assert_eq!(
      local_midnight(&far_east, instant),
      (date(2024, 6, 15), time(0, 0, 0)),
      "at +14:00 that instant is the 15th"
    );
    assert_eq!(
      local_midnight(&far_west, instant),
      (date(2024, 6, 14), time(0, 0, 0)),
      "at -11:00 the same instant is still the 14th"
    );
  }

  /// PHASE-04 — `today_local` answers a **local** date at midnight, which is
  /// why it reads the zone as well as the clock.
  ///
  /// Nothing here pins a date: the machine's today is not knowable from a
  /// test. What is asserted is the shape the seed slots need — a real civil
  /// date, and a time of exactly 00:00:00 — and that the date is the one the
  /// *system zone* is having, which is what separates a local date from a UTC
  /// one on a machine whose offset crosses midnight.
  #[test]
  fn today_local_answers_midnight_on_the_date_the_system_zone_is_having() {
    let (today, midnight) = today_local();

    assert_eq!(
      midnight,
      time(0, 0, 0),
      "the pickers open at the day's start"
    );

    let here = wall_clock()
      .expect("the machine's clock is readable")
      .instant()
      .to_zoned(TimeZone::system())
      .date();
    assert_eq!(
      today,
      date(
        i32::from(here.year()),
        i32::from(here.month()),
        i32::from(here.day())
      ),
      "and on the date the system zone is having, not UTC's"
    );
  }
}
