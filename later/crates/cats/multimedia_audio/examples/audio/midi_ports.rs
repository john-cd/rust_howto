#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Lists MIDI input ports with `midir` and decodes raw MIDI messages.

use midir::MidiInput;

/// A decoded channel voice message.
#[derive(Debug, PartialEq, Eq)]
enum MidiMessage {
    /// A key was pressed.
    NoteOn {
        /// Channel, from 0 to 15.
        channel: u8,
        /// Key number, from 0 to 127. 60 is middle C.
        key: u8,
        /// How hard the key was pressed, from 1 to 127.
        velocity: u8,
    },
    /// A key was released.
    NoteOff {
        /// Channel, from 0 to 15.
        channel: u8,
        /// Key number, from 0 to 127.
        key: u8,
    },
}

/// Decodes a raw message. Returns `None` for messages that are not handled.
fn parse_message(bytes: &[u8]) -> Option<MidiMessage> {
    // The high nibble of the status byte is the kind of message,
    // the low nibble is the channel.
    let (&status, data) = bytes.split_first()?;
    let channel = status & 0x0F;
    match (status & 0xF0, data) {
        // A "note on" with velocity 0 is, by convention, a "note off".
        (0x90, &[key, 0]) | (0x80, &[key, _]) => {
            Some(MidiMessage::NoteOff { channel, key })
        }
        (0x90, &[key, velocity]) => Some(MidiMessage::NoteOn {
            channel,
            key,
            velocity,
        }),
        _ => None,
    }
}

/// Returns the names of the available MIDI input ports.
fn list_input_ports() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let midi_in = MidiInput::new("rust_howto")?;
    let mut names = Vec::new();
    for port in midi_in.ports() {
        names.push(midi_in.port_name(&port)?);
    }
    Ok(names)
}

/// Lists the ports, if any, and decodes some messages.
fn main() {
    // The machine may have no MIDI support at all, so do not fail.
    match list_input_ports() {
        Ok(names) if names.is_empty() => println!("No MIDI input port."),
        Ok(names) => println!("MIDI input ports: {names:?}"),
        Err(error) => println!("MIDI is not available: {error}"),
    }

    // Note On, channel 1, middle C, velocity 100.
    assert_eq!(
        parse_message(&[0x90, 60, 100]),
        Some(MidiMessage::NoteOn {
            channel: 0,
            key: 60,
            velocity: 100
        })
    );
    // Note On with velocity 0 means Note Off.
    assert_eq!(
        parse_message(&[0x93, 60, 0]),
        Some(MidiMessage::NoteOff {
            channel: 3,
            key: 60
        })
    );
    // A program change is not handled; neither is a truncated message.
    assert_eq!(parse_message(&[0xC0, 5]), None);
    assert_eq!(parse_message(&[0x90, 60]), None);
    assert_eq!(parse_message(&[]), None);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the midi ports example through its entry point.
    #[test]
    fn test_midi_ports_example_entry_point() {
        main();
    }

    #[test]
    fn note_off_message() {
        assert_eq!(
            parse_message(&[0x8F, 127, 64]),
            Some(MidiMessage::NoteOff {
                channel: 15,
                key: 127
            })
        );
    }
}
