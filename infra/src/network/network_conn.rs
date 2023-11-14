use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpStream};
use wd_tools::PFOk;
use crate::network::TcpHandle;

pub struct TcpConn{
    pub addr : SocketAddr,
    pub stream: BufStream<TcpStream>
}
impl TcpConn{
    pub fn new(stream:TcpStream,addr:SocketAddr)->Self{
        let stream = BufStream::new(stream);
        TcpConn{ addr, stream }
    }
}
impl Drop for TcpConn{
    fn drop(&mut self) {
        // let _ = self.stream.get_mut().flush();
        let _ = self.stream.get_mut().shutdown();
    }
}

pub struct ConnListener{
    handle: Arc<dyn TcpHandle+Send+Sync+'static>,
    listener:TcpListener,
}

impl ConnListener{
    pub async fn new<A:Into<SocketAddr> ,T:TcpHandle+Send+Sync+'static>(addr:A,handle:T)->anyhow::Result<Self>{
        let handle = Arc::new(handle);
        let listener = TcpListener::bind(addr.into()).await?;
        Self{handle,listener}.ok()
    }
    pub async fn start_listen(&self,){
        loop {
            let (tcp_stream,addr) = match self.listener.accept().await {
                Ok(o)=>o,
                Err(e)=>{
                    wd_log::log_prefix("ConnListener.accept error:").error(e);
                    continue
                }
            };
            let conn = TcpConn::new(tcp_stream, addr);
            let handle = self.handle.clone();
            tokio::spawn(async move {
                let handle = handle.handle(conn);
                if let Err(e) = handle.await{
                    wd_log::log_prefix("ConnListener.handle error:").error(e);
                }
            });
        }
    }
}