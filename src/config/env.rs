use clap::Parser;

#[derive(Debug, Parser)]
pub struct DingTalkConfig {
    #[clap(required = true, env)]
    pub dding_url: String,
    #[clap(required = true, env)]  
    pub dding_title: String,
}
