use std::cmp::Ordering;

use chrono::{NaiveDate, Utc};
use regex::Regex;

pub fn validate_ymd_string(ymd_string: &str) -> bool {
    let ymd_re = Regex::new(r"^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$").unwrap();

    if !ymd_re.is_match(ymd_string) {
        return false;
    }

    match NaiveDate::parse_from_str(ymd_string, "%Y-%m-%d") {
        Ok(date) => {
            if Utc::now().date_naive().cmp(&date) == Ordering::Less {
                return false;
            }

            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use chrono::Days;

    use super::*;

    #[test]
    fn test_validate_ymd_string() {
        let tomorrow = format!(
            "{}",
            Utc::now()
                .checked_add_days(Days::new(1))
                .unwrap()
                .format("%Y-%m-%d")
        );

        assert!(validate_ymd_string("1970-01-01"));
        assert!(validate_ymd_string("2020-02-29"));
        assert!(!validate_ymd_string("70-01-01"));
        assert!(!validate_ymd_string("2023-02-29"));
        assert!(!validate_ymd_string(&tomorrow));
    }
}
