use midir::{MidiOutput, MidiOutputPort};
use std::io::{stdin, stdout, Write};

pub struct MidiController {
    pub midi_output: MidiOutput,
    pub midi_port: MidiOutputPort,
}

pub fn init_midi() -> Result<MidiController, String> {
    let midi_out = MidiOutput::new("My Test Output").unwrap();

    // Get an output port (read from console if multiple are available)
    let mut out_ports = midi_out.ports();
    let out_port_index = match out_ports.len() {
        0 => {
            println!("no output port found");
            None
        }
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            Some(0)
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let int_port = input.trim().parse::<usize>().unwrap();
            Some(int_port)
        }
    };
    match out_port_index {
        None => Err(String::from("No output port found")),
        Some(index) => Ok(MidiController {
            midi_output: midi_out,
            midi_port: out_ports.remove(index),
        }),
    }
}
