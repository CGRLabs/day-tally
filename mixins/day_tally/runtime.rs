// Day Tally: the day of the world, in the bottom left corner, in Roman
// numerals.
//
// The tally appears when the day turns and goes away again a few seconds
// later, so it tells you the moment a night passes and then leaves the screen
// to the world.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

pub const ID: &str = "day_tally";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str =
    "The day of the world, in Roman numerals, in the bottom left corner. It appears when the day turns, and goes away again.";

/// How long the tally stays up after the day changes.
pub const SHOW_FOR: Duration = Duration::from_secs(5);
/// The last stretch of it: it shrinks away.
pub const FADE_FOR: Duration = Duration::from_secs(2);

/// The ticks in a day.
pub const DAY: i64 = 24_000;

/// The day the world is on.
pub fn day_of(day_time: i64) -> i64 {
    day_time.div_euclid(DAY)
}

/// The day in Roman numerals. The Romans had no zero, so the first day is
/// "N" for nulla.
pub fn roman(day: i64) -> String {
    const NUMERALS: &[(i64, &str)] = &[
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    if day <= 0 {
        return "N".to_owned();
    }
    let mut left = day;
    let mut out = String::new();
    for (value, name) in NUMERALS {
        while left >= *value {
            out.push_str(name);
            left -= value;
        }
    }
    out
}

/// The day, and when it turned.
struct Tally {
    day: i64,
    at: Instant,
}

static TALLY: LazyLock<Mutex<Option<Tally>>> = LazyLock::new(|| Mutex::new(None));

/// Note the day, if it is not the day that was noted last.
pub fn note(day: i64) {
    let mut tally = TALLY.lock().unwrap();
    match tally.as_ref() {
        Some(t) if t.day == day => {}
        _ => *tally = Some(Tally { day, at: Instant::now() }),
    }
}

/// The day to show and how big it is: `None` when the tally has gone.
pub fn visible() -> Option<(i64, f32)> {
    let tally = TALLY.lock().unwrap();
    let tally = tally.as_ref()?;
    let elapsed = tally.at.elapsed();
    if elapsed >= SHOW_FOR {
        return None;
    }
    let left = (SHOW_FOR - elapsed).as_secs_f32();
    let scale = (left / FADE_FOR.as_secs_f32()).clamp(0.0, 1.0);
    Some((tally.day, scale))
}

/// Whether the Day Counter content mod has been told that the tally is here.
static FLAG: AtomicBool = AtomicBool::new(false);
/// How many times the flag has been tried.
static TRIES: AtomicU32 = AtomicU32::new(0);

pub fn flag_set() -> bool {
    FLAG.load(Ordering::Relaxed)
}

pub fn set_flag() {
    FLAG.store(true, Ordering::Relaxed);
}

pub fn tries() -> u32 {
    TRIES.load(Ordering::Relaxed)
}

pub fn note_a_try() {
    TRIES.fetch_add(1, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_day_is_nulla() {
        // The Romans had no zero.
        assert_eq!(roman(0), "N");
        assert_eq!(roman(-1), "N");
    }

    #[test]
    fn the_small_numerals_are_right() {
        assert_eq!(roman(1), "I");
        assert_eq!(roman(2), "II");
        assert_eq!(roman(3), "III");
        assert_eq!(roman(4), "IV");
        assert_eq!(roman(5), "V");
        assert_eq!(roman(9), "IX");
        assert_eq!(roman(14), "XIV");
    }

    #[test]
    fn the_bigger_numerals_are_right() {
        assert_eq!(roman(40), "XL");
        assert_eq!(roman(49), "XLIX");
        assert_eq!(roman(90), "XC");
        assert_eq!(roman(400), "CD");
        assert_eq!(roman(900), "CM");
        assert_eq!(roman(1990), "MCMXC");
        assert_eq!(roman(2026), "MMXXVI");
    }

    #[test]
    fn a_day_is_the_clock_divided_by_the_length_of_a_day() {
        assert_eq!(day_of(0), 0);
        assert_eq!(day_of(23_999), 0);
        assert_eq!(day_of(24_000), 1);
        assert_eq!(day_of(48_001), 2);
        // A clock that went backwards stays before the first day.
        assert_eq!(day_of(-1), -1);
    }

    #[test]
    fn the_tally_shows_and_then_goes() {
        note(7);
        assert!(visible().is_some(), "the tally did not show");
        let (day, scale) = visible().expect("the tally is showing");
        assert_eq!(day, 7);
        assert_eq!(scale, 1.0, "the tally did not start at full size");
    }

    #[test]
    fn the_same_day_does_not_note_the_tally_again() {
        note(3);
        let first = visible().expect("the tally is showing");
        note(3);
        let second = visible().expect("the tally is showing");
        // The same day must not restart the timer.
        assert_eq!(first.0, second.0);
        assert!(second.1 >= first.1, "the tally restarted");
    }
}
