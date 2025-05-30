pub const AUTH_HEADER: &str = "Authorization";
pub const BEARER_PREFIX: &str = "Bearer ";
pub const JWT_SECRET_MIN_LENGTH: usize = 32;
pub const JWT_ISSUER: &str = "m5_api";
pub const JWT_ALGORITHM: &str = "HS256";
pub const TOKEN_EXPIRATION_BUFFER: i64 = 300; 
pub const MAX_FAILED_LOGIN_ATTEMPTS: u32 = 5;
pub const FAILED_LOGIN_TIMEOUT: u64 = 900;
pub const PASSWORD_HASH_MEMORY_COST: u32 = 65536;
pub const PASSWORD_HASH_TIME_COST: u32 = 2;
pub const PASSWORD_HASH_PARALLELISM: u32 = 1;
pub const PASSWORD_SALT_LENGTH: usize = 32;
pub const ACCESS_TOKEN_EXPIRY: i64 = 3600;
pub const REFRESH_TOKEN_EXPIRY: i64 = 2592000;
pub const TOKEN_BLACKLIST_TTL: u64 = 86400;
pub const MAX_LOGIN_RATE: u32 = 5;
pub const MAX_TOKEN_REFRESH_RATE: u32 = 10;