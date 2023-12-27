use std::sync::Arc;

use axum:: {Json, Extension};
use rs_sync_jen_notifier::{dto::{DingTalkMessageRequest, Payload, JenkinsBuildRequest, DingTalkMessage, ActionCard, GitLabelChangeRequest, State}, http::request, utils::{git_to_jenkins_project_info, git_change_content}};

pub async fn hello() -> &'static str {
  "Hello, world!"
}

// 发送钉钉消息
pub async fn send_dingtalk_message(state: Extension<Arc<State>>, body: Json<DingTalkMessageRequest>) -> Json<Payload> {
  let request_body = body.0;
  let url = state.dingtalk.url.to_string();
  let params = DingTalkMessage { 
      msgtype: "actionCard".to_string(),
      actionCard: ActionCard {
        title: state.dingtalk.title.to_string(),
        text: request_body.text,
        btnOrientation: "0".to_string(),
      },
  };

  let body_content = serde_json::to_string(&params).unwrap_or_else(|_| "".to_string());

  request(reqwest::Method::POST, url, None, Some(body_content)).await
}

// 触发jenkins构建
pub async fn launch_jenkins_build(state: Extension<Arc<State>>, body: Json<JenkinsBuildRequest>) -> Json<Payload> {
  let url = format!("{url}/job/{job_name}/buildWithParameters?git_branch={git_branch}", url = state.jenkins.url, git_branch = body.0.git_branch, job_name = body.0.job_name);
  let auth = (state.jenkins.username.to_string(), Some(state.jenkins.password.to_string()));

  request(reqwest::Method::POST, url, Some(auth), None).await
}

// 获取jenkins信息
pub async fn get_jenkins_info(state: Extension<Arc<State>>) -> Json<Payload> {
  
  let url = format!("{url}/api/json", url = state.jenkins.url);
  println!("url{}", url);
  let auth = (state.jenkins.username.to_string(), Some(state.jenkins.password.to_string()));
  request(reqwest::Method::GET, url, Some(auth), None).await
}

// 接受gitlabel的webhook
// 触发jenkins构建
// 推送钉钉消息
pub async fn reciver_git_change(state: Extension<Arc<State>>, body: Json<GitLabelChangeRequest>) {
    let info = git_to_jenkins_project_info(&body.0.project.name, &state.config.clone());
    let git_content = git_change_content(body.0.commits, state.config.clone());
    if info.is_some() {
        let project_info = info.unwrap();
        if project_info.ref_ == body.0.reference {
            let build_request = JenkinsBuildRequest {
                job_name: project_info.job_name.clone(),
                git_branch: project_info.branch_name.clone(),
            };
            let result = launch_jenkins_build(state.clone(), Json(build_request)).await;
            if result.0.code == 201 {
                let ding_talk_request = DingTalkMessageRequest {
                    text: format!(
                        "{project}\n---\n状态: 开始更新\n---\n提交更新人: {author}\n---\n更新内容:{content}",
                        project = project_info.project_name.clone(),
                        author = git_content.author,
                        content = git_content.content,
                    ),
                 };
                let send_result = send_dingtalk_message(state.clone(), Json(ding_talk_request)).await;
                println!("result: {:?}", send_result);
            }
        } else {
            println!("分支不支持")
        }
        
    } else {
        println!("未能匹配")
    }
}
