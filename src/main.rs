use std::io;

fn main() -> io::Result<()> {
    println!("Cachones app!");
    println!("Ingresa el nombre del evento");

    let mut event_name = String::new();

    let terminal_input_reader = io::stdin();

    terminal_input_reader.read_line(&mut event_name)?;

    // create and instace of event
   println!("nombre del evento {}", event_name);


    Ok(())
}
