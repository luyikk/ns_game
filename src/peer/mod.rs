mod call_game_services;
mod game_services;

use anyhow::{Context, Result};
use netxserver::prelude::*;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

use crate::controller::{IProxy, ___impl_IProxy_call};
use crate::services::IProxyService;
use crate::static_def::{BROADCAST_SERVICE, PROXY};
use crate::time::timestamp;

pub type Peer<T> = Arc<GamePeer<T>>;

/// 用于定位长链接的peer
pub struct GamePeer<T> {
    pub token: AtomicU64,
    pub proxy_id: AtomicUsize,
    pub account_id: AtomicI32,
    pub is_disconnect: AtomicBool,
    pub last_net_time: AtomicI64,
    pub inner: T,
}

impl<T> Drop for GamePeer<T> {
    fn drop(&mut self) {
        log::trace!("drop peer:{}", self);
    }
}

impl<T> Display for GamePeer<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "account_id:{},token:{},proxy_id:{}",
            self.account_id.load(Ordering::Acquire),
            self.token.load(Ordering::Acquire),
            self.proxy_id.load(Ordering::Acquire)
        )
    }
}

impl<T> GamePeer<T> {
    #[inline]
    pub(crate) fn update(&self) {
        self.last_net_time.store(timestamp(), Ordering::Release);
    }

    /// 获取token
    #[inline]
    pub(crate) fn get_token(&self) -> u64 {
        self.token.load(Ordering::Acquire)
    }

    /// 获取account id
    #[inline]
    pub(crate) fn get_account_id(&self) -> i32 {
        self.account_id.load(Ordering::Acquire)
    }

    /// 获取代理id
    #[inline]
    pub(crate) fn get_proxy_id(&self) -> usize {
        self.proxy_id.load(Ordering::Acquire)
    }

    /// 获取是否断线
    #[inline]
    pub(crate) fn is_disconnect(&self) -> bool {
        self.is_disconnect.load(Ordering::Acquire)
    }

    /// token 断线
    #[inline]
    pub(crate) async fn on_disconnect(&self) -> Result<()> {
        log::info!("peer:{self} on disconnect");
        Ok(())
    }

    /// 对比时间 返回差值 tick
    #[inline]
    pub(crate) fn comparison_time(&self, timestamp: i64) -> i64 {
        timestamp - self.last_net_time.load(Ordering::Acquire)
    }
}

impl<T> GamePeer<T> {
    /// 推送数据到当前token
    #[inline]
    pub async fn send_to_token(&self, data: &[u8]) -> Result<()> {
        let token = self.token.load(Ordering::Acquire);
        let proxy_id = self.proxy_id.load(Ordering::Acquire);
        let netx_token = PROXY
            .get(proxy_id)
            .await
            .with_context(|| format!("not found proxy id:{proxy_id}"))?;

        let proxy = impl_ref!(netx_token=>IProxy);
        proxy.send_to_token(token, data).await;
        Ok(())
    }

    /// 广播到 所有服务器的此玩家 所有连接
    /// 一般用于资产发生变化
    #[inline]
    async fn broadcast_to_account_id(&self, data: &[u8]) -> Result<()> {
        let account_id = self.account_id.load(Ordering::Acquire);
        BROADCAST_SERVICE
            .broadcast_to_account_id(account_id, data)
            .await
    }

    /// 广播到所有用户的所有连接
    /// 一般用于紧急公告 跑马灯等
    #[inline]
    pub async fn broadcast_to_all_users(&self, data: &[u8]) -> Result<()> {
        BROADCAST_SERVICE.broadcast_to_all_users(data).await
    }

    /// 广播到 此服务器 的所有连接
    /// 一般用于游戏内部通知
    #[inline]
    async fn broadcast_to_server_id(&self, data: &[u8]) -> Result<()> {
        BROADCAST_SERVICE.broadcast_to_server_id(data).await
    }

    /// 广播到 此服务器的,此玩家 所有连接
    /// 一般用于大厅客服服务
    #[inline]
    pub async fn broadcast_to_server_id_and_account_id(&self, data: &[u8]) -> Result<()> {
        let account_id = self.account_id.load(Ordering::Acquire);
        BROADCAST_SERVICE
            .broadcast_to_server_id_and_account_id(account_id, data)
            .await
    }
}
