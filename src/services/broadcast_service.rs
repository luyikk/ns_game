use crate::controller::{IProxy, ___impl_IProxy_call};
use crate::services::IProxyService;
use crate::static_def::PROXY;
use anyhow::Result;
use netxserver::prelude::*;

/// 广播服务
pub struct BroadcastService;

impl BroadcastService {
    pub fn new() -> Self {
        BroadcastService
    }

    /// 广播到 所有服务器的此玩家 所有连接
    /// 一般用于资产发生变化
    #[inline]
    pub async fn broadcast_to_account_id(&self, account_id: i32, data: &[u8]) -> Result<()> {
        for netx_token in PROXY.get_all_token().await? {
            let proxy = impl_ref!(netx_token=>IProxy);
            proxy.broadcast_to_account_id(account_id, data).await;
        }
        Ok(())
    }

    /// 广播到所有用户的所有连接
    /// 一般用于紧急公告 跑马灯等
    #[inline]
    pub async fn broadcast_to_all_users(&self, data: &[u8]) -> Result<()> {
        for netx_token in PROXY.get_all_token().await? {
            let proxy = impl_ref!(netx_token=>IProxy);
            proxy.broadcast_to_all_users(data).await;
        }
        Ok(())
    }

    /// 广播到 此服务器 的所有连接
    /// 一般用于游戏内部通知
    #[inline]
    pub async fn broadcast_to_server_id(&self, data: &[u8]) -> Result<()> {
        for netx_token in PROXY.get_all_token().await? {
            let proxy = impl_ref!(netx_token=>IProxy);
            proxy.broadcast_to_server_id(0, data).await;
        }
        Ok(())
    }

    /// 广播到 此服务器的,此玩家 所有连接
    /// 一般用于大厅客服服务
    #[inline]
    pub async fn broadcast_to_server_id_and_account_id(
        &self,
        account_id: i32,
        data: &[u8],
    ) -> Result<()> {
        for netx_token in PROXY.get_all_token().await? {
            let proxy = impl_ref!(netx_token=>IProxy);
            proxy
                .broadcast_to_server_id_and_account_id(0, account_id, data)
                .await;
        }
        Ok(())
    }
}
