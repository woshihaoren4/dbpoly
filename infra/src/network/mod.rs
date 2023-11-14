mod network_conn;

use std::future::Future;
pub use network_conn::*;

#[async_trait::async_trait]
pub trait TcpHandle{
    async fn handle(&self,tc:TcpConn)->anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T,F> TcpHandle for T
where T:Fn(TcpConn)-> F + Send + Sync,
    F:Future<Output=()> + Send
{
    async fn handle(&self, tc: TcpConn) -> anyhow::Result<()> {
        self(tc).await;Ok(())
    }
}

#[cfg(test)]
mod test{
    use std::net::SocketAddr;
    use crate::network::{ConnListener, TcpConn};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

    //sudo cargo test --color=always --lib network::test::test_network --no-fail-fast --manifest-path /Users/bytedance/project/work/github/dbpoly/infra/Cargo.toml -- unstable-options --show-output --nocapture
    #[tokio::test]
    async fn test_network(){
        let addr:SocketAddr = "127.0.0.1:6666".parse().unwrap();
        ConnListener::new(addr,|mut tc:TcpConn|async move{
            loop {
                let mut buf =String::new();
                let n = tc.stream.read_line(&mut buf).await.unwrap();
                if n <= 0 {
                    break
                }
                wd_log::log_prefix("client--->").info(&buf);
                tc.stream.write_all(buf.as_bytes()).await.unwrap();
                let _ = tc.stream.flush().await;
            }
        }).await.unwrap().start_listen().await;
    }
}