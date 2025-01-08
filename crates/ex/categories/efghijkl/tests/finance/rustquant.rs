// ANCHOR: example
use rustquant::models::black_scholes::BlackScholes;
use rustquant::prelude::*;
fn main() {
    // Parameters for the Black-Scholes model
    let spot_price = 100.0; // Current price of the underlying asset
    let strike_price = 105.0; // Strike price of the option
    let time_to_maturity = 0.5; // Time to maturity in years
    let volatility = 0.2; // Volatility of the underlying asset
    let risk_free_rate = 0.05; // Risk-free interest rate

    // Create a Black-Scholes model
    let bs = BlackScholes::new(
        spot_price,
        strike_price,
        time_to_maturity,
        volatility,
        risk_free_rate,
    );

    // Calculate the price of a European call option
    let call_price = bs.call_price();
    println!("European Call Option Price: {:.2}", call_price);

    // Calculate the price of a European put option
    let put_price = bs.put_price();
    println!("European Put Option Price: {:.2}", put_price);
}
// ANCHOR_END: example

#[test]
#[ignore = "not yet implemented"]
fn test() {
    main();
}
// [P2](https://github.com/john-cd/rust_howto/issues/764) review
