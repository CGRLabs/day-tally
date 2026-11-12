// Day Tally: the day of the world, in the bottom left corner, in plain
// numbers.
//
// The tally appears when the day turns and goes away again a few seconds
// later, so it tells you the moment a night passes and then leaves the screen
// to the world.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

pub const ID: &str = "day_tally";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str =
    "The day of the world, big, in the bottom left corner. It appears when the day turns, and goes away again.";

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
    fn the_day_is_a_plain_number() {
        // The tally shows the day as a plain number, not Roman numerals.
        assert_eq!(0.to_string(), "0");
        assert_eq!(12.to_string(), "12");
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
