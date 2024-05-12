
use futures_util::{SinkExt, StreamExt};
use tokio::time::Instant;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message};
use url::Url;
pub async fn ping_server(ping_url: &str) -> Result<(i32, i32), tungstenite::error::Error> {
    let mut i: i32 = 0;
    let mut ping: i32;
    let mut last_ping_time: i64 = 0;
    let mut ping_time: Vec<i64> = Vec::new();
    let mut jitter_temp: Vec<i32> = Vec::new();

    let mut url = Url::parse(ping_url).expect("Can't connect to case count URL");
    url.query_pairs_mut().append_pair("transport", "websocket");
    url.set_scheme("wss").unwrap();

    // let (mut wss_stream, _response) = connect_async(url).await.expect("Failed to connect");
    let (mut wss_stream, _response) = connect_async_tls_with_config(url, None, false, None)
        .await
        .expect("Failed to connect");

    wss_stream
        .send(Message::from("HI"))
        .await
        .expect("Failed to send HI");

    while let Some(_msg) = wss_stream.next().await {
        let now = (Instant::now()).elapsed().as_secs() as i64;
        i += 1;
        if i > 1 {
            jitter_temp.push((now - last_ping_time) as i32);
        }
        last_ping_time = now;
        if i < 10 {
            ping_time.push(now);
        }

        wss_stream
            .send(Message::from(format!("PING {}", now)))
            .await
            .expect("Failed for send message");

        if i >= 11 {
            ping_time.push(last_ping_time);
            break;
        }
    }

    let mut jitter_sum = 0;
    let mut jitter_list: Vec<i32> = Vec::new();
    ping = jitter_temp[0];
    let count = jitter_temp.len();
    for i in 0..(count - 1) {
        let temp = jitter_temp[i];
        if temp < ping {
            ping = temp;
        }
        if i > 0 {
            let jitter = (jitter_temp[i + 1] - temp).abs() as i32;
            jitter_sum += jitter;
            jitter_list.push(jitter);
        }
    }
    let jitter = jitter_sum / count as i32;
    // println!("jitter:{}", jitter);

    Ok((jitter, ping))
}