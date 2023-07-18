// // // // // // // // // use tokio::sync::oneshot;
// // // // // // // // //
// // // // // // // // // #[tokio::main]
// // // // // // // // // async fn main() {
// // // // // // // // //     let (tx1, rx1) = oneshot::channel();
// // // // // // // // //     let (tx2, rx2) = oneshot::channel();
// // // // // // // // //
// // // // // // // // //     tokio::spawn(async {
// // // // // // // // //         let _ = tx1.send("one");
// // // // // // // // //     });
// // // // // // // // //
// // // // // // // // //     tokio::spawn(async {
// // // // // // // // //         let _ = tx2.send("two");
// // // // // // // // //     });
// // // // // // // // //
// // // // // // // // //     tokio::select! {
// // // // // // // // //         val = rx1 => {
// // // // // // // // //             println!("rx1 completed first with {:?}", val);
// // // // // // // // //         }
// // // // // // // // //         val = rx2 => {
// // // // // // // // //             println!("rx2 completed first with {:?}", val);
// // // // // // // // //         }
// // // // // // // // //     }
// // // // // // // // // }
// // // // // // // //
// // // // // // // // use tokio::sync::oneshot;
// // // // // // // //
// // // // // // // // async fn some_operation() -> String {
// // // // // // // //     // Compute value here
// // // // // // // //     "op".to_string()
// // // // // // // // }
// // // // // // // //
// // // // // // // // #[tokio::main]
// // // // // // // // async fn main() {
// // // // // // // //     let (mut tx1, rx1) = oneshot::channel();
// // // // // // // //     let (tx2, rx2) = oneshot::channel();
// // // // // // // //
// // // // // // // //     tokio::spawn(async {
// // // // // // // //         // Select on the operation and the oneshot's
// // // // // // // //         // `closed()` notification.
// // // // // // // //         tokio::select! {
// // // // // // // //             val = some_operation() => {
// // // // // // // //                 let _ = tx1.send(val);
// // // // // // // //             }
// // // // // // // //             _ = tx1.closed() => {
// // // // // // // //                 // `some_operation()` is canceled, the
// // // // // // // //                 // task completes and `tx1` is dropped.
// // // // // // // //             }
// // // // // // // //         }
// // // // // // // //     });
// // // // // // // //
// // // // // // // //     tokio::spawn(async {
// // // // // // // //         let _ = tx2.send("two");
// // // // // // // //     });
// // // // // // // //
// // // // // // // //     tokio::select! {
// // // // // // // //         val = rx1 => {
// // // // // // // //             println!("rx1 completed first with {:?}", val);
// // // // // // // //         }
// // // // // // // //         val = rx2 => {
// // // // // // // //             println!("rx2 completed first with {:?}", val);
// // // // // // // //         }
// // // // // // // //     }
// // // // // // // // }
// // // // // // //
// // // // // // // use tokio::sync::oneshot;
// // // // // // // use std::future::Future;
// // // // // // // use std::pin::Pin;
// // // // // // // use std::task::{Context, Poll};
// // // // // // //
// // // // // // // struct MySelect {
// // // // // // //     rx1: oneshot::Receiver<&'static str>,
// // // // // // //     rx2: oneshot::Receiver<&'static str>,
// // // // // // // }
// // // // // // //
// // // // // // // impl Future for MySelect {
// // // // // // //     type Output = ();
// // // // // // //
// // // // // // //     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
// // // // // // //         if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
// // // // // // //             println!("rx1 completed first with {:?}", val);
// // // // // // //             return Poll::Ready(());
// // // // // // //         }
// // // // // // //
// // // // // // //         if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
// // // // // // //             println!("rx2 completed first with {:?}", val);
// // // // // // //             return Poll::Ready(());
// // // // // // //         }
// // // // // // //
// // // // // // //         Poll::Pending
// // // // // // //     }
// // // // // // // }
// // // // // // //
// // // // // // // #[tokio::main]
// // // // // // // async fn main() {
// // // // // // //     let (tx1, rx1) = oneshot::channel();
// // // // // // //     let (tx2, rx2) = oneshot::channel();
// // // // // // //
// // // // // // //     // use tx1 and tx2
// // // // // // //
// // // // // // //     tokio::spawn(async {
// // // // // // //         let _ = tx1.send("one");
// // // // // // //     });
// // // // // // //     tokio::spawn(async {
// // // // // // //         let _ = tx2.send("two");
// // // // // // //     });
// // // // // // //
// // // // // // //
// // // // // // //     MySelect {
// // // // // // //         rx1,
// // // // // // //         rx2,
// // // // // // //     }.await;
// // // // // // // }
// // // // // //
// // // // // //
// // // // // // use tokio::net::TcpStream;
// // // // // // use tokio::sync::oneshot;
// // // // // //
// // // // // // #[tokio::main]
// // // // // // async fn main() {
// // // // // //     let (tx, rx) = oneshot::channel();
// // // // // //
// // // // // //     // Spawn a task that sends a message over the oneshot
// // // // // //     tokio::spawn(async move {
// // // // // //
// // // // // //         tx.send("done").unwrap();
// // // // // //     });
// // // // // //
// // // // // //     tokio::select! {
// // // // // //         socket = TcpStream::connect("localhost:3465") => {
// // // // // //             println!("Socket connected {:?}", socket);
// // // // // //         }
// // // // // //         msg = rx => {
// // // // // //             println!("received message first {:?}", msg);
// // // // // //         }
// // // // // //     }
// // // // // // }
// // // // //
// // // // // use tokio::net::TcpListener;
// // // // // use tokio::sync::oneshot;
// // // // // use std::io;
// // // // //
// // // // // #[tokio::main]
// // // // // async fn main() -> io::Result<()> {
// // // // //     let (tx, rx) = oneshot::channel();
// // // // //
// // // // //     tokio::spawn(async move {
// // // // //         tx.send(()).unwrap();
// // // // //     });
// // // // //
// // // // //     let mut listener = TcpListener::bind("localhost:3465").await?;
// // // // //
// // // // //     tokio::select! {
// // // // //         _ = async {
// // // // //             loop {
// // // // //                 let (socket, _) = listener.accept().await?;
// // // // //                 tokio::spawn(async move { process(socket) });
// // // // //             }
// // // // //
// // // // //             // Help the rust type inferencer out
// // // // //             Ok::<_, io::Error>(())
// // // // //         } => {}
// // // // //         _ = rx => {
// // // // //             println!("terminating accept loop");
// // // // //         }
// // // // //     }
// // // // //
// // // // //     Ok(())
// // // // // }
// // // //
// // // //
// // // // async fn computation1() -> String {
// // // //     // .. computation
// // // //     "1".to_string()
// // // // }
// // // //
// // // // async fn computation2() -> String {
// // // //     // .. computation
// // // //     "2".to_string()
// // // // }
// // // //
// // // // #[tokio::main]
// // // // async fn main() {
// // // //     let out = tokio::select! {
// // // //         res1 = computation1() => res1,
// // // //         res2 = computation2() => res2,
// // // //     };
// // // //
// // // //     println!("Got = {}", out);
// // // // }
// // //
// // // use tokio::net::{TcpListener, TcpStream};
// // // use tokio::sync::oneshot;
// // // use std::io;
// // //
// // // #[tokio::main]
// // // async fn main() -> io::Result<()> {
// // //     // [setup `rx` oneshot channel]
// // //     let (tx, rx) = oneshot::channel();
// // //
// // //     let listener = TcpListener::bind("localhost:3465").await?;
// // //
// // //     tokio::select! {
// // //         res = async {
// // //             loop {
// // //                 let (socket, _) = listener.accept().await?;
// // //                 tokio::spawn(async move { process(socket)});
// // //             }
// // //
// // //             // Help the rust type inferencer out
// // //             Ok::<_, io::Error>(())
// // //         } => {
// // //             res?;
// // //         }
// // //         _ = rx => {
// // //             println!("terminating accept loop");
// // //         }
// // //     }
// // //
// // //     Ok(())
// // // }
// //
// //
// // use futures::TryFutureExt;
// // use tokio::sync::mpsc;
// //
// // #[tokio::main]
// // async fn main() {
// //     let (mut tx1, mut rx1) = mpsc::channel(128);
// //     let (mut tx2, mut rx2) = mpsc::channel(128);
// //
// //     tokio::spawn(async move {
// //         // Do something w/ `tx1` and `tx2`
// //         tx1.send("1").await.unwrap();
// //         tx2.send("2").await.unwrap();
// //     });
// //
// //     tokio::select! {
// //         Some(v) = rx1.recv() => {
// //             println!("Got {:?} from rx1", v);
// //         }
// //         Some(v) = rx2.recv() => {
// //             println!("Got {:?} from rx2", v);
// //         }
// //         else => {
// //             println!("Both channels closed");
// //         }
// //     }
// // }
//
// use tokio::sync::mpsc;
//
// #[tokio::main]
// async fn main() {
//     let (tx1, mut rx1) = mpsc::channel(128);
//     let (tx2, mut rx2) = mpsc::channel(128);
//     let (tx3, mut rx3) = mpsc::channel(128);
//
//     tokio::spawn(  async move{
//         tx1.send("1").await.unwrap();
//         tx2.send("2").await.unwrap();
//         tx3.send("3").await.unwrap();
//     });
//
//     loop {
//         let msg = tokio::select! {
//             Some(msg) = rx1.recv() => msg,
//             Some(msg) = rx2.recv() => msg,
//             Some(msg) = rx3.recv() => msg,
//             else => { break }
//         };
//
//         println!("Got {:?}", msg);
//     }
//
//     println!("All channels have been closed.");
// }

async fn action(input: Option<i32>) -> Option<String> {
    // If the input is `None`, return `None`.
    // This could also be written as `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    // async logic here
    Some(i.to_string())
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(2).await;
        let _ = tx.send(4).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    // `.set` is a method on `Pin`.
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        }
    }
}