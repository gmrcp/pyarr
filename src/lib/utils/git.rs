use std::process::{Command, Stdio};
use std::error::Error;

use execute::{Execute, command};

pub fn current_branch_name() -> Result<String, Box<dyn Error>> {
    let mut command = command("git symbolic-ref --short -q HEAD");
    command.stdout(Stdio::piped());
    let output = command.execute_output()?;

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }

    let result = String::from_utf8(output.stdout)?;
    Ok(result.trim_end().to_string())
}

pub fn push_branch(branch_name: &String) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("git");
    command.arg("push").arg("--porcelain").arg("--set-upstream").arg("origin").arg(branch_name);
    command.stdout(Stdio::piped());
    let output = command.execute_output()?;
    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Git push Ok.");
        } else {
            eprintln!("Git push Failed.");
        }
    } else {
        eprintln!("Git push Interrupted!");
    }
    Ok(())
}

pub fn status() -> Result<(), Box<dyn Error>>  {
    let mut comm = command("git status --porcelain");
    comm.stdout(Stdio::piped());
    let output = comm.execute_output()?;
    if !output.stdout.is_empty() {
        println!("These files are uncommited");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Please commit your changes!".into());
    }
    // command.execute_check_exit_status_code(0)?;
    // println!("{output:?}");
    Ok(())
}