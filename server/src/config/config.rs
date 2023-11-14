use serde::{Deserialize, Serialize};
use std::path::Path;

macro_rules! field_generate {
    ($cfg:tt; $($name:tt,$ty:ty,$default:expr,$default_desc:tt);*) => {
        #[derive(Debug,Serialize,Deserialize,Clone)]
        pub struct $cfg{
            $(
            #[serde(default=$default_desc)]
            pub $name : $ty,
            )*

        }
        impl $cfg{
            $(
            fn $name()->$ty{
                $default
            }
            )*
        }
        impl Default for $cfg{
            fn default() -> Self {
                Self{
                $(
                    $name : $default,
                )*
                }
            }
        }
    };
}

field_generate!(Server;
    name,String,String::from("rust-grpc-proxy"),"Server::name";
    addr,String,String::from("0.0.0.0:6789"),"Server::addr";
    log_level,String,String::from("debug"),"Server::log_level"
);

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "Server::default")]
    pub server: Server,
}