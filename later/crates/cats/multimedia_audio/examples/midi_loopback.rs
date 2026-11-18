// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Sends a note on and off through an explicitly selected MIDI loopback pair.

use std::error::Error;
use std::io;
use std::sync::mpsc::sync_channel;
use std::time::Duration;

use midir::MidiIO;
use midir::MidiInput;
use midir::MidiOutput;

const NOTE_ON: [u8; 3] = [0x90, 60, 100];
const NOTE_OFF: [u8; 3] = [0x80, 60, 0];

/// Finds a unique port by its exact name, refusing missing or ambiguous names.
fn select_port<T: MidiIO>(midi: &T, name: &str) -> Result<T::Port, Box<dyn Error>> {
    let mut selected = None;
    for port in midi.ports() {
        if midi.port_name(&port)? == name {
            if selected.is_some() {
                return Err(io::Error::other(format!("ambiguous MIDI port: {name}")).into());
            }
            selected = Some(port);
        }
    }
    selected.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("MIDI port not found: {name}"),
        )
        .into()
    })
}

/// Prints the exact names exposed by the current native backend.
fn list_ports() -> Result<(), Box<dyn Error>> {
    let input = MidiInput::new("rust-howto loopback input")?;
    let output = MidiOutput::new("rust-howto loopback output")?;
    println!("Inputs:");
    for port in input.ports() {
        println!("  {}", input.port_name(&port)?);
    }
    println!("Outputs:");
    for port in output.ports() {
        println!("  {}", output.port_name(&port)?);
    }
    Ok(())
}

/// Opens existing loopback endpoints and verifies both messages within a bounded wait.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--list"] {
        return list_ports();
    }
    let [input_name, output_name] = args.as_slice() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: midi_loopback --list | midi_loopback INPUT_PORT OUTPUT_PORT",
        )
        .into());
    };

    let input = MidiInput::new("rust-howto loopback input")?;
    let output = MidiOutput::new("rust-howto loopback output")?;
    let input_port = select_port(&input, input_name)?;
    let output_port = select_port(&output, output_name)?;

    // The callback runs on a backend thread and must not wait for the receiver.
    let (sender, receiver) = sync_channel(2);
    let _input_connection = input.connect(
        &input_port,
        "rust-howto receive",
        move |_, bytes, ()| {
            if let Ok(message @ (NOTE_ON | NOTE_OFF)) = <[u8; 3]>::try_from(bytes) {
                let _ = sender.try_send(message);
            }
        },
        (),
    )?;
    let mut output_connection = output.connect(&output_port, "rust-howto send")?;

    // Release the note before waiting even if the receive path is disconnected.
    output_connection.send(&NOTE_ON)?;
    output_connection.send(&NOTE_OFF)?;
    for expected in [NOTE_ON, NOTE_OFF] {
        let received = receiver.recv_timeout(Duration::from_secs(2))?;
        if received != expected {
            return Err(
                io::Error::other(format!("expected {expected:02X?}, got {received:02X?}")).into(),
            );
        }
        println!("Received {received:02X?}");
    }
    Ok(())
}
// ANCHOR_END: example
