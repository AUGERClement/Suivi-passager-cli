use std::env;

// Update the state of the bus for the stop, and log it.
fn update_bus(stop: &str) -> () {
    println!("[{}] Number of incoming passagers :", stop);
    println!("[{}] Number of outgoing passagers :", stop);
    // Prompt "[stop] Number of ingoing passagers ?"
    // Prompt "[stop] Number of outgoing passagers ?"
    // Log status
    // Append in file
}

fn main() {
    let stop = match env::var("STOPS_LIST") { // set by Docker compose
        Ok(stop) => stop,
        Err(_) => panic!("Couldn't find env STOPS_LIST")
    };
    let stops = stop.split(',');
    let trimmed = stops.map(|stop| stop.trim());
    for stop in trimmed {// Yeah map doesn't behave well with print
        update_bus(stop);
    }
}
