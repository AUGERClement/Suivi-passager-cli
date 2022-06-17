use std::env; // To obtain config
use std::fmt; // For formatting structs print
use std::io::stdin; // To get user input

// Define a snapshot for the current bus situation
// It can then be logged
struct BusSnapshot {
    stop_name: String, // The current stop of the bus
    incoming: i32, // The number of entering passengers in this stop
    outgoing: i32, // The number of outgoing passengers in this stop
    current: i32, // The current number of passengers in the bus
}

// Print formatting for BusSnapshot
impl fmt::Display for BusSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{}", self.stop_name, self.incoming, self.outgoing, self.current)
    }
}

// The type of change in passenger the get_input must ask
enum TypeUpdate {
    Incoming,
    Outgoing,
}

//Print formatting for TypeUpdate
impl fmt::Display for TypeUpdate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           TypeUpdate::Incoming => write!(f, "incoming"),
           TypeUpdate::Outgoing => write!(f, "outgoing"),
       }
    }
}

// Get a user input for number of passengers stepping in our out.
// In case of failure, the update bus will reiterate whole registration for the stop.
fn get_input(stop: &str, status: TypeUpdate) -> i32 {
    let mut input = String::new();
    //let mut input = String::from("3");

    println!("[{}] Please input number of {} passagers : ", stop, status);
    stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("input is {}", input);
    let passengers = match input.parse::<i32>() {
        Ok(val) if val >= 0 => val,
        _ => panic!("error : expected positive integer"), // Cover err and negative vals
    };
    println!("[{}] Registering {} {} passagers", stop, passengers, status);
    passengers
}

// Update the state of the bus for the stop, and log it.
// Takes a stop name and the population from the last stop.
fn update_bus(stop: &str, population:i32) -> () {
    let bus_snapshot = BusSnapshot {
        stop_name: String::from(stop),
        incoming: get_input(stop, TypeUpdate::Incoming),
        outgoing: get_input(stop, TypeUpdate::Outgoing),
        current: population,//+ incoming - outgoing,
    };
    println!("Snapshot : {}", bus_snapshot);
    // Append in file
}

fn main() {
    let stop = match env::var("STOPS_LIST") { // set by Docker compose
        Ok(stop) => stop,
        Err(_) => panic!("error : Couldn't find env STOPS_LIST")
    };
    let stops = stop.split(',');
    let trimmed = stops.map(|stop| stop.trim());
    for stop in trimmed {// Yeah map doesn't behave well with print
        update_bus(stop, 0);
    }
}
