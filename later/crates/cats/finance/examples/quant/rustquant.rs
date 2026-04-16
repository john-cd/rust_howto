// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
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
