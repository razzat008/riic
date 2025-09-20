#![allow(dead_code, unused_imports, unused_variables)]
use std::{
    fmt::format,
    net::{self, SocketAddr, TcpListener, TcpStream},
    time::Duration,
};
/* Nicknames MUST NOT contain any of the following characters: space (' ', 0x20), comma (',', 0x2C), asterisk ('*', 0x2A), question mark ('?', 0x3F), exclamation mark ('!', 0x21), at sign ('@', 0x40).
Nicknames MUST NOT start with any of the following characters: dollar ('$', 0x24), colon (':', 0x3A).
Nicknames MUST NOT start with a character listed as a channel type, channel membership prefix, or prefix listed in the IRCv3 multi-prefix Extension.
Nicknames SHOULD NOT contain any dot character ('.', 0x2E). */
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

        Ok(Client {
            nick_name: args[1].clone(),
            server: args[2].clone(),
            channel_name: args[3].clone(),
            port_number: args[4].clone().parse::<u16>().unwrap_or(80),
        })
    }
}

// connecting to the remote IRC server
pub fn connect_to_server(client: Client) {
    let server_address = format!("{}:{}", client.server, client.port_number)
        .parse::<SocketAddr>()
        .unwrap();
    let stream = TcpStream::connect_timeout(&server_address, Duration::new(5, 0))
        .expect("Couldn't establish connection!");
}
