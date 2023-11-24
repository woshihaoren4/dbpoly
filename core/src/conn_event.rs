use infra::TcpConn;

#[async_trait::async_trait]
pub trait Handle{
    async fn auth(&self)->anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait Codec{
    async fn encode(&self,conn:&mut TcpConn);
}


#[async_trait::async_trait]
pub trait ConnectionManager{
    async fn client_initiate_conn(&self,conn:TcpConn)->anyhow::Result<()>;
    async fn client_close_conn(&self,);
}