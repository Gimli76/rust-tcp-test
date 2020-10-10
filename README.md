# rust-tts

The name means RUST TCP Test Server. It's meant to be an example server.

The functionality is as follows:
 - Bind to a TCP port and wait for conections
 - When a client connects, start a new thread to attend that connection
 - Send a message to the client
 - Finish the connection and close the thread