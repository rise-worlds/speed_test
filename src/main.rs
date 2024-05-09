mod speed_test_structure;

pub extern crate tokio;
use chrono::prelude::*;
use futures_util::{SinkExt, StreamExt};
use reqwest;
use std::error::Error;
use std::str;
use std::time::Duration;

pub extern crate rand;
use tokio::time::Instant;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message};
use url::Url;

pub use speed_test_structure::*;

async fn get_location(url: &str) -> Result<LocationInfo, reqwest::Error> {
    let body = reqwest::get(url).await?.json::<LocationInfo>().await?;

    //println!("body = {:?}", body);
    Ok(body)
}

async fn get_server_list(url: &str) -> Result<SpeedTestServerInfo, reqwest::Error> {
    let body = reqwest::get(url).await?.json::<SpeedTestServerInfo>().await?;

    //println!("body = {:?}", body);
    Ok(body)
}

fn timestamp() -> i64 {
    let dt = Local::now();
    dt.timestamp_millis()
}

async fn ping_server(ping_url: &str) -> Result<(i32, i32), tungstenite::error::Error> {
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
        let now = timestamp();
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

async fn download_server(download_url: &str) -> Result<(i32, i32), reqwest::Error> {
    let mut response = reqwest::get(download_url).await?;
    let total_size = response.content_length().unwrap_or(0);
    let mut jitter = 0;
    let mut downloaded_size: f64 = 0.0;
    // 初始化下载的字节数
    let mut total_bytes: f64 = 0.0;
    let start_time = Instant::now();
    // 检查是否成功响应
    // if response.status().is_success() {
    //     // 当前时间
    //     let mut current = start_time;
    //
    //     // 循环直到30秒结束
    //     while current.duration_since(start_time) < Duration::new(30, 0) {
    //         // 检查是否有可读的数据
    //         let bytes_read = response.content_length().unwrap_or(0) as f64;
    //         if bytes_read == 0.0 {
    //             break;
    //         }
    //
    //         // 更新总字节数
    //         total_bytes += bytes_read;
    //
    //         // 更新当前时间
    //         current = Instant::now();
    //     }

        while let Some(chunk) = response.chunk().await? {
            downloaded_size += chunk.len() as f64;
            // 计算下载速度
            let elapsed_time = start_time.elapsed().as_secs_f64();
            let download_speed = (downloaded_size / elapsed_time) / 1024.0; // KB/s
            println!(
                "Downloaded {:.2}% ({:.2} KB/s)",
                (downloaded_size / total_size as f64) * 100.0,
                download_speed
            );
            // download_time.push(elapsed_time as i64);
            // jitter_temp.push(elapsed_time as i32);
        }
        //
        // let mut jitter_sum: i32 = 0;
        // let mut jitter_list: Vec<i32> = Vec::new();
        // let mut speed = jitter_temp[0];
        // let count = jitter_temp.len();
        // for i in 0..(count - 1) {
        //     let temp = jitter_temp[i];
        //     if temp < speed {
        //         speed = temp;
        //     }
        //     if i > 0 {
        //         let jitter = (jitter_temp[i + 1] - temp).abs() as i32;
        //         jitter_sum += jitter;
        //         jitter_list.push(jitter);
        //     }
        // }
        // jitter = jitter_sum / count as i32;
    // }
    let elapsed_time = start_time.elapsed().as_secs_f64();
    let download_speed = (total_bytes / elapsed_time) / 1024.0; // KB/s
    Ok((jitter, download_speed as i32))
}

async fn upload_server(_upload_url: &str) -> Result<(i32, i32), tungstenite::Error> {
    Ok((0, 0))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");
    // tokio::spawn(async move {
    // });

    let location = get_location("https://forge.speedtest.cn/api/location/info").await?;
    // println!("{:?}", location);

    let url = std::format!(
        "https://nodes.speedtest.cn/?https=1&browser=1&page=1&lat={}&lon={}&q=",
        location.lat,
        location.lon
    );
    // println!("{:?}", url);
    let servers = get_server_list(url.as_str()).await?;
    // println!("{:?}", servers);

    let recent_server = servers.data.get(0).unwrap();
    println!("{:?}", recent_server);

    // let ping_url = recent_server.websocket_url.as_str();
    // let (jitter, ping) = ping_server(ping_url).await?;
    // println!("{:?}, ping:{:?}, jitter:{:?}", ping_url, ping, jitter);

    let download_url = std::format!(
        "{}?size=50000000&r={}",
        recent_server.download_url.as_str(),
        rand::random::<f64>()
    );
    let (jitter, download) = download_server(download_url.as_str()).await?;
    println!("{:?}, download:{:?}, jitter:{:?}", download_url, download, jitter);

    let upload_url = recent_server.upload_url.as_str();
    let upload = upload_server(upload_url).await?;
    println!("{:?}, upload:{:?}", upload_url, upload);

    Ok(())
}
