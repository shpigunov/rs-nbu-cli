mod currency;
mod nbu_api;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "nbu", version, about = "NBU currency helper")]
struct Cli {
    /// Currency code, e.g. USD, EUR
    #[arg(value_enum)]
    currency: currency::Currency,

    /// Amount to convert (optional). If omitted, use 1.0
    #[arg()]
    amount: Option<f32>,

    /// Pipe-friendly output
    #[arg(short = 'p', long = "pipe", default_value_t = false)]
    pipe_output: bool,
}

fn main() {
    let args = Cli::parse();

    let rate = nbu_api::get_rate(args.currency.clone());

    // Handle `pipe` mode with machine-readable output
    if args.pipe_output == true {
        println!("{}", rate)
    }
    // Handle normal mode with human-readable output
    else {
        println!(
            "1 {} = {} UAH",
            args.currency.to_string().to_uppercase(),
            rate
        );
        if let Some(amount) = args.amount {
            println!(
                "{} {} = {:.2} UAH",
                amount,
                args.currency.to_string().to_uppercase(),
                amount * rate
            );
        }
    }
}
