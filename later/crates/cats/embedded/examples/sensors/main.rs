//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use embedded_hal::i2c::ErrorType;
use embedded_hal::i2c::I2c;
use embedded_hal::i2c::Operation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BusError {
    UnexpectedTransaction,
}

impl core::fmt::Display for BusError {
    /// Formats the simulated bus error for diagnostics.
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(formatter, "the simulated sensor rejected the transaction")
    }
}

impl std::error::Error for BusError {}

impl embedded_hal::i2c::Error for BusError {
    /// Maps this example's error to the portable I2C error category.
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

#[derive(Default)]
struct MockI2c;

impl ErrorType for MockI2c {
    type Error = BusError;
}

impl I2c for MockI2c {
    /// Serves the simulated temperature register in a combined I2C transaction.
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        match operations {
            [Operation::Write(register), Operation::Read(bytes)]
                if address == 0x48
                    && *register == [0x00]
                    && bytes.len() == 2 =>
            {
                bytes.copy_from_slice(&215_i16.to_be_bytes());
                Ok(())
            }
            _ => Err(BusError::UnexpectedTransaction),
        }
    }
}

/// Reads the simulated sensor's signed temperature register in tenths of a
/// degree.
fn read_temperature_c<I2C: I2c>(bus: &mut I2C) -> Result<f32, I2C::Error> {
    let mut bytes = [0; 2];
    bus.write_read(0x48, &[0x00], &mut bytes)?;
    Ok(f32::from(i16::from_be_bytes(bytes)) / 10.0)
}

/// Reads and prints one measurement from a host-side simulated I2C sensor.
fn main() -> anyhow::Result<()> {
    run_example()
}

/// Reads and prints one measurement from the mock I2C sensor.
fn run_example() -> anyhow::Result<()> {
    let mut bus = MockI2c;
    let temperature = read_temperature_c(&mut bus)?;
    println!("temperature: {temperature:.1} °C");
    Ok(())
}
// ANCHOR_END: example

/// Runs the book example from the example crate's module runner.
pub fn run() -> anyhow::Result<()> {
    run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example entry point reads a measurement from the mock.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }

    /// Verifies register decoding without an I2C controller or attached sensor.
    #[test]
    fn reads_temperature_from_the_register_bytes() {
        let mut bus = MockI2c;
        assert_eq!(
            read_temperature_c(&mut bus).expect("the mock sensor responds"),
            21.5
        );
    }
}
