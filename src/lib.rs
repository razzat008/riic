#![allow(unused_variables)]
use std::{
    io::{BufRead, BufReader, Error, Read, Write},
    net::TcpStream,
    thread::{self, JoinHandle},
};

#[derive(Clone)]
pub struct Client {
    nick_name: String,
    server: String,
    // channel_name: String,
    port_number: u32,
}

// initializing parameters
impl Client {
    pub fn build(args: &[String]) -> Result<Client, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Arguments!");
        }
        let nick_name = validate_nickname(&args[1]).unwrap();

        Ok(Client {
            nick_name,
            server: args[2].to_owned(),
            // channel_name: args[3].to_owned(),
            port_number: args[3].to_owned().parse::<u32>().unwrap_or(6667),
        })
    }

    pub fn initialize_connection(&self) -> JoinHandle<()> {
        let out_stream = connect(self.clone());
        let in_stream = out_stream.try_clone().expect("Couldn't clone the stream");
        thread::spawn(move || receiver(&in_stream).expect("Couldn't spawn a thread for receiver"))
    }
}
// checking validity according to the ruleset
fn validate_nickname(nickname: &String) -> Result<String, &'static str> {
    let haystack = [" ", ",", "*", "?", "!", "@", "$", ":", "."];

    if nickname.starts_with("#") {
        Err("Nicknames cannot start with a '#'")
    } else if haystack.iter().any(|c| nickname.contains(c)) {
        Err("Please enter a valid nickname(no special characters)")
    } else {
        Ok(nickname.to_string())
    }
}

// connecting to the remote IRC server
fn connect(client: Client) -> TcpStream {
    let server_address = format!("{}:{}", client.server, client.port_number);
    let outgoing_stream = TcpStream::connect(&server_address).unwrap();
    let message = format!("{} * * :{}", client.nick_name, "This is a test.");
    command(&outgoing_stream, "NICK", &client.nick_name, &client).unwrap();
    command(&outgoing_stream, "USER", &message, &client).unwrap();
    outgoing_stream
}

// What a typical exchange looks like
//  foo.example.com                               bar.example.com
//  +-------------+                               +-------------+
//  |  IRC Client |                               |  IRC Server |
//  +-------------+                               +-------------+
//         |                                              |
//         |  (1) NICK amy                                |
//         | -------------------------------------------> |
//         |                                              |
//         |  (2) USER amy * * :Amy Pond                  |
//         | -------------------------------------------> |
//         |                                              |
//         |  (3) :bar.example.com 001 amy :Welcome to    |
//         |      the Internet Relay Network              |
//         |      amy!amy@foo.example.com                 |
//         | <------------------------------------------- |
//         |                                              |

fn command(
    mut stream: &TcpStream,
    command: &str,
    message: &str,
    client: &Client,
) -> Result<usize, Error> {
    let full_command = format!("{} {}\r\n", command, message);
    stream.write(full_command.as_bytes())
}

// in_stream handling
fn receiver(stream: &TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let mut i = 0;
    line.clear();
    loop {
        let bytes_read = reader.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        let line = line.trim_end();
        if !line.is_empty() {
            println!("{}: {}", i, line);
            i += 1;
        }
    }
    Ok(())
}
