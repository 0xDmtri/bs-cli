use std::{env, f64::consts::PI, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // get inputs from CLI args
    let inputs: Option = Option::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // call funcs
    let call_price = call(&inputs);
    let put_price = put(&inputs);

    // print output
    println!("Call price: {call_price}");
    println!("Put price: {put_price}");
}

pub struct Option {
    under: f64,
    strike: f64,
    days: f64,
    rate: f64,
    vol: f64,
}

impl Option {
    fn new(args: &[String]) -> Result<Option, &str> {
        if args.len() != 6 {
            println!("{args:#?}");
            return Err::<Option, &str>("Wrong number of arguments!");
        }

        let under = args[1].parse::<f64>().unwrap();
        let strike = args[2].parse::<f64>().unwrap();
        let tte = args[3].parse::<f64>().unwrap() / 365.;
        let rate = args[4].parse::<f64>().unwrap();
        let vol = args[5].parse::<f64>().unwrap();

        Ok(Option {
            under,
            strike,
            days: tte,
            rate,
            vol,
        })
    }
}

pub fn phi__(x: f64) -> f64 {
    let part1 = -x * x / 2.0;
    let part2 = f64::sqrt(2.0 * PI);
    let value = f64::exp(part1) / part2;
    value
}

pub fn pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let value = phi__((x - mu) / sigma) / sigma;
    value
}

pub fn phi(z: f64) -> f64 {
    if z < -8.0 {
        0.0
    } else if z > 8.0 {
        1.0
    } else {
        let mut total = 0.0;
        let mut term = z;
        let mut i = 3.0;
        while total != total + term {
            total += term;
            term *= z * z / i;
            i += 2.0;
        }
        let value = 0.5 + total * phi__(z);
        value
    }
}

pub fn cdf(z: f64, mu: f64, sigma: f64) -> f64 {
    let value = phi((z - mu) / sigma);
    value
}

pub fn d1(s: f64, k: f64, t: f64, r: f64, vol: f64) -> f64 {
    let sd1 = (f64::ln(s / k) + (r + 0.5 * vol * vol) * t) / (vol * (f64::sqrt(t)));
    sd1
}

pub fn d2(s: f64, k: f64, t: f64, r: f64, vol: f64) -> f64 {
    let d2 = d1(s, k, t, r, vol) - vol * f64::sqrt(t);
    d2
}

pub fn call(option: &Option) -> f64 {
    let call = option.under
        * cdf(
            d1(
                option.under,
                option.strike,
                option.days,
                option.rate,
                option.vol,
            ),
            0.,
            1.,
        )
        - option.strike
            * f64::exp(-option.rate * option.days)
            * cdf(
                d2(
                    option.under,
                    option.strike,
                    option.days,
                    option.rate,
                    option.vol,
                ),
                0.,
                1.,
            );
    call
}

pub fn put(option: &Option) -> f64 {
    let put = option.strike
        * f64::exp(-option.rate * option.days)
        * cdf(
            -d2(
                option.under,
                option.strike,
                option.days,
                option.rate,
                option.vol,
            ),
            0.,
            1.,
        )
        - option.under
            * cdf(
                -d1(
                    option.under,
                    option.strike,
                    option.days,
                    option.rate,
                    option.vol,
                ),
                0.,
                1.,
            );
    put
}
