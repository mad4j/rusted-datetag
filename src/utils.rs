use chrono::NaiveDate;
use core::str::FromStr;


pub fn try_date_from_str(s: &str) -> Result<NaiveDate, &'static str> {
    date_from_str_opt(s).ok_or("conversion error")
}

pub fn date_from_str_opt(s: &str) -> Option<NaiveDate> {

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_empty() {
        let o = date_from_str_opt("");
        assert!(o.is_none());
    }
    #[test]
    fn test_year_valid() {
        let d = date_from_str_opt("2022").unwrap();
        assert_eq!(d.year(), 2022);
    }

    #[test]
    fn test_year_invalid() {
        let d = date_from_str_opt("abcd");
        assert!(d.is_none());
    }

    #[test]
    fn test_year_month_valid() {
        let d = date_from_str_opt("202203").unwrap();
        assert_eq!(d.year(), 2022);
        assert_eq!(d.month(), 3);
    }

    #[test]
    fn test_year_month_invalid() {
        let d = date_from_str_opt("abcdef");
        assert!(d.is_none());
    }

    #[test]
    fn test_year_month_day_valid() {
        let d = date_from_str_opt("20220321").unwrap();
        assert_eq!(d.year(), 2022);
        assert_eq!(d.month(), 3);
        assert_eq!(d.day(), 21);
    }

    #[test]
    fn test_year_month_day_invalid() {
        let d = date_from_str_opt("abcdefhi");
        assert!(d.is_none());
    }
}
