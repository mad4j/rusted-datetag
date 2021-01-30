use chrono::Local;
use chrono::{Datelike, NaiveDate};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "datetag", about = "display a customizable date tag")]
struct Opt {
    /// tag prefix
    #[structopt(short, long)]
    prefix: Option<String>,

    /// date tag format
    #[structopt(short, long, default_value = "%Y%m")]
    format: String,

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

fn main() {
    // parse command-line parameters
    let opt = Opt::from_args();

    // parse date related options
    let date: NaiveDate = match opt.date {
        Some(v) => NaiveDate::parse_from_str(&v, &opt.format).unwrap(),
        None => Local::now().naive_local().date(),
    };

    // parse offset related options
    let offset: i32 = opt.add.unwrap_or_default() as i32 - opt.sub.unwrap_or_default() as i32;

    // apply date offset
    let d = 12 * date.year() + offset;
    let date = NaiveDate::from_ymd(d / 12, (d as u32 % 12) + 1, 1);

    // display date tag
    print!(
        "{}{}",
        opt.prefix.unwrap_or_default(),
        date.format(&opt.format)
    )
}