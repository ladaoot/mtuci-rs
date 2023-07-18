// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};
//
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("tokio/src/foo.txt").await?;
//     let mut buffer = [0; 10];
//
//     // read up to 10 bytes
//     let n = f.read(&mut buffer[..]).await?;
//
//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }

// use tokio::io::{self, AsyncReadExt};
// use tokio::fs::File;
//
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("tokio/src/foo.txt").await?;
//     let mut buffer = Vec::new();
//
//     let mut buf = [0;10];
//     let n = f.read(&mut buf[..]).await?;
//     println!("The bytes 10: {:?}", &buf[..n]);
//
//
//     let mut f = File::open("tokio/src/foo.txt").await?;
//     // read the whole file
//     f.read_to_end(&mut buffer).await?;
//     println!("The bytes: {:?}", &buffer);
//     Ok(())
// }

// use tokio::io::{self, AsyncWriteExt};
// use tokio::fs::File;
//
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo.txt").await?;
//
//     // Writes some prefix of the byte string, but not necessarily all of it.
//     let n = file.write(b"some bytes").await?;
//
//     println!("Wrote the first {} bytes of 'some bytes'.", n);
//     Ok(())
// }

// use tokio::io::{self, AsyncWriteExt};
// use tokio::fs::File;
//
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo1.txt").await?;
//
//     file.write_all(b"some bytes").await?;
//     Ok(())
// }

// use tokio::fs::File;
// use tokio::io;
//
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut reader: &[u8] = b"hello";
//     let mut file = File::create("foo.txt").await?;
//
//     io::copy(&mut reader, &mut file).await?;
//     Ok(())
// }

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        // Sometimes, the rust type inferencer needs
        // a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}