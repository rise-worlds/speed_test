mod speed_test_structure;

pub extern crate tokio;
use chrono::prelude::*;
use futures_util::{SinkExt, Stream, StreamExt};
// use http::HeaderMap;
use native_tls::TlsConnector;
use reqwest;
use std::error::Error;
use std::fmt::{format, Debug};
use std::pin::Pin;
use std::str;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
// use tokio_tungstenite::*;
use tokio_tungstenite::{
    connect_async, connect_async_tls_with_config, tungstenite::protocol::Message, Connector,
};
// use tungstenite::*;
// use tungstenite::{connect, Message};
use tungstenite::client::IntoClientRequest;
use tungstenite::error::UrlError;
use url::{ParseError, Url};

pub use speed_test_structure::*;

async fn get_location(url: &str) -> Result<LocationInfo, reqwest::Error> {
    let body = reqwest::get(url).await?.json::<LocationInfo>().await?;

    //println!("body = {:?}", body);
    Ok(body)
}

async fn get_server_list(url: &str) -> Result<SpeedTestServerInfo, reqwest::Error> {
    let body = reqwest::get(url)
        .await?
        .json::<SpeedTestServerInfo>()
        .await?;

    //println!("body = {:?}", body);
    Ok(body)
}

fn timestamp() -> i64 {
    let dt = Local::now();
    dt.timestamp_millis()
}

async fn ping_server(ping_url: &str) -> Result<i32, tungstenite::Error> {
    let mut i: i32 = 0;
    let mut ping: i32 = 0;
    let mut last_ping_time: i64 = 0;
    let mut ping_time: Vec<i64> = Vec::new();
    let mut jitter_temp: Vec<i32> = Vec::new();

    // let mut url = Url::parse(ping_url).expect("Can't connect to case count URL");
    // url.query_pairs_mut().append_pair("transport", "websocket");
    // url.set_scheme("wss").unwrap();

    // // let (mut wss_stream, _response) = connect_async(url).await.expect("Failed to connect");
    // let (mut wss_stream, _response) = connect_async_tls_with_config(url, None, None)
    //     .await
    //     .expect("Failed to connect");

    // wss_stream
    //     .send(Message::from("HI"))
    //     .await
    //     .expect("Failed to send HI");
    //
    // while let Some(_msg) = wss_stream.next().await {
    //     let now = timestamp();
    //     i += 1;
    //     if i > 1 {
    //         jitter_temp.push((now - last_ping_time) as i32);
    //     }
    //     last_ping_time = now;
    //     if i < 10 {
    //         ping_time.push(now);
    //     }
    //
    //     wss_stream
    //         .send(Message::from(format!("PING {}", now)))
    //         .await
    //         .expect("Failed for send message");
    //
    //     if i >= 11 {
    //         ping_time.push(last_ping_time);
    //         break;
    //     }
    // }
    //
    // let mut jitter_sum = 0;
    // let mut jitter_list: Vec<i32> = Vec::new();
    // let count = jitter_temp.len();
    // for i in 0..count {
    //     let temp = jitter_temp[i];
    //     if temp < ping {
    //         ping = temp;
    //     }
    //     if i > 0 {
    //         let jitter = (jitter_temp[i + 1] - temp).abs() as i32;
    //         jitter_sum += jitter;
    //         jitter_list.push(jitter);
    //     }
    // }
    // let jitter = jitter_sum / count as i32;
    // println!("jitter:{}", jitter);

    let uri = Uri::from_static(ping_url);
    let (mut client, _) = ClientBuilder::from_uri(uri).connect().await?;
    client.send(Message::from("HI")).await?;

    while let Some(Ok(msg)) = client.next().await {
        let now = timestamp();   //     i += 1;
        if i > 1 {
            jitter_temp.push((now - last_ping_time) as i32);
        }
        last_ping_time = now;
        if i < 10 {
            ping_time.push(now);
        }
        if let Some(text) = msg.as_text() {
            assert_eq!(text, "Hello world!");
            // We got one message, just stop now
            client.close().await?;
        }
    }

    Ok(ping)
}

async fn download_server(download_url: &str) -> Result<i32, tungstenite::Error> {
    OK(0)
}

async fn upload_server(upload_url: &str) -> Result<i32, tungstenite::Error> {
    OK(0)
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

    let ping_url = recent_server.websocket_url.as_str();
    let ping = ping_server(ping_url).await?;
    println!("{:?}, ping:{:?}", ping_url, ping);

    let download_url = recent_server.download_url.as_str();
    let download = download_server(download_url).await?;
    println!("{:?}, download:{:?}", download_url, download);

    let upload_url = recent_server.upload_url.as_str();
    let upload = upload_server(upload_url).await?;
    println!("{:?}, upload:{:?}", upload_url, upload);

    Ok(())
}
