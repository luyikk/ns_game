use anyhow::Result;
use std::fmt::Display;

/// PEER 接口
#[async_trait::async_trait]
pub trait IPeer: Display + Send + Sync {
    /// 新建peer
    fn create(token: u64, account_id: i32) -> Self;
    /// 更新peer
    fn update(&self);
    /// 获取account id
    fn get_account_id(&self) -> i32;
    /// 获取token
    fn get_token(&self) -> u64;
    /// 获取代理id
    fn get_proxy_id(&self) -> usize;
    /// 设置代理id
    fn set_proxy_id(&self, proxy_id: usize);
    /// 设置断线状态
    fn set_disconnect(&self, disconnect: bool);
    /// 断线回调
    async fn on_disconnect(&self) -> Result<()>;
    /// 获取是否断线
    fn is_disconnect(&self) -> bool;
    /// 对比时间 返回差值 tick
    fn comparison_time(&self, timestamp: i64) -> i64;
}
