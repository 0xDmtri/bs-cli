use crate::types::OptionType;
use std::f64::consts::PI;

pub struct Option {
    option_type: OptionType,
    under: f64,
    strike: f64,
    tte: f64,
    rate: f64,
    vola: f64,
}

impl Option {
    pub fn new(option_type: OptionType, params: &[f64; 5]) -> Self {
        Option {
            option_type: option_type,
            under: params[0],
            strike: params[1],
            tte: params[2],
            rate: params[3],
            vola: params[4],
        }
    }

    fn __phi(&self, x: f64) -> f64 {
        let part1 = -x * x / 2.0;
        let part2 = f64::sqrt(2.0 * PI);
        let value = f64::exp(part1) / part2;
        value
    }

    fn __call(&self) -> f64 {
        let call = self.under * self.cdf(self.d1(), 0., 1.)
            - self.strike * f64::exp(-self.rate * self.tte) * self.cdf(self.d2(), 0., 1.);
        call
    }

    fn __put(&self) -> f64 {
        let put = self.strike * f64::exp(-self.rate * self.tte) * self.cdf(-self.d2(), 0., 1.)
            - self.under * self.cdf(-self.d1(), 0., 1.);
        put
    }

    fn __call_delta(&self) -> f64 {
        let delta = self.cdf(self.d1(), 0., 1.);
        delta
    }

    fn __put_delta(&self) -> f64 {
        let delta = self.__call_delta() - 1.;
        delta
    }

    fn __call_theta(&self) -> f64 {
        let ct = -(self.under * self.vola * self.cdf(self.d1(), 0., 1.))
            / (2. * f64::sqrt(self.tte)
                - self.rate
                    * self.strike
                    * f64::exp(-self.rate * self.tte)
                    * self.cdf(self.d2(), 0., 1.));
        ct / 360.
    }

    fn __put_theta(&self) -> f64 {
        let ct = -(self.under * self.vola * self.cdf(self.d1(), 0., 1.))
            / (2. * f64::sqrt(self.tte)
                - self.rate
                    * self.strike
                    * f64::exp(-self.rate * self.tte)
                    * (1. - self.cdf(self.d2(), 0., 1.)));
        ct / 360.
    }

    fn __call_rho(&self) -> f64 {
        let call_rho = 0.01
            * self.under
            * self.tte
            * f64::exp(-self.rate * self.tte)
            * self.cdf(self.d1(), 0., 1.);
        call_rho
    }

    fn ___put_rho(&self) -> f64 {
        let call_rho = 0.01
            * self.under
            * self.tte
            * f64::exp(-self.rate * self.tte)
            * (1. - self.cdf(self.d1(), 0., 1.));
        call_rho
    }

    pub fn pdf(&self, x: f64, mu: f64, sigma: f64) -> f64 {
        let value = self.__phi((x - mu) / sigma) / sigma;
        value
    }

    pub fn phi(&self, z: f64) -> f64 {
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
            let value = 0.5 + total * self.__phi(z);
            value
        }
    }

    pub fn cdf(&self, z: f64, mu: f64, sigma: f64) -> f64 {
        let value = self.phi((z - mu) / sigma);
        value
    }

    pub fn d1(&self) -> f64 {
        let sd1 = (f64::ln(self.under / self.strike)
            + (self.rate + 0.5 * self.vola * self.vola) * self.tte)
            / (self.vola * (f64::sqrt(self.tte)));
        sd1
    }

    pub fn d2(&self) -> f64 {
        let d2 = self.d1() - self.vola * f64::sqrt(self.tte);
        d2
    }

    pub fn price(&self) -> f64 {
        match self.option_type {
            OptionType::Call => self.__call(),
            OptionType::Put => self.__put(),
        }
    }

    pub fn delta(&self) -> f64 {
        match self.option_type {
            OptionType::Call => self.__call_delta(),
            OptionType::Put => self.__put_delta(),
        }
    }

    pub fn theta(&self) -> f64 {
        match self.option_type {
            OptionType::Call => self.__call_theta(),
            OptionType::Put => self.__put_theta(),
        }
    }

    pub fn gamma(&self) -> f64 {
        let gamma = self.cdf(self.d1(), 0., 1.) / (self.under * (self.vola * f64::sqrt(self.tte)));
        gamma
    }

    pub fn vega(&self) -> f64 {
        let vega = 0.01 * self.under * f64::sqrt(self.tte) * self.cdf(self.d1(), 0., 1.);
        vega
    }

    pub fn rho(&self) -> f64 {
        match self.option_type {
            OptionType::Call => self.__call_rho(),
            OptionType::Put => self.___put_rho(),
        }
    }

    pub fn moneyness(&self) -> f64 {
        let norm_strk =
            (f64::ln(self.strike) - f64::ln(self.under)) / (self.vola * f64::sqrt(self.tte));
        norm_strk
    }
}

// pub fn implied_vol_call(s: f64, k: f64, t: f64, r: f64, target: f64) -> f64 {
//     let mut high = 20.;
//     let mut low = 0.;
//     while (high - low) > 0.0001 {
//         let option = Option {
//             under: s,
//             strike: k,
//             days: t,
//             rate: r,
//             vol: (high + low) / 2.,
//         };
//         if call(&option) > target {
//             high = (high + low) / 2.;
//         } else {
//             low = (high + low) / 2.;
//         }
//     }
//     let ivol = (high + low) / 2.;
//     ivol
// }

// pub fn implied_vol_put(s: f64, k: f64, t: f64, r: f64, target: f64) -> f64 {
//     let mut high = 20.;
//     let mut low = 0.;
//     while (high - low) > 0.0001 {
//         let option = Option {
//             under: s,
//             strike: k,
//             days: t,
//             rate: r,
//             vol: (high + low) / 2.,
//         };
//         if put(&option) > target {
//             high = (high + low) / 2.;
//         } else {
//             low = (high + low) / 2.;
//         }
//     }
//     let ivol = (high + low) / 2.;
//     ivol
// }

// #[cfg(test)]
// mod tests {
//     use crate::implied_vol_call;
//     use crate::implied_vol_put;

//     #[test]
//     fn test_call_ivol() {
//         let result = implied_vol_call(20000., 20000., 7. / 360., 0.03, 673.);
//         assert_eq!(result, 0.6000137329101563);
//     }

//     #[test]
//     fn test_put_ivol() {
//         let result = implied_vol_put(20000., 20000., 7. / 360., 0.03, 661.);
//         assert_eq!(result, 0.5997085571289063);
//     }
// }
