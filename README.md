Learning to work with sockets in Rust
=====================================

`echo` function establishes a TCP server and waits for a connection, connects
to it, receives the data, prints it on the screen, echo's the same data back
and closes the connection.

`echo_continuously` function establishes a TCP server and waits for a
connection, when a client connects to it, it accepts the connection; receives
the data; prints on the screen, the data and the client port number who sent
that data; echo's the same data back and closes the connection with that
client. However, the server is then waiting for another client to connect,
indefinitely.

`socket_file` function creates a Unix domain socket in `/tmp` folder with a
hard-coded name, once the socket file is created it waits to get a connection,
the client connects to this file and sends the data, the server receives the
data prints it on the screen and closes the connection with the client. This
function does not return anything to the client.

`chat_client` and `chat_server` functions work together.
- The `chat_client` function tries to connect to the server that created by the
  `char_server` function.
- On successful connection, the client waits (blocks) for user input, once the
  input is received it is sent to the server.
- The server upon connection is waiting (blocking) to receive data from the
  client, once the data is received, it is printed on the screen and the server
  is waiting (blocking) for the input of user.
- When the client has sent the data it is now waiting (blocking) to receive
  something from the server.
- There is also a lot of conversion back and forth between `String` and `u8`,
  the data that is to be sent from either side has to be of type `u8` and upon
  receiving on both sides it is converted back to string, all the while where
  the input from the user on both sides is a string.

TODO:
-----
[ ] communication using `mpsc` (multi-producer single-consumer).
    [ ] or check to see if there can be a single-producer and single-consumer.
    [ ] two way communication.
        - can this be done on single pipe?
        - or we need to have two pipes, each side being consumer of one pipe.
[ ] using Unix domain socket, communication between two different processes.
[ ] use UDP to send and receive some data.
