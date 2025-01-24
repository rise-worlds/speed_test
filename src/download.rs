use anyhow::{Error, Result};
use futures::future::join_all;
use futures::StreamExt;
use reqwest::header;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, RANGE};
use reqwest::IntoUrl;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::time::Instant;

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

pub async fn download(url: &str, total_size: u64, chunk_size: u64, parallel_task_num: usize) -> Result<(i32, u64)> {
    let download_url = std::format!("{}?size={}&r={}", url, total_size, rand::random::<f64>());
    let url: &str = &download_url.clone();
    let semaphore = Arc::new(Semaphore::new(parallel_task_num));

    TOTAL_DOWNLOAD_BYTES.store(0, Ordering::Relaxed);
    // let mut jitter_temp: Vec<i32> = Vec::new();
    let jitter_temp = Arc::new(Mutex::new(Vec::new()));

    // 计算需要下载的分段数量
    let chunks = (total_size as f64 / chunk_size as f64).ceil() as u64;
    let mut tasks = vec![];
    tasks.push(tokio::spawn(async {
        let mut last_count: u64 = 0;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let count = TOTAL_DOWNLOAD_BYTES.load(Ordering::Relaxed);
            println!("download {}b/s", count - last_count);
            last_count = count;
        }
    }));
    for i in 0..chunks {
        // 线程数必须大于等于1
        let semaphore = semaphore.clone();
        let url = url.to_string();
        let mut last_download_time: i64 = 0;
        let jitter_temp_clone = Arc::clone(&jitter_temp);

        // 每个任务在开始前都会获取信号量的许可
        let permit = semaphore.acquire_owned().await?;
        let task = tokio::spawn(async move {
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, total_size);

            let now = Instant::now().elapsed().as_millis() as i64; // 使用毫秒级别的精度
            if i > 0 {
                let mut guard = jitter_temp_clone.lock().await;
                guard.push((now - last_download_time) as i32);
            }
            last_download_time = now;

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

    // if jitter_temp.is_empty() {
    //     return Err(Error::msg("No jitter data collected"));
    // }

    let mut jitter_sum = 0;
    let mut jitter_temp_guard = jitter_temp.lock().await;
    let count = jitter_temp_guard.len();
    for i in 0..(count - 1) {
        let temp = jitter_temp_guard[i];
        let jitter = (jitter_temp_guard[i + 1] - temp).abs() as i32;
        jitter_sum += jitter;
    }
    let jitter = jitter_sum / count as i32;

    Ok((jitter, TOTAL_DOWNLOAD_BYTES.load(Ordering::Relaxed)))
}
