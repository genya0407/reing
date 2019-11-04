use chrono::prelude::*;
use chrono::Duration;

pub fn recognizable_datetime(datetime: DateTime<Local>) -> String {
    RecognizableDateTime::of(datetime, Local::now()).string()
}

#[test]
fn test_recognizable_datetime() {
    let now = Local::now();

    assert_eq!(
        RecognizableDateTime::of(now, now),
        RecognizableDateTime::Now
    );
    assert_eq!(
        RecognizableDateTime::of(now - Duration::seconds(10), now),
        RecognizableDateTime::SecondsAgo(10)
    );
    assert_eq!(
        RecognizableDateTime::of(now - Duration::minutes(10), now),
        RecognizableDateTime::MinutesAgo(10)
    );
    assert_eq!(
        RecognizableDateTime::of(now - Duration::hours(10), now),
        RecognizableDateTime::HoursAgo(10)
    );
    assert_eq!(
        RecognizableDateTime::of(now - Duration::days(3), now),
        RecognizableDateTime::DaysAgo(3),
    );
    let ten_days_ago = now - Duration::days(10);
    assert_eq!(
        RecognizableDateTime::of(ten_days_ago, now),
        RecognizableDateTime::MonthDay(ten_days_ago.month() as i64, ten_days_ago.day() as i64),
    );
    let one_year_ago = now - Duration::days(365);
    assert_eq!(
        RecognizableDateTime::of(one_year_ago, now),
        RecognizableDateTime::YearMonthDay(
            one_year_ago.year() as i64,
            one_year_ago.month() as i64,
            one_year_ago.day() as i64
        ),
    );
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RecognizableDateTime {
    Now,
    SecondsAgo(i64),
    MinutesAgo(i64),
    HoursAgo(i64),
    DaysAgo(i64),
    MonthDay(i64, i64),
    YearMonthDay(i64, i64, i64),
}

impl RecognizableDateTime {
    fn of(datetime: DateTime<Local>, now: DateTime<Local>) -> Self {
        let duration = now.signed_duration_since(datetime);

        if duration < Duration::seconds(1) {
            Self::Now
        } else if duration < Duration::minutes(1) {
            Self::SecondsAgo(duration.num_seconds())
        } else if duration < Duration::hours(1) {
            Self::MinutesAgo(duration.num_minutes())
        } else if duration < Duration::days(1) {
            Self::HoursAgo(duration.num_hours())
        } else if duration < Duration::days(7) {
            Self::DaysAgo(duration.num_hours() / 24)
        } else if duration < Duration::days(30 * 11) {
            Self::MonthDay(datetime.month() as i64, datetime.day() as i64)
        } else {
            Self::YearMonthDay(
                datetime.year() as i64,
                datetime.month() as i64,
                datetime.day() as i64,
            )
        }
    }

    fn string(&self) -> String {
        match self {
            Self::Now => String::from("今"),
            Self::SecondsAgo(secs) => format!("{}秒前", secs),
            Self::MinutesAgo(mins) => format!("{}分前", mins),
            Self::HoursAgo(hours) => format!("{}時間前", hours),
            Self::DaysAgo(days) => format!("{}日前", days),
            Self::MonthDay(month, day) => format!("{:02}/{:02}", month, day),
            Self::YearMonthDay(year, month, day) => format!("{:04}/{:02}/{:02}", year, month, day),
        }
    }
}
