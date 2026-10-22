//! Temporal values for the Cypher profile's date, time, datetime, and duration
//! functions.
//!
//! The framework's result model has no date/time variant — the Core Language
//! Profile's own limitations note records that — so a temporal value is carried
//! internally by this type and rendered as its canonical ISO-8601 text when it
//! reaches a `ResultStream`.  Comparisons are numeric against the parsed
//! magnitude, which is what makes `duration('P1D') > duration('PT12H')` true
//! even though `"P1D" < "PT12H"` as text.
//!
//! Only the components the profile uses are parsed.  Years and months in a
//! duration have no exact length, so they are **rejected** rather than silently
//! approximated with a 365- or 30-day convention.

use metagdb_core::{Error, Result};

/// Which temporal family a value belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalKind {
    Date,
    Time,
    LocalTime,
    DateTime,
    LocalDateTime,
    Duration,
}

/// A parsed temporal value: its family, a comparable magnitude, and its text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Temporal {
    pub kind: TemporalKind,
    /// Days for a date, nanoseconds for everything else.
    pub magnitude: i128,
    /// Canonical ISO-8601 rendering.
    pub text: String,
}

impl Temporal {
    /// Parse a temporal literal produced by `date`, `time`, `datetime`, or
    /// `duration`.
    pub fn parse(kind: TemporalKind, source: &str) -> Result<Self> {
        match kind {
            TemporalKind::Date => parse_date(source),
            TemporalKind::Time | TemporalKind::LocalTime => parse_time(source, kind),
            TemporalKind::DateTime | TemporalKind::LocalDateTime => parse_datetime(source, kind),
            TemporalKind::Duration => parse_duration(source),
        }
    }

    /// Read one of the accessor properties (`year`, `month`, …).
    pub fn accessor(&self, name: &str) -> Result<Option<i64>> {
        match (self.kind, name) {
            (TemporalKind::Date, _) | (TemporalKind::DateTime, _) | (TemporalKind::LocalDateTime, _) => {
                let (days, nanos_of_day) = match self.kind {
                    TemporalKind::Date => (self.magnitude, 0i128),
                    _ => {
                        let day = self.magnitude.div_euclid(NANOS_PER_DAY);
                        let rest = self.magnitude.rem_euclid(NANOS_PER_DAY);
                        (day, rest)
                    }
                };
                let (year, month, day) = civil_from_days(days);
                Ok(match name {
                    "year" => Some(year),
                    "month" => Some(month),
                    "day" => Some(day),
                    // A date has no clock component, so time accessors are absent
                    // rather than a fabricated zero.
                    "hour" | "minute" | "second" if self.kind == TemporalKind::Date => None,
                    "hour" => Some((nanos_of_day / NANOS_PER_HOUR) as i64),
                    "minute" => Some(((nanos_of_day / NANOS_PER_MINUTE) % 60) as i64),
                    "second" => Some(((nanos_of_day / NANOS_PER_SECOND) % 60) as i64),
                    _ => None,
                })
            }
            (TemporalKind::Time | TemporalKind::LocalTime, _) => Ok(match name {
                "hour" => Some((self.magnitude / NANOS_PER_HOUR) as i64),
                "minute" => Some(((self.magnitude / NANOS_PER_MINUTE) % 60) as i64),
                "second" => Some(((self.magnitude / NANOS_PER_SECOND) % 60) as i64),
                _ => None,
            }),
            (TemporalKind::Duration, _) => Ok(None),
        }
    }
}

const NANOS_PER_SECOND: i128 = 1_000_000_000;
const NANOS_PER_MINUTE: i128 = 60 * NANOS_PER_SECOND;
const NANOS_PER_HOUR: i128 = 60 * NANOS_PER_MINUTE;
const NANOS_PER_DAY: i128 = 24 * NANOS_PER_HOUR;

fn bad(source: &str, what: &str) -> Error {
    Error::InvalidQuery(format!("'{source}' is not a valid {what}"))
}

fn parse_date(source: &str) -> Result<Temporal> {
    let mut parts = source.split('-');
    let year = parts
        .next()
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| bad(source, "date"))?;
    let month = parts
        .next()
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| bad(source, "date"))?;
    let day = parts
        .next()
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| bad(source, "date"))?;
    if parts.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(bad(source, "date"));
    }
    let days = days_from_civil(year, month, day);
    Ok(Temporal {
        kind: TemporalKind::Date,
        magnitude: days,
        text: format!("{year:04}-{month:02}-{day:02}"),
    })
}

