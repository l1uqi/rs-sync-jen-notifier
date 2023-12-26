use axum::{routing::{get, post}, Router};

mod handlers;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = Router::new()
       .route("/", get(handlers::hello))
       .route("/jenkins/info", get(handlers::get_jenkins_info))
       .route("/reciver", post(handlers::reciver_git_change))
       .route("/jenkins/lanunchBuild", post(handlers::launch_jenkins_build))
       .route("/dingtalk/send", post(handlers::send_dingtalk_message));

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
