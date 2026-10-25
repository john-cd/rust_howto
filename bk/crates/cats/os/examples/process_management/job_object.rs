#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Make a Windows child die with its parent by using a Job Object.

use std::mem;
use std::os::windows::io::AsRawHandle as _;
use std::process::Child;
use std::process::Command;

use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;
use windows_sys::Win32::System::JobObjects::CreateJobObjectW;
use windows_sys::Win32::System::JobObjects::JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
use windows_sys::Win32::System::JobObjects::JOBOBJECT_EXTENDED_LIMIT_INFORMATION;
use windows_sys::Win32::System::JobObjects::JobObjectExtendedLimitInformation;
use windows_sys::Win32::System::JobObjects::SetInformationJobObject;

/// A Job Object that terminates every process assigned to it when it closes.
struct KillOnCloseJob(HANDLE);

impl KillOnCloseJob {
    /// Creates an unnamed job with the "kill on close" limit.
    fn new() -> std::io::Result<Self> {
        // SAFETY: both arguments may be null; the call returns a new handle.
        let handle =
            unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if handle.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        // From here on, `Drop` closes the handle if anything fails.
        let job = Self(handle);

        // SAFETY: an all-zero value is a valid initial state for this struct.
        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION =
            unsafe { mem::zeroed() };
        info.BasicLimitInformation.LimitFlags =
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        let size = u32::try_from(mem::size_of_val(&info))
            .map_err(std::io::Error::other)?;
        // SAFETY: `info` is valid for the size passed.
        let ok = unsafe {
            SetInformationJobObject(
                job.0,
                JobObjectExtendedLimitInformation,
                std::ptr::from_ref(&info).cast(),
                size,
            )
        };
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(job)
    }

    /// Puts `child` (and the processes it later starts) into the job.
    fn assign(&self, child: &Child) -> std::io::Result<()> {
        // SAFETY: both handles are valid for the duration of the call.
        let ok = unsafe {
            AssignProcessToJobObject(self.0, child.as_raw_handle().cast())
        };
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for KillOnCloseJob {
    /// Closing the last handle to the job kills all of its processes.
    fn drop(&mut self) {
        // SAFETY: the handle is owned by `self` and closed exactly once.
        unsafe { CloseHandle(self.0) };
    }
}

/// Starts a long-running child inside a job, then closes the job.
pub fn run() -> anyhow::Result<()> {
    let job = KillOnCloseJob::new()?;
    let mut child = Command::new("ping")
        .args(["-n", "60", "127.0.0.1"])
        .stdout(std::process::Stdio::null())
        .spawn()?;
    job.assign(&child)?;
    assert!(child.try_wait()?.is_none(), "the child is still running");

    // Dropping the job terminates the child and its descendants.
    drop(job);
    // `wait` returns promptly instead of blocking for 60 seconds.
    child.wait()?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a completed command returns its output and exit status.
    #[test]
    fn run_completes() -> anyhow::Result<()> {
        run()
    }
}
