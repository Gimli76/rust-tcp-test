use std::net::TcpListener;
use std::io::prelude::*;

fn main() -> std::io::Result<()>  {

    // listener is a TcpListener. It binds to an IP:port and listen to incoming TCP connections.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // listener.incoming returns an Iterator. Each time we ask the Iterator for the next element,
    // it blocks until a new connection is established, and then returns a TCPStream which allows
    // us to comunicate with "the client". If we want to attend concurrently different clients, we
    // need to create a new thread or process to handle that client. Or maybe hand over the
    // TCPStrem to another process or thread somehow.
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let lines = [
            "Hakuna matata!\n",
            "\n",
            "Test 1, 2, 3...\n"
        ];
        for line in &lines {
            stream.write(line.as_bytes()).unwrap();
        }
        println!("Connection established!");
    }

    Ok(())
}
