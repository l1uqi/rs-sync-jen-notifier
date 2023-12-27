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

pub fn git_change_content(commits: vec::Vec<GitLabelCommits>) -> GitContent {
  let mut author = String::new();
  let mut commit_msg_str = String::new();
  commit_msg_str.push_str("\n\n");
  // 循环commits 获取提交信息 
  for commit in commits {
    println!("commit: {:?}", commit.message);
    let commit_message: String = commit.message.trim().to_string();
    // 指定分支
    author = match commit.author {
      Some(author) => git_account_to_author(author.name.as_str()).to_string(),
      None => "".to_string(),
    };
    let commit_message_with_linebreak = commit_message.replace("\\n", "");
    commit_msg_str.push_str("-");
    commit_msg_str.push_str(&author);
    commit_msg_str.push_str(&commit_message_with_linebreak);
    commit_msg_str.push_str("\n\n");
  }
  println!("commit_msg_str: {}", commit_msg_str);
  GitContent {
    author: author,
    content: commit_msg_str,
  }
}

// gitid to 姓名
pub fn git_account_to_author(name: &str) -> &str {
  println!("git_account_to_author: {}", name);
  match name {
    "fatTiger" => "祝震",
    "c37\\c37csq" => "陈思琪",
    "pgy" => "彭国洋",
    "xuxinsheng" => "许新胜",
    "liuqi" => "刘琦",
    "wucai" =>"吴偲",
    "fz" => "邓方正",
    _ => name,
  }
}

pub fn git_to_jenkins_project_info(name: String) -> Option<ProjectInfo> {
  let project_info: Option<ProjectInfo> = match name.as_str() {
    "onek-medicine-erp" => Some(ProjectInfo {
        ref_: String::from("refs/heads/auto-test"),
        branch_name: String::from("auto-test"),
        job_name: String::from("erp测试web"),
        project_name: String::from("ERP测试环境"),
    }),
    "econ-preview" => Some(ProjectInfo {
      ref_: String::from("refs/heads/auto-test"),
      branch_name: String::from("auto-test"),
      job_name: String::from("econ-front-test"),
      project_name: String::from("Econ测试环境"),
    }),
    "econ-h5" => Some(ProjectInfo {
      ref_: String::from("refs/heads/auto-test"),
      branch_name: String::from("auto-test"),
      job_name: String::from("econ-h5-test"),
      project_name: String::from("EconH5测试环境"),
    }),
    "BIReportPreview" => Some(ProjectInfo {
      ref_: String::from("refs/heads/auto-test"),
      branch_name: String::from("auto-test"),
      job_name: String::from("210-front-dbi"),
      project_name: String::from("DBI测试环境"),
    }),
    _ => None,
  };
  project_info
}
