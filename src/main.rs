//! datetag allows to generatate and manage date tags (e.g. TEST_202008).
//! A datetag is a label similar to:
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

mod datetag;
mod utils;

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use clap::Parser;

use datetag::DateTagType;

#[derive(Debug, Parser)]
#[command(
    name = "datetag",
    version,
    about = r#"display a customizable date tag (e.g. TEST_202110)

EXAMPLES:
 
    $ datetag --offset 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -o 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403"#
)]

struct Opt {
    /// tag type [d | m | y | daily | monthly | yearly]
    #[arg(value_enum, short, long, default_value = "m")]
    tag_type: DateTagType,

    /// tag prefix
    #[arg(short, long)]
    prefix: Option<String>,

    /// date tag value (one of 'yyyymmdd', 'yyyymm', 'yyyy')
    #[arg(short, long, value_parser=utils::try_date_from_str)]
    date: Option<NaiveDate>,

    /// date tag offset
    #[arg(short, long, default_value = "0")]
    offset: i32,
}

fn main() -> Result<()> {
    // parse command-line parameters
    let opt = Opt::parse();

    // parse date related parameters
    let date = opt
        .date
        .unwrap_or_else(|| Local::now().naive_local().date());

    // apply date offset
    let date = utils::checked_add_offset(&date, opt.offset, &opt.tag_type)
        .with_context(|| "wrong date offset".to_string())?;

    // display date tag
    print!(
        "{}{}",
        opt.prefix.unwrap_or_default(),
        date.format(opt.tag_type.get_format())
    );

    Ok(())
}
