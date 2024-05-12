
pub use crate::speed_test_structure::*;

pub async fn get_location(url: &str) -> Result<LocationInfo, reqwest::Error> {
    let body = reqwest::get(url).await?.json::<LocationInfo>().await?;

    //println!("body = {:?}", body);
    Ok(body)
}

pub async fn get_server_list(url: &str) -> Result<SpeedTestServerInfo, reqwest::Error> {
    let body = reqwest::get(url).await?.json::<SpeedTestServerInfo>().await?;

    //println!("body = {:?}", body);
    Ok(body)
}
