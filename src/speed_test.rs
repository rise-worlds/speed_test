pub use crate::speed_test_structure::*;
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct ApiV3Response {
    code: i32,
    data: ApiV3LocationInfo,
    #[serde(rename = "'msg'")]
    msg: String,
}

#[derive(Deserialize, Debug)]
struct ApiV3LocationInfo {
    ip: String,
    country: String,
    province: String,
    city: String,
    district: String,
    isp: String,
    operator: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    lon: String,
    lat: String,
}

impl From<ApiV3LocationInfo> for LocationInfo {
    fn from(api_info: ApiV3LocationInfo) -> Self {
        LocationInfo {
            ip: api_info.ip.clone(),
            full_ip: api_info.ip.clone(),
            country: api_info.country,
            province: api_info.province,
            city: api_info.city,
            distinct: api_info.district,
            isp: api_info.isp,
            operator: api_info.operator,
            country_code: api_info.country_code,
            lon: api_info.lon,
            lat: api_info.lat,
            net_str: "".to_string(),
        }
    }
}

pub async fn get_location() -> Result<LocationInfo> {
    let url1 = "https://forge.speedtest.cn/api/location/info";
    let url2 = "https://api-v3.speedtest.cn/ip";

    let response1 = reqwest::get(url1).await;
    if let Ok(resp) = response1 {
        if resp.status().is_success() {
            let location = resp.json::<LocationInfo>().await?;
            return Ok(location);
        }
    }

    let response2 = reqwest::get(url2).await?;
    if response2.status().is_success() {
        let body = response2.text().await?;
        println!("{:#?}", body);
        let api_response: ApiV3Response = serde_json::from_str(&body).unwrap();
        // let api_response = response2.json::<ApiV3Response>().await?;
        return Ok(api_response.data.into());
    }

    Err(anyhow::anyhow!("Failed to get location from both URLs"))
}

pub async fn get_server_list(url: &str) -> Result<SpeedTestServerInfo> {
    let body = reqwest::get(url).await?.json::<SpeedTestServerInfo>().await?;
    Ok(body)
}
