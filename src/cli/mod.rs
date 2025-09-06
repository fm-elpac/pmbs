//! 命令行相关处理
use std::{
    env,
    fs::{create_dir_all, write},
    path::PathBuf,
    process::{Command, ExitCode},
};

use log::{debug, info};
use serde::Serialize;

mod help;
mod sh;

pub use sh::sh_run;

use crate::{
    clean::{Snapshot, decide, ls_snapshot, safe_rm_subvol_list},
    config::{ConfigEnv, PmbsConfigFile, get_env_config, list_config, read_config},
    snapshot::{format_t_local, get_t, make_snapshot},
};

use help::bad_cli_arg;

/// pmbs snapshot SUBVOL
fn c_snapshot(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 1 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let subvol = &a[0];

    // 读取环境配置
    let c = get_env_config();
    // 创建快照
    make_snapshot(&c, subvol)
}

/// pmbs ls SUBVOL
fn c_ls(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 1 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let subvol = &a[0];

    // 获取所有快照
    let mut list = ls_snapshot(subvol);
    // 排序 (按时间降序, 最新的在最前面)
    list.sort_by(|a, b| b.t.cmp(&a.t));

    // 输出
    for i in list {
        let latest = if i.latest { "\t*latest" } else { "" };

        println!(
            "{}\t{}{}",
            i.p.to_string_lossy(),
            format_t_local(i.t),
            latest
        );
    }
    Ok(())
}

/// 获取自己的可执行文件路径
fn get_exe() -> PathBuf {
    env::current_exe().unwrap()
}

/// 读取所有配置文件, 调用自己, 使用 subprocess 执行每个配置文件
fn run_config(c: &ConfigEnv, a: &str) -> Result<(), ExitCode> {
    let mut r = Ok(());

    for i in list_config(c) {
        // 调用自己, 在 subprocess 中实际执行
        let mut c = Command::new(get_exe());
        c.arg("config").arg(a).arg(i);

        let code = sh_run(c);
        // 忽略错误, 稍后返回错误
        if 0 != code {
            r = Err(ExitCode::from(1));
        }
    }
    r
}

/// pmbs config snapshot {PATH}
fn c_config_snapshot(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    let path = match a.len() {
        0 => None,
        1 => Some(&a[0]),
        _ => {
            bad_cli_arg();
            return Err(ExitCode::from(1));
        }
    };
    // 读取环境配置
    let c = get_env_config();

    match path {
        // 执行指定配置文件
        Some(path) => match read_config(&PathBuf::from(path)) {
            Some(config) => {
                debug!("config  {}", serde_json::to_string(&config).unwrap());
                // 创建快照
                make_snapshot(&c, &config.config.subvol)
            }
            None => {
                return Err(ExitCode::from(1));
            }
        },
        // 读取所有配置文件 (执行 pmbs config snapshot)
        None => run_config(&c, "snapshot"),
    }
}

/// pmbs config clean {PATH}
fn c_config_clean(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    let path = match a.len() {
        0 => None,
        1 => Some(&a[0]),
        _ => {
            bad_cli_arg();
            return Err(ExitCode::from(1));
        }
    };
    // 读取环境配置
    let c = get_env_config();

    match path {
        // 执行指定配置文件
        Some(path) => match get_clean(path) {
            Some((config, keep, clean)) => {
                write_clean_log(&c, &config, &keep, &clean);

                // 执行清理
                safe_rm_subvol_list(
                    &c,
                    clean
                        .iter()
                        .map(|x| x.p.to_string_lossy().to_string())
                        .collect(),
                );
                Ok(())
            }
            None => {
                return Err(ExitCode::from(1));
            }
        },
        // 读取所有配置文件 (执行 pmbs config clean)
        None => run_config(&c, "clean"),
    }
}

/// 清理日志
#[derive(Debug, Clone, Serialize)]
pub struct CleanLog {
    /// 时间戳 (清理时间)
    pub t: u64,
    /// 配置
    pub config: PmbsConfigFile,
    /// 保留的快照
    pub keep: Vec<String>,
    /// 清理的快照
    pub clean: Vec<String>,
}

