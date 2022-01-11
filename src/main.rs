mod speed_test_structure;

use reqwest;
use std::collections::HashMap;
use std::fmt::format;
use std::str;
use serde::{Deserialize};

use speed_test_structure::*;

async fn get_location(url:&str) -> Result<(LocationInfo), reqwest::Error> {
    let body = reqwest::get(url).await?.json::<LocationInfo>().await?;

    println!("body = {:?}", body);

    Ok(body)
}

async fn get_server_list(url: &str) -> Result<([SpeedTestServerInfo]), reqwest::Error> {
    let body = reqwest::get(url).await?.json().await?;

    println!("body = {:?}", body);

    let tmp: [SpeedTestServerInfo] = body["data"];
    Ok(tmp)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // tokio::spawn(async move {
    // });

    let location = get_location("https://forge.speedtest.cn/api/location/info").await;
    println!("{:?}", location);

    let url = format(format_args!("https://nodes.speedtest.cn/?https=1&browser=1&page=1&lat={}&lon={}&q=", location.lat, location.lon)).as_str();
    let servers = get_server_list(url).await;
    println!("{:?}", servers);
}
