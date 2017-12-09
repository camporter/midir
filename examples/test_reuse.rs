extern crate midir;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, MidiOutput, Ignore};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    
    let mut midi_in = MidiInput::new("My Test Input")?;
    midi_in.ignore(Ignore::None);
    let mut midi_out = MidiOutput::new("My Test Output")?;
    
    println!("Available input ports:");
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i).unwrap());
    }
    print!("Please select input port: ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let in_port: usize = input.trim().parse()?;
    
    println!("\nAvailable output ports:");
    for i in 0..midi_out.port_count() {
        println!("{}: {}", i, midi_out.port_name(i).unwrap());
    }
    print!("Please select output port: ");
    stdout().flush()?;
    input.clear();
    stdin().read_line(&mut input)?;
    let out_port: usize = input.trim().parse()?;
    
    // This shows how to reuse input and output objects:
    // Open/close the connections twice using the same MidiInput/MidiOutput objects
    for _ in 0..2 {
        println!("\nOpening connections");
        let log_all_bytes = Vec::new(); // We use this as an example custom data to pass into the callback
        let conn_in = midi_in.connect(in_port, "midir-test", |stamp, message, log| {
            // The last of the three callback parameters is the object that we pass in as last parameter of `connect`.
            println!("{}: {:?} (len = {})", stamp, message, message.len());
            log.extend_from_slice(message);
        }, log_all_bytes)?;
        
        // One could get the log back here out of the error
        let mut conn_out = midi_out.connect(out_port, "midir-test")?;
        
        println!("Connections open, enter `q` to exit ...");
        
        loop {
            input.clear();
            stdin().read_line(&mut input)?;
            if input.trim() == "q" {
                break;
            } else {
                conn_out.send(&[144, 60, 1])?;
                sleep(Duration::from_millis(200));
                conn_out.send(&[144, 60, 0])?;
            }
        }
        println!("Closing connections");
        let (midi_in_, log_all_bytes) = conn_in.close();
        midi_in = midi_in_;
        midi_out = conn_out.close();
        println!("Connections closed");
        println!("Received bytes: {:?}", log_all_bytes);
    }
    Ok(())
}