/// 写入清理日志
fn write_clean_log(
    c: &ConfigEnv,
    config: &PmbsConfigFile,
    keep: &Vec<Snapshot>,
    clean: &Vec<Snapshot>,
) {
    // 日志文件名
    let t = get_t();
    let filename = format!("clean-{}-{}.log.json", t, config.path);

    let log = CleanLog {
        t,
        config: config.clone(),
        keep: keep.iter().map(|x| x.path.clone()).collect(),
        clean: clean.iter().map(|x| x.path.clone()).collect(),
    };
    let text = serde_json::to_string_pretty(&log).unwrap();

    let mut p = PathBuf::from(&c.dir_log);
    p.push(filename);

    debug!("write clean log {}", p.to_string_lossy());
    create_dir_all(&c.dir_log).unwrap();
    write(p, text.as_bytes()).unwrap();
}

/// pmbs config test
fn c_config_test(a: Vec<String>) -> Result<(), ExitCode> {
    // 无命令行参数
    if a.len() > 0 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    // 读取环境配置
    let c = get_env_config();

    for i in list_config(&c) {
        info!("check {}", i.to_string_lossy());

        match read_config(&i) {
            Some(c) => {
                debug!("config  {}", serde_json::to_string(&c).unwrap());
            }
            None => {
                return Err(ExitCode::from(1));
            }
        }
    }
    Ok(())
}

/// pmbs config test-clean PATH
fn c_config_test_clean(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 1 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let path = &a[0];

    match get_clean(path) {
        Some((_, keep, clean)) => {
            for i in keep {
                println!("keep {}  {}", i.path, format_t_local(i.t));
            }
            for i in clean {
                println!("clean {}  {}", i.path, format_t_local(i.t));
            }
            Ok(())
        }
        None => {
            return Err(ExitCode::from(1));
        }
    }
}

/// 获取清理列表
fn get_clean(path: &str) -> Option<(PmbsConfigFile, Vec<Snapshot>, Vec<Snapshot>)> {
    // 加载配置文件
    match read_config(&PathBuf::from(path)) {
        Some(config) => {
            debug!("config  {}", serde_json::to_string(&config).unwrap());

            // 列出全部快照
            let snapshot = ls_snapshot(&config.config.subvol);
            let total = snapshot.len();
            // 检查清理
            let (keep, clean) = decide(config.config.keep.clone(), snapshot);

            debug!(
                "total = {}, keep = {}, clean = {}",
                total,
                keep.len(),
                clean.len()
            );
            // 检查错误
            if total != (keep.len() + clean.len()) {
                panic!(
                    "bad clean, total = {}, keep = {}, clean = {}",
                    total,
                    keep.len(),
                    clean.len()
                );
            }
            Some((config, keep, clean))
        }
        None => None,
    }
}

/// pmbs config *
fn c_config(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() < 1 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    // 第 1 个参数: 命令
    let r: Vec<String> = (&a[1..]).into();
    match a[0].as_str() {
        "snapshot" => c_config_snapshot(r),
        "clean" => c_config_clean(r),
        "test" => c_config_test(r),
        "test-clean" => c_config_test_clean(r),

        _ => {
            bad_cli_arg();
            Err(ExitCode::from(1))
        }
    }
}

/// 命令行执行入口
pub fn main(a: Vec<String>) -> Result<(), ExitCode> {
    // 命令行参数解析处理
    if a.len() > 0 {
        // 第 1 个参数: 命令
        let r: Vec<String> = (&a[1..]).into();
        match a[0].as_str() {
            "--help" => {
                help::help_en();
                Ok(())
            }
            "--帮助" => {
                help::help_zh();
                Ok(())
            }

            "snapshot" => c_snapshot(r),
            "ls" => c_ls(r),

            "config" => c_config(r),

            _ => {
                eprintln!("ERROR: Bad command `{}`, try --help", a[0]);
                Err(ExitCode::from(1))
            }
        }
    } else {
        eprintln!("ERROR: Bad command, try --help");
        Err(ExitCode::from(1))
    }
}
