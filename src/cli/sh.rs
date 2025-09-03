//! 调用执行命令 (shell)
use std::process::Command;

use log::{error, info};

/// 执行 shell 命令
pub fn sh_run(mut c: Command) -> i32 {
    info!("run {:?}", c);

    let code = c.status().unwrap().code().unwrap();
    if 0 != code {
        error!("exit code {}", code);
    }
    code
}
