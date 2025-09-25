# riic

      simple Internet Relay Chat(IRC) client written in Rust
      

## Features / What Works
#### Generated Using AI

These are features currently implemented or known to work:

- Establish initial TCP connection to an IRC server  
- Receive lines/messages from the server (with basic parsing)  
- Respond to `PING` messages with `PONG` (so the connection stays alive)  
- Print incoming lines to the console  
- Send messages / chat to channels / users  
- Minimal command handling for server pings  


## Todo / Missing Features

Here are things you might want to implement next:

- Full support for all IRC commands 
- Send messages / chat to channels / users  
- Better error handling, reconnection logic  
- Command‐line interface or console input to let the user type commands  
- Support for multiple channels  
- Graceful shutdown  
- Logging, configuration, and possibly TLS support  
- Tests & examples  


## Installation

```bash
# Clone the repository
git clone https://github.com/razzat008/riic.git
cd riic

# Build
cargo build --release
````

If you want a development build:

```bash
cargo build
```


## Usage

To run the client, you’ll invoke the compiled binary. Right now the project expects command‐line arguments (via `env::args`) that are passed to `Client::build(...)`. The exact expected arguments depend on how `Client::build` is implemented.

Example:

```bash
# (From project root, assuming binary named `riic`)
cargo run -- coolnickname chat.freenode.net 6667
```

Or if built in release:

```bash
./target/release/riic <args>
```

Once running, the client will connect, start receiving messages, reply to PINGs, and print lines to the console (if they’re nonempty).


## Examples

```text
# Suppose you pass args: server, port, nickname, etc.
# After connecting, you might see:

0: :irc.example.com NOTICE AUTH :*** Looking up your hostname
1: :irc.example.com NOTICE AUTH :*** Checking Ident
2: :irc.example.com NOTICE AUTH :*** Your host is irc.example.com
...
5: PING :1234567890
====The position: 0 and token is: 1234567890====
6: :NickServ!service@services NOTICE your_nick :This nickname is registered.
...
```

This demonstrates the PING → client responds with PONG, and lines get printed.


## Configuration

At this point, there is **no configuration file** support. All configuration
must be passed via command-line arguments and handled inside
`Client::build(...)`. You should examine `build` in your `lib.rs` or wherever
`Client::build` is defined to see exactly what arguments are expected (server,
port, nickname, maybe channel names, etc.).

---

## References
- https://chi.cs.uchicago.edu/chirc/irc_examples.html#joining-talking-in-and-leaving-a-channel
- https://wiki.freenode.net/view/User_Commands#JOIN
