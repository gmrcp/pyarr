use clap::Parser;
use confy;
use serde::{Serialize, Deserialize};
use dialoguer::MultiSelect;

use std::error::Error;

mod utils;
use utils::{github, util};

#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    first_time_use: bool,
    org: String,
    squad: String,
}

impl std::default::Default for MyConfig {
    fn default() -> Self { Self { first_time_use: true, org: "eatkitch".into(), squad: "".into() } }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    /// Allow human-readable durations
    #[clap(short = 'C', long, action)]
    config: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _args = Cli::parse();
    let cfg = MyConfig { first_time_use: true, org: "eatkitch".into(), squad: "".into() };
    confy::store("grrs", cfg)?;
    // println!("{:?}", cfg);
    // cfg.org = "some other org".into();
    // confy::store("grrs", cfg)?;
    // println!("Status: {}", output.status);
    github::check_status()?;
    github::check_auth()?;
    // let current_repo = util::check_current_dir()?;
    let current_repo = String::from("api");
    let org = String::from("eatkitch");
    // github::get_repo_labels(&org, &current_repo)?;
    // github::get_repo_teams(&org)?;
    // github::get_repo_contributors(&org, &current_repo)?;

    let results = github::parallel(&org, &current_repo);

    println!("yo mamma");

    let chosen : Vec<usize> = MultiSelect::new()
        .with_prompt("Select labels")
        .items(&results[0])
        .interact()?;

    Ok(())
    // println!("First arg: {}, second: {}", args.pattern, args.path.display());
}

// TODO
// 1. config toml
// 2. input se nao passar flag de usar config
// 2.1 menu para escolher label e reviewers + reviewers tams 
// 3. async
// 4. criaçao de link para jira
// 5. criaçao do PR
// 6. abrir link em pagina
