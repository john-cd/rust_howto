//! A two-register virtual machine with a bounded fetch/decode/execute loop.

use core::fmt;

/// A malformed instruction, invalid address, or exhausted execution budget.
#[derive(Debug, PartialEq, Eq)]
enum EmuError {
    /// The instruction pointer is outside the loaded program.
    InvalidAddress(usize),
    /// No instruction matches this opcode and operand pair.
    UnknownInstruction([u8; 2]),
    /// The program failed to halt within the host's instruction budget.
    StepLimit,
}

impl fmt::Display for EmuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAddress(pc) => {
                write!(f, "invalid instruction address {pc}")
            }
            Self::UnknownInstruction(bytes) => {
                write!(f, "unknown instruction {bytes:02x?}")
            }
            Self::StepLimit => {
                f.write_str("program exceeded the instruction budget")
            }
        }
    }
}

impl core::error::Error for EmuError {}

// ANCHOR: example
/// CPU state; each instruction is an opcode byte followed by an operand byte.
struct Cpu<'a> {
    /// Borrowed, read-only program memory.
    program: &'a [[u8; 2]],
    /// Index of the next instruction, rather than a byte offset.
    pc: usize,
    /// Eight-bit accumulator (r0) and loop counter (r1).
    registers: [u8; 2],
    /// Whether the halt instruction has executed.
    halted: bool,
}

impl Cpu<'_> {
    /// Fetches, decodes, and executes one instruction; halted CPUs do nothing.
    ///
    /// # Errors
    /// Returns an error for an invalid address or unknown instruction.
    fn step(&mut self) -> Result<(), EmuError> {
        if self.halted {
            return Ok(());
        }
        let instruction = *self
            .program
            .get(self.pc)
            .ok_or(EmuError::InvalidAddress(self.pc))?;
        let mut next = self.pc + 1;
        match instruction {
            [0x00, 0] => self.halted = true,
            [0x10, value] => self.registers[0] = value,
            [0x11, value] => self.registers[1] = value,
            [0x20, 0] => {
                self.registers[0] =
                    self.registers[0].wrapping_add(self.registers[1])
            }
            [0x30, 0] => self.registers[1] = self.registers[1].wrapping_sub(1),
            [0x40, address] => {
                if self.registers[1] != 0 {
                    next = usize::from(address);
                    if next >= self.program.len() {
                        return Err(EmuError::InvalidAddress(next));
                    }
                }
            }
            _ => return Err(EmuError::UnknownInstruction(instruction)),
        }
        self.pc = next;
        Ok(())
    }
}
// ANCHOR_END: example

/// Runs a program on a new Cpu up to `step_limit` steps or until halted.
fn run_program<'a>(program: &'a [[u8; 2]], step_limit: usize) -> Result<Cpu<'a>, EmuError> {
    let mut cpu = Cpu {
        program,
        pc: 0,
        registers: [0; 2],
        halted: false,
    };
    for _ in 0..step_limit {
        cpu.step()?;
        if cpu.halted {
            return Ok(cpu);
        }
    }
    Err(EmuError::StepLimit)
}

/// Sums five down to one and stops after at most 64 guest instructions.
///
/// # Errors
/// Propagates malformed instructions, invalid addresses, and budget exhaustion.
fn main() -> Result<(), EmuError> {
    // r0 = 0; r1 = 5; loop: r0 += r1; r1 -= 1; jump to loop if r1 != 0; halt.
    let program = [
        [0x10, 0],
        [0x11, 5],
        [0x20, 0],
        [0x30, 0],
        [0x40, 2],
        [0x00, 0],
    ];
    let cpu = run_program(&program, 64)?;
    println!("Sum of 1..=5: {}", cpu.registers[0]);
    Ok(())
}

/// Runs the self-contained virtual machine without a guest file.
///
/// # Errors
/// Propagates guest instruction and execution-budget errors.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(main()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }

    #[test]
    fn test_unknown_instruction_error() {
        let program = [[0xFF, 0x00]];
        let mut cpu = Cpu {
            program: &program,
            pc: 0,
            registers: [0; 2],
            halted: false,
        };
        assert_eq!(cpu.step(), Err(EmuError::UnknownInstruction([0xFF, 0x00])));
    }

    #[test]
    fn test_invalid_address_error() {
        let program = [[0x40, 0x05]]; // jump to 5 (out of bounds) when r1 != 0
        let mut cpu = Cpu {
            program: &program,
            pc: 0,
            registers: [0, 1],
            halted: false,
        };
        assert_eq!(cpu.step(), Err(EmuError::InvalidAddress(5)));
    }

    #[test]
    fn test_step_limit_error() {
        let program = [[0x40, 0x00]]; // infinite jump loop to 0
        let res = run_program(&program, 10);
        assert_eq!(res, Err(EmuError::StepLimit));
    }
}
