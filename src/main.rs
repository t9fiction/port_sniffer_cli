use std::{env, net::IpAddr};

const MAX_PORT: u16 = 65535;  // Maximum allowable port number

fn scan_port(ip: IpAddr, port: u16) -> bool {
    println!("Scanning port {}", port);
    return true;
}

fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();
    
    // Check if the correct number of arguments has been provided
    if args.len() != 4 {
        eprintln!("Usage: port_sniffer <IP> <start_port> <end_port>");
        return;
    }
    
    
    // Store the program name (optional, could be removed if not needed)
    let program = args[0].clone();
    
    // Parse the IP address from the command-line arguments
    let ip: IpAddr = args[1].parse().expect("Invalid IP address format.");
    
    // Parse the starting and ending port numbers, handling invalid inputs with `expect`
    let start_port: u16 = args[2].parse().expect("Invalid start port.");
    let end_port: u16 = args[3].parse().expect("Invalid end port.");
    
    if start_port > end_port {
        eprintln!("Error: start_port must be less than or equal to end_port.");
        return;
    }
    
    // Output scanning information for the user
    println!("Scanning ports {}-{} on {}", start_port, end_port, ip);

    // Optionally, print the program name if needed (could be removed)
    println!("{}", program);
}
