use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    //let mut listner = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    let listner = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listner.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has closed
                    Ok(0) => return,
                    Ok(n) => {
                        println!("Received: {} - {:?}", n, &buf[..n]);
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error.
                            // There isn't much we can do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error.
                        // There isn't much we can do here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}
