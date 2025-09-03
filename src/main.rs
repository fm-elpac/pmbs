//! pmbs CLI
//!
//! TODO
//!
//! <https://github.com/fm-elpac/pmbs>
#![deny(unsafe_code)]

use std::process::ExitCode;

use pm_bin::{cli_arg, init_env_logger, pm_init};
pm_init!();

use pmbs::cli;

fn main() -> Result<(), ExitCode> {
    init_env_logger();

    if let Some(a) = cli_arg(print_version) {
        cli::main(a)
    } else {
        // pm-bin 会处理 `--version` 和 `--版本`
        Ok(())
    }
}
