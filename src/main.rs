use clap::{Parser};

#[derive(Parser, Debug)]
#[command(
version = "0.0.1",
author = "Elminson De Oleo Baez",
about = "Timestamp in Rust",
)]

struct Opts {

    #[clap(short, long, help = "Add the time to the current timestamp", default_value_t = 0)]
    add_time: i32,

    #[clap(short, long, help = "Set the format for timestamp rfc3339", default_value = "")]
    format: String,

    #[clap(short, long, help = "Set the unit to add seconds, minutes, hours, days", default_value = "")]
    unit: String,

}

fn main() {
    let mut opts = Opts::parse();
    if opts.add_time < 0 {
        eprintln!("Error: Time must be a positive integer");
        std::process::exit(1);
    }

    if opts.format.to_string() != "".to_string() && opts.format.to_string() != "rfc3339".to_string() {
        eprintln!("Error: Invalid format. Please use 'rfc3339' or leave it empty");
        std::process::exit(1);
    }
    match opts.unit.as_str() {
        "seconds" => {}
        "minutes" => {
            opts.add_time = opts.add_time * 60;
        }
        "hours" => {
            opts.add_time = opts.add_time * 3600;
        }
        "days" => {
            opts.add_time = opts.add_time * 86400;
        }
        _ => {
            eprintln!("Error: Invalid unit. Please use 'seconds', 'minutes', 'hours', or 'days'");
            std::process::exit(1);
        }
    }

    let now = chrono::Utc::now();
    let new_time = now + chrono::Duration::seconds(opts.add_time as i64);

    if opts.format.to_string() == "rfc3339".to_string() {

        println!("Current time: {}", now.to_rfc3339());
        println!("New time: {}", new_time.to_rfc3339());
    } else {
        println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S").to_string());
        println!("New time: {}", new_time.format("%Y-%m-%d %H:%M:%S").to_string());
    }
}
