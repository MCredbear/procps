// This file is part of the uutils procps package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::{crate_version, Arg, Command};
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use uucore::error::{UResult, USimpleError};
use uucore::{format_usage, help_about, help_usage};

const ABOUT: &str = help_about!("pwdx.md");
const USAGE: &str = help_usage!("pwdx.md");

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let matches = uu_app().try_get_matches_from(args)?;

    let pids = matches.get_many::<String>("pid").unwrap();

    for pid_str in pids {
        let pid = match pid_str.parse::<i32>() {
            // PIDs start at 1, hence 0 is invalid
            Ok(0) | Err(_) => {
                return Err(USimpleError::new(
                    1,
                    format!("invalid process id: {pid_str}"),
                ))
            }
            Ok(pid) => pid,
        };

        let cwd_link = format!("/proc/{}/cwd", pid);

        match fs::read_link(Path::new(&cwd_link)) {
            Ok(path) => println!("{}: {}", pid, path.display()),
            Err(e) => {
                eprintln!("pwdx: failed to read link for PID {}: {}", pid, e);
                process::exit(1);
            }
        }
    }

    Ok(())
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .arg(
            Arg::new("pid")
                .value_name("PID")
                .help("Process ID")
                .required(true)
                .num_args(1..)
                .index(1),
        )
}
