use serde::{Serialize, Deserialize};
use toml::value::Array;


#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub code: u16,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Deserialize,Debug)]
pub struct DingTalkMessageRequest {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct JenkinsBuildRequest {
    pub job_name: String,
    pub git_branch: String
}

#[derive(Deserialize, Debug)]
pub struct GitLabelProject {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct GitLabelCommits {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct GitLabelChangeRequest {
    pub commits: Vec<GitLabelCommits>,
    pub project: GitLabelProject
}

#[derive(Serialize, Debug)]
pub struct ActionCard {
    pub title: String,
    pub text: String,
    pub btnOrientation: String,
}

#[derive(Serialize, Debug)]
pub struct DingTalkMessage {
   pub msgtype: String,
   pub actionCard: ActionCard,
}
