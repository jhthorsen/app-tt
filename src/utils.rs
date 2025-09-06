use chrono::Datelike;

pub const DASH: &str = "—";

pub fn format_date(d: &chrono::NaiveDateTime, format: &'static str) -> String {
    match format {
        "full" => d.format("%Y-%m-%d %H:%M").to_string(),
        "ymd" => d.format("%Y-%m-%d").to_string(),
        "hm" => d.format("%H:%M").to_string(),
        f => todo!("{f} is not supported"),
    }
}

pub fn format_duration(duration: &chrono::Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{}h {:02}m", hours, minutes)
}

pub fn is_same_date(a: &chrono::NaiveDateTime, b: &chrono::NaiveDateTime) -> bool {
    a.year() == b.year() && a.month() == b.month() && a.day() == b.day()
}

pub fn to_naive_date(date_str: Option<&String>) -> chrono::NaiveDate {
    if let Some(date_str) = date_str
        && let Ok(date) = date_str.parse::<chrono::NaiveDate>()
    {
        return date;
    }

    chrono::Local::now().naive_local().date()
}
