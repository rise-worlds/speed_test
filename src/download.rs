use std::sync::Arc;
use anyhow::{Error, Result};
use futures::future::join_all;
use futures::StreamExt;
use reqwest::header;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, RANGE};
use reqwest::IntoUrl;
use tokio::sync::Semaphore;
use tokio::time::Instant;

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

async fn download_partial<U: IntoUrl>(url: U, (mut start, end): (u64, u64)) -> Result<()> {
    let req = reqwest::Client::new().get(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3");

    let req = {
        if end == u64::MAX {
            req.header(RANGE, format!("bytes={}-{}", start, ""))
        } else {
            req.header(RANGE, format!("bytes={}-{}", start, end))
        }
    };
    let rep = req.send().await?;
    if !rep.status().is_success() {
        return Err(Error::msg("request fail"));
    }
    let mut stream = rep.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        start += chunk.len() as u64;
    }
    Ok(())
}

pub async fn download(url: &str, download_size: u64, task_length: u64, parallel_task_num: usize) -> Result<()> {
    let download_url = std::format!(
        "{}?size={}&r={}",
        url,
        download_size,
        rand::random::<f64>()
    );
    let url: &str = &download_url.clone();
    let mut handles = vec![];
    let is_error = {
        let task_num = task_length / download_size;
        let semaphore = Arc::new(Semaphore::new(parallel_task_num));
        for i in 0..(task_num - 1) {
            // 线程数必须大于等于1
            let semaphore = Arc::clone(&semaphore);
            handles.push(tokio::spawn(async {
                let _permit = semaphore.acquire().await.unwrap();
                download_partial(url.clone(), (task_length * i, task_length * (i + 1) - 1))
            }));
        }
        {
            let semaphore = Arc::clone(&semaphore);
            handles.push(tokio::spawn(async {
                let _permit = semaphore.acquire().await.unwrap();
                download_partial(url.clone(), (task_length * (task_num - 1), u64::MAX))
            }));
        }

        let ret = join_all(handles).await;
        ret.into_iter().flatten().any(|n| n.is_err())
    };
    if is_error {
        Err(Error::msg("download error"))
    } else {
        Ok(())
    }
}
