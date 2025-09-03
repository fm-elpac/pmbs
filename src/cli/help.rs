//! 输出命令行帮助信息

/// --help
pub fn help_en() {
    println!(
        r#"pmbs: Make btrfs snapshot (every minute), and auto clean.
Usage: pmbs COMMAND ARG..

pmbs snapshot SUBVOL
    Create a snapshot of the btrfs SUBVOL (path).

pmbs ls SUBVOL
    List all snapshots of the SUBVOL (path).

----
Batch command:

pmbs config snapshot
    Read all config files, and create snapshots (usually run as a systemd timer).

pmbs config snapshot PATH
    Read the config file, and create a snapshot.

pmbs config clean
    Read all config files, and do auto clean (usually run as a systemd timer).

pmbs config clean PATH
    Read the config file, and clean snapshots.

----
Test command:

pmbs config test
    Test read config files (check errors in config files).

pmbs config test-clean PATH
    Read the config file, and test clean snapshots (not execute clean actually).

----
pmbs --version
    Show version info.

pmbs --help
    Show this help info.

pmbs --版本
    Show version info.

pmbs --帮助
    Show help info (Chinese).

More info: <https://github.com/fm-elpac/pmbs> <https://crates.io/crates/pmbs>"#
    );
}

/// --帮助
pub fn help_zh() {
    println!(
        r#"胖喵必快 (pmbs): (每分钟) 创建 btrfs 快照, 并自动清理.
用法: pmbs 命令 参数..

pmbs snapshot SUBVOL
    创建指定 btrfs subvol 的快照.

pmbs ls SUBVOL
    列出对应 subvol 的所有快照.

----
批量执行命令:

pmbs config snapshot
    读取所有配置文件, 并创建相应快照 (通常在 systemd timer 中定期执行).

pmbs config snapshot PATH
    读取指定配置文件, 并创建快照.

pmbs config clean
    读取所有配置文件, 并执行自动清理 (通常在 systemd timer 中定期执行).

pmbs config clean PATH
    读取指定配置文件, 并清理对应快照.

----
测试命令:

pmbs config test
    测试读取配置文件 (检查配置文件错误).

pmbs config test-clean PATH
    读取指定配置文件, 测试清理快照 (并不实际执行).

----
pmbs --版本
    显示版本信息.

pmbs --帮助
    显示此帮助信息.

pmbs --version
    显示版本信息.

pmbs --help
    显示帮助信息 (英文).

更多信息: <https://github.com/fm-elpac/pmbs> <https://crates.io/crates/pmbs>"#
    );
}

/// 命令行参数错误信息
pub fn bad_cli_arg() {
    eprintln!("ERROR: Bad command arg, try --help");
}
