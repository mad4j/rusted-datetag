use clap::ValueEnum;

use crate::datestyle::DateStyle;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DateTag {
    Y,
    /// yearly tags (e.g. 2022)
    Yearly,
    W,
    /// weekly ISO 8601 tags (e.g. 202234)
    Weekly,
    M,
    /// monthly tags (e.g. 202212)
    Monthly,
    D,
    /// daily tags (e.g. 20221230)
    Daily,
}

/// associate a specific string format to each value
impl DateTag {
    pub fn get_format(&self, style: DateStyle) -> &str {
        match (self, style) {
            // Yealy tags
            (DateTag::Y, DateStyle::Plain) => "%Y",
            (DateTag::Y, DateStyle::Dot) => "%Y",
            (DateTag::Y, DateStyle::Slash) => "%Y",
            (DateTag::Y, DateStyle::Colon) => "%Y",
            (DateTag::Y, DateStyle::Dash) => "%Y",
            (DateTag::Yearly, DateStyle::Plain) => "%Y",
            (DateTag::Yearly, DateStyle::Dot) => "%Y",
            (DateTag::Yearly, DateStyle::Slash) => "%Y",
            (DateTag::Yearly, DateStyle::Colon) => "%Y",
            (DateTag::Yearly, DateStyle::Dash) => "%Y",
            // Monthly tags
            (DateTag::M, DateStyle::Plain) => "%Y%m",
            (DateTag::M, DateStyle::Dot) => "%Y.%m",
            (DateTag::M, DateStyle::Slash) => "%Y/%m",
            (DateTag::M, DateStyle::Colon) => "%Y:%m",
            (DateTag::M, DateStyle::Dash) => "%Y-%m",
            (DateTag::Monthly, DateStyle::Plain) => "%Y%m",
            (DateTag::Monthly, DateStyle::Dot) => "%Y.%m",
            (DateTag::Monthly, DateStyle::Slash) => "%Y/%m",
            (DateTag::Monthly, DateStyle::Colon) => "%Y:%m",
            (DateTag::Monthly, DateStyle::Dash) => "%Y-%m",
            // Weekly tags
            (DateTag::W, DateStyle::Plain) => "%G%V",
            (DateTag::W, DateStyle::Dot) => "%G.%V",
            (DateTag::W, DateStyle::Slash) => "%G/%V",
            (DateTag::W, DateStyle::Colon) => "%G:%V",
            (DateTag::W, DateStyle::Dash) => "%G-%V",
            (DateTag::Weekly, DateStyle::Plain) => "%G%V",
            (DateTag::Weekly, DateStyle::Dot) => "%G.%V",
            (DateTag::Weekly, DateStyle::Slash) => "%G/%V",
            (DateTag::Weekly, DateStyle::Colon) => "%G:%V",
            (DateTag::Weekly, DateStyle::Dash) => "%G-%V",
            // Dayly tags
            (DateTag::D, DateStyle::Plain) => "%Y%m%d",
            (DateTag::D, DateStyle::Dot) => "%Y.%m.%d",
            (DateTag::D, DateStyle::Slash) => "%Y/%m/%d",
            (DateTag::D, DateStyle::Colon) => "%Y:%m:%d",
            (DateTag::D, DateStyle::Dash) => "%Y-%m-%d",
            (DateTag::Daily, DateStyle::Plain) => "%Y%m%d",
            (DateTag::Daily, DateStyle::Dot) => "%Y.%m.%d",
            (DateTag::Daily, DateStyle::Slash) => "%Y/%m/%d",
            (DateTag::Daily, DateStyle::Colon) => "%Y:%m:%d",
            (DateTag::Daily, DateStyle::Dash) => "%Y-%m-%d",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_format_year_plain() {
        // test year-related variants
        let d = DateTag::Yearly;
        assert!(d.get_format(DateStyle::Plain) == "%Y");
        let d = DateTag::Y;
        assert!(d.get_format(DateStyle::Plain) == "%Y");
    }

    #[test]
    fn test_get_format_year_dot() {
        // test year-related variants
        let d = DateTag::Yearly;
        assert!(d.get_format(DateStyle::Dot) == "%Y");
        let d = DateTag::Y;
        assert!(d.get_format(DateStyle::Dot) == "%Y");
    }

    #[test]
    fn test_get_format_year_slash() {
        // test year-related variants
        let d = DateTag::Yearly;
        assert!(d.get_format(DateStyle::Slash) == "%Y");
        let d = DateTag::Y;
        assert!(d.get_format(DateStyle::Slash) == "%Y");
    }

    #[test]
    fn test_get_format_year_colon() {
        // test year-related variants
        let d = DateTag::Yearly;
        assert!(d.get_format(DateStyle::Colon) == "%Y");
        let d = DateTag::Y;
        assert!(d.get_format(DateStyle::Colon) == "%Y");
    }

    #[test]
    fn test_get_format_year_dash() {
        // test year-related variants
        let d = DateTag::Yearly;
        assert!(d.get_format(DateStyle::Dash) == "%Y");
        let d = DateTag::Y;
        assert!(d.get_format(DateStyle::Dash) == "%Y");
    }

    #[test]
    fn test_get_format_month_plain() {
        // test month-related variants
        let d = DateTag::Monthly;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m");
        let d = DateTag::M;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m");
    }

    #[test]
    fn test_get_format_month_dot() {
        // test month-related variants
        let d = DateTag::Monthly;
        assert!(d.get_format(DateStyle::Dot) == "%Y.%m");
        let d = DateTag::M;
        assert!(d.get_format(DateStyle::Dot) == "%Y.%m");
    }

    #[test]
    fn test_get_format_month_slash() {
        // test month-related variants
        let d = DateTag::Monthly;
        assert!(d.get_format(DateStyle::Slash) == "%Y/%m");
        let d = DateTag::M;
        assert!(d.get_format(DateStyle::Slash) == "%Y/%m");
    }

    #[test]
    fn test_get_format_month_colon() {
        // test month-related variants
        let d = DateTag::Monthly;
        assert!(d.get_format(DateStyle::Colon) == "%Y:%m");
        let d = DateTag::M;
        assert!(d.get_format(DateStyle::Colon) == "%Y:%m");
    }

    #[test]
    fn test_get_format_month_dash() {
        // test month-related variants
        let d = DateTag::Monthly;
        assert!(d.get_format(DateStyle::Dash) == "%Y-%m");
        let d = DateTag::M;
        assert!(d.get_format(DateStyle::Dash) == "%Y-%m");
    }

    #[test]
    fn test_get_format_week_plain() {
        // test month-related variants
        let d = DateTag::Weekly;
        assert!(d.get_format(DateStyle::Plain) == "%G%V");
        let d = DateTag::W;
        assert!(d.get_format(DateStyle::Plain) == "%G%V");
    }

    #[test]
    fn test_get_format_week_dot() {
        // test month-related variants
        let d = DateTag::Weekly;
        assert!(d.get_format(DateStyle::Dot) == "%G.%V");
        let d = DateTag::W;
        assert!(d.get_format(DateStyle::Dot) == "%G.%V");
    }

    #[test]
    fn test_get_format_week_slash() {
        // test month-related variants
        let d = DateTag::Weekly;
        assert!(d.get_format(DateStyle::Slash) == "%G/%V");
        let d = DateTag::W;
        assert!(d.get_format(DateStyle::Slash) == "%G/%V");
    }

    #[test]
    fn test_get_format_week_colon() {
        // test month-related variants
        let d = DateTag::Weekly;
        assert!(d.get_format(DateStyle::Colon) == "%G:%V");
        let d = DateTag::W;
        assert!(d.get_format(DateStyle::Colon) == "%G:%V");
    }

    #[test]
    fn test_get_format_week_dash() {
        // test month-related variants
        let d = DateTag::Weekly;
        assert!(d.get_format(DateStyle::Dash) == "%G-%V");
        let d = DateTag::W;
        assert!(d.get_format(DateStyle::Dash) == "%G-%V");
    }

    #[test]
    fn test_get_format_day_plain() {
        // test day-related variants
        let d = DateTag::Daily;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m%d");
        let d = DateTag::D;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m%d");
    }

    #[test]
    fn test_get_format_day_dot() {
        // test day-related variants
        let d = DateTag::Daily;
        assert!(d.get_format(DateStyle::Dot) == "%Y.%m.%d");
        let d = DateTag::D;
        assert!(d.get_format(DateStyle::Dot) == "%Y.%m.%d");
    }

    #[test]
    fn test_get_format_day_slash() {
        // test day-related variants
        let d = DateTag::Daily;
        assert!(d.get_format(DateStyle::Slash) == "%Y/%m/%d");
        let d = DateTag::D;
        assert!(d.get_format(DateStyle::Slash) == "%Y/%m/%d");
    }

    #[test]
    fn test_get_format_day_colon() {
        // test day-related variants
        let d = DateTag::Daily;
        assert!(d.get_format(DateStyle::Colon) == "%Y:%m:%d");
        let d = DateTag::D;
        assert!(d.get_format(DateStyle::Colon) == "%Y:%m:%d");
    }

    #[test]
    fn test_get_format_day_dash() {
        // test day-related variants
        let d = DateTag::Daily;
        assert!(d.get_format(DateStyle::Dash) == "%Y-%m-%d");
        let d = DateTag::D;
        assert!(d.get_format(DateStyle::Dash) == "%Y-%m-%d");
    }
}
