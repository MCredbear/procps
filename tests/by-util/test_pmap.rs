// This file is part of the uutils procps package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::common::util::TestScenario;

#[test]
fn test_no_args() {
    new_ucmd!().fails().code_is(1);
}

#[test]
#[cfg(target_os = "linux")]
fn test_existing_pid() {
    use std::process;

    let pid = process::id();
    // TODO ensure that the output format is correct, which is not the case currently
    new_ucmd!().arg(pid.to_string()).succeeds();
}

#[test]
fn test_non_existing_pid() {
    new_ucmd!().arg("999999").fails().code_is(42).no_output();
}

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails().code_is(1);
}
