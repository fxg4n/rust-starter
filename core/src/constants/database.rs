use std::time::Duration;

pub const MIN_CONNECTIONS: u32 = 2;
pub const MAX_CONNECTIONS: u32 = 30;
pub const MAX_LIFETIME: Duration = Duration::from_secs(30 * 60);
pub const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(30);
pub const IDLE_TIMEOUT: Duration = Duration::from_secs(10 * 60);
pub const CONNECTION_RETRY_ATTEMPTS: u32 = 3;
pub const CONNECTION_RETRY_DELAY: Duration = Duration::from_secs(5);
pub const STATEMENT_TIMEOUT: Duration = Duration::from_secs(30);
pub const POOL_TIMEOUT: Duration = Duration::from_secs(15);
pub const MAX_QUERY_SIZE: usize = 10_000_000;