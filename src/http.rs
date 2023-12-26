use axum::Json;

use crate::dto::Payload;

pub async fn request(method: reqwest::Method, url: String, auth: Option<(String, Option<String>)>, body: Option<String>) -> Json<Payload> {
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
          
          Payload {
              code,
              message,
              data,
          }
      }
      Err(e) => {
          Payload {
              code: 500,
              message: format!("Failed to fetch data: {:?}", e),
              data: None,
          }
      }
  };
  Json(result)
}
