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
//
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;
//
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let socket = TcpStream::connect("127.0.0.1:6142").await?;
//     let (mut rd, mut wr) = io::split(socket);
//
//     // Write data in the background
//     tokio::spawn(async move {
//         wr.write_all(b"hello\r\n").await?;
//         wr.write_all(b"world\r\n").await?;
//
//         // Sometimes, the rust type inferencer needs
//         // a little help
//         Ok::<_, io::Error>(())
//     });
//
//     let mut buf = vec![0; 128];
//
//     loop {
//         let n = rd.read(&mut buf).await?;
//
//         if n == 0 {
//             break;
//         }
//
//         println!("GOT {:?}", &buf[..n]);
//     }
//
//     Ok(())
// }

// use tokio::net::TcpStream;
//
// async fn my_async_fn() {
//     println!("hello from async");
//     let _socket = TcpStream::connect("127.0.0.1:8888").await.unwrap();
//     println!("async TCP operation complete");
// }
//
// #[tokio::main]
// async fn main() {
//     let what_is_this = my_async_fn();
//     // Nothing has been printed yet.
//     what_is_this.await;
//     // Text has been printed and socket has been
//     // established and closed.
// }

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::thread;
use futures::task;


struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}


enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await` line.
    State1(Delay),
    // The future has completed.
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<()>
    {
        use MainFuture::*;

        loop {
            match *self {
                State0 => {
                    let when = Instant::now() +
                        Duration::from_millis(10);
                    let future = Delay { when };
                    *self = State1(future);
                    // println!("State0");
                }
                State1(ref mut my_future) => {
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            // println!("State1");
                            assert_eq!(out, "done");
                            *self = Terminated;

                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            // println!("Pen");
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    // println!("Terminated");
                    panic!("future polled after completion")
                }
            }
        }
    }
}


#[tokio::main]
fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}

struct MiniTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output=()> + Send>>;

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    /// Spawn a future onto the mini-tokio instance.
    fn spawn<F>(&mut self, future: F)
        where
            F: Future<Output=()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}