use std::fmt::Display;
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TaskError {
    Interrupted,
    Other,
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Just call `Debug`.
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

impl std::error::Error for TaskError {}
