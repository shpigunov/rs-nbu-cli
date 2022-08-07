mod currency;
mod nbu_api;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    currency: currency::Currency,
    amount: Option<f32>,
}

fn main() {
    let args = Cli::parse();

    if args.currency == currency::Currency::Eur {
        print!("EUR: ");
    } else if args.currency == currency::Currency::Usd {
        print!("USD: ");
    } else {
        println!("No currency");
    }

    let rate = nbu_api::get_rate(args.currency);
    println!("{:?}", rate);

    if let Some(amount) = args.amount {
        println!("= {:.2} UAH", amount * rate);
    }
}
