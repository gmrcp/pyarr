use std::error::Error;
use std::process;

use clap::{Parser, Subcommand};
use confy;
use serde::{Serialize, Deserialize};
use dialoguer::{Select, MultiSelect, Confirm, Input};
use colored::Colorize;
use cli_table::{print_stdout, Table, WithTitle};

mod lib;
use lib::{io, regexpr};
use lib::utils::{github, git};

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
        Self { orgs: vec![] }
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
            let repo_owner = String::from("eatkitch");
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

                let chosen_team = io::select("Select a team", &user_teams)?;

                cfg.orgs.push(Organization { name: repo_owner, default_team: user_teams.remove(chosen_team) });

                confy::store("pyarr", cfg)?;
            } else {
                println!("You don't belong to any team of this repo's organization!");
            }

            process::exit(0);
        },
        Commands::New { bare, default } => {
            git::status()?;

            let current_branch = git::current_branch_name()?;
            
            git::push_branch(&current_branch)?;
            
            let pr_type = io::select_pr_type()?;
            let title = format!("{}/{}", pr_type, current_branch);

            let mut description = String::from("");
            if let Some(jira_ticket) = regexpr::get_jira_ticket_from_branch_name(&current_branch) {
                description = format!("[Task](https://kitch.atlassian.net/browse/{}", jira_ticket);
            }

            let repo_owner = String::from("eatkitch");
            let repo_name = String::from("api");

            let available_labels = github::get_repo_labels(&repo_owner, &repo_name)?;
            let chosen_labels = io::multiselect("Select labels", available_labels)?;

            let available_reviewers = github::parallel(&repo_owner, &repo_name);
            let chosen_reviewers = io::multiselect("Select reviewers", available_reviewers)?;

            github::create_pr(title, description, &chosen_labels, &chosen_reviewers)?;
            github::open_pr_in_browser(&current_branch)?;

            process::exit(0);
        }
    } 
}

// TODO
// 2. input se nao passar flag de usar config
// 2.1 menu para escolher label e reviewers + reviewers tams 
// 3. async
// 4. criaçao de link para jira
// 5. criaçao do PR
