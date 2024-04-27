use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
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
    pub fn get_format(&self) -> &str {
        match *self {
            DateTagType::Yearly | DateTagType::Y => "%Y",
            DateTagType::Monthly | DateTagType::M => "%Y%m",
            DateTagType::Daily | DateTagType::D => "%Y%m%d",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_format_year() {
        // test year-related variants
        let d = DateTagType::Yearly;
        assert!(d.get_format() == "%Y");
        let d = DateTagType::Y;
        assert!(d.get_format() == "%Y");
    }

    #[test]
    fn test_get_format_month() {
        // test month-related variants
        let d = DateTagType::Monthly;
        assert!(d.get_format() == "%Y%m");
        let d = DateTagType::M;
        assert!(d.get_format() == "%Y%m");
    }

    #[test]
    fn test_get_format_day() {
        // test day-related variants
        let d = DateTagType::Daily;
        assert!(d.get_format() == "%Y%m%d");
        let d = DateTagType::D;
        assert!(d.get_format() == "%Y%m%d");
    }
}