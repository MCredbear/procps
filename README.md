[![Crates.io](https://img.shields.io/crates/v/procps.svg)](https://crates.io/crates/procps)
[![Discord](https://img.shields.io/badge/discord-join-7289DA.svg?logo=discord&longCache=true&style=flat)](https://discord.gg/wQVJbvJ)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/uutils/procps/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/uutils/procps/status.svg)](https://deps.rs/repo/github/uutils/procps)

[![CodeCov](https://codecov.io/gh/uutils/procps/branch/master/graph/badge.svg)](https://codecov.io/gh/uutils/procps)

# procps

Rust reimplementation of the procps project

Provides command line and full screen utilities for browsing procfs, a "pseudo" file system dynamically generated by the kernel to provide information about the status of entries in its process table (such as whether the process is running, stopped, or a "zombie").

Ongoing:
* `free`: Shows the amount of free and used memory in the system.
* `pgrep`: Searches for processes based on name and other attributes.
* `pidof`: Find the process ID of a running program.
* `pidwait`: Waits for a specific process to terminate.
* `pmap`: Displays the memory map of a process.
* `ps`: Displays information about active processes.
* `pwdx`: Shows the current working directory of a process.
* `slabtop`: Displays detailed kernel slab cache information in real time.
* `snice`: Changes the scheduling priority of a running process.
* `top`: Displays real-time information about system processes.
* `w`: Shows who is logged on and what they are doing.
* `watch`: Executes a program periodically, showing output fullscreen.

TODO:
* `pkill`: Kills processes based on name and other attributes.
* `skill`: Sends a signal to processes based on criteria like user, terminal, etc.
* `tload`: Prints a graphical representation of system load average to the terminal.
* `vmstat`: Reports information about processes, memory, paging, block IO, traps, and CPU activity.

Elsewhere:

 * `kill` is already implemented in https://github.com/uutils/coreutils
 * `uptime`: Shows how long the system has been running, including load average.
   is already implemented in https://github.com/uutils/coreutils

## Installation

Ensure you have Rust installed on your system. You can install Rust through [rustup](https://rustup.rs/).

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/uutils/procps.git
cd procps
cargo build --release
cargo run --release
```

## License

procps is licensed under the MIT License - see the `LICENSE` file for details
