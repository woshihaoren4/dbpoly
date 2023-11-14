mod config;

use wd_tools::PFOk;
pub use config::*;

pub fn load_config<Path:AsRef<str>>(path:Path)->anyhow::Result<Config>{
    if path.as_ref().is_empty() {
        Config::default().ok()
    }else{
        wd_run::load_config(path.as_ref())
    }
}

#[cfg(test)]
mod test{
    use crate::config::load_config;

    #[test]
    fn load_config_test(){
        let cfg = load_config("").unwrap();
        println!("{:?}",cfg)
    }
}