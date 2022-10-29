use std::io;

struct Event {
    name: String,
    friends: Vec<Friend>
}

struct Friend {
    name: String
}

fn main() {
    println!("Cachones app!");
    println!("Ingresa el nombre del evento");

    let mut event_name = String::new();

    let terminal_input_reader = io::stdin();

    terminal_input_reader.read_line(&mut event_name).unwrap();
    
    println!("Ingresa el nombre de los cachones que te acompañan al evento");

    let mut should_add_friend = true;

    let mut friends = Vec::new();

    while should_add_friend  {
        let mut friend_name = String::new();
        println!("Ingresa el nombre del cachon:");
        terminal_input_reader.read_line(&mut friend_name).unwrap();
        friends.push(friend_name);

        println!("quieres agregar otro? (Y/N)");
        let mut response = String::new();
        terminal_input_reader.read_line(&mut response).unwrap();

        match response {
            "Y".to_string() => should_add_friend = false,
            _ => should_add_friend = true
        }
    }
        
    let event = Event { name: event_name, friends: friends };

}
