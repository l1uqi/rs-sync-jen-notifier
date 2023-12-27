use std::vec;

use crate::dto::GitLabelCommits;

#[derive(Debug)]
pub struct ProjectInfo {
  pub ref_: String,
  pub branch_name: String,
  pub job_name: String,
  pub project_name: String,
}

#[derive(Debug)]
pub struct GitContent {
  pub author: String,
  pub content: String,
}

pub fn git_change_content(commits: vec::Vec<GitLabelCommits>, config: serde_json::Value) -> GitContent {
  let mut author = String::new();
  let mut commit_msg_str = String::new();
  commit_msg_str.push_str("\n\n");
  // 循环commits 获取提交信息 
  for commit in commits {
    println!("commit: {:?}", commit.message);
    let commit_message: String = commit.message.trim().to_string();
    // 指定分支
    author = match commit.author {
      Some(author) => git_account_to_author(author.name.as_str(), &config).to_string(),
      None => "".to_string(),
    };
    let commit_message_with_linebreak = commit_message.replace("\\n", "");
    commit_msg_str.push_str("-");
    commit_msg_str.push_str(&author);
    commit_msg_str.push_str(" : ");
    commit_msg_str.push_str(&commit_message_with_linebreak);
    commit_msg_str.push_str("\n\n");
  }
  GitContent {
    author: author,
    content: commit_msg_str,
  }
}

// gitid to 姓名
pub fn git_account_to_author<'a>(name: &'a str, config: &'a serde_json::Value) -> &'a str {
  match config["accounts"].get(name) {
      Some(author) => author.as_str().unwrap(),
      None => name,
  }
}

pub fn git_to_jenkins_project_info<'a>(name: &str, config: &'a serde_json::Value) -> Option<ProjectInfo> {
  let project_info: Option<ProjectInfo> = match config["projects"].get(name) {
      Some(details) => {
          Some(ProjectInfo {
              ref_: details["ref_"].as_str().unwrap().to_string(),
              branch_name: details["branch_name"].as_str().unwrap().to_string(),
              job_name: details["job_name"].as_str().unwrap().to_string(),
              project_name: details["project_name"].as_str().unwrap().to_string(),
          })
      },
      None => None,
  };
  project_info
}
