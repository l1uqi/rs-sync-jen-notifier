# rs-sync-jen-notifier

Rust Web练手项目 基于[Axum](https://github.com/tokio-rs/axum)

通过GitLab WebHook分支变动推送, 触发Jenkids构建并通知钉钉群。

![](1703585175799.jpg)

### 功能


```
/reciver // 配置gitlab webhook
/jenkins/info // 获取jenkins 信息
/jenkins/lanunchBuild // 开启jenkins构建
/dingtalk/send // 发送钉钉消息

```


### 配置
.env 目录
```
DINGTALK.URL = 你的钉钉消息URL(url + token完整路径)

DINGTALK.TITLE = 钉钉标题

JENKINS.URL = 你的Jenkins URL

JENKINS.USERNAME = 你的Jenkins用户名

JENKINS.PASSWORD =你的Jenkins密码

```


### 后续
适配微信群机器人等.
