use chrono::Duration as ChronoDuration;
use once_cell::sync::Lazy;
use actix_web::cookie::time::Duration as ActixDuration;

const ACCESS_TOKEN_EXPIRED: i64 = 1;
const REFRESH_TOKEN_EXPIRED: i64 = 2;

pub const CHRONO_ACCESS_TOKEN_EXPIRED: Lazy<ChronoDuration> = Lazy::new(|| {
    ChronoDuration::minutes(ACCESS_TOKEN_EXPIRED)
});
pub const CHRONO_REFRESH_TOKEN_EXPIRED: Lazy<ChronoDuration> = Lazy::new(|| {
    ChronoDuration::minutes(REFRESH_TOKEN_EXPIRED)
});

pub static ACTIX_REFRESH_TOKEN_EXPIRED: ActixDuration = ActixDuration::minutes(REFRESH_TOKEN_EXPIRED);