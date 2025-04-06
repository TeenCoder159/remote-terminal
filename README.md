# Remote Terminal

A lightweight remote terminal application built in Rust that allows you to connect to your Raspberry Pi over TCP.

## Repository Structure

The repository is organized into two main directories:
- `client/` - Contains the client-side application that runs on your local machine
- `rpi/` - Contains the server-side application that runs on your Raspberry Pi

## Features

- Secure TCP connections between your local machine and Raspberry Pi
- Command execution on your Raspberry Pi from anywhere
- Simple and efficient design using Rust's standard library
- Cross-platform client support

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.60.0 or later)
- Cargo (comes with Rust)
- A Raspberry Pi with network connectivity
- Port forwarding configured on your network (if connecting from outside your local network)

## Setup Instructions

### On Your Raspberry Pi

1. Clone the repository:
   ```bash
   git clone https://github.com/TeenCoder159/remote-terminal.git
   cd remote-terminal
   ```

2. Navigate to the RPI server directory:
   ```bash
   cd rpi-side
   ```

3. Build and run the server:
   ```bash
   cargo run
   ```

4. The server will start listening for incoming connections. Note the IP address and port displayed in the console.

### On Your Local Machine

1. Clone the repository (if you haven't already):
   ```bash
   git clone https://github.com/TeenCoder159/remote-terminal.git
   cd remote-terminal
   ```

2. Navigate to the client directory:
   ```bash
   cd client-side
   ```

3. Build and run the client:
   ```bash
   cargo run
   ```

4. Follow the prompts to enter the IP address and port of your Raspberry Pi.

## Usage

Once connected, you can send commands to your Raspberry Pi from the client application. The output of these commands will be displayed in your terminal.

## Configuration

Both the client and server applications can be configured by editing their respective configuration files or by providing command-line arguments. Refer to the documentation in each directory for specific configuration options.

## Security Considerations

- This application transmits data over TCP. For enhanced security, consider implementing encryption or using it within a secure network.
- Be cautious about which ports you expose to the internet if using this application remotely.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- Built with Rust and its standard library
- Inspired by the need for simple, efficient remote access to Raspberry Pi devices

---

Feel free to [open an issue](https://github.com/TeenCoder159/remote-terminal/issues) if you encounter any problems or have suggestions for improvements!
