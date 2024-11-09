use chrono::prelude::*;

/**
 * 获取当前Utc时间
 */
pub fn utc_now() -> NaiveDateTime {
    let now = Utc::now();
    now.naive_utc()
}