mod controller;
mod interface;

use crate::services::master_service::controller::MasterController;
use crate::GAME;
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
        let result = server.get_player_info(account_id).await?;
        if result.is_none() {
            if let Some(game) = GAME.get() {
                game.peers.clean_by_account_id(account_id).await;
            }
        }
        Ok(result)
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
        let result = server.req_slot_spin(account_id, token, req).await;
        if result.is_err() {
            if let Some(game) = GAME.get() {
                game.peers.clean_by_account_id(account_id).await;
            }
        }
        result
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
        let result = server.req_slot_refund(account_id, token, req).await;
        if result.is_err() {
            if let Some(game) = GAME.get() {
                game.peers.clean_by_account_id(account_id).await;
            }
        }
        result
    }

    /// 获取彩金信息
    #[inline]
    pub async fn get_lottery_info(&self, game_id: u32) -> Result<Vec<LotteryInfo>> {
        let server = impl_ref!(self.client=>IMaster);
        server.get_game_lottery(game_id).await
    }

    /// 从money cache 移动钱到 money
    #[inline]
    pub async fn move_money_cache(&self, account_id: i32, money: i64) -> Result<MoneyContext> {
        let server = impl_ref!(self.client=>IMaster);
        let result = server.move_money_cache(account_id, money).await;
        if result.is_err() {
            if let Some(game) = GAME.get() {
                game.peers.clean_by_account_id(account_id).await;
            }
        }
        result
    }

    /// 机器人彩金旋转
    #[inline]
    pub async fn robot_lottery_spin(
        &self,
        loop_count: u32,
        lottery_id: i32,
        coin: f64,
    ) -> Result<i64> {
        let server = impl_ref!(self.client=>IMaster);
        server
            .robot_lottery_spin(loop_count, lottery_id, coin)
            .await
    }

    /// peer账号信息保活 不结存
    #[inline]
    pub async fn alive_account(&self, account_ids: &[i32]) {
        let server = impl_ref!(self.client=>IMaster);
        server.alive_account(account_ids).await
    }
}
