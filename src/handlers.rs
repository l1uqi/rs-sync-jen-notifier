use axum::Json;
use rs_sync_jen_notifier::{dto::{DingTalkMessageRequest, Payload, JenkinsBuildRequest, DingTalkMessage, ActionCard, GitLabelChangeRequest}, http::request};


// 钉钉机器人URL
const DING_TALK_URL: &str = "";

// 钉钉消息标题
const DING_TITLE: &str = "自动化通知";

// jenkins url
const JENKINS_URL: &str = "";

const JENKIDS_USER_NAME: &str = "";

const JENKIDS_PASSWORD: &str = "";

pub async fn hello() -> &'static str {
  "Hello, world!"
}

// 发送钉钉消息
pub async fn send_dingtalk_message(body: Json<DingTalkMessageRequest>) -> Json<Payload> {
  let request_body = body.0;
  let url = DING_TALK_URL.to_string();
  let params = DingTalkMessage { 
      msgtype: "actionCard".to_string(),
      actionCard: ActionCard {
          title: DING_TITLE.to_string(),
          text: request_body.text,
          btnOrientation: "0".to_string(),
      },
  };

  let body_content = serde_json::to_string(&params).unwrap_or_else(|_| "".to_string());

  request(reqwest::Method::POST, url, None, Some(body_content)).await
}

// 触发jenkins构建
pub async fn launch_jenkins_build(body: Json<JenkinsBuildRequest>) -> Json<Payload> {
  let url = format!("{url}/job/{job_name}/buildWithParameters?git_branch={git_branch}", url = JENKINS_URL, git_branch = body.0.git_branch, job_name = body.0.job_name);
  let auth = (JENKIDS_USER_NAME.to_string(), Some(JENKIDS_PASSWORD.to_string()));

  request(reqwest::Method::POST, url, Some(auth), None).await
}

// 获取jenkins信息
pub async fn get_jenkins_info() -> Json<Payload> {
  let url = format!("{url}/api/json", url = JENKINS_URL);
  let auth = (JENKIDS_USER_NAME.to_string(), Some(JENKIDS_PASSWORD.to_string()));
  request(reqwest::Method::GET, url, Some(auth), None).await
}

// 接受gitallabel的webhook
// 触发jenkins构建
pub async fn reciver_git_change(body: Json<GitLabelChangeRequest>) {
    println!("projectName: {}", body.0.project.name);
    println!("commits: {}", body.0.commits.len());
    println!("message: {}", body.0.commits[0].message);
    println!("id: {}", body.0.commits[0].id);
    println!("timestamp: {}", body.0.commits[0].timestamp);
    println!("url: {}", body.0.commits[0].url);
//   let url = format!("{url}/api/json", url = JENKINS_URL);
//   let auth = (JENKIDS_USER_NAME.to_string(), Some(JENKIDS_PASSWORD.to_string()));
//   request(reqwest::Method::GET, url, Some(auth), None).await
}
