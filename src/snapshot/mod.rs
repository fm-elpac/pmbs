//! 创建 btrfs 快照
use std::{
    fs::{create_dir_all, rename},
    os::unix::fs::symlink,
    path::PathBuf,
    process::{Command, ExitCode},
};

use log::{debug, info, warn};

use crate::{clean::ls_snapshot, cli::sh_run, config::ConfigEnv};

mod t;

pub use t::{DIR_PMBS, SYMLINK_LATEST, format_t, format_t_local, get_t, get_year};

/// 创建指定 subvol 的快照
pub fn make_snapshot(config: &ConfigEnv, subvol: &str) -> Result<(), ExitCode> {
    // 获取当前时间, 比如 1756392923
    let t = get_t();
    let year = get_year(t);
    let now = format_t(t);
    debug!("snapshot t = {}  {}", t, now);

    // 目标目录, 比如 /.pmbs/2025/1756392923
    // subvol/.pmbs
    let mut p = PathBuf::from(subvol);
    p.push(DIR_PMBS);
    // subvol/.pmbs/2025
    let mut y = p.clone();
    y.push(format!("{}", year));
    // subvol/.pmbs/2025/1756392923
    let mut to = y.clone();
    to.push(format!("{}", t));

    info!("snapshot {} -> {}", subvol, to.to_string_lossy());
    // 创建目录
    create_dir_all(y).unwrap();

    // 执行命令, 比如 btrfs subvol snapshot -r /home /home/.pmbs/2025/1756392923
    let mut c = Command::new(config.bin_btrfs.clone());
    c.arg("subvol")
        .arg("snapshot")
        .arg("-r")
        .arg(subvol)
        .arg(to);
    let code = sh_run(c);
    if 0 != code {
        return Err(ExitCode::from(1));
    }

    // subvol/.pmbs/latest
    let mut latest = p.clone();
    latest.push(SYMLINK_LATEST);
    // subvol/.pmbs/latest.1756392923
    let mut latest_tmp = p.clone();
    latest_tmp.push(format!("{}.{}", SYMLINK_LATEST, t));

    // 创建 latest 符号链接 (write-replace)
    let mut link_to = PathBuf::new();
    link_to.push(format!("{}", year));
    link_to.push(format!("{}", t));
    info!(
        "symlink {} -> {}",
        latest.to_string_lossy(),
        link_to.to_string_lossy()
    );
    // write
    symlink(link_to, &latest_tmp).unwrap();
    // replace
    rename(&latest_tmp, latest).unwrap();

    // 创建快照后的检测: 是否还有更新的快照.
    // 如果存在, 可能是系统时间配置错误 !
    let list = ls_snapshot(subvol);
    let max_t = list.iter().map(|x| x.t).max().unwrap();
    if max_t > t {
        warn!("time error !  {} > {}  ({})", max_t, t, max_t - t);
    }

    Ok(())
}
