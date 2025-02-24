use midir::{MidiOutput, MidiOutputConnection};
use std::io::{stdin, stdout, Write};

const MIDI_CLIENT_NAME: &str = "Dummy MIDI Client Name";
const MIDI_PORT_NAME: &str = "dummy-midi-port-name";

pub fn init_midi_controller(
    preferred_output_port_index: Option<usize>,
) -> Result<MidiOutputConnection, String> {
    let midi_output =
        MidiOutput::new(MIDI_CLIENT_NAME).expect("Midi controller: unable to create output");

    let mut midi_output_ports = midi_output.ports();
    let mut midi_port_index = None;
    if let Some(preferred_index) = preferred_output_port_index {
        if preferred_index < midi_output_ports.len() {
            midi_port_index = Some(preferred_index);
        }
    }
    if midi_port_index.is_none() {
        midi_port_index = match midi_output_ports.len() {
            0 => {
                println!("no output port found");
                None
            }
            1 => {
                println!(
                    "Choosing the only available output port: {}",
                    midi_output.port_name(&midi_output_ports[0]).unwrap()
                );
                Some(0)
            }
            _ => {
                println!("\nAvailable output ports:");
                for (i, p) in midi_output_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_output.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                stdout().flush().unwrap();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                let int_port = input.trim().parse::<usize>().unwrap();
                Some(int_port)
            }
        };
    }

    match midi_port_index {
        None => Err("Midi controller: unable to find output port".into()),
        Some(index) => {
            let output_port = midi_output_ports.remove(index);
            let midi_output_connection = midi_output.connect(&output_port, MIDI_PORT_NAME).unwrap();
            Ok(midi_output_connection)
        }
    }
}
