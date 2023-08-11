mod controller;
mod interface;

use crate::services::master_service::controller::MasterController;
use anyhow::Result;
pub use interface::*;
use netxclient::prelude::*;

/// Master 服务
pub struct MasterService {
    client: NetxClientArcDef,
}

impl MasterService {
    pub fn new(config: ServerOption) -> Self {
        let client = NetXClient::new(config, DefaultSessionStore::default());
        Self { client }
    }

    /// 安装控制器
    pub(crate) async fn init(&self, server_id: u32) -> Result<()> {
        self.client
            .init(MasterController::new(server_id, self.client.clone()))
            .await?;

        self.client.connect_network().await
    }

    /// 获取玩家基本信息
    #[inline]
    pub async fn get_account_info(&self, account_id: i32) -> Result<Option<AccountInfoRet>> {
        let server = impl_ref!(self.client=>IMaster);
        server.get_player_info(account_id).await
    }

    /// 老虎机请求旋转
    #[inline]
    pub async fn req_slot_spin(
        &self,
        account_id: i32,
        token: u64,
        req: ReqSlotSpin,
    ) -> Result<SlotSpinRet> {
        let server = impl_ref!(self.client=>IMaster);
        server.req_slot_spin(account_id, token, req).await
    }

    /// 老虎机请求退钱
    #[inline]
    pub async fn req_slot_refund(
        &self,
        account_id: i32,
        token: u64,
        req: ReqSlotRefund,
    ) -> Result<SlotRefundRet> {
        let server = impl_ref!(self.client=>IMaster);
        server.req_slot_refund(account_id, token, req).await
    }

    /// 获取彩金信息
    #[inline]
    pub async fn get_lottery_info(&self, game_id: u32) -> Result<Vec<LotteryInfo>> {
        let server = impl_ref!(self.client=>IMaster);
        server.get_game_lottery(game_id).await
    }

    #[inline]
    pub async fn move_money_cache(&self,account_id:i32, money:i64)->Result<MoneyContext>{
        let server = impl_ref!(self.client=>IMaster);
        server.move_money_cache(account_id,money).await
    }
}