fn parse_time(source: &str, kind: TemporalKind) -> Result<Temporal> {
    let mut parts = source.split(':');
    let hour = parts
        .next()
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| bad(source, "time"))?;
    let minute = parts
        .next()
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| bad(source, "time"))?;
    let seconds = parts.next().unwrap_or("0");
    if parts.next().is_some() {
        return Err(bad(source, "time"));
    }
    let (second, nanos) = split_seconds(seconds).ok_or_else(|| bad(source, "time"))?;
    if !(0..24).contains(&hour) || !(0..60).contains(&minute) || !(0..60).contains(&second) {
        return Err(bad(source, "time"));
    }
    let magnitude =
        hour as i128 * NANOS_PER_HOUR + minute as i128 * NANOS_PER_MINUTE + nanos;
    Ok(Temporal {
        kind,
        magnitude,
        text: format!("{hour:02}:{minute:02}:{second:02}"),
    })
}

fn parse_datetime(source: &str, kind: TemporalKind) -> Result<Temporal> {
    let (date, rest) = source
        .split_once('T')
        .ok_or_else(|| bad(source, "datetime"))?;
    let date = parse_date(date)?;
    let (time_text, offset) = split_offset(rest);
    let time = parse_time(time_text, kind)?;
    let magnitude = date.magnitude * NANOS_PER_DAY + time.magnitude - offset;
    let text = match &offset {
        _ if rest.ends_with('Z') => format!("{}T{}Z", date.text, time.text),
        _ => format!("{}T{}{}", date.text, time.text, offset_text(offset)),
    };
    Ok(Temporal {
        kind,
        magnitude,
        text,
    })
}

/// Split `HH:MM:SS[.fff][Z|±HH:MM]` into the time text and the UTC offset.
fn split_offset(rest: &str) -> (&str, i128) {
    if let Some(stripped) = rest.strip_suffix('Z') {
        return (stripped, 0);
    }
    // A positive offset starts with '+'; a negative one has a '-' after the
    // time's own separators, so search past the first two colons.
    if let Some(index) = rest.rfind('+') {
        return (&rest[..index], parse_offset(&rest[index..]));
    }
    if let Some(index) = rest.rfind('-') {
        return (&rest[..index], parse_offset(&rest[index..]));
    }
    (rest, 0)
}

fn parse_offset(text: &str) -> i128 {
    let sign = if text.starts_with('-') { -1 } else { 1 };
    let body = text.trim_start_matches(['+', '-']);
    let mut parts = body.split(':');
    let hours: i128 = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    let minutes: i128 = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    sign * (hours * NANOS_PER_HOUR + minutes * NANOS_PER_MINUTE)
}

fn offset_text(offset: i128) -> String {
    if offset == 0 {
        return String::new();
    }
    let sign = if offset < 0 { '-' } else { '+' };
    let absolute = offset.abs();
    format!(
        "{sign}{:02}:{:02}",
        absolute / NANOS_PER_HOUR,
        (absolute % NANOS_PER_HOUR) / NANOS_PER_MINUTE
    )
}

fn split_seconds(text: &str) -> Option<(i64, i128)> {
    match text.split_once('.') {
        Some((seconds, fraction)) => {
            let seconds = seconds.parse::<i64>().ok()?;
            let mut nanos = fraction.parse::<i128>().ok()?;
            let mut digits = fraction.len() as u32;
            while digits < 9 {
                nanos *= 10;
                digits += 1;
            }
            Some((seconds, nanos))
        }
        None => Some((text.parse::<i64>().ok()?, 0)),
    }
}

fn parse_duration(source: &str) -> Result<Temporal> {
    let body = source
        .strip_prefix('P')
        .ok_or_else(|| bad(source, "duration"))?;
    if body.contains('Y') || body.contains('M') && !body.contains('T') {
        return Err(Error::Unsupported(format!(
            "'{source}' uses a year or month duration component, which has no exact length"
        )));
    }
    let (day_part, time_part) = match body.split_once('T') {
        Some((days, times)) => (days, Some(times)),
        None => (body, None),
    };
    let mut total: i128 = 0;
    if !day_part.is_empty() {
        let days = day_part
            .strip_suffix('D')
            .ok_or_else(|| bad(source, "duration"))?;
        let days = days.parse::<i64>().map_err(|_| bad(source, "duration"))?;
        total += days as i128 * NANOS_PER_DAY;
    }
    if let Some(times) = time_part {
        let mut rest = times;
        for (suffix, unit) in [
            ('H', NANOS_PER_HOUR),
            ('M', NANOS_PER_MINUTE),
            ('S', NANOS_PER_SECOND),
        ] {
            let Some(index) = rest.find(suffix) else {
                continue;
            };
            let value = rest[..index]
                .parse::<i64>()
                .map_err(|_| bad(source, "duration"))?;
            total += value as i128 * unit;
            rest = &rest[index + suffix.len_utf8()..];
        }
        if !rest.is_empty() {
            return Err(bad(source, "duration"));
        }
    }
    Ok(Temporal {
        kind: TemporalKind::Duration,
        magnitude: total,
        text: canonical_duration(total),
    })
}

