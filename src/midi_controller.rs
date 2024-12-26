use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::io::{stdin, stdout, Write};

pub struct MidiController {
    pub output: MidiOutput,
    pub output_port: MidiOutputPort,
}
impl MidiController {
    pub fn connect_and_get(self) -> MidiOutputConnection {
        self.output
            .connect(&self.output_port, "rust-midi-volca-drum")
            .unwrap()
    }
}

pub fn init_midi_controller(
    preferred_output_port_index: Option<usize>,
) -> Result<MidiController, String> {
    let midi_output = MidiOutput::new("RUST Volca Drum MIDI Output").unwrap();

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
        None => Err(String::from("No output port found")),
        Some(index) => Ok(MidiController {
            output: midi_output,
            output_port: midi_output_ports.remove(index),
        }),
    }
}

// TODO: Create wrapper for MIDI to avoid this function
pub fn bridge_send_message(volca_drum: &mut MidiOutputConnection, a: u8, b: u8, c: u8) {
    let _ = volca_drum.send(&[a, b, c]);
    println!(
        "Send msg -> [{:10} {:10} {:10}]\n            [{:#10x} {:#10x} {:#10x}]\n            [{:#10b} {:#10b} {:#10b}]",
        a, b, c,
        a, b, c,
        a, b, c,
    );
}
