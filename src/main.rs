use std::{sync::Arc, fs::File, io::Read, env};

use axum::{routing::{get, post}, Router, Extension};
use rs_sync_jen_notifier::dto::{Config, State};

mod handlers;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let cfg = Config::from_env().expect("初始化项目配置失败");

    let mut file = File::open("config.json").expect("Config file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read config file");

    let config: serde_json::Value = serde_json::from_str(&contents).expect("Failed to parse config file");

    let state = Arc::new(State {
        dingtalk: cfg.dingtalk,
        jenkins: cfg.jenkins,
        config: config
    });

    tracing_subscriber::fmt::init();

    let app = Router::new()
       .route("/", get(handlers::hello))
       .route("/reciver", post(handlers::reciver_git_change))
       .route("/jenkins/info", get(handlers::get_jenkins_info))
       .route("/jenkins/lanunchBuild", post(handlers::launch_jenkins_build))
       .route("/dingtalk/send", post(handlers::send_dingtalk_message))
       .layer(Extension(state));

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