fn canonical_duration(total: i128) -> String {
    let sign = if total < 0 { "-" } else { "" };
    let absolute = total.abs();
    let days = absolute / NANOS_PER_DAY;
    let rest = absolute % NANOS_PER_DAY;
    let hours = rest / NANOS_PER_HOUR;
    let minutes = (rest % NANOS_PER_HOUR) / NANOS_PER_MINUTE;
    let seconds = (rest % NANOS_PER_MINUTE) / NANOS_PER_SECOND;
    let mut text = format!("{sign}P");
    if days != 0 {
        text.push_str(&format!("{days}D"));
    }
    if hours != 0 || minutes != 0 || seconds != 0 || days == 0 {
        text.push('T');
        if hours != 0 {
            text.push_str(&format!("{hours}H"));
        }
        if minutes != 0 {
            text.push_str(&format!("{minutes}M"));
        }
        if seconds != 0 || (hours == 0 && minutes == 0) {
            text.push_str(&format!("{seconds}S"));
        }
    }
    text
}

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant's
/// `days_from_civil`).
fn days_from_civil(year: i64, month: i64, day: i64) -> i128 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = (year - era * 400) as i128;
    let month = month as i128;
    let day = day as i128;
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era as i128 * 146_097 + day_of_era - 719_468
}

/// The inverse of [`days_from_civil`].
fn civil_from_days(days: i128) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    (year as i64, month as i64, day as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_parse_and_order_numerically() {
        let first = Temporal::parse(TemporalKind::Date, "2020-01-01").unwrap();
        let second = Temporal::parse(TemporalKind::Date, "2020-01-02").unwrap();
        assert!(first.magnitude < second.magnitude);
        assert_eq!(first.text, "2020-01-01");
        assert_eq!(
            Temporal::parse(TemporalKind::Date, "2020-01-01").unwrap(),
            first
        );
    }

    #[test]
    fn date_accessors_match_the_source() {
        let value = Temporal::parse(TemporalKind::Date, "2020-05-06").unwrap();
        assert_eq!(value.accessor("year").unwrap(), Some(2020));
        assert_eq!(value.accessor("month").unwrap(), Some(5));
        assert_eq!(value.accessor("day").unwrap(), Some(6));
        assert_eq!(value.accessor("hour").unwrap(), None);
    }

    #[test]
    fn datetimes_order_across_the_offset() {
        let earlier = Temporal::parse(TemporalKind::DateTime, "2019-01-01T00:00:00Z").unwrap();
        let later = Temporal::parse(TemporalKind::DateTime, "2020-01-01T00:00:00Z").unwrap();
        assert!(earlier.magnitude < later.magnitude);
        assert_eq!(later.text, "2020-01-01T00:00:00Z");
    }

    #[test]
    fn durations_compare_by_length_not_by_text() {
        let day = Temporal::parse(TemporalKind::Duration, "P1D").unwrap();
        let half = Temporal::parse(TemporalKind::Duration, "PT12H").unwrap();
        assert!(day.magnitude > half.magnitude);
        assert!("P1D" < "PT12H", "the text order really is the opposite");
    }

    #[test]
    fn times_order_by_clock_position() {
        let noon = Temporal::parse(TemporalKind::Time, "12:00:00").unwrap();
        let afternoon = Temporal::parse(TemporalKind::Time, "13:00:00").unwrap();
        assert!(noon.magnitude < afternoon.magnitude);
    }

    #[test]
    fn malformed_temporal_input_is_rejected() {
        assert!(Temporal::parse(TemporalKind::Date, "not-a-date").is_err());
        assert!(Temporal::parse(TemporalKind::DateTime, "2020-01-01").is_err());
        assert!(Temporal::parse(TemporalKind::Duration, "P1D2H").is_err());
    }

    #[test]
    fn durations_with_inexact_components_are_refused_not_approximated() {
        let error = Temporal::parse(TemporalKind::Duration, "P1Y").expect_err("refused");
        assert!(matches!(error, Error::Unsupported(_)), "got {error}");
        let error = Temporal::parse(TemporalKind::Duration, "P1M").expect_err("refused");
        assert!(matches!(error, Error::Unsupported(_)), "got {error}");
    }
}
