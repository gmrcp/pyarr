use std::error::Error;
use std::io::{self, Write};
use strum::{Display, EnumIter, IntoEnumIterator};
use dialoguer::{Select, MultiSelect, Confirm, Input};

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

#[derive(Display, EnumIter)]
#[strum(serialize_all="snake_case")]
pub enum PullRequestType {
    Bug,
    Feature,
    Hotfix,
}

pub fn multiselect(prompt: &str, strings: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    // clear_console();
    let chosen_strings_idx: Vec<usize> = MultiSelect::new()
        .with_prompt(format!("{} (select with Spacebar, continue with Enter)", prompt))
        .items(&strings)
        .interact()?;
    println!("indexes: {chosen_strings_idx:?}");
    let chosen_strings = strings
        .into_iter()
        .enumerate()
        .filter(|(index, _)| chosen_strings_idx.contains(index))
        .map(|(_, ele)| ele)
        .collect();
    println!("chosen strings: {chosen_strings_idx:?}");
    Ok(chosen_strings)
}

pub fn select(prompt: &str, strings: &Vec<String>) -> Result<usize, Box<dyn Error>> {
    // clear_console();
    let chosen_idx = Select::new()
        .with_prompt(prompt)
        .items(strings)
        .default(0)
        .interact()?;
    Ok(chosen_idx)
}

pub fn select_pr_type() -> Result<String, Box<dyn Error>> {
    clear_console();
    let types = PullRequestType::iter()
        .map(|pr_type| pr_type.to_string())
        .collect::<Vec<String>>();
    let chosen_idx = select("Choose type of PR", &types)?;
    Ok(types[chosen_idx].clone())
}

pub fn clear_console() {
    io::stdout().flush().unwrap();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}