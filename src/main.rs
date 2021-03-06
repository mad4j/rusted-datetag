//! datetag is a simple application to generatate e manage date tags.
//! A datetag is a label similar to 
//! * 20201113
//! * TEST_202008
//! 
//! datetag contains:
//!  * an optional prefix (e.g. 'TEST_')
//!  * a date reference
//! 
//! datetag references belong to one of the following types:
//!  * YEARLY (i.e. match the format '%Y')
//!  * MONTHLY (i.e. match the format '%Y%m')
//!  * DAILY (i.e. match the format '%Y%m&d')
//! 
//! It is possible to add/subtract an offset to specific datetag.
//! The offset value reprensents:
//!  * years
//!  * months
//!  * days
//! depending on the datetag type.
//! 
//! It is possible to obtain the NOW datetag or provide a date reference.

use anyhow::{Context, Result};
use chrono::{Datelike, Duration, Local, NaiveDate};
use core::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum DateTagType {
    Yearly,
    Monthly,
    Daily,
}

/// associate a specific string format to each value
impl DateTagType {
    fn get_format(&self) -> &str {
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

#[derive(Debug, StructOpt)]
#[structopt(name = "datetag", about = "display a customizable date tag")]
struct Opt {
    /// tag type [d | m | y]
    #[structopt(short, long, default_value = "m")]
    tag_type: DateTagType,

    /// tag prefix
    #[structopt(short, long)]
    prefix: Option<String>,

    /// date tag value
    #[structopt(short, long)]
    date: Option<String>,

    /// date tag positive offset
    #[structopt(short, long, conflicts_with = "sub")]
    add: Option<u32>,

    /// date tag negative offset
    #[structopt(short, long, conflicts_with = "add")]
    sub: Option<u32>,
}

fn main() -> Result<()> {
    // parse command-line parameters
    let opt = Opt::from_args();

    // parse date related parameters
    let date: NaiveDate = match opt.date {
        Some(v) => NaiveDate::parse_from_str(&v, &opt.tag_type.get_format())
            .with_context(|| format!("date does not match format"))?,
        None => Local::now().naive_local().date(),
    };

    // parse offset related parameters
    let offset: i32 = opt.add.unwrap_or_default() as i32 - opt.sub.unwrap_or_default() as i32;

    // apply date offset
    let date = match opt.tag_type {
        DateTagType::Yearly => NaiveDate::from_ymd(date.year() + offset, date.month(), date.day()),
        DateTagType::Monthly => {
            let d = 12 * date.year() + offset;
            NaiveDate::from_ymd(d / 12, (d as u32 % 12) + 1, 1)
        }
        DateTagType::Daily => date
            .checked_add_signed(Duration::days(offset as i64))
            .with_context(|| format!("wrong date offset"))?,
    };

    // display date tag
    print!(
        "{}{}",
        opt.prefix.unwrap_or_default(),
        date.format(&opt.tag_type.get_format())
    );

    Ok(())
}
