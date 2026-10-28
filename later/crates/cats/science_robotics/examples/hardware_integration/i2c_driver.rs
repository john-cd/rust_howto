//! Generic I2C driver example using `embedded-hal` 1.0 traits.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use embedded_hal::i2c::I2c;

/// Sensor I2C address (7-bit address 0x48 for a typical temperature sensor).
const SENSOR_ADDR: u8 = 0x48;

/// Register address for the device identification register.
const REG_DEVICE_ID: u8 = 0x00;

/// Register address for the temperature measurement data.
const REG_TEMP: u8 = 0x01;

/// Driver for a hardware temperature sensor generic over an `embedded-hal` I2C
/// bus.
pub struct TemperatureSensor<I2C> {
    i2c: I2C,
}

impl<I2C, E> TemperatureSensor<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Creates a new driver wrapping the provided I2C peripheral interface.
    pub const fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Reads the device identification byte from the sensor.
    pub fn read_device_id(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(SENSOR_ADDR, &[REG_DEVICE_ID], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Reads the current temperature in degrees Celsius.
    pub fn read_temperature(&mut self) -> Result<f32, E> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(SENSOR_ADDR, &[REG_TEMP], &mut buffer)?;

        // Convert big-endian 16-bit raw integer to degrees Celsius.
        let raw = i16::from_be_bytes(buffer);
        Ok(f32::from(raw) / 256.0)
    }
}

/// Demonstrates using the generic I2C driver with a mock I2C implementation.
fn main() {
    let mock_bus = MockI2cBus {
        device_id: 0xA5,
        temp_raw: [0x19, 0x00], // 0x1900 = 6400 raw units = 25.0 °C
    };

    let mut sensor = TemperatureSensor::new(mock_bus);

    let id = sensor
        .read_device_id()
        .expect("Failed to read device ID from sensor.");
    println!("Sensor Device ID: 0x{id:02X}");
    assert_eq!(id, 0xA5);

    let temperature = sensor
        .read_temperature()
        .expect("Failed to read temperature from sensor.");
    println!("Temperature: {temperature:.1} °C");
    assert!((temperature - 25.0).abs() < f32::EPSILON);
}
// ANCHOR_END: example

/// A mock I2C bus implementation for host-side testing without physical
/// hardware.
struct MockI2cBus {
    device_id: u8,
    temp_raw: [u8; 2],
}

#[derive(Debug)]
struct MockError;

impl embedded_hal::i2c::Error for MockError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

impl embedded_hal::i2c::ErrorType for MockI2cBus {
    type Error = MockError;
}

impl embedded_hal::i2c::I2c for MockI2cBus {
    fn read(
        &mut self,
        _address: u8,
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(
        &mut self,
        _address: u8,
        _write: &[u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write_read(
        &mut self,
        _address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        if write == [REG_DEVICE_ID] && read.len() == 1 {
            read[0] = self.device_id;
        } else if write == [REG_TEMP] && read.len() == 2 {
            read.copy_from_slice(&self.temp_raw);
        }
        Ok(())
    }

    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Entry point helper for unit testing and execution.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i2c_driver() {
        main();
    }
}
