use reqwest::{StatusCode, Error};
use serde::{Serialize, Deserialize};

use axum::{
    routing::{get, post},
    Json,
    Router, http::{response, request},
};

#[derive(Deserialize, Debug)]
struct RequestJenkinsBuild {
    job_name: String,
    git_branch: String
}

#[derive(Deserialize,Debug)]
struct RequestDingTalk {
    text: String,
}

#[derive(Debug)]
enum ResponseCode {
    Success(u16),
    ServerError(u16),
    Fail(u16),
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomResult {
    code: u16,
    message: String,
    data: Option<serde_json::Value>,
}

#[derive(Serialize, Debug)]
struct ActionCard {
    title: String,
    text: String,
    btnOrientation: String,
}

#[derive(Serialize, Debug)]
struct DingTalkRequest {
    msgtype: String,
    actionCard: ActionCard,
}

// 钉钉机器人URL
const DING_TALK_URL: &str = "钉钉机器人地址";

// 钉钉消息标题
const DING_TITLE: &str = "自动化通知";

// jenkins url
const JENKINS_URL: &str = "JENKINS 地址";

const JENKIDS_USER_NAME: &str = "JENKINS 账号";

const JENKIDS_PASSWORD: &str = "JENKINS 密码";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
       .route("/", get(hello))
       .route("/jenkins/info", get(get_jenkins_info))
       .route("/jenkins/lanunchBuild", post(launch_jenkins_build))
       .route("/dingtalk/send", post(send_dingtalk_message));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await;
    match listener {
        Ok(listener) => {
            println!("listening on 0.0.0.0:3005");
            axum::serve(listener, app).await.unwrap();
        }
        Err(err) => {
            println!("err: {}", err);
        }
    }
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn send_request(method: reqwest::Method, url: String, auth: Option<(String, Option<String>)>, body: Option<String>) -> Json<CustomResult> {
    println!("request: {}", url);
    let client = reqwest::Client::new();
    let mut request = client.request(method, &url);
    
    if let Some((username, password)) = auth {
        request = request.basic_auth(username, password);
    }

    if let Some(body_content) = body {
        request = request.body(body_content);
    }

    
    let resp = request.header("Content-Type", "application/json").send().await;

    let result = match resp {
        Ok(response) => {
            let code = response.status().as_u16();
            let message = if response.status().is_success() { "Success".to_string() } else { "Failed".to_string() };
            let data = response.text().await.ok().and_then(|body| serde_json::from_str::<serde_json::Value>(&body).ok());
            
            CustomResult {
                code,
                message,
                data,
            }
        }
        Err(e) => {
            CustomResult {
                code: 500,
                message: format!("Failed to fetch data: {:?}", e),
                data: None,
            }
        }
    };

    println!("result: {:#?}", &result);
    Json(result)
}

async fn send_dingtalk_message(body: Json<RequestDingTalk>) -> Json<CustomResult> {
    let request_body = body.0;
    let url = DING_TALK_URL.to_string();
    let request = DingTalkRequest { 
        msgtype: "actionCard".to_string(),
        actionCard: ActionCard {
            title: DING_TITLE.to_string(),
            text: request_body.text,
            btnOrientation: "0".to_string(),
        },
    };

    let body_content = serde_json::to_string(&request).unwrap_or_else(|_| "".to_string());

    send_request(reqwest::Method::POST, url, None, Some(body_content)).await
}

async fn launch_jenkins_build(body: Json<RequestJenkinsBuild>) -> Json<CustomResult> {
    let url = format!("{url}/job/{job_name}/buildWithParameters?git_branch={git_branch}", url = JENKINS_URL, git_branch = body.0.git_branch, job_name = body.0.job_name);
    let auth = (JENKIDS_USER_NAME.to_string(), Some(JENKIDS_PASSWORD.to_string()));

    send_request(reqwest::Method::POST, url, Some(auth), None).await
}

async fn get_jenkins_info() -> Json<CustomResult> {
    let url = format!("{url}/api/json", url = JENKINS_URL);
    let auth = (JENKIDS_USER_NAME.to_string(), Some(JENKIDS_PASSWORD.to_string()));

    send_request(reqwest::Method::GET, url, Some(auth), None).await
}
