//! datetag allows to generatate and manage date tags (e.g. TEST_202008).
//! A datetag is a label similar to:
//! * 20201113
//! * TEST_202008
//! * 202404_rel
//!
//! datetag contains:
//!  * an optional prefix label (e.g. 'TEST_')
//!  * a date reference (e.g. 202404)
//!  * an optional suffix label (e.g '_rel')
//!
//! datetag references belong to one of the following types:
//!  * YEARLY (i.e. match the format '%Y')
//!  * MONTHLY (i.e. match the format '%Y%m')
//!  * DAILY (i.e. match the format '%Y%m&d')
//!
//! datetag refereces can be plain formated (e.g. 20240424) or
//! formatted using a separator character (i.e. '.', ':' or '/'):
//!  * 2024.04.24
//!  * 2024-04-24
//!  * 2024/04/24
//!
//! It is possible to add/subtract an offset to specific datetag.
//! The offset value should be expressed in:
//!  * years
//!  * months
//!  * days
//! depending on the datetag type.
//!
//! In case of generation of a single datetag then the offset will
//! be immediately added (or subtracted) to the given reference
//! date. Otherwise, the first datetag will represents the given
//! reference date and the offset will be will be used to increase
//! subsequent datetags.
//!
//! It is possible to obtain the NOW datetag or provide the current
//! reference date.

mod datestyle;
mod datetag;
mod texts;
mod utils;

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use clap::Parser;

use datestyle::DateStyle;
use datetag::DateTag;

#[derive(Debug, Parser)]
#[command(
    name = "datetag",
    version,
    about = texts::ABOUT,
    long_about = None,
    after_help = format!("by {}", texts::AUTHORS),
    after_long_help = format!("{}\nby {}",texts::EXAMPLES, texts::AUTHORS),
)]

struct Args {
    /// Reference date, using today is not specified (e.g. 'yyyymmdd', 'yyyymm',
    /// 'yyyy', allowed field separators: '.-/:').
    #[arg(value_parser=utils::try_date_from_str)]
    date: Option<NaiveDate>,

    /// Tag type [d | m | y | daily | monthly | yearly]
    #[arg(value_enum, short, long, default_value_t = DateTag::M)]
    tag_type: DateTag,

    /// Date tag style
    #[arg(value_enum, short, long, default_value_t = DateStyle::Plain)]
    style: DateStyle,

    /// Tag prefix (e.g. 'LAB_202404')
    #[arg(short, long)]
    prefix: Option<String>,

    /// Tag suffix (e.g. '202404_rel')
    #[arg(short = 'x', long)]
    suffix: Option<String>,

    /// Date offset (offset unit depends on -t value)
    #[arg(short, long, allow_hyphen_values = true, default_value_t = 0)]
    offset: i32,

    /// Generate more date tags
    #[arg(short, long, value_parser=clap::value_parser!(u8).range(1..))]
    repeat: Option<u8>,

    /// Append an end-of-line to each generated tag
    #[arg(short, long, default_value_t = false)]
    new_line: bool,
}

fn main() -> Result<()> {
    // parse command-line parameters
    let args = Args::parse();

    // parse date related parameters
    let mut date = args
        .date
        .unwrap_or_else(|| Local::now().naive_local().date());

    // retrive date tag prefix label
    let prefix = args.prefix.unwrap_or_default();

    // retrive date tag suffix label
    let suffix = args.suffix.unwrap_or_default();

    // retrive repeat value
    let repeat = args.repeat.unwrap_or(1);

    // with no repetitions, apply offset immediately
    if repeat == 1 {
        // apply date offset
        date = utils::checked_add_offset(&date, args.offset, &args.tag_type)
            .with_context(|| "wrong date offset".to_string())?;
    }

    // generate date tags
    for _ in 1..=repeat {
        // display date tag
        print!(
            "{}{}{}",
            prefix,
            date.format(args.tag_type.get_format(args.style)),
            suffix
        );

        // append an end-of-line if requested or needed
        if args.new_line || repeat > 1 {
            println!();
        }

        // apply date offset for the next repetition
        date = utils::checked_add_offset(&date, args.offset, &args.tag_type)
            .with_context(|| "wrong date offset".to_string())?;
    }

    Ok(())
}
