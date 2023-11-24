use std::sync::Arc;
use std::time::Duration;
use wd_run::{Context, Task,TaskBuild, TaskInfo};
use wd_tools::PFArc;

#[derive(Default)]
pub struct Run{
    config_path: String
}

#[wd_run::async_trait]
impl TaskBuild for Run{
    fn args(&self) -> TaskInfo {
        TaskInfo::new("run","run server")
            .register_arg("c","./config.toml","config file path")
    }

    async fn build(&mut self, mut ctx: Context) -> Arc<dyn Task> {
        let config_path:String = ctx.load("c").unwrap();
        Run{config_path}.arc()
    }
}
#[wd_run::async_trait]
impl Task for Run{
    async fn run(&self) -> anyhow::Result<()> {
        wd_log::log_field("config:",self.config_path.as_str()).info("config path");
        for i in 0..10{
            wd_log::log_prefix("server running --->").info(i);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }
    async fn exit(&self) -> anyhow::Result<()> {
        wd_log::log_prefix("server exit").debug(" success");
        Ok(())
    }
}