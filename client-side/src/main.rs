use std::io::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn read_line() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input.trim().to_string()
}

fn looper(stream: &mut TcpStream) -> Result<(), Error> {
    loop {
        let mut buffer = [0; 1 << 20];

        let n = stream
            .read(&mut buffer)
            .expect("Error while trying to read to buffer");

        if n == 0 {
            break;
        }

        let buffer = &buffer[..n];

        println!(
            "{}\n",
            str::from_utf8(buffer).expect("Error converting to string")
        );

        println!("Enter your command: ");
        let command_to_run = read_line();

        if command_to_run == "/quit" {
            stream
                .write_all(b"/quit")
                .expect("Error writing /quit to stream");
            break;
        } else if command_to_run == "quit" {
            break;
        }

        stream.write_all(command_to_run.as_bytes())?;
    }
    Ok(())
}

fn main() {
    let ip_addr = std::env::var("RPI")
        .expect("Please set an env variable with your RPI's IP address and port");

    let mut stream = match TcpStream::connect(ip_addr) {
        Ok(stream) => {
            println!("Connected to TcpStream Successfully");
            stream
        }
        Err(e) => {
            eprintln!(
                "An error occured while trying to connect: {e}\nPlease start your TCP connection end"
            );
            std::process::exit(1);
        }
    };

    looper(&mut stream).expect("Error while running the loop");
}
