//! 列出现有的所有快照
use std::{
    fs::{read_dir, read_link},
    path::PathBuf,
    str::FromStr,
};

use log::{debug, warn};
use regex::Regex;

use crate::snapshot::{DIR_PMBS, SYMLINK_LATEST};

/// 检查年, 比如 `/.pmbs/2025/1756392923` 中的 `2025`
pub fn get_re_year() -> Regex {
    // 年至少 4 位数字
    Regex::new(r"^[1-9][0-9]{3,}$").unwrap()
}

/// 检查时间戳, 比如 `/.pmbs/2025/1756392923` 中的 `1756392923`
pub fn get_re_t() -> Regex {
    // 时间戳 (UNIX_EPOCH 开始的秒数) 至少 10 位数字
    Regex::new(r"^[1-9][0-9]{9,}$").unwrap()
}

/// 一个快照
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// 路径 (字符串 2025/1756392923 格式)
    pub path: String,
    /// 年
    pub year: i32,
    /// 时间戳
    pub t: u64,
    /// 是否被 latest 符号链接指向
    pub latest: bool,
    /// 路径 (PathBuf)
    pub p: PathBuf,
}

/// 列出指定目录下的所有快照
pub fn ls_snapshot(path: &str) -> Vec<Snapshot> {
    let mut o: Vec<Snapshot> = Vec::new();
    // 检查 path/.pmbs 是否存在
    let mut p = PathBuf::from(path);
    p.push(DIR_PMBS);
    if p.is_dir() {
        // 读取 latest
        let mut p_latest = p.clone();
        p_latest.push(SYMLINK_LATEST);
        let latest_symlink: Option<String> = match read_link(&p_latest) {
            Ok(p) => Some(p.to_string_lossy().to_string()),
            _ => {
                debug!("symlink not exist  {}", p_latest.to_string_lossy());
                None
            }
        };

        // 初始化正则表达式 (避免在循环内)
        let re_year = get_re_year();
        let re_t = get_re_t();

        // 列出年
        for i in read_dir(p).unwrap() {
            let d = i.unwrap();
            // 检查名称
            let name = d.file_name().to_string_lossy().to_string();
            if !re_year.is_match(&name) {
                // 忽略
                continue;
            }

            let p_year = d.path();
            if p_year.is_dir() {
                let year: i32 = FromStr::from_str(&name).unwrap();

                // 列出时间戳
                for i in read_dir(p_year).unwrap() {
                    let d = i.unwrap();
                    // 检查名称
                    let name = d.file_name().to_string_lossy().to_string();
                    if !re_t.is_match(&name) {
                        // 忽略
                        continue;
                    }

                    let p_t = d.path();
                    if p_t.is_dir() {
                        let t: u64 = FromStr::from_str(&name).unwrap();

                        let path = format!("{}/{}", year, t);
                        let latest = match &latest_symlink {
                            Some(p) => &path == p,
                            None => false,
                        };
                        // 发现一个快照
                        o.push(Snapshot {
                            path,
                            year,
                            t,
                            latest,
                            p: p_t,
                        });
                    } else {
                        warn!("not dir  {}", p_t.to_string_lossy());
                    }
                }
            } else {
                warn!("not dir  {}", p_year.to_string_lossy());
            }
        }
    } else {
        debug!("dir not exist  {}", p.to_string_lossy());
    }
    o
}

/// 对正则表达式匹配进行测试
#[cfg(test)]
mod test_re {
    use super::*;

    // 正常匹配
    #[test]
    fn re_year_should_match() {
        let re = get_re_year();

        assert_eq!(re.is_match("2025"), true);
        assert_eq!(re.is_match("9999"), true);
        assert_eq!(re.is_match("10000"), true);
    }

    #[test]
    fn re_t_should_match() {
        let re = get_re_t();

        assert_eq!(re.is_match("1756392923"), true);
        assert_eq!(re.is_match("1999999999"), true);
        assert_eq!(re.is_match("20000000000"), true);
    }

    // 不匹配: 空
    #[test]
    fn re_year_should_not_match_empty() {
        let re = get_re_year();

        assert_eq!(re.is_match(""), false);
    }

    #[test]
    fn re_t_should_not_match_empty() {
        let re = get_re_t();

        assert_eq!(re.is_match(""), false);
    }

    // 不匹配: 太短
    #[test]
    fn re_year_should_not_match_short() {
        let re = get_re_year();

        assert_eq!(re.is_match("1"), false);
        assert_eq!(re.is_match("23"), false);
        assert_eq!(re.is_match("456"), false);
        assert_eq!(re.is_match("0789"), false);
    }

    #[test]
    fn re_t_should_not_match_short() {
        let re = get_re_t();

        assert_eq!(re.is_match("175639292"), false);
        assert_eq!(re.is_match("0756392923"), false);
        assert_eq!(re.is_match("1"), false);
        assert_eq!(re.is_match("17"), false);
        assert_eq!(re.is_match("56392923"), false);
    }

    // 不匹配: 非数字
    #[test]
    fn re_year_should_not_match_char() {
        let re = get_re_year();

        assert_eq!(re.is_match("202a"), false);
        assert_eq!(re.is_match("X025"), false);
        assert_eq!(re.is_match("202/"), false);
        assert_eq!(re.is_match("/2025"), false);
        assert_eq!(re.is_match("2_25"), false);
        assert_eq!(re.is_match("202-5"), false);
        assert_eq!(re.is_match("202."), false);
        assert_eq!(re.is_match(".2025"), false);
        assert_eq!(re.is_match("20p5"), false);
    }

    #[test]
    fn re_t_should_not_match_char() {
        let re = get_re_t();

        assert_eq!(re.is_match("175639292C"), false);
        assert_eq!(re.is_match("a175639292"), false);
        assert_eq!(re.is_match("175639292/"), false);
        assert_eq!(re.is_match("/1756392923"), false);
        assert_eq!(re.is_match("1_756392923"), false);
        assert_eq!(re.is_match("1756392923-"), false);
        assert_eq!(re.is_match(".1756392923"), false);
        assert_eq!(re.is_match("1756392.923"), false);
        assert_eq!(re.is_match("17563m2923"), false);
    }
}
