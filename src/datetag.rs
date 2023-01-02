use core::str::FromStr;

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

impl FromStr for DateTagType {
    type Err = i32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "y" | "yearly" => Ok(DateTagType::Yearly),
            "m" | "monthly" => Ok(DateTagType::Monthly),
            "d" | "daily" => Ok(DateTagType::Daily),
            _ => Err(-1),
        }
    }
}
