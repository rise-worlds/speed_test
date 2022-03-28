mod speed_test_structure;

pub extern crate futures;
pub extern crate tokio;

use futures_util::{future, pin_mut, SinkExt, StreamExt};
use reqwest;
use std::error::Error;
use std::fmt::format;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::*;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tungstenite::*;
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

async fn ping_server(url: &str) -> i32 {
    let ping: i32 = 0;

    let request = Url::parse(url).unwrap();
    let (mut ws_stream, _) = connect_async(request).await.expect("Failed to connect");
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await;
        }
    }

    return ping;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // tokio::spawn(async move {
    // });

    let location = get_location("https://forge.speedtest.cn/api/location/info").await?;
    // println!("{:?}", location);

    let url = format(format_args!(
        "https://nodes.speedtest.cn/?https=1&browser=1&page=1&lat={}&lon={}&q=",
        location.lat, location.lon
    ));
    // println!("{:?}", url);
    let servers = get_server_list(url.as_str()).await?;
    // println!("{:?}", servers);

    let recent_server = servers.data.get(0).unwrap();
    println!("{:?}", recent_server);

    let ws = recent_server.websocket_url.as_str();
    let ping = ping_server(ws).await;
    println!("{:?}, ping:{:?}", ws, ping);

    Ok(())
}
