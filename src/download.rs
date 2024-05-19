use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use anyhow::{Error, Result};
use futures::future::join_all;
use futures::StreamExt;
use reqwest::header;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, RANGE};
use reqwest::IntoUrl;
use tokio::sync::Semaphore;

static TOTAL_DOWNLOAD_BYTES: AtomicU64 = AtomicU64::new(0);
#[warn(dead_code)]
async fn check_request_range<U: IntoUrl>(url: U) -> Result<(bool, u64)> {
    let mut range = false;
    let client = reqwest::Client::new();
    let response = client
        .head(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(Error::msg("request fail"));
    }
    let headers = response.headers();
    if headers
        .get(ACCEPT_RANGES)
        .map(|val| (val.to_str().ok()?.eq("bytes")).then(|| ()))
        .flatten()
        .is_some()
    {
        range = true;
    }
    let length = headers
        .get(CONTENT_LENGTH)
        .map(|val| val.to_str().ok())
        .flatten()
        .map(|val| val.parse().ok())
        .flatten()
        .ok_or(Error::msg("get length fail"))?;
    Ok((range, length))
}

async fn download_partial<U: IntoUrl>(url: U, (start, end): (u64, u64)) -> Result<u64, Error> {
    let client = reqwest::Client::new();

    let req = client.get(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .header(RANGE, format!("bytes={}-{}", start, end));

    let rep = req.send().await.unwrap();
    if !rep.status().is_success() {
        return Err(Error::msg("request fail"));
    }
    let mut download_size = 0;
    let mut stream = rep.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let size = chunk.len() as u64;
        TOTAL_DOWNLOAD_BYTES.fetch_add(size, Ordering::Relaxed);
        download_size += size;
        // println!("download part {}:{}, {}", start, end, download_size);
    }
    Ok(download_size)
}

pub async fn download(url: &str, total_size: u64, chunk_size: u64, parallel_task_num: usize) -> Result<u64> {
    let download_url = std::format!(
        "{}?size={}&r={}",
        url,
        total_size,
        rand::random::<f64>()
    );
    let url: &str = &download_url.clone();
    let semaphore = Arc::new(Semaphore::new(parallel_task_num));

    TOTAL_DOWNLOAD_BYTES.store(0, Ordering::Relaxed);
    // 计算需要下载的分段数量
    let chunks = (total_size as f64 / chunk_size as f64).ceil() as u64;
    let mut tasks  = vec![];
    for i in 0..chunks {
        // 线程数必须大于等于1
        let semaphore = semaphore.clone();
        let url = url.to_string();

        // 每个任务在开始前都会获取信号量的许可
        let permit = semaphore.acquire_owned().await.unwrap();
        let task = tokio::spawn(async move {
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, total_size);

            println!("start download part {}:{}", start, end);
            let result = download_partial(url, (start, end)).await;
            if result.is_ok() {
                println!("download part {}:{} end {}", start, end, result.unwrap());
            } else {
                println!("download part end {}:{} error: {}", start, end, result.unwrap_err());
            }

            // 释放信号量的许可
            drop(permit);
        });
        tasks.push(task);
    }
    // 等待所有任务完成
    join_all(tasks).await;

    Ok(TOTAL_DOWNLOAD_BYTES.load(Ordering::Relaxed))
}
