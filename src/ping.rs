use futures_util::{SinkExt, StreamExt};
use tokio::time::Instant;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub async fn ping_server(ping_url: &str) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut i: i32 = 0;
    let mut ping: i32;
    let mut last_ping_time: i64 = 0;
    let mut ping_time: Vec<i64> = Vec::new();
    let mut jitter_temp: Vec<i32> = Vec::new();

    let (mut wss_stream, _response) = connect_async(ping_url).await?;

    wss_stream.send(Message::from("HI")).await?;

    while let Some(_msg) = wss_stream.next().await {
        let now = Instant::now().elapsed().as_millis() as i64; // 使用毫秒级别的精度
        i += 1;
        if i > 1 {
            jitter_temp.push((now - last_ping_time) as i32);
        }
        last_ping_time = now;
        if i < 10 {
            ping_time.push(now);
        }

        wss_stream.send(Message::from(format!("PING {}", now))).await?;

        if i >= 11 {
            ping_time.push(last_ping_time);
            break;
        }
    }

    if jitter_temp.is_empty() {
        return Err("No jitter data collected".into());
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

    Ok((jitter, ping))
}
