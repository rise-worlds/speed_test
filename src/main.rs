use reqwest;
use tokio::task;

async fn getBody() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://httpbin.org/ip").await?.text().await?;

    println!("body = {:?}", body);

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // tokio::spawn(async move {
    //     let test = getBody().await;
    //     println!("{:#?}", test);
    // });
    let test = getBody().await;
    println!("{:#?}", test);
}
