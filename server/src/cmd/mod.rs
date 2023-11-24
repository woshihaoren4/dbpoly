mod run;
mod test;

pub async fn start() ->anyhow::Result<()>{
    wd_run::AppEntity::new("dbpoly")
        .add_builder(run::Run::default())
        .run().await;

    Ok(())
}