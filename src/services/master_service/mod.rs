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
    pub async fn init(&self, server_id: u32) -> Result<()> {
        self.client
            .init(MasterController::new(server_id, self.client.clone()))
            .await?;

        self.client.connect_network().await
    }

    /// 获取玩家基本信息
    #[inline]
    pub async fn get_account_info(&self, account_id: i32) -> Result<Option<AccountInfo>> {
        let server = impl_ref!(self.client=>IMaster);
        server.get_player_info(account_id).await
    }
}
