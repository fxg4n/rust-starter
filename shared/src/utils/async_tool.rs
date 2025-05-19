use std::future::Future;
use std::time::Duration;
use tokio::time::{sleep, timeout};

pub async fn with_timeout<F, T>(future: F, timeout_duration: Duration) -> Result<T, &'static str>
where
    F: Future<Output = T>,
{
    timeout(timeout_duration, future)
        .await
        .map_err(|_| "operation timed out")
}

pub async fn retry<F, Fut, T, E>(mut f: F, retries: u32, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    for _ in 0..retries {
        match f().await {
            Ok(val) => return Ok(val),
            Err(_) => sleep(delay).await,
        }
    }
    f().await
}
