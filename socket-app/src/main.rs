use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:9001";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Error during the websocket handshake");
            println!(
                "New WebSocket connection: {}",
                ws_stream.get_ref().peer_addr().unwrap()
            );

            let (mut write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                let message = message.expect("Error reading message");
                if message.is_text() || message.is_binary() {
                    write.send(message).await.expect("Error sending message");
                }
            }
        });
    }
}
