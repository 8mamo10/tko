use bytes::ByteMut;
use mini_redis::{Frame, Result};
use tokio::net::TcpStream;

struct Connection {
    stream: TcpStream,
    buffer: ByteMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: ByteMut::with_capacity(4096),
        }
    }
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {}
    pub async fn write_frame(&mut self, frame: &Frame) -> Result {}
}
