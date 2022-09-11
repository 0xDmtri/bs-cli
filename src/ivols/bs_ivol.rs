use crate::types::OptionType;

pub struct Ivol {
    option_type: OptionType,
    under: f64,
    strike: f64,
    tte: f64,
    rate: f64,
    price: f64,
}
