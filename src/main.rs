use std::os::unix::net::UnixListener;
use std::io::Read;

fn main() {
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
