use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    //let mut listner = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    let listner = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    loop {
        let (mut _socket, _) = listner.accept().await?;
        tokio::spawn(async move {});
    }
}
