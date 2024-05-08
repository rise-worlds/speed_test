use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, thread, time};

use futures_util::{future, pin_mut, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let ten_millis = time::Duration::from_millis(500);

    loop {
        let url = std::format!(
            "hello {}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        );
        tx.unbounded_send(Message::binary(url.as_bytes())).unwrap();
        thread::sleep(ten_millis);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connect_addr = env::args().nth(1).unwrap_or_else(|| "ws://127.0.0.1:8080/".to_string());

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            println!("Received a message : {}", message.unwrap().to_text().unwrap());
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;

    Ok(())
}
