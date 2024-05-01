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
mod utils;

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use clap::Parser;

use datestyle::DateStyle;
use datetag::DateTagType;

#[derive(Debug, Parser)]
#[command(
    name = "datetag",
    version,
    about = r#"Display a customizable date tag (e.g. TEST_202404)

EXAMPLES:
 
    $ datetag --offset 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -o 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403"#
)]

struct Opt {
    /// Tag type [d | m | y | daily | monthly | yearly]
    #[arg(value_enum, short, long, default_value_t = DateTagType::M)]
    tag_type: DateTagType,

    /// Date tag style
    #[arg(value_enum, short, long, default_value_t = DateStyle::Plain)]
    style: DateStyle,

    /// Tag prefix (e.g. 'LAB_202404')
    #[arg(short, long)]
    prefix: Option<String>,

    /// Tag suffix (e.g. '202404_rel')
    #[arg(short = 'x', long)]
    suffix: Option<String>,

    /// Date value (one of 'yyyymmdd', 'yyyymm', 'yyyy')
    #[arg(short, long, value_parser=utils::try_date_from_str)]
    date: Option<NaiveDate>,

    /// Date offset (offset unit depends on -t value)
    #[arg(short, long, allow_hyphen_values = true, default_value_t = 0)]
    offset: i32,

    /// Generate more date tags
    #[arg(short, long, default_value_t = 1)]
    repeat: u8,

    /// Append an end-of-line to each generated tag
    #[arg(short, long, default_value_t = false)]
    new_line: bool,

    /// Print this help as markdown document
    #[arg(long)]
    markdown_help: bool,
}

fn main() -> Result<()> {
    // parse command-line parameters
    let opt = Opt::parse();

    // dump markdown help and exit
    if opt.markdown_help {
        clap_markdown::print_help_markdown::<Opt>();
        return Ok(());
    }

    // parse date related parameters
    let mut date = opt
        .date
        .unwrap_or_else(|| Local::now().naive_local().date());

    // retrive date tag prefix label
    let prefix = opt.prefix.unwrap_or_default();

    // retrive date tag suffix label
    let suffix = opt.suffix.unwrap_or_default();

    // with no repetitions, apply offset immediately
    if opt.repeat == 1 {
        // apply date offset
        date = utils::checked_add_offset(&date, opt.offset, &opt.tag_type)
            .with_context(|| "wrong date offset".to_string())?;
    }

    // generate date tags
    for _ in 1..=opt.repeat {
        // display date tag
        print!(
            "{}{}{}",
            prefix,
            date.format(opt.tag_type.get_format(opt.style)),
            suffix
        );

        // append an end-of-line if requested or needed
        if opt.new_line || opt.repeat > 1 {
            println!();
        }

        // apply date offset for the next repetition
        date = utils::checked_add_offset(&date, opt.offset, &opt.tag_type)
            .with_context(|| "wrong date offset".to_string())?;
    }

    Ok(())
}
