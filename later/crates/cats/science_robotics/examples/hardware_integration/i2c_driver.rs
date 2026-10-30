//! Generic I2C driver example using `embedded-hal` 1.0 traits.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use embedded_hal::i2c::I2c;
use embedded_hal_mock::eh1::i2c::Mock;
use embedded_hal_mock::eh1::i2c::Transaction;

/// Seven-bit address of the fictional teaching sensor.
const SENSOR_ADDR: u8 = 0x48;

/// Register containing the fictional sensor's device identifier.
const REG_DEVICE_ID: u8 = 0x00;

/// Register containing the fictional sensor's signed temperature value.
const REG_TEMP: u8 = 0x01;

/// Expected identifier returned by the fictional teaching sensor.
const DEVICE_ID: u8 = 0xA5;

/// Driver for the fictional sensor, generic over an `embedded-hal` I2C bus.
pub struct TemperatureSensor<I2C> {
    i2c: I2C,
}

impl<I2C, E> TemperatureSensor<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Creates a sensor driver around an I2C bus implementation.
    pub const fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Reads the fictional sensor's device identification byte.
    pub fn read_device_id(&mut self) -> Result<u8, E> {
        let mut buffer = [0_u8; 1];
        self.i2c
            .write_read(SENSOR_ADDR, &[REG_DEVICE_ID], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Reads signed, big-endian temperature data in degrees Celsius.
    pub fn read_temperature(&mut self) -> Result<f32, E> {
        let mut buffer = [0_u8; 2];
        self.i2c.write_read(SENSOR_ADDR, &[REG_TEMP], &mut buffer)?;

        // This fictional register stores signed big-endian units of 1/256 °C.
        #[allow(
            clippy::big_endian_bytes,
            reason = "the fictional sensor register is big-endian"
        )]
        let raw = i16::from_be_bytes(buffer);
        Ok(f32::from(raw) / 256.0)
    }

    /// Returns the owned I2C bus after the driver is no longer needed.
    pub fn into_inner(self) -> I2C {
        self.i2c
    }
}

/// Runs the strict-mock demonstration and checks both expected readings.
fn main() {
    let expectations = [
        Transaction::write_read(
            SENSOR_ADDR,
            vec![REG_DEVICE_ID],
            vec![DEVICE_ID],
        ),
        Transaction::write_read(
            SENSOR_ADDR,
            vec![REG_TEMP],
            vec![0x19, 0x00], // 0x1900 / 256 = 25 °C
        ),
    ];
    let bus = Mock::new(&expectations);
    let mut sensor = TemperatureSensor::new(bus);

    let id = sensor
        .read_device_id()
        .expect("fictional sensor device ID read succeeds");
    println!("Sensor Device ID: 0x{id:02X}");
    assert_eq!(id, DEVICE_ID);

    let temperature = sensor
        .read_temperature()
        .expect("fictional sensor temperature read succeeds");
    println!("Temperature: {temperature:.1} °C");
    assert!((temperature - 25.0).abs() < f32::EPSILON);
    sensor.into_inner().done();
}
// ANCHOR_END: example

/// Provides the example entry point to its parent runner.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use embedded_hal::i2c::ErrorKind;

    use super::*;

    /// Runs the example entry point and its strict transaction expectations.
    #[test]
    fn runs_the_example() {
        main();
    }

    /// Decodes a negative big-endian temperature using the fictional scale.
    #[test]
    fn decodes_negative_temperature() {
        let expectations = [Transaction::write_read(
            SENSOR_ADDR,
            vec![REG_TEMP],
            vec![0xF6, 0x80], // -2432 / 256 = -9.5 °C
        )];
        let mut sensor = TemperatureSensor::new(Mock::new(&expectations));

        let temperature = sensor
            .read_temperature()
            .expect("fictional sensor temperature read succeeds");

        assert!((temperature + 9.5).abs() < f32::EPSILON);
        sensor.into_inner().done();
    }

    /// Propagates an I2C error instead of returning a temperature value.
    #[test]
    fn propagates_bus_errors() {
        let expectations =
            [
                Transaction::write_read(
                    SENSOR_ADDR,
                    vec![REG_TEMP],
                    vec![0, 0],
                )
                .with_error(ErrorKind::Other),
            ];
        let mut sensor = TemperatureSensor::new(Mock::new(&expectations));

        assert_eq!(sensor.read_temperature(), Err(ErrorKind::Other));
        sensor.into_inner().done();
    }
}
