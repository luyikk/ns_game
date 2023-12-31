use crate::packers::GetTokenResult;
use anyhow::{bail, ensure, Result};
use aqueue::Actor;
use std::collections::HashMap;
use std::sync::Arc;

use crate::peer::IPeer;
use crate::static_def::BASE_CONFIG;
use crate::time::{timestamp, timestamp_nanos, SECOND, TICK};

/// PEER管理器
pub struct LinkPeerManager<T> {
    peers: HashMap<u64, Arc<T>>,
}

impl<T> Default for LinkPeerManager<T> {
    fn default() -> Self {
        Self {
            peers: Default::default(),
        }
    }
}

impl<T: IPeer + 'static> LinkPeerManager<T> {
    ///新建PEER
    #[inline]
    fn create_peer(&mut self, account_id: i32) -> Result<u64> {
        let token = timestamp_nanos() % 100000000000000;

        ensure!(!self.peers.contains_key(&token), "token exits");

        self.peers
            .insert(token, Arc::new(IPeer::create(token, account_id)));

        log::info!("create peer token:{}", token);

        Ok(token)
    }

    /// 获取peer
    #[inline]
    fn get_peer(&self, token: u64) -> Option<Arc<T>> {
        self.peers.get(&token).cloned()
    }

    /// 长连接携带token链接
    /// 返回false表示token没找到 或者用户名对不上
    #[inline]
    fn peer_connect(&self, proxy_id: usize, account_id: i32, token: u64) -> Result<()> {
        if let Some(peer) = self.peers.get(&token) {
            if peer.get_account_id() == account_id {
                peer.set_proxy_id(proxy_id);
                peer.set_disconnect(false);
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
    fn get_peer_by_account_id(&self, account_id: i32) -> Vec<Arc<T>> {
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

    /// 获取此账号的所有token状态
    #[inline]
    fn get_token_state_by_account_id(&self, account_id: i32) -> Vec<GetTokenResult> {
        let now = timestamp();
        self.peers
            .values()
            .filter_map(|peer| {
                if peer.get_account_id() == account_id {
                    Some(GetTokenResult {
                        token: peer.get_token(),
                        last_elapsed_time: peer.comparison_time(now),
                        timeout: BASE_CONFIG.base.peer_clean_timeout_sec,
                        is_wss_connect: !peer.is_disconnect(),
                    })
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
            peer.set_disconnect(true);
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

    /// 删除指定玩家的peer
    #[inline]
    async fn clean_by_account_id(&mut self, account_id: i32) {
        let remove_list = self
            .peers
            .iter()
            .filter_map(|(key, player)| {
                if player.get_account_id() == account_id {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for remove_key in remove_list {
            if let Some(peer) = self.peers.remove(&remove_key) {
                if let Err(err) = peer.on_clean().await {
                    log::error!("clean peer:{peer} token:{remove_key} error:{err} 2")
                }
            }
        }
    }

    /// 清理需要清理的peer
    #[inline]
    async fn cleans(&mut self) -> Result<()> {
        let now = timestamp();
        let timeout = BASE_CONFIG.base.peer_clean_timeout_sec * SECOND * TICK;
        // self.peers
        //     .drain(|_, v| !v.is_disconnect() || v.comparison_time(now) < timeout);

        let cleans = self
            .peers
            .iter()
            .filter_map(|(k, v)| {
                if v.is_disconnect() && v.comparison_time(now) >= timeout {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let clean_peers = cleans
            .into_iter()
            .filter_map(|k| self.peers.remove(&k))
            .collect::<Vec<_>>();

        for peer in clean_peers {
            if let Err(err) = peer.on_clean().await {
                log::error!("clean peer:{} error:{err}", peer)
            }
        }

        Ok(())
    }

    /// 清理所有peer
    #[inline]
    async fn clear_all(&mut self) {
        let keys = self.peers.keys().cloned().collect::<Vec<_>>();
        let clean_peers = keys
            .into_iter()
            .filter_map(|k| self.peers.remove(&k))
            .collect::<Vec<_>>();
        for peer in clean_peers {
            if let Err(err) = peer.on_clean().await {
                log::error!("clear all peer:{} error:{err}", peer)
            }
        }
    }
}

#[async_trait::async_trait]
pub trait ILinkPeerManager: Send + Sync {
    /// 新建PEER
    async fn create_peer(&self, account_id: i32) -> Result<u64>;
    /// 长连接携带token链接
    /// 返回false表示token没找到
    async fn connect_token(&self, proxy_id: usize, account_id: i32, token: u64) -> Result<()>;
    /// 获取此账号的所有token状态
    async fn get_token_state_by_account_id(&self, account_id: i32) -> Vec<GetTokenResult>;
    /// 断线
    async fn disconnect_token(&self, token: u64);
    /// 清理需要清理的peer
    async fn cleans(&self) -> Result<()>;
    /// 从网关断线所有peer
    async fn disconnect_for_proxy(&self, proxy_id: usize);
    /// 清理指定account id的账号
    async fn clean_by_account_id(&self, account_id: i32);
    /// 清理所有token
    async fn clear_all(&self);
}

pub trait ILinkPeerManagerPeer<T>: ILinkPeerManager {
    /// 获取peer
    fn get_peer(&self, token: u64) -> Option<Arc<T>>;
    /// 根据账号id 获取所有的peer
    /// 一个账号可对应多个peer
    fn get_peer_by_account_id(&self, account_id: i32) -> Vec<Arc<T>>;
    /// 获取所有peer
    fn get_all_peer(&self) -> Vec<Arc<T>>;
}

#[async_trait::async_trait]
impl<T: IPeer + 'static> ILinkPeerManager for Actor<LinkPeerManager<T>> {
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
    async fn get_token_state_by_account_id(&self, account_id: i32) -> Vec<GetTokenResult> {
        self.inner_call(
            |inner| async move { inner.get_mut().get_token_state_by_account_id(account_id) },
        )
        .await
    }

    #[inline]
    async fn disconnect_token(&self, token: u64) {
        self.inner_call(|inner| async move { inner.get_mut().disconnect(token) })
            .await
    }

    #[inline]
    async fn cleans(&self) -> Result<()> {
        self.inner_call(|inner| async move { inner.get_mut().cleans().await })
            .await
    }

    #[inline]
    async fn disconnect_for_proxy(&self, proxy_id: usize) {
        self.inner_call(|inner| async move { inner.get_mut().disconnect_for_proxy(proxy_id) })
            .await
    }

    #[inline]
    async fn clean_by_account_id(&self, account_id: i32) {
        self.inner_call(
            |inner| async move { inner.get_mut().clean_by_account_id(account_id).await },
        )
        .await
    }
    #[inline]
    async fn clear_all(&self) {
        self.inner_call(|inner| async move { inner.get_mut().clear_all().await })
            .await
    }
}

impl<T: IPeer + 'static> ILinkPeerManagerPeer<T> for Actor<LinkPeerManager<T>> {
    #[inline]
    fn get_peer(&self, token: u64) -> Option<Arc<T>> {
        unsafe { self.deref_inner().get_peer(token) }
    }

    #[inline]
    fn get_peer_by_account_id(&self, account_id: i32) -> Vec<Arc<T>> {
        unsafe { self.deref_inner().get_peer_by_account_id(account_id) }
    }

    #[inline]
    fn get_all_peer(&self) -> Vec<Arc<T>> {
        unsafe { self.deref_inner().peers.values().cloned().collect() }
    }
}
