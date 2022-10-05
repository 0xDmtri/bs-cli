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

    fn __ph1(&self, x: f64) -> f64 {
        let part1 = -x * x / 2.0;
        let part2 = f64::sqrt(2.0 * PI);
        let value = f64::exp(part1) / part2;
        value
    }

    fn __call(&self) -> f64 {
        let call = self.under * self.__cdf(self.__d1(), 0., 1.)
            - self.strike * f64::exp(-self.rate * self.tte) * self.__cdf(self.__d2(), 0., 1.);
        call
    }

    fn __put(&self) -> f64 {
        let put = self.strike * f64::exp(-self.rate * self.tte) * self.__cdf(-self.__d2(), 0., 1.)
            - self.under * self.__cdf(-self.__d1(), 0., 1.);
        put
    }

    fn __call_delta(&self) -> f64 {
        let delta = self.__cdf(self.__d1(), 0., 1.);
        delta
    }

    fn __put_delta(&self) -> f64 {
        let delta = self.__call_delta() - 1.;
        delta
    }

    fn __call_theta(&self) -> f64 {
        let ct = -(self.under * self.vola * self.__cdf(self.__d1(), 0., 1.))
            / (2. * f64::sqrt(self.tte)
                - self.rate
                    * self.strike
                    * f64::exp(-self.rate * self.tte)
                    * self.__cdf(self.__d2(), 0., 1.));
        ct / 360.
    }

    fn __put_theta(&self) -> f64 {
        let ct = -(self.under * self.vola * self.__cdf(self.__d1(), 0., 1.))
            / (2. * f64::sqrt(self.tte)
                - self.rate
                    * self.strike
                    * f64::exp(-self.rate * self.tte)
                    * (1. - self.__cdf(self.__d2(), 0., 1.)));
        ct / 360.
    }

    fn __call_rho(&self) -> f64 {
        let call_rho = 0.01
            * self.under
            * self.tte
            * f64::exp(-self.rate * self.tte)
            * self.__cdf(self.__d1(), 0., 1.);
        call_rho
    }

    fn ___put_rho(&self) -> f64 {
        let call_rho = 0.01
            * self.under
            * self.tte
            * f64::exp(-self.rate * self.tte)
            * (1. - self.__cdf(self.__d1(), 0., 1.));
        call_rho
    }

    fn __pdf(&self, x: f64, mu: f64, sigma: f64) -> f64 {
        let value = self.__ph1((x - mu) / sigma) / sigma;
        value
    }

    fn __phi2(&self, z: f64) -> f64 {
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
            let value = 0.5 + total * self.__ph1(z);
            value
        }
    }

    fn __cdf(&self, z: f64, mu: f64, sigma: f64) -> f64 {
        let value = self.__phi2((z - mu) / sigma);
        value
    }

    fn __d1(&self) -> f64 {
        let sd1 = (f64::ln(self.under / self.strike)
            + (self.rate + 0.5 * self.vola * self.vola) * self.tte)
            / (self.vola * (f64::sqrt(self.tte)));
        sd1
    }

    fn __d2(&self) -> f64 {
        let d2 = self.__d1() - self.vola * f64::sqrt(self.tte);
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
        let gamma =
            self.__cdf(self.__d1(), 0., 1.) / (self.under * (self.vola * f64::sqrt(self.tte)));
        gamma
    }

    pub fn vega(&self) -> f64 {
        let vega = 0.01 * self.under * f64::sqrt(self.tte) * self.__cdf(self.__d1(), 0., 1.);
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

#[cfg(test)]
mod tests {
    use crate::pricer::Option;
    use crate::types::OptionType;

    #[test]
    fn test_prices() {
        let params = [100., 100., 7. / 360., 0.03, 0.80];

        let call = Option::new(OptionType::Call, &params);
        let put = Option::new(OptionType::Put, &params);

        let call_delta = call.delta();
        let call_gamma = call.gamma();
        let call_vega = call.vega();
        let call_theta = call.theta();
        let call_rho = call.rho();
        let call_moneyness = call.moneyness();

        let put_delta = put.delta();
        let put_gamma = put.gamma();
        let put_vega = put.vega();
        let put_theta = put.theta();
        let put_rho = put.rho();
        let put_moneyness = put.moneyness();

        assert_eq!(call_delta, 1.0 + put_delta);
        assert_eq!(call_gamma, put_gamma);
        assert_eq!(call_vega, put_vega);
        assert_eq!(call_moneyness, put_moneyness);

        assert_eq!(call_theta, 0.10046214845877008);
        assert_eq!(put_theta, 0.09098031400937832);
        assert_eq!(call_rho, 0.010189223373781598);
        assert_eq!(put_rho, 0.009243881785683248)
    }
}
