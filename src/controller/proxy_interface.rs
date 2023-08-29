use anyhow::Result;
use netxserver::prelude::*;

///服调度控制器
#[build]
pub trait IProxy {
    /// 获取代理id
    #[tag(2001)]
    async fn get_server_id(&self) -> Result<usize>;
    /// 广播到所有用户连接
    #[tag(2010)]
    async fn broadcast_to_all_users(&self, data: &[u8]);
    /// 广播到所有此玩家连接
    #[tag(2011)]
    async fn broadcast_to_account_id(&self, account_id: i32, data: &[u8]);
    /// 广播到所有服务器连接
    #[tag(2012)]
    async fn broadcast_to_server_id(&self, server_id: i32, data: &[u8]);
    /// 广播到所有此服务器的玩家连接
    #[tag(2013)]
    async fn broadcast_to_server_id_and_account_id(
        &self,
        server_id: i32,
        account_id: i32,
        data: &[u8],
    );
    /// 发送到token
    #[tag(2020)]
    async fn send_to_token(&self, token: u64, data: &[u8]);

    /// 通知一批账号还活着 不要结存
    #[tag(107)]
    async fn alive_account(&self, account_ids: &[i32]);
}
