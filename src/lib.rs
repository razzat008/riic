// - Nicknames MUST NOT contain any of the following characters: space (' ', 0x20), comma (',',
// 0x2C), asterisk ('*', 0x2A), question mark ('?', 0x3F), exclamation mark ('!', 0x21), at
// sign ('@', 0x40). - Nicknames MUST NOT start with any of the following characters: dollar
// ('$', 0x24), colon (':', 0x3A). - Nicknames MUST NOT start with a character listed as a
// channel type, channel membership prefix, or prefix listed in the IRCv3 multi-prefix
// Extension. - Nicknames SHOULD NOT contain any dot character ('.', 0x2E).
#![allow(dead_code, unused_variables)]
use std::{
    io::{Error, Write, WriterPanicked},
    net::{SocketAddr, TcpStream},
    thread,
    time::Duration,
    usize,
};

pub struct Client {
    nick_name: String,
    server: String,
    channel_name: String,
    port_number: u16,
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
            channel_name: args[3].to_owned(),
            port_number: args[4].to_owned().parse::<u16>().unwrap_or(80),
        })
    }
}
// checking validity according to the ruleset
fn validate_nickname(nickname: &String) -> Result<String, &'static str> {
    let haystack = [" ", ",", "*", "?", "!", "@", "$", ":", "."];

    if nickname.starts_with("#") {
        Err("Nicknames cannot start with a '#'.")
    } else if haystack.iter().any(|c| nickname.contains(c)) {
        Err("Please enter a valid nickname(no special characters).")
    } else {
        Ok(nickname.to_string())
    }
}

// connecting to the remote IRC server
pub fn connect(client: Client) -> TcpStream {
    let server_address: SocketAddr = format!("{}:{}", client.server, client.port_number)
        .parse()
        .expect("Parsing error!");
    let outgoing_stream = TcpStream::connect(&server_address).unwrap();
    let message = format!("{} * * : {}", client.nick_name, client.nick_name);
    command(&outgoing_stream, "NICK", &client.nick_name, &client)
        .expect_err("Couldn't send the command.");
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
    let mut command = command.to_string();
    command.push_str(" ");
    command.push_str(message);
    stream.write(command.as_bytes())
}
