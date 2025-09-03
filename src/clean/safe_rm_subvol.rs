//! 安全删除快照 (btrfs subvol)
use std::process::Command;

use log::debug;
use regex::Regex;

use crate::{cli::sh_run, config::ConfigEnv};

/// 检查要删除的 subvol 路径.
///
/// 因为 btrfs 快照只是特殊的 subvol, 在删除快照时, 为了避免误删别的 subvol,
/// 此处额外增加了对目标路径的检查.
/// 只有结尾符合类似 `/.pmbs/2025/1756392923` 格式的 subvol 才会通过检查, 允许删除.
pub fn get_re_safe_check_path() -> Regex {
    // 年至少 4 位数字
    // 时间戳 (UNIX_EPOCH 开始的秒数) 至少 10 位数字
    Regex::new(r"/\.pmbs/[1-9][0-9]{3,}/[1-9][0-9]{9,}$").unwrap()
}

/// 删除 subvol (列表)
///
/// ## panic
///
/// + 如果有路径未通过检查
/// + 如果执行删除命令失败
pub fn safe_rm_subvol_list(config: &ConfigEnv, list: Vec<String>) {
    let re = get_re_safe_check_path();

    for i in list {
        debug!("check {}", i);

        if re.is_match(&i) {
            // 检查通过, 可以删除

            // 执行命令, 比如 btrfs subvol delete /home/.pmbs/2025/1756392923
            let mut c = Command::new(config.bin_btrfs.clone());
            c.arg("subvol").arg("delete").arg(&i);
            let code = sh_run(c);
            if 0 != code {
                panic!("can not rm subvol {}", i);
            }
        } else {
            // 错误路径 !
            panic!("bad subvol path {}", i);
        }
    }
}

/// 对正则表达式匹配进行测试
#[cfg(test)]
mod test_re {
    use super::*;

    // 正常匹配
    #[test]
    fn re_should_match() {
        let re = get_re_safe_check_path();

        assert_eq!(re.is_match("/home/.pmbs/2025/1756392923"), true);
        assert_eq!(re.is_match("/.pmbs/2025/1756392923"), true);
        assert_eq!(re.is_match("/.pmbs/9999/1756392923"), true);
        assert_eq!(re.is_match("/.pmbs/2025/9999999999"), true);
        assert_eq!(re.is_match("/.pmbs/10000/20000000000"), true);
    }

    // 不匹配: 空
    #[test]
    fn re_should_not_match_empty() {
        let re = get_re_safe_check_path();

        assert_eq!(re.is_match(""), false);
        assert_eq!(re.is_match("/"), false);
        assert_eq!(re.is_match("/.pmbs"), false);
        assert_eq!(re.is_match("/.pmbs/"), false);
        assert_eq!(re.is_match("/.pmbs//1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/2025/"), false);
        assert_eq!(re.is_match("/.pmbs/2025"), false);
        assert_eq!(re.is_match("/.pmbs/2025"), false);
        assert_eq!(re.is_match("/2025/1756392923"), false);
        assert_eq!(re.is_match("2025/1756392923"), false);
        assert_eq!(re.is_match("//2025/1756392923"), false);
    }

    // 不匹配: 太短
    #[test]
    fn re_should_not_match_short() {
        let re = get_re_safe_check_path();

        assert_eq!(re.is_match("/.pmbs/1/1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/20/1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/202/1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/0025/1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/2025/175639292"), false);
        assert_eq!(re.is_match("/.pmbs/2025/17"), false);
        assert_eq!(re.is_match("/.pmbs/2025/56392923"), false);
        assert_eq!(re.is_match("/.pmbs/2025/0175639292"), false);
        assert_eq!(re.is_match("/pmbs/2025/1756392923"), false);
        assert_eq!(re.is_match("/.pbs/2025/1756392923"), false);
        assert_eq!(re.is_match("/.pmb/2025/1756392923"), false);
        assert_eq!(re.is_match("/.s/2025/1756392923"), false);
        assert_eq!(re.is_match("/./2025/1756392923"), false);
    }

    // 不匹配: 系统
    #[test]
    fn re_should_not_match_system() {
        let re = get_re_safe_check_path();

        assert_eq!(re.is_match("/"), false);
        assert_eq!(re.is_match("/home"), false);
        assert_eq!(re.is_match("/usr"), false);
        assert_eq!(re.is_match("/var"), false);
        assert_eq!(re.is_match("/etc"), false);
        assert_eq!(re.is_match("/mnt"), false);
        assert_eq!(re.is_match("/srv"), false);
        assert_eq!(re.is_match("/root"), false);
        assert_eq!(re.is_match("/boot"), false);
    }

    // 不匹配: 错误字符
    #[test]
    fn re_should_not_match_char() {
        let re = get_re_safe_check_path();

        assert_eq!(re.is_match("/.pmbs/2025/1756392923x"), false);
        assert_eq!(re.is_match("/.pmbs/2025/c1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/2025/.1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/2025/175_6392923"), false);
        assert_eq!(re.is_match("/.pmbs/2025/17563929-23"), false);
        assert_eq!(re.is_match("/.pmbs/.2025/1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/2a25/1756392923"), false);
        assert_eq!(re.is_match("/.pmbs/202_/1756392923"), false);
        assert_eq!(re.is_match("/.pcbs/2025/1756392923"), false);
        assert_eq!(re.is_match("/.pmbb/2025/1756392923"), false);
        assert_eq!(re.is_match("/apmbs/2025/1756392923"), false);
    }
}
