use core::str::FromStr;

#[derive(Debug)]
pub enum DateTagType {
    Yearly,
    Monthly,
    Daily,
}

/// associate a specific string format to each value
impl DateTagType {
    pub fn get_format(&self) -> &str {
        match *self {
            DateTagType::Yearly => "%Y",
            DateTagType::Monthly => "%Y%m",
            DateTagType::Daily => "%Y%m%d",
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
