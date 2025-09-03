//! 从环境变量获取配置 (或默认配置)
use std::env;

use log::debug;

// PMBS_DIR_ETC=/etc/pmbs
const ENV_PMBS_DIR_ETC: &'static str = "PMBS_DIR_ETC";
const DEFAULT_PMBS_DIR_ETC: &'static str = "/etc/pmbs";

// PMBS_DIR_LOG=/var/log/pmbs
const ENV_PMBS_DIR_LOG: &'static str = "PMBS_DIR_LOG";
const DEFAULT_PMBS_DIR_LOG: &'static str = "/var/log/pmbs";

// PMBS_BIN_BTRFS=btrfs
const ENV_PMBS_BIN_BTRFS: &'static str = "PMBS_BIN_BTRFS";
const DEFAULT_PMBS_BIN_BTRFS: &'static str = "btrfs";

/// 环境变量配置
#[derive(Debug, Clone)]
pub struct ConfigEnv {
    /// 配置目录
    pub dir_etc: String,
    /// 日志目录
    pub dir_log: String,
    /// btrfs 命令
    pub bin_btrfs: String,
}

impl ConfigEnv {
    /// 读取环境变量配置 (并处理默认值)
    pub fn new() -> Self {
        Self {
            dir_etc: env::var(ENV_PMBS_DIR_ETC).unwrap_or(DEFAULT_PMBS_DIR_ETC.into()),
            dir_log: env::var(ENV_PMBS_DIR_LOG).unwrap_or(DEFAULT_PMBS_DIR_LOG.into()),
            bin_btrfs: env::var(ENV_PMBS_BIN_BTRFS).unwrap_or(DEFAULT_PMBS_BIN_BTRFS.into()),
        }
    }
}

/// 获取环境变量配置
pub fn get_env_config() -> ConfigEnv {
    let c = ConfigEnv::new();

    debug!("config {:?}", c);
    c
}
