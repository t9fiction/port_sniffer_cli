Here is a suggested `README.md` file for your port scanner program:

---

# Port Scanner (Rust)

A simple multi-threaded port scanner written in Rust that scans a range of ports on a given IP address and reports which ports are open.

## Features

- Scans a range of ports on a given IP address.
- Multi-threaded design for improved performance.
- Uses TCP connection attempts with a timeout to check if ports are open.
- Outputs the open ports in the terminal.

## Requirements

- Rust (Stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/port-scanner.git
   cd port-scanner
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

To use the port scanner, you need to pass three command-line arguments: the IP address, the starting port, and the ending port.

### Command-line arguments:

- **IP Address**: The IP address of the target to scan.
- **Start Port**: The first port in the range to scan.
- **End Port**: The last port in the range to scan.

### Example:

To scan ports from 80 to 100 on IP `192.168.1.1`, run:

```bash
cargo run 192.168.1.1 80 100
```

### Output:
The program will output the open ports, for example:

```
Scanning ports 80-100 on 192.168.1.1
Port 80 is open
Port 443 is open
Scanning completed.
```

### Error Handling:

- The program expects exactly three command-line arguments: IP address, start port, and end port.
- It will display an error if the arguments are not provided correctly.

## How It Works

1. **Input Parsing**: The program accepts an IP address and a port range as arguments.
2. **Port Scanning**: For each port in the specified range, the program tries to establish a TCP connection to the target IP address using a 100ms timeout.
3. **Concurrency**: A thread is spawned for each port to be scanned, allowing for parallel processing and faster results.
4. **Channel Communication**: Open ports are communicated back to the main thread via a Rust `mpsc` (multi-producer, single-consumer) channel.
5. **Output**: The program outputs all open ports once all threads finish.

## Notes

- The timeout for each port scan is set to 100ms. You can adjust this value based on your network conditions.
- The maximum allowable port is set to 65535, which is the standard for TCP/UDP ports.
- This tool can be used for network security purposes (e.g., identifying open ports in your own network) but should only be used ethically and with permission.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

### Enhancements and Considerations:
- The program uses simple thread spawning and does not manage a thread pool, which could be an enhancement if the number of ports to scan is large.
- Error handling could be improved by handling the potential panic from `unwrap()` calls.
- You may consider adding support for scanning a list of IPs or integrating a timeout for the entire scan process.

