#[allow(dead_code)]
fn echo() {
    use std::net::TcpListener;
    use std::io::{Read, Write};

    let tcp_listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(listener) => listener,
        Err(e) => { eprintln!("Failed to create a TCP Listener due to: {e}"); std::process::exit(1); }
    };

    println!("Listening on address: {}", tcp_listener.local_addr().unwrap());

    match tcp_listener.accept() {
        Ok((mut tcp_stream, client_addr)) => {
            println!("Connection established from: {client_addr}");
            let mut buffer = String::new();
            let _ = tcp_stream.read_to_string(&mut buffer);
            println!("Received data: {buffer}");
            let _ = tcp_stream.write(buffer.as_bytes());
        },
        Err(e) => eprintln!("Failed to accept TCP connection due to: {e}"),
    }
}

#[allow(dead_code)]
fn echo_continously() {
    use std::net::TcpListener;
    use std::io::{Read, Write};

    let tcp_listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(listener) => listener,
        Err(e) => { eprintln!("Failed to create a TCP Listener due to: {e}"); std::process::exit(1); }
    };

    println!("Listening on address: {}", tcp_listener.local_addr().unwrap());

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 50];
                let peer_addr = stream.peer_addr().unwrap();
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!("Read {size} bytes from {peer_addr}");
                        println!("Msg: {:?}", std::str::from_utf8(&buffer[0..size]).unwrap());
                        let _ = stream.write(&buffer[0..size]);
                    },
                    Err(e) => {
                        eprintln!("Error reading from ({peer_addr}) due to: {e}");
                        std::process::exit(1);
                    }
                }
            },
            Err(e) => eprintln!("Failed to accept TCP connection due to: {e}"),
        }
    }
}

#[allow(dead_code)]
fn socket_file() {
    use std::os::unix::net::UnixListener;
    use std::io::Read;

    let listener = match UnixListener::bind("/tmp/socket-rs-hello.socket") {
        Ok(socket) => socket,
        Err(e) => { eprintln!("Failed to create the socket due to: {e}"); std::process::exit(1); }
    };

    let mut buffer = [0; 20];
    match listener.accept() {
        Ok((mut socket, addr)) => {
            match socket.read(&mut buffer) {
                Ok(size) => print!("Size: {size}\nMsg: {}", String::from_utf8(buffer.to_vec()).unwrap()),
                Err(e) => eprintln!("Failed to read from socket (addr: {addr:?}) into buffer due to: {e}"),
            }
        },
        Err(e) => eprintln!("Failed to accept connection due to: {e}"),
    }
}

#[allow(dead_code)]
fn chat_client() {
    use std::net::TcpStream;
    use std::io::{Write, Read};

    match TcpStream::connect("127.0.0.1:8000") {
        Ok(mut stream) => {
            println!("Connected to: {}", stream.peer_addr().unwrap());
            let mut buffer = [0; 100];
            let mut input = String::new();
            loop {
                match std::io::stdin().read_line(&mut input) {
                    Ok(size) => {
                        println!("Read: {size} bytes");

                        let size = stream.write(input.as_bytes()).unwrap();
                        println!("Sent {size} bytes");
                        input.clear();

                        let size = stream.read(&mut buffer).unwrap();
                        println!("Read: {size} bytes from connection");
                        println!("Read: {:?}", std::str::from_utf8(&buffer[0..size]).unwrap());
                    },
                    Err(e) => { eprintln!("Failed to read input due to: {e}"); std::process::exit(0); }
                }
            }
        }
        Err(e) => { eprintln!("Failed to connect to TCP server due to: {e}"); std::process::exit(1); }
    }
}

#[allow(dead_code)]
fn chat_server() {
    use std::net::TcpListener;
    use std::io::{Write, Read};

    let tcp_listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(listener) => listener,
        Err(e) => { eprintln!("Failed to create a TCP Listener due to: {e}"); std::process::exit(1); }
    };

    println!("Listening on address: {}", tcp_listener.local_addr().unwrap());

    match tcp_listener.accept() {
        Ok((mut stream, addr)) => {
            println!("Connection established with: {}", addr);
            let mut buffer = [0; 100];
            let mut input = String::new();
            loop {
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!("Read: {size} bytes from connection");
                        println!("Read: {:?}", std::str::from_utf8(&buffer[0..size]).unwrap());

                        let size = std::io::stdin().read_line(&mut input).unwrap();
                        println!("Read: {size} bytes of input");

                        let size = stream.write(input.as_bytes()).unwrap();
                        println!("Wrote {size} bytes to connection");
                        input.clear()
                    },
                    Err(e) => {
                        eprintln!("Failed reading from connection due to {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
        Err(e) => { eprintln!("Failed to accept connection due to: {e}"); std::process::exit(1); }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "-c" { // client
        chat_client()
    } else if args[1] == "-s" { // server
        chat_server()
    }
}
