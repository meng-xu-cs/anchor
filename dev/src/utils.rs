use std::process::Command;

use anyhow::{bail, Result};
use itertools::Itertools;

/// Execute the command, expect success and extract both stdout and stderr as string
pub fn execute_command_expect_output(
    command: &mut Command,
) -> Result<(Option<String>, Option<String>)> {
    let output = match command.output() {
        Ok(r) => r,
        Err(e) => bail!(
            "unable to execute command {} {}: {e}",
            command.get_program().display(),
            command.get_args().map(|a| a.display()).join(" "),
        ),
    };
    if !output.status.success() {
        bail!(
            "command {} {} returned a non-zero exit code: {}",
            command.get_program().display(),
            command.get_args().map(|a| a.display()).join(" "),
            output.status
        );
    }

    let stdout = if output.stdout.is_empty() {
        None
    } else {
        match str::from_utf8(&output.stdout) {
            Ok(s) => Some(s.trim_end().to_owned()),
            Err(e) => bail!(
                "command {} {} outputs non-utf8 stdout: {e}",
                command.get_program().display(),
                command.get_args().map(|a| a.display()).join(" "),
            ),
        }
    };
    let stderr = if output.stderr.is_empty() {
        None
    } else {
        match str::from_utf8(&output.stderr) {
            Ok(s) => Some(s.trim_end().to_owned()),
            Err(e) => bail!(
                "command {} outputs non-utf8 stderr: {e}",
                command.get_args().map(|a| a.display()).join(" "),
            ),
        }
    };
    Ok((stdout, stderr))
}

/// Execute the command, expect success and extract only stdout as string
pub fn execute_command_expect_stdout_only(command: &mut Command) -> Result<String> {
    let (stdout, stderr) = execute_command_expect_output(command)?;
    match stderr {
        None => Ok(stdout.unwrap_or_else(String::new)),
        Some(stderr) => bail!(
            "command {} outputs stderr unexpectedly: {stderr}",
            command.get_args().map(|a| a.display()).join(" ")
        ),
    }
}
