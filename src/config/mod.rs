//! 配置文件读取和处理
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
    str::FromStr,
};

use log::{debug, error, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};

mod config_env;

pub use config_env::{ConfigEnv, get_env_config};

// *.toml
const CONFIG_FILE_TOML: &'static str = ".toml";
// pmbs = 1
const CONFIG_FILE_VERSION: u32 = 1;

/// pmbs 配置文件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmbsConfig {
    /// 配置文件版本
    pub pmbs: u32,
    /// 目标 btrfs subvol 路径
    pub subvol: String,
    /// 快照保留规则
    pub keep: Vec<PmbsConfigKeep>,
}

/// 快照保留规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmbsConfigKeep {
    /// 间隔时间, 接受后缀 m (分钟), h (小时), d (天)
    pub time: String,
    /// 保留快照个数
    pub n: u32,

    /// (内部实现) 间隔时间 (秒)
    #[serde(skip)]
    pub s: u64,
}

impl PmbsConfigKeep {
    /// 用于调试: 只含有 s 和 n 的规则
    pub fn new_sn(s: u64, n: u32) -> Self {
        Self {
            time: "".into(),
            n,
            s,
        }
    }
}

/// *.toml 配置文件 (含文件名)
#[derive(Debug, Clone, Serialize)]
pub struct PmbsConfigFile {
    /// 文件名
    pub path: String,
    /// 配置内容
    pub config: PmbsConfig,
}

/// 列出配置文件
pub fn list_config(config: &ConfigEnv) -> Vec<PathBuf> {
    let p = PathBuf::from(&config.dir_etc);
    if p.is_dir() {
        read_dir(p)
            .unwrap()
            .filter_map(|i| {
                let f = i.unwrap();
                // 检查名称 *.toml
                let name = f.file_name().to_string_lossy().to_string();
                if name.ends_with(CONFIG_FILE_TOML) {
                    let p = f.path();
                    if p.is_file() {
                        // 检查通过
                        Some(PathBuf::from(p))
                    } else {
                        // 不是普通文件, 忽略
                        warn!("not regular file  {}", p.to_string_lossy());
                        None
                    }
                } else {
                    // 忽略
                    None
                }
            })
            .collect()
    } else {
        warn!("config dir not exist  {}", &config.dir_etc);
        Vec::new()
    }
}

/// 读取配置文件 toml
fn read_config_toml(path: &Path) -> Option<PmbsConfigFile> {
    match read_to_string(path) {
        Ok(s) => match toml::from_str::<PmbsConfig>(&s) {
            Ok(config) => Some(PmbsConfigFile {
                path: path.file_name().unwrap().to_string_lossy().to_string(),
                config,
            }),
            Err(e) => {
                error!("can not parse toml  {:?}", e);
                None
            }
        },
        Err(e) => {
            error!("can not read file  {:?}", e);
            None
        }
    }
}

/// 检查配置文件 keep.time 输入
pub fn get_re_keep_time() -> Regex {
    Regex::new(r"^[1-9][0-9_]*[mhd]$").unwrap()
}

/// 解析 time 字符串, 转换为秒
fn time_to_s(time: &str) -> u64 {
    let mut time = time.to_string();
    // 最后一个字符 (单位)
    // 之间已经通过了正则表达式的检查, 此处的字符串已经是纯 ASCII, 可以不考虑多字节 utf-8
    let unit = time.split_off(time.len() - 1);

    let time: u64 = FromStr::from_str(&time).unwrap();
    let unit: u64 = match unit.as_str() {
        // 分钟 = 60 秒
        "m" => 60,
        // 小时 = 3600 秒
        "h" => 3600,
        // 天 = 86400 秒
        "d" => 86400,

        _ => unreachable!(),
    };

    // 计算时间
    time * unit
}

