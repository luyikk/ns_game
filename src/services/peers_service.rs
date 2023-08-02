use anyhow::{bail, ensure, Result};
use aqueue::Actor;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

use crate::peer::{GamePeer, Peer};
use crate::static_def::CONFIG;
use crate::time::{timestamp, SECOND, TICK};

/// PEER管理器
pub struct LinkPeerManager<T> {
    peers: HashMap<u64, Peer<T>>,
    index: u64,
}

impl<T: Default + Send + Sync + 'static> LinkPeerManager<T> {
    pub fn new() -> Self {
        Self {
            peers: Default::default(),
            index: 0,
        }
    }

    ///新建PEER
    #[inline]
    fn create_peer(&mut self, account_id: i32) -> Result<u64> {
        self.index += 1;
        let token = self.index;

        ensure!(!self.peers.contains_key(&token), "token exits");

        self.peers.insert(
            token,
            Arc::new(GamePeer {
                token: AtomicU64::new(token),
                proxy_id: AtomicUsize::new(0),
                account_id: AtomicI32::new(account_id),
                is_disconnect: AtomicBool::new(true),
                last_net_time: AtomicI64::new(timestamp()),
                inner: Default::default(),
            }),
        );

        log::info!("create peer token:{}", token);

        Ok(token)
    }

    /// 获取peer
    #[inline]
    fn get_peer(&self, token: u64) -> Option<Peer<T>> {
        self.peers.get(&token).cloned()
    }

    /// 长连接携带token链接
    /// 返回false表示token没找到 或者用户名对不上
    #[inline]
    fn peer_connect(&self, proxy_id: usize, account_id: i32, token: u64) -> Result<()> {
        if let Some(peer) = self.peers.get(&token) {
            if peer.account_id.load(Ordering::Acquire) == account_id {
                peer.proxy_id.store(proxy_id, Ordering::Release);
                peer.is_disconnect.store(false, Ordering::Release);
                log::info!("peer token:{} connect", token);
                Ok(())
            } else {
                bail!("account id:{account_id} not is token")
            }
        } else {
            bail!("account id:{account_id} not found")
        }
    }

    /// 根据账号id 获取所有的peer
    /// 一个账号可对应多个peer
    #[inline]
    fn get_peer_by_account_id(&self, account_id: i32) -> Vec<Peer<T>> {
        self.peers
            .clone()
            .values()
            .filter_map(|peer| {
                if peer.get_account_id() == account_id {
                    Some(peer.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// 断线
    #[inline]
    fn disconnect(&mut self, token: u64) {
        if let Some(peer) = self.peers.get(&token) {
            peer.is_disconnect.store(true, Ordering::Release);
            log::debug!("peer token:{} disconnect", token);
            let peer = peer.clone();
            tokio::spawn(async move {
                if let Err(err) = peer.on_disconnect().await {
                    log::error!("peer:{peer} On Disconnect err:{err}");
                }
            });
        } else {
            log::warn!("disconnect peer not found:{}", token)
        }
    }

    /// 代理断线设置所有peer状态
    #[inline]
    fn disconnect_for_proxy(&mut self, proxy_id: usize) {
        let tokens = self
            .peers
            .values()
            .filter_map(|p| {
                if p.get_proxy_id() == proxy_id {
                    Some(p.get_token())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for token in tokens {
            self.disconnect(token);
        }
    }

    /// 清理需要清理的peer
    #[inline]
    fn cleans(&mut self) -> Result<()> {
        let now = timestamp();
        let timeout = CONFIG.base.peer_clean_timeout_sec * SECOND * TICK;
        self.peers
            .retain(|_, v| !v.is_disconnect() || v.comparison_time(now) < timeout);
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait ILinkPeerManager<T> {
    /// 新建PEER
    async fn create_peer(&self, account_id: i32) -> Result<u64>;
    /// 长连接携带token链接
    /// 返回false表示token没找到
    async fn connect_token(&self, proxy_id: usize, account_id: i32, token: u64) -> Result<()>;
    /// 获取peer
    fn get_peer(&self, token: u64) -> Option<Peer<T>>;
    /// 根据账号id 获取所有的peer
    /// 一个账号可对应多个peer
    fn get_peer_by_account_id(&self, account_id: i32) -> Vec<Peer<T>>;
    /// 断线
    async fn disconnect_token(&self, token: u64);
    /// 清理需要清理的peer
    async fn cleans(&self) -> Result<()>;
    /// 从网关断线所有peer
    async fn disconnect_for_proxy(&self, proxy_id: usize);
}

#[async_trait::async_trait]
impl<T: Default + Send + Sync + 'static> ILinkPeerManager<T> for Actor<LinkPeerManager<T>> {
    #[inline]
    async fn create_peer(&self, account_id: i32) -> Result<u64> {
        self.inner_call(|inner| async move { inner.get_mut().create_peer(account_id) })
            .await
    }

    #[inline]
    async fn connect_token(&self, proxy_id: usize, account_id: i32, token: u64) -> Result<()> {
        self.inner_call(
            |inner| async move { inner.get_mut().peer_connect(proxy_id, account_id, token) },
        )
        .await
    }

    #[inline]
    fn get_peer(&self, token: u64) -> Option<Peer<T>> {
        unsafe { self.deref_inner().get_peer(token) }
    }

    #[inline]
    fn get_peer_by_account_id(&self, account_id: i32) -> Vec<Peer<T>> {
        unsafe { self.deref_inner().get_peer_by_account_id(account_id) }
    }

    #[inline]
    async fn disconnect_token(&self, token: u64) {
        self.inner_call(|inner| async move { inner.get_mut().disconnect(token) })
            .await
    }

    #[inline]
    async fn cleans(&self) -> Result<()> {
        self.inner_call(|inner| async move { inner.get_mut().cleans() })
            .await
    }

    #[inline]
    async fn disconnect_for_proxy(&self, proxy_id: usize) {
        self.inner_call(|inner| async move { inner.get_mut().disconnect_for_proxy(proxy_id) })
            .await
    }
}
