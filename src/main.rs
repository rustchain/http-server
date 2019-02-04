extern crate config;
extern crate mio;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};

// tokens identify event
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() {
    let mut settings = config::Config::default();
    settings
        // add Settings.toml
        .merge(config::File::with_name("Settings")).unwrap()
        // add environment prefix APP
        .merge(config::Environment::with_prefix("APP")).unwrap();

    let listen = settings.get_str("listen").unwrap();
    let addr = listen.parse().unwrap();
    let sock = TcpStream::connect(&addr).unwrap();

    let server = TcpListener::bind(&addr).unwrap();

    let poll = Poll::new().unwrap();

    poll.register(&server, SERVER, Ready::readable(),
                  PollOpt::edge()).unwrap();

    poll.register(&sock, CLIENT, Ready::readable(),
                  PollOpt::edge()).unwrap();

    // events storage
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    println!("SERVER");
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let _ = server.accept();
                }
                CLIENT => {
                    println!("CLIENT");
                    // The server just shuts down the socket, let's just exit
                    // from our event loop.
                    //return;
                }
                _ => unreachable!(),
            }
        }
    }
}