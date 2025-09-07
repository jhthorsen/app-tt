use anyhow::anyhow;

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

pub fn min_duration() -> Result<i64, anyhow::Error> {
    Ok(std::env::var("TT_MIN_DURATION")
        .unwrap_or_else(|_| "300".to_string())
        .parse()?)
}

pub fn to_naive_date_time(
    human_date: Option<&String>,
    now: Option<&chrono::NaiveDateTime>,
) -> Result<chrono::NaiveDateTime, anyhow::Error> {
    let now = now
        .cloned()
        .unwrap_or_else(|| chrono::Local::now().naive_local());

    let Some(human_date) = human_date else {
        return Ok(now);
    };

    // Special case
    if human_date.trim().to_lowercase() == "now" {
        return Ok(now);
    }

    let mut parts: Vec<String> = human_date
        .trim()
        .replace(" ", "T")
        .split("T")
        .map(|i| i.to_string())
        .collect();

    if parts.len() == 1 {
        if parts[0].contains(":") {
            parts.insert(0, now.format("%Y-%m-%d").to_string());
        } else {
            parts.push(now.format("%H:%M:%S").to_string());
        }
    }

    if parts[1].split(":").count() == 2 {
        parts[1] = format!("{}:00", parts[1]);
    }

    let date_str = parts.join("T");
    date_str
        .parse::<chrono::NaiveDateTime>()
        .map_err(|_| anyhow!("Unable to parse date \"{human_date}\""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDateTime, Utc};

    // 2025-09-07 03:16:40
    static NOW: NaiveDateTime = DateTime::<Utc>::from_timestamp(1757233000, 0)
        .unwrap()
        .naive_utc();

    fn format(d: chrono::NaiveDateTime) -> String {
        d.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[test]
    fn test_valid_date() {
        let s = "2023-10-10T12:34:56  ".to_string();
        assert_eq!(
            format(to_naive_date_time(Some(&s), Some(&NOW)).unwrap()).as_str(),
            "2023-10-10 12:34:56"
        );

        let s = "  2023-09-10 11:34:56".to_string();
        assert_eq!(
            format(to_naive_date_time(Some(&s), Some(&NOW)).unwrap()).as_str(),
            "2023-09-10 11:34:56"
        );

        let s = "  2023-08-10".to_string();
        assert_eq!(
            format(to_naive_date_time(Some(&s), Some(&NOW)).unwrap()),
            "2023-08-10 08:16:40".to_string()
        );

        let s = "  09:05:55".to_string();
        assert_eq!(
            format(to_naive_date_time(Some(&s), Some(&NOW)).unwrap()),
            "2025-09-07 09:05:55".to_string()
        );

        let s = "  09:05".to_string();
        assert_eq!(
            format(to_naive_date_time(Some(&s), Some(&NOW)).unwrap()),
            "2025-09-07 09:05:00".to_string()
        );

        let s = "noW".to_string();
        assert_eq!(
            format(to_naive_date_time(Some(&s), Some(&NOW)).unwrap()),
            "2025-09-07 08:16:40".to_string()
        );
    }

    #[test]
    fn test_invalid_date() {
        for s in ["invalid date", "2023-10-10T40:34:56", "2023", "50", "02"] {
            assert!(to_naive_date_time(Some(&s.to_string()), Some(&NOW)).is_err());
        }
    }

    #[test]
    fn test_none_date() {
        assert_eq!(
            format(to_naive_date_time(None, Some(&NOW)).unwrap()),
            "2025-09-07 08:16:40".to_string()
        );
    }
}
