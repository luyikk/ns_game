mod account;
mod packer;

use anyhow::Result;
use netxclient::prelude::*;

pub use account::*;
pub use packer::*;

#[build]
pub trait IMaster {
    /// 注册老虎机服务
    #[tag(11)]
    async fn register_slot_server(&self, server_id: u32) -> Result<bool>;

    /// 获取账号信息
    #[tag(12)]
    async fn get_player_info(&self, account_id: i32) -> Result<Option<AccountInfo>>;
    /// 老虎机请求旋转
    #[tag(102)]
    async fn req_slot_spin(
        &self,
        account_id: i32,
        token: u64,
        req: ReqSlotSpin,
    ) -> Result<SlotSpinRet>;
    /// 老虎机请求退钱
    #[tag(103)]
    async fn req_slot_refund(
        &self,
        account_id: i32,
        token: u64,
        req: ReqSlotRefund,
    ) -> Result<SlotRefundRet>;
}
