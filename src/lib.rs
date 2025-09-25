use std::{
    collections::HashMap,
    fmt::format,
    io::{self, BufRead, BufReader, Error, Read, Stdin, Write},
    net::TcpStream,
    thread::{self},
    vec,
};

#[derive(Clone)]
pub struct Client {
    nick_name: String,
    server: String,
    // channel_name: String,
    commands: HashMap<&'static str, &'static str>,
    port_number: u32,
}

// initializing parameters
impl Client {
    pub fn build(args: &[String]) -> Result<Client, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Arguments!");
        }
        let nick_name = validate_nickname(&args[1]).unwrap();
        // setting up commands to send to the IRC server later on
        let mut commands: HashMap<&'static str, &'static str> = HashMap::new();
        commands.insert("/join", "/join <#channel>[,<#channel>]+ [<key>[,<key>]+]");
        commands.insert("/kick", "/kick <#channel> <nick>[,<nick>]+ [:<reason>]");
        commands.insert("/msg", " /msg <target>[,<target>]+ :<message> ");
        commands.insert("/nick", "/nick <newnick>");
        commands.insert("/part", "/part <#channel>[,<#channel>]+ [:<reason>]");

        Ok(Client {
            nick_name,
            server: args[2].to_owned(),
            // channel_name: args[3].to_owned(),
            commands,
            port_number: args[3].to_owned().parse::<u32>().unwrap_or(6667),
        })
    }

    pub fn initialize_connection(&self) {
        let out_stream = connect(self.clone());
        let in_stream = out_stream.try_clone().expect("Couldn't clone the stream");
        let client_clone = self.clone();
        thread::spawn(move || receiver(&in_stream, &client_clone).expect("Couldn't spawn thread"));
        sender(&self.clone(), &out_stream);
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

fn validate_command_msg(msg: &str) {
    if msg.len() > 3 {
        todo!()
    }
}

// handles sending of messages
fn sender(client: &Client, stream: &TcpStream) {
    loop {
        let mut message: String = String::new();
        match io::stdin().read_line(&mut message) {
            Ok(_) => {
                let msg: Vec<&str> = message.trim().split(' ').collect();
                let cmd: &str = msg.get(0).unwrap();
                match cmd {
                    "help" => client
                        .commands
                        .iter()
                        .for_each(|(key, value)| println!("{key},{value}")),
                    "/join" => {
                        let msg = msg.get(1).unwrap();
                        command(&stream, "JOIN", &msg, &client).unwrap();
                    }
                    "/msg" => {
                        let mut text = String::new();
                        let receiver = msg.get(1).unwrap();
                        msg.iter().for_each(|word| {
                            text.push_str(&word);
                            text.push_str(" ")
                        });
                        let message = format!("{} {}\r\n", receiver, text);

                        command(&stream, "PRIVMSG", message.as_str(), client).unwrap();
                    }
                    "/part" => {
                        let channel = msg.get(1).unwrap();
                        command(&stream, "PART", channel, client).unwrap();

                    }
                    _ => {
                        println!("Cannot find command:{cmd}");
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}

// connecting to the remote IRC server
fn connect(client: Client) -> TcpStream {
    let server_address = format!("{}:{}", client.server, client.port_number);
    let outgoing_stream = TcpStream::connect(&server_address).unwrap(); // nick registration
    let message = format!("{} * * :{}", client.nick_name, "Jhon Doe"); // connection initiation
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
fn receiver(stream: &TcpStream, client: &Client) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let mut i = 0;
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        let line = line.trim_end();
        if !line.is_empty() {
            if let Some(pos) = line.find("PING :") {
                let token = &line[pos + 6..pos + 16]; // length of "PING :" is 6
                println!("====The position: {pos} and token is: {token}====");
                command(&stream, "PONG", token, client).unwrap();
            }
            println!("{}: {}", i, line);
            i += 1;
        }
    }
    Ok(())
}
