use reqwest::{StatusCode, Error};
use serde::{Serialize, Deserialize};

use axum::{
    routing::{get, post},
    Json,
    Router, http::{response, request},
};
use serde_json::Value;

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
    data: Option<Value>,
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

#[derive(Deserialize,Debug)]
struct RequestDingTalk {
    text: String,
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

async fn send_dingtalk_message(body: Json<RequestDingTalk>) -> Json<CustomResult> {
    let request_body = body.0;
    let client = reqwest::Client::new();
    let request = DingTalkRequest { 
        msgtype: "actionCard".to_string(),
        actionCard: ActionCard {
            title: DING_TITLE.to_string(),
            text: request_body.text,
            btnOrientation: "0".to_string(),
        },
    };
    let resp = client
       .post(DING_TALK_URL)
       .body(serde_json::to_string(&request).unwrap_or_else(|_| "".to_string()))
       .header("Content-Type", "application/json")
       .send()
       .await;
    
       let result = match resp {
        Ok(response) => {
            if response.status().is_success() {
                // 当请求成功时，构建 CustomResult 结构体
                let body = response.text().await.unwrap_or_else(|_| "".to_string());
                let parsed_body = serde_json::from_str::<serde_json::Value>(&body).unwrap_or_else(|_| serde_json::Value::Null);
                CustomResult {
                    code: 200,
                    message: "Success".to_string(),
                    data: Some(parsed_body)
                }
            } else {
                CustomResult {
                    code: response.status().as_u16(),
                    message: "Failed".to_string(),
                    data: None
                }
            }
        }
        Err(_) => {
            CustomResult {
                code: 500, // Internal Server Error
                message: "Failed to fetch data".to_string(),
                data: None
            }
        }
    };
    Json(result)
}

async fn get_jenkins_info() -> Json<CustomResult> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{url}/api/json", url = JENKINS_URL))
        .basic_auth(JENKIDS_USER_NAME, Some(JENKIDS_PASSWORD))
        .send()
        .await;
   
    // 处理请求结果
    let result = match resp {
        Ok(response) => {
            if response.status().is_success() {
                // 当请求成功时，构建 CustomResult 结构体
                let body = response.text().await.unwrap_or_else(|_| "".to_string());
                let parsed_body = serde_json::from_str::<serde_json::Value>(&body).unwrap_or_else(|_| serde_json::Value::Null);
                CustomResult {
                    code: 200,
                    message: "Success".to_string(),
                    data: Some(parsed_body)
                }
            } else {
                CustomResult {
                    code: response.status().as_u16(),
                    message: "Failed".to_string(),
                    data: None
                }
            }
        }
        Err(_) => {
            CustomResult {
                code: 500, // Internal Server Error
                message: "Failed to fetch data".to_string(),
                data: None
            }
        }
    };
    println!("result: {:#?}", &result);
    Json(result)
}
