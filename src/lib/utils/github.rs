use std::process::Command;
use std::error::Error;
use std::thread;
use std::sync::Arc;

use serde::Deserialize;
use execute::Execute;
use anyhow::{Context, Result};

pub fn check_status() -> Result<(), Box<dyn Error>> {
    Command::new("gh").arg("--version")
        .execute_check_exit_status_code(0)
        .with_context(|| format!("gh CLI not installed."))?;
    println!("✅ gh CLI installed.");
    Ok(())
}

pub fn check_auth() -> Result<(), Box<dyn Error>> {
    Command::new("gh").arg("auth").arg("status")
        .execute_check_exit_status_code(0)
        .with_context(|| format!("gh auth not setup."))?;
    println!("✅ gh is authenticated.");
    Ok(())
}

pub enum RepoParameters {
    Name,
    Owner,
}

pub fn get_repo_parameter(parameter: RepoParameters) -> Result<String, Box<dyn Error>> {
    let args = match parameter {
        RepoParameters::Owner => ("owner", ".owner | .login"),
        RepoParameters::Name => ("name", ".name")
    };

    let output = Command::new("gh")
        .arg("repo").arg("view").arg("--json").arg(args.0)
        .arg("-q").arg(args.1)
        .output()?;
    let parameter = String::from_utf8(output.stdout)?;
    Ok(parameter)
}

pub enum GithubEntity {
    Teams,
    Contributors,
    Labels
}

impl GithubEntity {
    fn as_str(&self) -> &'static str {
        match self {
            GithubEntity::Teams => "teams",
            GithubEntity::Contributors => "contributors",
            GithubEntity::Labels => "labels",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct LabelOrTeam {
    name: String,
}

pub fn get_repo_labels(org: &String, repo: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let data = run_list_repos(GithubEntity::Labels, &org, &repo)?;
    let labels_obj: Vec<LabelOrTeam> = serde_json::from_str(&data)?;
    let labels: Vec<String> = labels_obj.into_iter().map(|label| label.name).collect();
    println!("Labels: {:?}", labels);
    Ok(labels)
}

pub fn get_repo_teams(org: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let output = get_api_command().arg(format!("/orgs/{}/teams", org)).output()?;
    let data = String::from_utf8(output.stdout)?;
    let teams_obj: Vec<LabelOrTeam> = serde_json::from_str(&data)?;
    let teams: Vec<String> = teams_obj.into_iter().map(|team| team.name).collect();
    println!("Teams: {:?}", teams);
    Ok(teams)
}

#[derive(Deserialize, Debug)]
pub struct Team {
    name: String,
    organization: Contributor
}

pub fn get_user_teams(repo_owner: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let output = get_api_command().arg("/user/teams").output()?;
    let data = String::from_utf8(output.stdout)?;
    let teams_obj: Vec<Team> = serde_json::from_str(&data)?;
    let teams = teams_obj.into_iter()
        .filter(|team| team.organization.login == repo_owner)
        .map(|team| team.name)
        .collect::<Vec<String>>();
    Ok(teams)
}

#[derive(Deserialize, Debug)]
pub struct Contributor {
    login: String,
}

pub fn get_repo_contributors(org: &String, repo: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let data = run_list_repos(GithubEntity::Contributors, org, repo)?;
    let contributors_obj: Vec<Contributor> = serde_json::from_str(&data)?;
    let contributors: Vec<String> = contributors_obj.into_iter().map(|contributor| contributor.login).collect();
    println!("Contributors: {:?}", contributors);
    Ok(contributors)
}

fn run_list_repos(gh_entity: GithubEntity, org: &String, repo: &String) -> Result<String, Box<dyn Error>>{
    let entity = gh_entity.as_str();
    let output = get_api_command().arg(format!("/repos/{}/{}/{}", org, repo, entity)).output()?;
    let result = String::from_utf8(output.stdout)?;
    Ok(result)
}

fn get_api_command() -> Command {
    let mut command = Command::new("gh");
    command.arg("api").arg("-H").arg("Accept: application/vnd.github+json");
    command
}

pub fn parallel(org: &String, repo: &String) -> Vec<String> {
    let mut threads = vec![];
    let org_arc = Arc::new(org.clone());
    let repo_arc = Arc::new(repo.clone());

    let thread_org = Arc::clone(&org_arc);
    threads.push(thread::spawn(move || -> Vec<String> {
        get_repo_teams(&thread_org).unwrap_or(vec![])
    }));

    let thread_org = Arc::clone(&org_arc);
    let thread_repo = Arc::clone(&repo_arc);
    threads.push(thread::spawn(move || -> Vec<String> {
        get_repo_contributors(&thread_org, &thread_repo).unwrap_or(vec![])
    }));

    let results = threads.into_iter().map(|thread| thread.join().unwrap()).flatten().collect::<Vec<String>>();

    println!("Final result: {:?}", results);
    results
}

pub fn create_pr(title: &String, description: &String, labels: &Vec<String>, reviewers: &Vec<String>) -> Command {
    let mut command = Command::new("gh");
    command.arg("pr").arg("create")
        .arg("--title").arg(title)
        .arg("--body").arg(description)
        .arg("--label").arg(labels.join(","))
        .arg ("--reviewer").arg(reviewers.join(","));
    command
}
