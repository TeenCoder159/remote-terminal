#[allow(unused)]
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Output;
use std::str;
use std::{io::prelude::*, process::Command};

fn main() {
    let ip_addr = std::env::var("RPI")
        .expect("Please set an env variable with your RPI's IP address and port");
    let listener = TcpListener::bind(ip_addr).expect("Error while binding to 192.168.18.1:8080");

    for stream_err in listener.incoming() {
        let mut stream = stream_err.expect("Error with stream connection");
        stream.write_all(b"Hello from your raspeberry pi!\n------------\nThe Connection was established succesfully!")
            .expect("Error while writing to stream");

        loop {
            let mut buffer = [0; 1 << 20];

            let n = stream
                .read(&mut buffer)
                .expect("Error while trying to read to buffer");

            if n == 0 {
                break;
            }
            let buffer_slice = &buffer[..n];
            let output = string_to_command(buffer_slice);
            println!(
                "Running the command: {}",
                str::from_utf8(buffer_slice)
                    .expect("Error while converting buffer_slice to string")
            );

            stream
                .write_all(output.stdout.as_slice())
                .expect("Error while trying to write to stream")
        }
    }
}

fn string_to_command(buffer: &[u8]) -> Output {
    let input = str::from_utf8(&buffer).expect("Error while parsing from bytes to string");

    let mut i = 0;
    let mut args: Vec<&str> = Vec::new();
    let mut file = String::new();

    for word in input.split_whitespace() {
        if i == 0 {
            i += 1;
            file = word.to_string()
        } else {
            args.push(word);
        }
    }

    let output = match Command::new(file).args(args).output() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Tried to run an unknown command");
            return Command::new("echo")
                .arg(format!("\"Unknown command! Error: {e}\""))
                .output()
                .expect("Error carrying out echo command");
        }
    };

    output
}
