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

fn main() {

}
