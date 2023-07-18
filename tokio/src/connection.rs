// // use bytes::BytesMut;
// // use tokio::net::TcpStream;
// // use tokio::io::AsyncReadExt;
// // use bytes::Buf;
// // use mini_redis::{Frame,Result};
// //
// // pub struct Connection {
// //     stream: TcpStream,
// //     buffer: BytesMut,
// // }
// //
// // impl Connection {
// //     pub fn new(stream: TcpStream) -> Connection {
// //         Connection {
// //             stream,
// //             // Allocate the buffer with 4kb of capacity.
// //             buffer: BytesMut::with_capacity(4096),
// //         }
// //     }
// //
// //
// //     pub async fn read_frame(&mut self)
// //                             -> Result<Option<Frame>>
// //     {
// //         loop {
// //             // Attempt to parse a frame from the buffered data. If
// //             // enough data has been buffered, the frame is
// //             // returned.
// //             if let Some(frame) = self.parse_frame()? {
// //                 return Ok(Some(frame));
// //             }
// //
// //             // There is not enough buffered data to read a frame.
// //             // Attempt to read more data from the socket.
// //             //
// //             // On success, the number of bytes is returned. `0`
// //             // indicates "end of stream".
// //             if 0 == self.stream.read_buf(&mut self.buffer).await? {
// //                 // The remote closed the connection. For this to be
// //                 // a clean shutdown, there should be no data in the
// //                 // read buffer. If there is, this means that the
// //                 // peer closed the socket while sending a frame.
// //                 if self.buffer.is_empty() {
// //                     return Ok(None);
// //                 } else {
// //                     return Err("connection reset by peer".into());
// //                 }
// //             }
// //         }
// //     }
// // }
//
//
// use tokio::net::TcpStream;
// use mini_redis::{Frame, Result};
// use mini_redis::frame::Error::Incomplete;
// use bytes::Buf;
// use std::io::Cursor;
// use tokio::io::AsyncReadExt;
//
// pub struct Connection {
//     stream: TcpStream,
//     buffer: Vec<u8>,
//     cursor: usize,
// }
//
// impl Connection {
//     pub fn new(stream: TcpStream) -> Connection {
//         Connection {
//             stream,
//             // Allocate the buffer with 4kb of capacity.
//             buffer: vec![0; 4096],
//             cursor: 0,
//         }
//     }
//
//     pub async fn read_frame(&mut self)
//                             -> Result<Option<Frame>>
//     {
//         loop {
//             if let Some(frame) = self.parse_frame()? {
//                 return Ok(Some(frame));
//             }
//
//             // Ensure the buffer has capacity
//             if self.buffer.len() == self.cursor {
//                 // Grow the buffer
//                 self.buffer.resize(self.cursor * 2, 0);
//             }
//
//             // Read into the buffer, tracking the number
//             // of bytes read
//             let n = self.stream.read(
//                 &mut self.buffer[self.cursor..]).await?;
//
//             if 0 == n {
//                 if self.cursor == 0 {
//                     return Ok(None);
//                 } else {
//                     return Err("connection reset by peer".into());
//                 }
//             } else {
//                 // Update our cursor
//                 self.cursor += n;
//             }
//         }
//     }
//
//
//     fn parse_frame(&mut self)
//                    -> Result<Option<Frame>>
//     {
//         // Create the `T: Buf` type.
//         let mut buf = Cursor::new(&self.buffer[..]);
//
//         // Check whether a full frame is available
//         match Frame::check(&mut buf) {
//             Ok(_) => {
//                 // Get the byte length of the frame
//                 let len = buf.position() as usize;
//
//                 // Reset the internal cursor for the
//                 // call to `parse`.
//                 buf.set_position(0);
//
//                 // Parse the frame
//                 let frame = Frame::parse(&mut buf)?;
//
//                 // Discard the frame from the buffer
//                 self.buffer.advance(len);
//
//                 // Return the frame to the caller.
//                 Ok(Some(frame))
//             }
//             // Not enough data has been buffered
//             Err(Incomplete) => Ok(None),
//             // An error was encountered
//             Err(e) => Err(e.into()),
//         }
//     }
// }

use tokio::io::BufWriter;
use tokio::net::TcpStream;
use bytes::BytesMut;
use tokio::io::{self, AsyncWriteExt};
use mini_redis::Frame;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    async fn write_frame(&mut self, frame: &Frame)
                         -> io::Result<()>
    {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!(),
        }

        self.stream.flush().await;

        Ok(())
    }
}