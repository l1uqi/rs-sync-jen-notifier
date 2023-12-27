use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct State {
    pub dingtalk: DingTalkConfig,
    pub jenkins: JenkinsConfig
 }

#[derive(Deserialize, Debug)]
pub struct Config {
    pub dingtalk: DingTalkConfig,
    pub jenkins: JenkinsConfig
}

#[derive(Deserialize, Debug)]
pub struct DingTalkConfig {
    pub url: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct JenkinsConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        // 尝试合并环境变量设置
        cfg.merge(config::Environment::new())?;
        // 转换成我们自己的Config对象
        cfg.try_into()
    }
}

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
pub struct GitLabelAuthor {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct GitLabelCommits {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Option<GitLabelAuthor>,
}

#[derive(Deserialize, Debug)]
pub struct GitLabelChangeRequest {
    pub commits: Vec<GitLabelCommits>,
    pub project: GitLabelProject,
    #[serde(rename = "ref")]
    pub reference: String,
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
