
use structopt::StructOpt;
use chrono::{DateTime, Local};

#[derive(Debug, StructOpt)]
#[structopt(name = "datetag", about = "display a custmizable date tag")]
struct Opt {
    /// date tag format
    #[structopt(short, long, default_value="%Y%m%d")]
    format: String,
}

fn main() {

    // parse command-line parameters
    let opt = Opt::from_args();

    let now: DateTime<Local> = Local::now();

    print!("{}", now.format(&opt.format))
}
