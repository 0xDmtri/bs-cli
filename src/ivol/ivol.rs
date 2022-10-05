use crate::pricer::Option;
use crate::types::OptionType;

pub fn implied_vol(
    option_type: OptionType,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    target_price: f64,
) -> f64 {
    let mut high = 20.;
    let mut low = 0.;
    while (high - low) > 0.0001 {
        let params = [s, k, t, r, (high + low) / 2.0];
        let option = Option::new(option_type, &params);
        if option.price() > target_price {
            high = (high + low) / 2.;
        } else {
            low = (high + low) / 2.;
        }
    }
    let ivol = (high + low) / 2.;
    ivol
}
