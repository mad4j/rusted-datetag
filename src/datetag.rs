use clap::ValueEnum;

use crate::datestyle::DateStyle;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DateTagType {
    Y,
    /// yearly tags (e.g. 2022)
    Yearly,
    M,
    /// monthly tags (e.g. 202212)
    Monthly,
    D,
    /// daily tags (e.g. 20221230)
    Daily,
}

/// associate a specific string format to each value
impl DateTagType {
    pub fn get_format(&self, style: DateStyle) -> &str {
        match (self, style) {
            (DateTagType::Y, DateStyle::Plain) => "%Y",
            (DateTagType::Y, DateStyle::Dot) => "%Y",
            (DateTagType::Y, DateStyle::Slash) => "%Y",
            (DateTagType::Y, DateStyle::Colon) => "%Y",
            (DateTagType::Y, DateStyle::Dash) => "%Y",
            (DateTagType::Yearly, DateStyle::Plain) => "%Y",
            (DateTagType::Yearly, DateStyle::Dot) => "%Y",
            (DateTagType::Yearly, DateStyle::Slash) => "%Y",
            (DateTagType::Yearly, DateStyle::Colon) => "%Y",
            (DateTagType::Yearly, DateStyle::Dash) => "%Y",
            (DateTagType::M, DateStyle::Plain) => "%Y%m",
            (DateTagType::M, DateStyle::Dot) => "%Y.%m",
            (DateTagType::M, DateStyle::Slash) => "%Y/%m",
            (DateTagType::M, DateStyle::Colon) => "%Y:%m",
            (DateTagType::M, DateStyle::Dash) => "%Y-%m",
            (DateTagType::Monthly, DateStyle::Plain) => "%Y%m",
            (DateTagType::Monthly, DateStyle::Dot) => "%Y.%m",
            (DateTagType::Monthly, DateStyle::Slash) => "%Y/%m",
            (DateTagType::Monthly, DateStyle::Colon) => "%Y:%m",
            (DateTagType::Monthly, DateStyle::Dash) => "%Y-%m",
            (DateTagType::D, DateStyle::Plain) => "%Y%m%d",
            (DateTagType::D, DateStyle::Dot) => "%Y.%m.%d",
            (DateTagType::D, DateStyle::Slash) => "%Y/%m/%d",
            (DateTagType::D, DateStyle::Colon) => "%Y:%m:%d",
            (DateTagType::D, DateStyle::Dash) => "%Y-%m-%d",
            (DateTagType::Daily, DateStyle::Plain) => "%Y%m%d",
            (DateTagType::Daily, DateStyle::Dot) => "%Y.%m.%d",
            (DateTagType::Daily, DateStyle::Slash) => "%Y/%m/%d",
            (DateTagType::Daily, DateStyle::Colon) => "%Y:%m:%d",
            (DateTagType::Daily, DateStyle::Dash) => "%Y-%m-%d",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_format_year_plain() {
        // test year-related variants
        let d = DateTagType::Yearly;
        assert!(d.get_format(DateStyle::Plain) == "%Y");
        let d = DateTagType::Y;
        assert!(d.get_format(DateStyle::Plain) == "%Y");
    }

    #[test]
    fn test_get_format_month_plain() {
        // test month-related variants
        let d = DateTagType::Monthly;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m");
        let d = DateTagType::M;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m");
    }

    #[test]
    fn test_get_format_day_plain() {
        // test day-related variants
        let d = DateTagType::Daily;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m%d");
        let d = DateTagType::D;
        assert!(d.get_format(DateStyle::Plain) == "%Y%m%d");
    }
}
