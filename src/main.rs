use clap::{Parser, Subcommand};
use confy;
use serde::{Serialize, Deserialize};
use dialoguer::{Select, MultiSelect, Confirm};
use colored::Colorize;
use cli_table::{print_stdout, Table, WithTitle};

use std::error::Error;

mod lib;
use lib::utils::{github};

#[derive(Serialize, Deserialize, Debug)]
struct PyarrConfig {
    orgs: Vec<Organization>,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Table)]
struct Organization {
    #[table(title = "Organisation name")]
    name: String,
    #[table(title = "Default team")]
    default_team: String,
}

impl std::default::Default for PyarrConfig {
    fn default() -> Self {
        Self {
            orgs: vec![]
        }
    }
}

/// Create PRs on github with defaulted parameters suchs as name, description, labels and reviewers  
#[derive(Parser, Debug)]
#[clap(name = "pyarr 🦜")]
#[clap(about = "Create streamlined PRs on github", long_about = None)]
#[clap(version)]
struct Pyarr {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Configuration file 
    #[clap(arg_required_else_help = true)]
    Config {
        /// Define the default team the current repo's owner
        #[clap(short = 't', long, action)]
        team: bool,
        /// Define the default team the current repo's owner
        #[clap(short = 's', long, action)]
        show: bool,
    },
    /// Creates a new PR on the current repo
    #[clap(arg_required_else_help = true)]
    New {
        /// Creates a PR with bare minimum info
        #[clap(short = 'b', long, action)]
        bare: bool,
        /// Creates a PR with the stored defaults
        #[clap(short = 'd', long, action)]
        default: bool,
    },
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = Pyarr::parse();
    let mut cfg: PyarrConfig = confy::load("pyarr")?;
    println!("{args:?}");

    github::check_status()?;
    github::check_auth()?;

    let repo_owner = github::get_repo_parameter(github::RepoParameters::Owner)?;
    let repo_name = github::get_repo_parameter(github::RepoParameters::Name)?;

    match args.command {
        Commands::Config { team: _, show } => {
            if show {
                print_stdout(cfg.orgs.with_title()).unwrap();
                return Ok(());
            }
            // let repo_owner = String::from("eatkitch");
            let mut user_teams = github::get_user_teams(&repo_owner).unwrap();

            if user_teams.len() > 0 {
                let org_already_saved = cfg.orgs.iter().find(|org| org.name == repo_owner);

                if let Some(org) = org_already_saved {
                    println!("{} already has {} as the default team.", org.name.bold(), org.default_team.cyan());
                    if Confirm::new().with_prompt("Choose a new default team?").interact()? {
                        println!("Looks like you want to continue!");
                        let index = cfg.orgs.iter().position(|org| org.name == repo_owner).unwrap();
                        cfg.orgs.swap_remove(index);
                    } else {
                        println!("nevermind then :(");
                        return Ok(());
                    }
                }

                let chosen_team = Select::new()
                    .with_prompt("Select a team")
                    .items(&user_teams)
                    .default(0)
                    .interact()?;

                println!("chosen team: {chosen_team:?}");

                cfg.orgs.push(Organization { name: repo_owner, default_team: user_teams.remove(chosen_team) });
                confy::store("pyarr", cfg)?;
            } else {
                println!("You don't belong to any team of this repo's organization!");
            }

            return Ok(());
        },
        Commands::New { bare, default } => {
            return Ok(());
        }
    } 

    // let results = github::parallel(&repo_owner, &repo_name);

    // let chosen : Vec<usize> = MultiSelect::new()
    //     .with_prompt("Select labels")
    //     .items(&results[0])
    //     .interact()?;

    // Ok(())
}

// TODO
// 2. input se nao passar flag de usar config
// 2.1 menu para escolher label e reviewers + reviewers tams 
// 3. async
// 4. criaçao de link para jira
// 5. criaçao do PR
// 6. abrir link em pagina
