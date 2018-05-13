use chrono::prelude::*;
use chrono::Duration;

pub fn recognizable_datetime(datetime: DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(datetime);

    if duration < Duration::minutes(1) {
        format!("{}秒前", duration.num_seconds())
    } else if duration < Duration::hours(1) {
        format!("{}分前", duration.num_minutes())
    } else if duration < Duration::days(1) {
        format!("{}時間前", duration.num_hours())
    } else {
        format!("{}", datetime.format("%m/%d"))
    }
}