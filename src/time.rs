use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

/// 1毫秒=10000TICK
pub const TICK: i64 = 10000;

/// 1秒=1000毫秒
pub const SECOND: i64 = 1000;

/// 1分钟的毫秒数
pub const MINUTE: i64 = 60 * SECOND;

/// 1小时的毫秒数
pub const HOUR: i64 = 60 * MINUTE;

/// 1天的毫秒数
pub const DAY: i64 = 24 * HOUR;

/// 获取Utc时间戳 秒后 7个0
#[inline]
pub fn timestamp() -> i64 {
    Utc::now().timestamp_nanos() / 100
}

/// 获取UTC 毫秒时间戳
#[inline]
pub fn timestamp_milliseconds() -> i64 {
    Utc::now().timestamp_nanos() / 1_000_000
}

/// 从本地时间获取时间戳
#[inline]
pub fn timestamp_from_time(time: DateTime<Local>) -> i64 {
    time.timestamp_nanos() / 100
}

/// 获取本地时间戳
#[inline]
pub fn local_timestamp() -> i64 {
    Local::now().timestamp_nanos() / 100
}

/// 从UTC时间获取时间戳
#[inline]
pub fn utc_timestamp_from_time(time: DateTime<Utc>) -> i64 {
    time.timestamp_nanos() / 100
}

/// 将时间戳转换成 时间
#[inline]
pub fn timestamp_to_time(timestamp: i64) -> DateTime<Local> {
    Local.timestamp_nanos(timestamp * 100)
}

/// 将时间戳转换成 UTC时间
#[inline]
pub fn timestamp_to_utc_time(utc_timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_nanos(utc_timestamp * 100)
}

/// 将当前UTC 时间 减天数
#[inline]
pub fn get_utc_sub_day(day: i64) -> i64 {
    (Utc::now().timestamp_millis() - day * DAY) * TICK
}

/// 将当前UTC 时间 加天数
#[inline]
pub fn get_utc_add_day(day: i64) -> i64 {
    (Utc::now().timestamp_millis() + day * DAY) * TICK
}

/// 将当前UTC 时间 减分钟
#[inline]
pub fn get_utc_sub_minute(minute: i64) -> i64 {
    (Utc::now().timestamp_millis() - minute * MINUTE) * 10000
}

/// 将当前UTC 时间 加分钟
#[inline]
pub fn get_utc_add_minute(minute: i64) -> i64 {
    (Utc::now().timestamp_millis() + minute * MINUTE) * TICK
}

/// 将当前时间 减天数
#[inline]
pub fn get_local_sub_day(day: i64) -> i64 {
    (Local::now().timestamp_millis() - day * 24 * HOUR) * TICK
}

/// 将当前时间 减天数
#[inline]
pub fn get_local_add_day(day: i64) -> i64 {
    (Local::now().timestamp_millis() + day * DAY) * TICK
}

/// 将当前时间 减分钟
#[inline]
pub fn get_local_sub_minute(minute: i64) -> i64 {
    (Local::now().timestamp_millis() - minute * MINUTE) * TICK
}

/// 将当前时间 减分钟
#[inline]
pub fn get_local_add_minute(minute: i64) -> i64 {
    (Local::now().timestamp_millis() + minute * MINUTE) * TICK
}

/// 给UTC时间 + 多少分钟时间 并算出时间戳
#[inline]
pub fn utc_time_add_minute(time: DateTime<Utc>, minute: i64) -> i64 {
    (time.timestamp_millis() + minute * MINUTE) * TICK
}

/// 给naive时间 + 多少分钟时间 并算出时间戳
#[inline]
pub fn naive_time_add_minute(time: NaiveDateTime, minute: i64) -> i64 {
    (time.timestamp_millis() + minute * MINUTE) * TICK
}

/// 给UTC时间 - 多少分钟时间 并算出时间戳
#[inline]
pub fn utc_time_sub_minute(time: DateTime<Utc>, minute: i64) -> i64 {
    (time.timestamp_millis() - minute * MINUTE) * TICK
}

/// 给本地时间 + 多少分钟时间 并算出时间戳
#[inline]
pub fn time_add_minute(time: DateTime<Local>, minute: i64) -> i64 {
    (time.timestamp_millis() + minute * MINUTE) * TICK
}

/// 给本地时间 - 多少分钟时间 并算出时间戳
#[inline]
pub fn time_sub_minute(time: DateTime<Local>, minute: i64) -> i64 {
    (time.timestamp_millis() - minute * MINUTE) * TICK
}

/// 给时间戳加上多少分钟
#[inline]
pub fn timestamp_add_minute(timestamp: i64, minute: i64) -> i64 {
    timestamp + (minute * MINUTE * TICK)
}

/// 给时间戳减去多少分钟
#[inline]
pub fn timestamp_sub_minute(timestamp: i64, minute: i64) -> i64 {
    timestamp - (minute * MINUTE * TICK)
}

/// 返回一个UUID
#[inline]
pub fn new_uuid() -> String {
    uuid::Uuid::new_v4().hyphenated().to_string()
}

/// 获取local当天0点0分0秒时间戳
#[inline]
pub fn get_now_day_timestamp(timezone_minute: i64) -> i64 {
    let utc_zero = Utc::now()
        .naive_utc()
        .date()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .timestamp_nanos()
        / 100;
    let timezone = timezone_minute * MINUTE * TICK;
    utc_zero - timezone
}

/// 获取local当天0点0分0秒时间戳+天数
#[inline]
pub fn get_now_add_day(timezone_minute: i64, day: i64) -> i64 {
    get_now_day_timestamp(timezone_minute) + day * 864_000_000_000i64
}

/// 获取local当天0点0分0秒时间戳-天数
#[inline]
pub fn get_now_sub_day(timezone_minute: i64, day: i64) -> i64 {
    get_now_day_timestamp(timezone_minute) - day * 864_000_000_000i64
}

/// 获取UTC 当天0点0分0秒时间戳
#[inline]
pub fn get_utc_now_day_timestamp() -> i64 {
    Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .timestamp_nanos()
        / 100
}

/// 根据时区返回 当天0点0分0秒时间戳
#[inline]
pub fn get_utc_now_day_timestamp_for_zone(timezone_minute: i64) -> i64 {
    get_now_day_timestamp(timezone_minute)
}

/// 获取UTC 当天0点0分0秒时间戳+天数
#[inline]
pub fn get_utc_now_add_day(day: i64) -> i64 {
    get_utc_now_day_timestamp() + day * 864_000_000_000i64
}

/// 获取UTC 当天0点0分0秒时间戳-天数
#[inline]
pub fn get_utc_now_sub_day(day: i64) -> i64 {
    get_utc_now_day_timestamp() - day * 864_000_000_000i64
}
