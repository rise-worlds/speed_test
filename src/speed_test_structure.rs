use std::str;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct LocationInfo
{
    pub ip: String,
    pub full_ip: String,
    pub country: String,
    pub country_code: String,
    pub province: String,
    pub city: String,
    pub distinct: String,
    pub isp: String,
    pub operator: String,
    pub lon: String,
    pub lat: String,
    pub net_str: String,
}

#[derive(Deserialize, Debug)]
pub struct SpeedTestServerItem
{
    pub id: String,
    pub active: String,
    pub https: String,
    pub cros: String,
    pub preferred: String,
    pub host: String,
    pub ver: String,
    pub lon: String,
    pub lat: String,
    pub distance: i32,
    #[serde(rename(deserialize = "pingUrl"))]
    pub ping_url: String,
    #[serde(rename(deserialize = "downloadUrl"))]
    pub download_url: String,
    #[serde(rename(deserialize = "uploadUrl"))]
    pub upload_url: String,
    #[serde(rename(deserialize = "websocketUrl"))]
    pub websocket_url: String,
}

#[derive(Deserialize, Debug)]
pub struct SpeedTestServerMeta
{
    pub current_page: i32,
    pub last_page: i32,
    pub per_page: i32,
    pub  total: i32,
    pub ip: String
}

#[derive(Deserialize, Debug)]
pub struct SpeedTestServerInfo
{
    pub data: Vec<SpeedTestServerItem>,
    pub meta: SpeedTestServerMeta
}
