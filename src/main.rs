use bs_cli::pricers::Option;
use bs_cli::types::OptionType;
use clap::Parser;

fn main() {
    // get args from the CLI
    let args = Args::parse();

    // construct inputs to make an object from CLI
    let inputs = [
        args.under,
        args.strike,
        args.days / 360., // covert days to decimals
        args.rate,
        args.vola,
    ];

    // create Call and Put objects
    let call = Option::new(OptionType::Call, &inputs);
    let put = Option::new(OptionType::Put, &inputs);

    println!("---------------------------------------------------------------------------------------------");
    println!("Moneyness: {:.2}", call.moneyness()); // call and put are the same here
    println!("---------------------------------------------------------------------------------------------");
    println!("CALL | price: {:.2} | delta:  {:.2} | gamma: {:.6} | vega: {:.2} | theta: {:.3} | rho: {:.3}",
        call.price(), call.delta(), call.gamma(), call.vega(), call.theta(), call.rho());
    println!("PUT  | price: {:.2} | delta: {:.2} | gamma: {:.6} | vega: {:.2} | theta: {:.3} | rho: {:.3}",
        put.price(), put.delta(), put.gamma(), put.vega(), put.theta(), put.rho());
    println!("---------------------------------------------------------------------------------------------");
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    under: f64,
    #[clap(short, long, value_parser)]
    strike: f64,
    #[clap(short, long, value_parser)]
    days: f64,
    #[clap(short, long, value_parser)]
    rate: f64,
    #[clap(short, long, value_parser)]
    vola: f64,
}
