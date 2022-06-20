use std::env; // To obtain config
use std::fmt; // For formatting structs print
use std::io::stdin; // To get user input
use std::fs::OpenOptions; // To append in file
use std::io::Write; // To write in file

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

// Start focused of side effect code (could become a module)

// Get a user input for number of passengers stepping in our out.
// Accept only valid integer and will prompt until it get one.
fn get_input(stop: &str, status: TypeUpdate) -> i32 {
    let mut input = String::new();

    println!("[{}] Please input number of {} passagers : ", stop, status);
    let passengers: i32 = loop {
        stdin().read_line(&mut input).expect("error: unable to read user input");
        if let Ok(val) = input.trim().parse::<i32>() { // Success read
            if val >= 0 {
                break val;
            }
        }
        println!("error : expected positive integer");
        input.clear(); // Clear the buffer
    };
    println!("[{}] Registering {} {} passagers", stop, passengers, status);
    passengers
}

// Log the update in statement.txt.
// No computation, side_effect only.
// Src : https://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file
fn log_update(bus_snapshot: &BusSnapshot) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("statement.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", bus_snapshot) {
        eprintln!("Couldn't write to file: {}", e);
    };
    println!("Snapshot : {}", bus_snapshot);
}

// This function symply create a file and setup the first line.
// Could use timestamp to make multiples reports without eraseing old ones.
// Eventually, could pass the handle around, but need premature optimisations to be worth it.
fn init_report() {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("statement.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "Arrêt,Montées,Descentes,Nombre de passagers") {
        eprintln!("Couldn't write to file: {}", e);
    };
}

// End focused on side effect code

// Return the result of (pop + incoming - outgoing)
// It wrap the result in a Result because it must be >= 0
// No vehicle can have -1 passenger
fn check_population(tmp_snapshot: &BusSnapshot, pop:i32) -> Result<i32, i32> {
    match pop + tmp_snapshot.incoming - tmp_snapshot.outgoing {
        val if val >= 0 => Ok(val),
        val => Err(val),
    }
}

// Update the state of the bus for the stop, and log it.
// Takes a stop name and the population from the last stop.
// Return the current population (used as arg for the next call)
fn update_bus(stop: &str, population:i32) -> i32 {
    let mut tmp_snapshot: BusSnapshot;// Mutable because I need filed to compute population
    let valid_snap = loop {
        tmp_snapshot = BusSnapshot { // Mutable because I need filed to compute population
            stop_name: String::from(stop),
            incoming: get_input(stop, TypeUpdate::Incoming),
            outgoing: get_input(stop, TypeUpdate::Outgoing),
            current: population,//+ incoming - outgoing,
        };
        if let Ok(val) = check_population(&tmp_snapshot, population) { // Success compute
            tmp_snapshot.current = val;
            break tmp_snapshot;
        }
        println!("error : Negative passenger number. Please ensure you inputed good data");
    };
    log_update(&valid_snap); // Append in file (not yet setup)
    valid_snap.current
}

fn main() {
    let stop = match env::var("STOPS_LIST") { // set by Docker compose
        Ok(stop) => stop,
        Err(_) => panic!("error : Couldn't find env STOPS_LIST")
    };
    let stops = stop.split(',');
    let trimmed = stops.map(|stop| stop.trim());
    let mut population:i32 = 0;
    
    init_report(); // Create the txt and write the header in
    for stop in trimmed {// A fold wouldn't be as readable for non FP cultists.
        population = update_bus(stop, population);
    }
    // Potential fold in pseudo_code :
    // fold(acc, list, fn);
    // fold(population, trimmed, update_bus(&1, acc))
}