/// 检查配置文件, 并解析 time 字符串
fn check_config(c: &mut PmbsConfig) -> bool {
    // 配置文件版本
    if c.pmbs != CONFIG_FILE_VERSION {
        error!("bad config file version  {}", c.pmbs);
        return false;
    }
    // subvol 路径
    if c.subvol.trim().len() < 1 {
        error!("empty subvol path");
        return false;
    }
    let p = PathBuf::from(&c.subvol);
    if !p.is_dir() {
        warn!("subvol not exist  {}", c.subvol);
    }

    // 初始化正则表达式 (避免在循环内)
    let re_time = get_re_keep_time();

    // 快照保留规则 (基本检查)
    for i in &mut c.keep {
        // n 不可为 0
        if i.n < 1 {
            error!("bad n = {}", i.n);
            return false;
        }

        // 检查时间格式
        if !re_time.is_match(&i.time) {
            error!("bad time = {}", i.time);
            return false;
        }
        i.s = time_to_s(&i.time);

        debug!("time {} = {}s", i.time, i.s);
    }

    // 更多对快照保留规则的检查 (警告)
    if c.keep.len() < 1 {
        // 没有配置保留规则
        warn!("empty keep rule !");
    }
    // 快照保留的总数
    let mut sum_n: u32 = 0;
    // 上一条规则的间隔时间
    let mut last_time: Option<String> = None;
    let mut last_s = 0;
    for i in &c.keep {
        // 保留快照太多
        if i.n > 200 {
            warn!("too big n = {} !", i.n);
        }
        // 间隔时间太长 (超过 31 天)
        if i.s > (31 * 86400) {
            warn!("too big time = {} !", i.time);
        }
        // 上一条规则的时间, 必须比下一条短
        if let Some(time) = last_time {
            if i.s <= last_s {
                warn!("next rule time is shorter !  {} <= {}", i.time, time);
            }
        }

        // 计算总数
        sum_n += i.n;
        // 更新上一条数据
        last_time = Some(i.time.clone());
        last_s = i.s;
    }
    // 保留了太多快照
    debug!("sum_n = {}", sum_n);
    if sum_n > 500 {
        warn!("too many rules !  {}", sum_n);
    }

    true
}

/// 读取配置文件并检查
pub fn read_config(path: &Path) -> Option<PmbsConfigFile> {
    debug!("read config  {}", path.to_string_lossy());

    match read_config_toml(path) {
        Some(mut c) => {
            if check_config(&mut c.config) {
                // 检查通过
                Some(c)
            } else {
                // 配置文件内容错误
                error!("bad config file  {}", path.to_string_lossy());
                None
            }
        }
        None => {
            // toml 格式错误 (serde)
            error!("bad config toml file  {}", path.to_string_lossy());
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// 解析配置文件中的 time 字符串
    #[test]
    fn parse_time() {
        assert_eq!(time_to_s("1m"), 60);
        assert_eq!(time_to_s("5m"), 300);
        assert_eq!(time_to_s("20m"), 1200);
        assert_eq!(time_to_s("1h"), 3600);
        assert_eq!(time_to_s("2h"), 7200);
        assert_eq!(time_to_s("1d"), 8_6400);
        assert_eq!(time_to_s("7d"), 60_4800);
        assert_eq!(time_to_s("28d"), 241_9200);
    }
}

/// 对正则表达式匹配进行测试
#[cfg(test)]
mod test_re {
    use super::*;

    #[test]
    fn re_keep_time_should_match() {
        let re = get_re_keep_time();

        assert_eq!(re.is_match("1m"), true);
        assert_eq!(re.is_match("5m"), true);
        assert_eq!(re.is_match("20m"), true);
        assert_eq!(re.is_match("1h"), true);
        assert_eq!(re.is_match("1d"), true);

        assert_eq!(re.is_match("7d"), true);
        assert_eq!(re.is_match("28d"), true);
        assert_eq!(re.is_match("30d"), true);
        assert_eq!(re.is_match("365d"), true);
        assert_eq!(re.is_match("2000d"), true);
    }

    #[test]
    fn re_keep_time_not_match() {
        let re = get_re_keep_time();

        // 空
        assert_eq!(re.is_match(""), false);
        // 纯数字, 没有单位
        assert_eq!(re.is_match("1"), false);
        assert_eq!(re.is_match("234"), false);
        // 没有数字
        assert_eq!(re.is_match("m"), false);
        assert_eq!(re.is_match("h"), false);
        assert_eq!(re.is_match("d"), false);
        // 不支持的单位
        assert_eq!(re.is_match("2w"), false);
        assert_eq!(re.is_match("1y"), false);
        assert_eq!(re.is_match("42min"), false);
        assert_eq!(re.is_match("5hours"), false);
        assert_eq!(re.is_match("5s"), false);
        // 随意错误格式
        assert_eq!(re.is_match("balabala"), false);
        assert_eq!(re.is_match("x666"), false);
    }
}
