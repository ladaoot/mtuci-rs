// use tokio_stream::StreamExt;
//
// #[tokio::main]
// async fn main() {
//     let mut stream = tokio_stream::iter(&[1, 2, 3]);
//
//     while let Some(v) = stream.next().await {
//         println!("GOT = {:?}", v);
//     }
// }\

use tokio_stream::StreamExt;
use mini_redis::client;

async fn publish() -> mini_redis::Result<()> {
    println!("2");
    let mut client = client::connect("127.0.0.1:6379").await?;

    println!("3");
    // Publish some data
    client.publish("numbers", "1".into()).await?;
    println!("4");
    client.publish("numbers", "two".into()).await?;
    println!("5");
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;

    println!("6");
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
    .into_stream()
    .filter(|msg| match msg {
        Ok(msg) if msg.content.len() == 1 => true,
        _ => false,
    })
    .map(|msg| msg.unwrap().content)
    .take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    println!("1");
    tokio::spawn(async {
        publish().await
    });

    println!("10");

    subscribe().await?;

    println!("DONE");

    Ok(())
}