use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    //let handle = tokio::spawn(async { "return value" });

    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listner.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }

    //let out = handle.await.unwrap();
    //println!("GOT {}", out);
}

/// The function `process` asynchronously reads a frame from a TCP stream, prints it, and then writes an
/// error response back to the stream.
///
/// Arguments:
///
/// * `socket`: The `socket` parameter in the `process` function is of type `TcpStream`, which
/// represents a TCP stream between the client and server for network communication.
async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::new();

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("umimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
