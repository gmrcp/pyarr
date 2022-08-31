pub fn check_current_dir() -> Result<String, Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;

    if let Some(repo_name) = cwd.file_name() {
        let converted_repo_name = repo_name.to_str().unwrap();
        println!("Current repo: {}", converted_repo_name);
        return Ok(converted_repo_name.into());
    } else {
        println!("OH NO");
        return Ok(String::from("sds"))
    }
}
