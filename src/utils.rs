use chrono::{Duration, Months, NaiveDate};
use regex::Regex;

use crate::datetag::DateTagType;

pub fn try_date_from_str(s: &str) -> Result<NaiveDate, &'static str> {
    checked_date_from_str(s).ok_or("conversion error")
}

pub fn checked_date_from_str(s: &str) -> Option<NaiveDate> {

    // remove any non-digit character
    let re = Regex::new("[^0-9]").unwrap();
    let mut temp = re.replace_all(s, "").to_string();

    // add a month, if needed
    if temp.len() == 4 {
        temp.push_str("01");
    }

    // add a day, if needed
    if temp.len() == 6 {
        temp.push_str("01");
    }

    // try to convert using default date format
    NaiveDate::parse_from_str(&temp, "%Y%m%d").ok()
}

pub fn checked_add_offset(
    date: &NaiveDate,
    offset: i32,
    tag_type: &DateTagType,
) -> Option<NaiveDate> {
    // apply date offset
    match tag_type {
        DateTagType::Yearly | DateTagType::Y => {
            if offset > 0 {
                date.checked_add_months(Months::new((offset * 12) as u32))
            } else {
                date.checked_sub_months(Months::new((-offset * 12) as u32))
            }
        }
        DateTagType::Monthly | DateTagType::M => {
            if offset > 0 {
                date.checked_add_months(Months::new(offset as u32))
            } else {
                date.checked_sub_months(Months::new(-offset as u32))
            }
        }
        DateTagType::Daily | DateTagType::D => {
            date.checked_add_signed(Duration::days(offset as i64))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    const YEAR: i32 = 2022;
    const MONTH: u32 = 10;
    const DAY: u32 = 5;

    fn ref_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(YEAR, MONTH, DAY).unwrap()
    }

    #[test]
    fn test_try_date_from_str_valid_plain() {
        let d = try_date_from_str("2024").unwrap();
        assert_eq!(d.year(), 2024);
        let d = try_date_from_str("202404").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        let d = try_date_from_str("20240403").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        assert_eq!(d.day(), 3);
    }

    #[test]
    fn test_try_date_from_str_valid_dash() {
        let d = try_date_from_str("2024").unwrap();
        assert_eq!(d.year(), 2024);
        let d = try_date_from_str("2024-04").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        let d = try_date_from_str("2024-04-03").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        assert_eq!(d.day(), 3);
    }

    #[test]
    fn test_try_date_from_str_valid_dot() {
        let d = try_date_from_str("2024").unwrap();
        assert_eq!(d.year(), 2024);
        let d = try_date_from_str("2024.04").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        let d = try_date_from_str("2024.04.03").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        assert_eq!(d.day(), 3);
    }

    #[test]
    fn test_try_date_from_str_valid_slash() {
        let d = try_date_from_str("2024").unwrap();
        assert_eq!(d.year(), 2024);
        let d = try_date_from_str("2024/04").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        let d = try_date_from_str("2024/04/03").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        assert_eq!(d.day(), 3);
    }

    #[test]
    fn test_try_date_from_str_valid_colon() {
        let d = try_date_from_str("2024").unwrap();
        assert_eq!(d.year(), 2024);
        let d = try_date_from_str("2024:04").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        let d = try_date_from_str("2024:04:03").unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 4);
        assert_eq!(d.day(), 3);
    }


    #[test]
    fn test_try_date_from_str_invalid() {
        let r = try_date_from_str("");
        assert!(r.is_err());
        let r = try_date_from_str("abcd");
        assert!(r.is_err());
        let r = try_date_from_str("1");
        assert!(r.is_err());
        let r = try_date_from_str("12");
        assert!(r.is_err());
        let r = try_date_from_str("123");
        assert!(r.is_err());
        let r = try_date_from_str("12345");
        assert!(r.is_err());
        let r = try_date_from_str("123456");
        assert!(r.is_err());
    }

    #[test]
    fn test_checked_date_from_str_empty() {
        let o = checked_date_from_str("");
        assert!(o.is_none());
    }
    #[test]
    fn test_checked_date_from_str_year_valid() {
        let d = checked_date_from_str("2022").unwrap();
        assert_eq!(d.year(), 2022);
    }

    #[test]
    fn test_checked_date_from_str_year_invalid() {
        let d = checked_date_from_str("abcd");
        assert!(d.is_none());
    }

    #[test]
    fn test_checked_date_from_str_year_month_valid() {
        let d = checked_date_from_str("202203").unwrap();
        assert_eq!(d.year(), 2022);
        assert_eq!(d.month(), 3);
    }

    #[test]
    fn test_checked_date_from_str_year_month_invalid() {
        let d = checked_date_from_str("abcdef");
        assert!(d.is_none());
    }

    #[test]
    fn test_checked_date_from_str_year_month_day_valid() {
        let d = checked_date_from_str("20220321").unwrap();
        assert_eq!(d.year(), 2022);
        assert_eq!(d.month(), 3);
        assert_eq!(d.day(), 21);
    }

    #[test]
    fn test_checked_date_from_str_year_month_day_invalid() {
        let d = checked_date_from_str("abcdefhi");
        assert!(d.is_none());
    }

    #[test]
    fn test_checked_add_offset_positive_year() {
        let date = checked_add_offset(&ref_date(), 1, &DateTagType::Yearly).unwrap();
        assert_eq!(date.year(), YEAR + 1);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_negative_year() {
        let date = checked_add_offset(&ref_date(), -1, &DateTagType::Yearly).unwrap();

        assert_eq!(date.year(), YEAR - 1);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_positive_year_wrapping_century() {
        let date = checked_add_offset(&ref_date(), 100, &DateTagType::Yearly).unwrap();

        assert_eq!(date.year(), YEAR + 100);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_negative_year_wrapping_century() {
        let date = checked_add_offset(&ref_date(), -100, &DateTagType::Yearly).unwrap();

        assert_eq!(date.year(), YEAR - 100);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_positive_day() {
        let date = checked_add_offset(&ref_date(), 1, &DateTagType::Daily).unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY + 1);
    }

    #[test]
    fn test_checked_add_offset_negative_day() {
        let date = checked_add_offset(&ref_date(), -1, &DateTagType::Daily).unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY - 1);
    }

    #[test]
    fn test_checked_add_offset_positive_day_wrapping_month() {
        let date = checked_add_offset(&ref_date(), 30, &DateTagType::Daily).unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH + 1);
        assert_eq!(date.day(), DAY - 1);
    }

    #[test]
    fn test_checked_add_offset_negative_day_wrapping_month() {
        let date = checked_add_offset(&ref_date(), -30, &DateTagType::Daily).unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH - 1);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_positive_month() {
        let date = checked_add_offset(&ref_date(), 1, &DateTagType::Monthly).unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH + 1);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_negative_month() {
        let date = checked_add_offset(&ref_date(), -1, &DateTagType::Monthly).unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH - 1);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_positive_month_wrapping_year() {
        let date = checked_add_offset(&ref_date(), 3, &DateTagType::Monthly).unwrap();

        assert_eq!(date.year(), YEAR + 1);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_negative_month_wrapping_year() {
        let date = checked_add_offset(&ref_date(), -12, &DateTagType::Monthly).unwrap();

        assert_eq!(date.year(), YEAR - 1);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }
}
