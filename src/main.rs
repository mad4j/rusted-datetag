use chrono::{DateTime, Local};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "datetag", about = "display a customizable date tag")]
struct Opt {
    /// date tag format
    #[structopt(short, long, default_value = "%Y%m%d")]
    format: String,

    /// date value
    #[structopt(short, long)]
    date: Option<String>,
}

fn main() {
    // parse command-line parameters
    let opt = Opt::from_args();

    //let date: DateTime<Local> = Local::now();
    
    let date: DateTime<Local> = match opt.date {
        Some(v) => DateTime::parse_from_str(opt.date)?,
        None => Local::now()
    }
    

    print!(
        "{}",
        date.format(&opt.format)
    )
}
