use chrono::{Duration, Months, NaiveDate};
use core::str::FromStr;

use crate::datetag::DateTagType;

pub fn try_date_from_str(s: &str) -> Result<NaiveDate, &'static str> {
    checked_date_from_str(s).ok_or("conversion error")
}

pub fn checked_date_from_str(s: &str) -> Option<NaiveDate> {
    // avoid out of bound panic
    let y = if s.len() > 3 {
        i32::from_str(&s[0..4]).ok()?
    } else {
        None?
    };

    // avoid out of bound panic
    let m = if s.len() > 5 {
        u32::from_str(&s[4..6]).ok()?
    } else {
        1
    };

    // avoid out of bound panic
    let d = if s.len() > 7 {
        u32::from_str(&s[6..8]).ok()?
    } else {
        1
    };

    NaiveDate::from_ymd_opt(y, m, d)
}

pub fn checked_add_offset(
    date: &NaiveDate,
    offset: i32,
    tag_type: &DateTagType,
) -> Option<NaiveDate> {
    // apply date offset
    match tag_type {
        DateTagType::Yearly => {
            if offset > 0 {
                date.checked_add_months(Months::new((offset * 12) as u32))
            } else {
                date.checked_sub_months(Months::new((-offset * 12) as u32))
            }
        }
        DateTagType::Monthly => {
            if offset > 0 {
                date.checked_add_months(Months::new(offset as u32))
            } else {
                date.checked_sub_months(Months::new(-offset as u32))
            }
        }
        DateTagType::Daily => date.checked_add_signed(Duration::days(offset as i64)),
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
        NaiveDate::from_ymd(YEAR, MONTH, DAY)
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
        let date = checked_add_offset(
            &ref_date(), 
            1, 
            &DateTagType::Yearly
        )
        .unwrap();
        assert_eq!(date.year(), YEAR + 1);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_negative_year() {
        let date = checked_add_offset(
            &ref_date(), 
            -1, 
            &DateTagType::Yearly
        )
        .unwrap();

        assert_eq!(date.year(), YEAR - 1);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_positive_year_wrapping_century() {
        let date = checked_add_offset(
            &ref_date(), 
            100, 
            &DateTagType::Yearly
        )
        .unwrap();
        
        assert_eq!(date.year(), YEAR + 100);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_negative_year_wrapping_century() {
        let date = checked_add_offset(
            &ref_date(), 
            -100, 
            &DateTagType::Yearly
        )
        .unwrap();

        assert_eq!(date.year(), YEAR-100);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY);
    }

    #[test]
    fn test_checked_add_offset_positive_day() {
        let date = checked_add_offset(
            &ref_date(), 
            1, 
            &DateTagType::Daily
        )
        .unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY+1);
    }

    #[test]
    fn test_checked_add_offset_negative_day() {
        let date = checked_add_offset(
            &ref_date(), 
            -1, 
            &DateTagType::Daily
        )
        .unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH);
        assert_eq!(date.day(), DAY-1);
    }

    #[test]
    fn test_checked_add_offset_positive_day_wrapping_month() {
        let date = checked_add_offset(
            &ref_date(), 
            30, 
            &DateTagType::Daily
        )
        .unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH+1);
        assert_eq!(date.day(), DAY-1);
    }

    #[test]
    fn test_checked_add_offset_negative_day_wrapping_month() {
        let date = checked_add_offset(
            &ref_date(), 
            -30, 
            &DateTagType::Daily
        )
        .unwrap();

        assert_eq!(date.year(), YEAR);
        assert_eq!(date.month(), MONTH-1);
        assert_eq!(date.day(), DAY);
    }
}
