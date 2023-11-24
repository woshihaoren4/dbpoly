mod cmd;
mod config;

#[tokio::main]
async fn main(){
    if let Err(e) = cmd::start().await {
        wd_log::log_prefix("start server error:").error(e)
    }
}
