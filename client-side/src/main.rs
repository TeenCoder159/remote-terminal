use std::io::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

pub fn read_line() -> String {
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

        print!("Enter your command: ");
        let command_to_run = read_line();

        if command_to_run == "/quit".to_string() {
            break;
        }

        stream.write_all(command_to_run.as_bytes())?;
    }
    Ok(())
}

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
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
