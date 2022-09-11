use lazy_static::lazy_static;
use regex::{Regex, Captures};

pub fn get_jira_ticket_from_branch_name(branch_name: &str) -> Option<&str> {
    lazy_static! {
        static ref RE_JIRA: Regex = Regex::new(r"([A-Z]{2}-\d+)").unwrap();
    }
    let result: Option<Captures> = RE_JIRA.captures(branch_name);
    match result {
        Some(captures) => Some(captures.get(1).unwrap().as_str()),
        None => None
    }
}
