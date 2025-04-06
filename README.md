# Remote-terminal is an alternative to ssh

The program on the RPI listens for incoming connections (using TcpStreams / listeners) and allows you to run commands. The code is still a bit wonky so I need to work on it a bit more, but its usable to a certain extent right now.

## Usage:
1. Clone down the repo (on both your RPI and Client side machine)
1. cd into the rpi-side dir or the client-side on the respective devices
1. Set the environment variable RPI to the IP address of your RPI and the port you want to use on both devices
1. Cargo run on the RPI **BEFORE** running cargo run on the Client device
1. Now you can do commands (As of now only commands that give an output and no running dual commands with &&)
