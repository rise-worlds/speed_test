
pub extern crate tokio;
use std::error::Error;

pub extern crate rand;

pub use speed_test::speed_test_structure::*;
pub use speed_test::ping::*;
pub use speed_test::download::*;
pub use speed_test::upload::*;
pub use speed_test::speed_test::*;

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

    let download_url = recent_server.download_url.as_str();
    let download = download(download_url, 500_000_000, 1_000_000, 8).await?;
    println!("{:?}, download:{:?}", download_url, download);

    // let upload_url = recent_server.upload_url.as_str();
    // let upload = upload_server(upload_url).await?;
    // println!("{:?}, upload:{:?}", upload_url, upload);

    Ok(())
}
