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
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        if let Some(frame) = self.parse_frame()? {
            return Ok(Some(frame));
        }
        if 0 == self.stream.read_buf(&mut self.buffer).await? {
            if self.buffer.is_empty() {
                return Ok(None);
            } else {
                return Err("connection reset by peer".into());
            }
        }
    }
    pub async fn write_frame(&mut self, frame: &Frame) -> Result {}
}
