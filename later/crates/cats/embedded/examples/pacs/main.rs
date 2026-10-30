//! Models the register block and ownership token of a peripheral access crate.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use core::cell::UnsafeCell;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

/// One 32-bit memory-mapped register, as `svd2rust` models it.
///
/// A real PAC places the block at a fixed address from the reference manual;
/// this host-side model keeps it in ordinary memory instead.
struct Register(UnsafeCell<u32>);

impl Register {
    const fn new(reset_value: u32) -> Self {
        Self(UnsafeCell::new(reset_value))
    }

    /// Reads the register. Volatile access keeps the compiler from caching
    /// or removing reads of hardware state.
    fn read(&self) -> u32 {
        // SAFETY: the pointer comes from a live `UnsafeCell<u32>`.
        unsafe { self.0.get().read_volatile() }
    }

    /// Writes the register.
    fn write(&self, value: u32) {
        // SAFETY: the pointer comes from a live `UnsafeCell<u32>`.
        unsafe { self.0.get().write_volatile(value) }
    }

    /// Reads, transforms, and writes back: the PAC's `modify` pattern.
    fn modify(&self, change: impl FnOnce(u32) -> u32) {
        self.write(change(self.read()));
    }
}

/// The GPIO port's registers, in the order and width the manual lists them.
#[repr(C)]
struct GpioRegisters {
    /// Mode register: two bits per pin; `0b01` selects output.
    mode: Register,
    /// Output data register: one bit per pin.
    output: Register,
}

/// Ownership token for the GPIO port. Only one can exist.
struct Gpio {
    registers: GpioRegisters,
}

/// All peripherals of the chip, handed out once, like a PAC's `Peripherals`.
struct Peripherals {
    gpio: Gpio,
}

impl Peripherals {
    /// Returns the peripherals the first time it is called, then `None`.
    fn take() -> Option<Self> {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        Self::take_once(&TAKEN)
    }

    /// Hands out the peripherals unless `taken` shows that they already were.
    fn take_once(taken: &AtomicBool) -> Option<Self> {
        if taken.swap(true, Ordering::AcqRel) {
            return None;
        }
        Some(Self {
            gpio: Gpio {
                registers: GpioRegisters {
                    mode: Register::new(0),
                    output: Register::new(0),
                },
            },
        })
    }
}

/// Configures pin 5 as an output and drives it high.
fn turn_on_led(gpio: &Gpio) {
    // Replace the two mode bits of pin 5 without touching the other pins.
    gpio.registers
        .mode
        .modify(|mode| (mode & !(0b11 << 10)) | (0b01 << 10));
    gpio.registers.output.modify(|output| output | (1 << 5));
}

/// Takes the peripherals, drives a pin, and shows that a second take fails.
fn main() {
    let peripherals = Peripherals::take().expect("first take succeeds");
    turn_on_led(&peripherals.gpio);
    println!(
        "mode = {:#010b}, output = {:#010b}",
        peripherals.gpio.registers.mode.read(),
        peripherals.gpio.registers.output.read()
    );
    println!("second take: {}", Peripherals::take().is_none());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main();
    }

    /// Verifies that only the first take returns the peripherals.
    #[test]
    fn peripherals_can_be_taken_once() {
        let taken = AtomicBool::new(false);
        assert!(Peripherals::take_once(&taken).is_some());
        assert!(Peripherals::take_once(&taken).is_none());
    }

    /// Verifies that configuring one pin leaves the others untouched.
    #[test]
    fn modify_preserves_other_pins() {
        let taken = AtomicBool::new(false);
        let peripherals =
            Peripherals::take_once(&taken).expect("first take succeeds");
        peripherals.gpio.registers.mode.write(0b11 << 2);
        turn_on_led(&peripherals.gpio);
        assert_eq!(
            peripherals.gpio.registers.mode.read(),
            (0b11 << 2) | (0b01 << 10)
        );
        assert_eq!(peripherals.gpio.registers.output.read(), 1 << 5);
    }
}
