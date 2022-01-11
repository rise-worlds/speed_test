use std::str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LocationInfo
{
    ip: String,
    full_ip: String,
    country: String,
    country_code: String,
    province: String,
    city: String,
    distinct: String,
    isp: String,
    operator: String,
    lon: String,
    lat: String,
    net_str: String,
}

#[derive(Deserialize, Debug)]
pub struct SpeedTestServerInfo
{
    id: String,
    active: String,
    https: String,
    cros: String,
    preferred: String,
    host: String,
    ver: String,
    lon: String,
    lat: String,
    distance: i32,
    pingUrl: String,
    downloadUrl: String,
    uploadUrl: String,
    websocketUrl: String,
}