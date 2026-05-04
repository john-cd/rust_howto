// ANCHOR: example
//! Example: calculate Black-Scholes prices for European call and put options.
//!
//! This example is self-contained and does not depend on an external finance
//! crate.

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum OptionType {
    Call,
    Put,
}

struct BlackScholesMerton {
    spot_price: f64,
    strike_price: f64,
    time_to_maturity: f64,
    volatility: f64,
    risk_free_rate: f64,
    cost_of_carry: f64,
    #[allow(dead_code)]
    option_type: OptionType,
}

impl BlackScholesMerton {
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

    fn d1(&self) -> f64 {
        let sigma_sq = self.volatility * self.volatility;
        (f64::ln(self.spot_price / self.strike_price)
            + (self.risk_free_rate - self.cost_of_carry + 0.5 * sigma_sq)
                * self.time_to_maturity)
            / (self.volatility * f64::sqrt(self.time_to_maturity))
    }

    fn d2(&self) -> f64 {
        self.d1() - self.volatility * f64::sqrt(self.time_to_maturity)
    }

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
        0.5 * (1.0 + sign * y)
    }

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

    println!("European Call Option Price: {:.4}", call_price);
    println!("European Put Option Price: {:.4}", put_price);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }

    // //! This example demonstrates how to use the `RustQuant` library to price
    // //! European call and put options using the Black-Scholes-Merton model.
    // //!
    // //! ## Prerequisites
    // //!
    // //! Ensure that the `RustQuant` library is added as a dependency in your
    // //! `Cargo.toml` file.

    // use RustQuant::prelude::*;

    // fn main() {
    //     // Parameters for the Black-Scholes model
    //     let spot_price = 100.0; // Current price of the underlying asset
    //     let strike_price = 105.0; // Strike price of the option
    //     let time_to_maturity = 0.5; // Time to maturity in years
    //     let volatility = 0.2; // Volatility of the underlying asset
    //     let risk_free_rate = 0.05; // Risk-free interest rate

    //     let cost_of_carry = 0.0; // Cost of carry (e.g., dividend yield)
    //     let underlying_price = spot_price;
    //     let evaluation_date = 0.0; // Assuming today is time 0
    //     let expiration_date = time_to_maturity;
    //     let option_type = OptionType::Call; // Example: Call option

    //     // Create a Black-Scholes-Merton model instance
    //     let bs = BlackScholesMerton::new(
    //         cost_of_carry,
    //         underlying_price,
    //         strike_price,
    //         volatility,
    //         risk_free_rate,
    //         evaluation_date,
    //         expiration_date,
    //         option_type,
    //     );

    //     // Calculate the price of a European call option
    //     let call_price = bs.call_price();
    //     println!("European Call Option Price: {:.2}", call_price);

    //     // Calculate the price of a European put option
    //     let put_price = bs.put_price();
    //     println!("European Put Option Price: {:.2}", put_price);
    // }

    // #[test]
    // fn test() {
    //     main();
    // }
    // // [finish; review](https://github.com/john-cd/rust_howto/issues/764)
}
