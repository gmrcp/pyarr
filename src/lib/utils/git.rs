use std::process::Command;
use std::error::Error;

use execute::Execute;
use anyhow::{Context, Result};

pub fn get_current_branch() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").arg("branch").arg("--show-current").output()?;
    let result = String::from_utf8(output.stdout)?;
    Ok(result)
}

pub fn check_remote_branch(branch_name: &String) -> Result<bool, Box<dyn Error>> {
    Command::new("git")
        .arg("ls-remote")
        .arg("--exit-code")
        .arg("--heads")
        .arg("origin")
        .arg(branch_name)
        .execute_check_exit_status_code(0)
        .with_context(|| format!("Branch not present in remote"))?;

    Ok(true)
}