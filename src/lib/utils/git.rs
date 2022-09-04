use std::process::Command;
use std::error::Error;

pub fn get_current_branch() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").arg("branch").arg("--show-current").output()?;
    let result = String::from_utf8(output.stdout)?;
    Ok(result)
}