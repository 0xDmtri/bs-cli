use clap::Parser;
use std::f64::consts::PI;

fn main() {
    // get args from the CLI
    let mut args = Option::parse();

    // covert days to decimals
    args.days = args.days / 360.;

    // call funcs
    let call_price = call(&args);
    let put_price = put(&args);

    // print output
    println!("Call price: {call_price}");
    println!("Put price: {put_price}");
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Option {
    #[clap(short, long, value_parser)]
    under: f64,
    #[clap(short, long, value_parser)]
    strike: f64,
    #[clap(short, long, value_parser)]
    days: f64,
    #[clap(short, long, value_parser)]
    rate: f64,
    #[clap(short, long, value_parser)]
    vol: f64,
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

pub fn d1(option: &Option) -> f64 {
    let sd1 = (f64::ln(option.under / option.strike)
        + (option.rate + 0.5 * option.vol * option.vol) * option.days)
        / (option.vol * (f64::sqrt(option.days)));
    sd1
}

pub fn d2(option: &Option) -> f64 {
    let d2 = d1(option) - option.vol * f64::sqrt(option.days);
    d2
}

pub fn call(option: &Option) -> f64 {
    let call = option.under * cdf(d1(option), 0., 1.)
        - option.strike * f64::exp(-option.rate * option.days) * cdf(d2(option), 0., 1.);
    call
}

pub fn put(option: &Option) -> f64 {
    let put = option.strike * f64::exp(-option.rate * option.days) * cdf(-d2(option), 0., 1.)
        - option.under * cdf(-d1(option), 0., 1.);
    put
}

pub fn get_call_delta(option: &Option) -> f64 {
    let delta = cdf(d1(option), 0., 1.);
    delta
}

pub fn get_put_delta(option: &Option) -> f64 {
    let delta = get_call_delta(option) - 1.;
    delta
}

pub fn get_call_theta(option: &Option) -> f64 {
    let ct = -(option.under * option.vol * cdf(d1(option), 0., 1.))
        / (2. * f64::sqrt(option.days)
            - option.rate
                * option.strike
                * f64::exp(-option.rate * option.days)
                * cdf(d2(option), 0., 1.));
    ct / 360.
}

pub fn get_put_theta(option: &Option) -> f64 {
    let ct = -(option.under * option.vol * cdf(d1(option), 0., 1.))
        / (2. * f64::sqrt(option.days)
            - option.rate
                * option.strike
                * f64::exp(-option.rate * option.days)
                * (1. - cdf(d2(option), 0., 1.)));
    ct / 360.
}

pub fn get_gamma(option: &Option) -> f64 {
    let gamma = cdf(d1(option), 0., 1.) / (option.under * (option.vol * f64::sqrt(option.days)));
    gamma
}

pub fn get_vega(option: &Option) -> f64 {
    let vega = 0.01 * option.under * f64::sqrt(option.days) * cdf(d1(option), 0., 1.);
    vega
}

pub fn get_call_rho(option: &Option) -> f64 {
    let call_rho = 0.01
        * option.under
        * option.days
        * f64::exp(-option.rate * option.days)
        * cdf(d1(option), 0., 1.);
    call_rho
}

pub fn get_put_rho(option: &Option) -> f64 {
    let call_rho = 0.01
        * option.under
        * option.days
        * f64::exp(-option.rate * option.days)
        * (1. - cdf(d1(option), 0., 1.));
    call_rho
}

pub fn implied_vol_call(s: f64, k: f64, t: f64, r: f64, target: f64) -> f64 {
    let mut high = 20.;
    let mut low = 0.;
    while (high - low) > 0.0001 {
        let option = Option {
            under: s,
            strike: k,
            days: t,
            rate: r,
            vol: (high + low) / 2.,
        };
        if call(&option) > target {
            high = (high + low) / 2.;
        } else {
            low = (high + low) / 2.;
        }
    }
    let ivol = (high + low) / 2.;
    ivol
}

pub fn implied_vol_put(s: f64, k: f64, t: f64, r: f64, target: f64) -> f64 {
    let mut high = 20.;
    let mut low = 0.;
    while (high - low) > 0.0001 {
        let option = Option {
            under: s,
            strike: k,
            days: t,
            rate: r,
            vol: (high + low) / 2.,
        };
        if put(&option) > target {
            high = (high + low) / 2.;
        } else {
            low = (high + low) / 2.;
        }
    }
    let ivol = (high + low) / 2.;
    ivol
}

pub fn moneyness(option: &Option) -> f64 {
    let norm_strk =
        (f64::ln(option.strike) - f64::ln(option.under)) / (option.vol * f64::sqrt(option.days));
    norm_strk
}

#[cfg(test)]
mod tests {
    use crate::implied_vol_call;

    #[test]
    fn test_call_ivol() {
        let result = implied_vol_call(20000., 20000., 7. / 360., 0.03, 673.);
        assert_eq!(result, 0.6000137329101563);
    }

    #[test]
    fn test_put_ivol() {
        use crate::implied_vol_put;

        let result = implied_vol_put(20000., 20000., 7. / 360., 0.03, 661.);
        assert_eq!(result, 0.5997085571289063);
    }
}
