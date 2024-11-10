use std::{
    env,
    net::{IpAddr, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

const MAX_PORT: u16 = 65535; // Maximum allowable port number

fn scan_port(ip: IpAddr, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let result = TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_millis(100));
    if result.is_ok() {
        println!("Port {} is open", port);
        true
    } else {
        false
    }
}

fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments has been provided
    if args.len() != 4 {
        eprintln!("Usage: port_sniffer <IP> <start_port> <end_port>");
        return;
    }

    // Parse the IP address from the command-line arguments
    let ip: IpAddr = args[1].parse().expect("Invalid IP address format.");

    // Parse the starting and ending port numbers, handling invalid inputs with `expect`
    let start_port: u16 = args[2].parse().expect("Invalid start port.");
    let end_port: u16 = args[3].parse().expect("Invalid end port.");
    if end_port > MAX_PORT {
        eprintln!("Error: end_port must be less than or equal to 65535.");
        return;
    }

    if start_port > end_port {
        eprintln!("Error: start_port must be less than or equal to end_port.");
        return;
    }
    
    // Output scanning information for the user
    println!("Scanning ports {}-{} on {}", start_port, end_port, ip);

    // Channel to collect results from threads
    let (tx, rx) = mpsc::channel();

    // Vector to hold thread handles to join later
    let mut handles = vec![];

    // Spawn a thread for each port in the range
    
    for port in start_port..=end_port {
        let tx = tx.clone();
        let ip = ip.clone();
        let handle = thread::spawn(move || {
            if scan_port(ip, port) {
                tx.send(port).expect("Failed to send data through channel.");
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish by joining them
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Drop the initial sender to close the channel when all threads are done
    drop(tx);

    // Collect and print open ports
    for port in rx {
        println!("Port {} is open", port);
    }

    println!("Scanning completed.");
}
