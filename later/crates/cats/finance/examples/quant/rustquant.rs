// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example: calculate Black-Scholes prices for European call and put options.
//!
//! This example is self-contained and does not depend on an external finance
//! crate.

/// The type of an option.
#[expect(dead_code, reason = "declared to illustrate it; never used")]
#[derive(Clone, Copy)]
enum OptionType {
    /// A call option.
    Call,
    /// A put option.
    Put,
}

/// The parameters of the Black-Scholes-Merton model.
struct BlackScholesMerton {
    /// The price of the underlying asset.
    spot_price: f64,
    /// The strike price of the option.
    strike_price: f64,
    /// The time to maturity in years.
    time_to_maturity: f64,
    /// The volatility of the underlying asset.
    volatility: f64,
    /// The risk-free interest rate.
    risk_free_rate: f64,
    /// The cost of carry.
    cost_of_carry: f64,
    /// The type of the option.
    #[expect(dead_code, reason = "declared to illustrate it; never used")]
    option_type: OptionType,
}

impl BlackScholesMerton {
    /// Creates the parameters of the model.
    fn new(
        spot_price: f64,
        strike_price: f64,
        time_to_maturity: f64,
        volatility: f64,
        risk_free_rate: f64,
        cost_of_carry: f64,
        option_type: OptionType,
    ) -> Self {
        Self {
            spot_price,
            strike_price,
            time_to_maturity,
            volatility,
            risk_free_rate,
            cost_of_carry,
            option_type,
        }
    }

    /// Returns the `d1` term of the model.
    fn d1(&self) -> f64 {
        let sigma_sq = self.volatility * self.volatility;
        (f64::ln(self.spot_price / self.strike_price)
            + (self.risk_free_rate - self.cost_of_carry + 0.5 * sigma_sq)
                * self.time_to_maturity)
            / (self.volatility * f64::sqrt(self.time_to_maturity))
    }

    /// Returns the `d2` term of the model.
    fn d2(&self) -> f64 {
        self.d1() - self.volatility * f64::sqrt(self.time_to_maturity)
    }

    /// Returns the cumulative distribution function of the standard normal
    /// distribution at `x`.
    fn norm_cdf(x: f64) -> f64 {
        // Abramowitz and Stegun approximation for the standard normal
        // cumulative distribution.
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let z = x.abs() / f64::sqrt(2.0);
        let t = 1.0 / (1.0 + p * z);
        let y = 1.0
            - (((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t)
                * f64::exp(-z * z);
        f64::midpoint(1.0, sign * y)
    }

    /// Returns the price of a European call option.
    fn call_price(&self) -> f64 {
        let d1 = self.d1();
        let d2 = self.d2();
        let discounted_spot = self.spot_price
            * f64::exp(-self.cost_of_carry * self.time_to_maturity);
        let discounted_strike = self.strike_price
            * f64::exp(-self.risk_free_rate * self.time_to_maturity);

        discounted_spot * Self::norm_cdf(d1)
            - discounted_strike * Self::norm_cdf(d2)
    }

    /// Returns the price of a European put option.
    fn put_price(&self) -> f64 {
        let d1 = self.d1();
        let d2 = self.d2();
        let discounted_spot = self.spot_price
            * f64::exp(-self.cost_of_carry * self.time_to_maturity);
        let discounted_strike = self.strike_price
            * f64::exp(-self.risk_free_rate * self.time_to_maturity);

        discounted_strike * Self::norm_cdf(-d2)
            - discounted_spot * Self::norm_cdf(-d1)
    }
}

/// Computes and prints the prices of a call and a put option.
fn main() {
    let spot_price = 100.0;
    let strike_price = 105.0;
    let time_to_maturity = 0.5;
    let volatility = 0.2;
    let risk_free_rate = 0.05;
    let cost_of_carry = 0.0;

    let bs = BlackScholesMerton::new(
        spot_price,
        strike_price,
        time_to_maturity,
        volatility,
        risk_free_rate,
        cost_of_carry,
        OptionType::Call,
    );

    let call_price = bs.call_price();
    let put_price = bs.put_price();

    println!("European Call Option Price: {call_price:.4}");
    println!("European Put Option Price: {put_price:.4}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rustquant example through its entry point.
    fn example_option() -> BlackScholesMerton {
        BlackScholesMerton::new(
            100.0,
            105.0,
            0.5,
            0.2,
            0.05,
            0.0,
            OptionType::Call,
        )
    }

    /// Runs the example through its entry point.
    #[test]
    fn test_rustquant_example_entry_point() {
        main();
    }

    #[test]
    fn put_call_parity_holds() {
        let option = example_option();
        let parity = option.spot_price
            * f64::exp(-option.cost_of_carry * option.time_to_maturity)
            - option.strike_price
                * f64::exp(-option.risk_free_rate * option.time_to_maturity);
        assert!(
            (option.call_price() - option.put_price() - parity).abs() < 1e-9
        );
    }

    #[test]
    fn prices_are_positive_and_respect_intrinsic_bounds() {
        let option = example_option();
        assert!(option.call_price() > 0.0);
        assert!(option.put_price() > 0.0);
        // The put is in the money at the strike of 105.
        assert!(
            option.put_price()
                > option.strike_price * f64::exp(-0.05 * 0.5) - 100.0
        );
    }

    #[test]
    fn normal_cdf_is_symmetric_and_centered() {
        assert!((BlackScholesMerton::norm_cdf(0.0) - 0.5).abs() < 1e-6);
        assert!(
            (BlackScholesMerton::norm_cdf(1.3)
                + BlackScholesMerton::norm_cdf(-1.3)
                - 1.0)
                .abs()
                < 1e-12
        );
    }
}
