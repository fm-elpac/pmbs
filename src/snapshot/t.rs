//! .pmbs 快照目录结构定义
use std::time::SystemTime;

use chrono::{DateTime, Datelike, Local, Utc, format::SecondsFormat};

/// subvol 快照保存目录 /.pmbs/2025/T
pub const DIR_PMBS: &'static str = ".pmbs";

/// 最新快照的符号链接 /.pmbs/latest -> 2025/T
pub const SYMLINK_LATEST: &'static str = "latest";

/// 获取当前时间戳 (UNIX_EPOCH 开始的秒数)
pub fn get_t() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn get_datetime(t: u64) -> DateTime<Utc> {
    DateTime::from_timestamp(t as i64, 0).unwrap()
}

/// 获取当前年
pub fn get_year(t: u64) -> i32 {
    get_datetime(t).year()
}

/// 时间戳 (UNIX_EPOCH 开始的秒数) 转换为可读文本
pub fn format_t(t: u64) -> String {
    get_datetime(t).to_rfc3339_opts(SecondsFormat::Secs, true)
}

/// 使用本地时区显示时间戳
pub fn format_t_local(t: u64) -> String {
    let local = get_datetime(t).with_timezone(&Local);
    local.to_rfc3339_opts(SecondsFormat::Secs, false)
}
