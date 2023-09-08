use crate::controller::ProxyController;
use crate::packers::GetTokenResult;
use crate::services::IProxyService;
use crate::static_def::PROXY;
use crate::GAME;
use anyhow::{ensure, Context, Result};
use netxserver::prelude::tcpserver::IPeer;
use netxserver::prelude::*;
use std::sync::atomic::Ordering;

#[build(ProxyController)]
pub trait IProxyController {
    #[tag(connect)]
    async fn connect(&self) -> Result<()>;
    #[tag(disconnect)]
    async fn disconnect(&self) -> Result<()>;
    /// 注册代理
    #[tag(10)]
    async fn register_proxy(&self, proxy_id: usize) -> Result<()>;
    /// 新建token
    #[tag(100)]
    async fn create_token(&self, account_id: i32) -> Result<u64>;
    /// 长连接携带token链接
    #[tag(101)]
    async fn connect_token(&self, account_id: i32, token: u64) -> Result<()>;
    /// peer 断线
    #[tag(102)]
    async fn disconnect_token(&self, token: u64);
    /// 数据request
    #[tag(2001)]
    async fn func(&self, account_id: i32, token: u64, data: Vec<u8>) -> Result<Vec<u8>>;
    /// 获取此用户所有token状态
    #[tag(2002)]
    async fn get_token_status(&self, account_id: i32) -> Result<Vec<GetTokenResult>>;
}

#[build_impl]
impl IProxyController for ProxyController {
    /// 代理链接
    #[inline]
    async fn connect(&self) -> Result<()> {
        if let Some(weak) = self.token.get_peer().await {
            if let Some(peer) = weak.upgrade() {
                log::info!(
                    "proxy:{} session:{} connect",
                    peer.addr(),
                    self.token.get_session_id()
                );
            }
        }
        Ok(())
    }

    /// 代理断线
    #[inline]
    async fn disconnect(&self) -> Result<()> {
        let proxy_id = self.proxy_id.load(Ordering::Acquire);

        if let Some(weak) = self.token.get_peer().await {
            if let Some(peer) = weak.upgrade() {
                log::info!(
                    "proxy:{proxy_id} addr:{} session {} disconnect",
                    peer.addr(),
                    self.token.get_session_id()
                )
            }
        }
        PROXY.remove(proxy_id).await;
        GAME.get()
            .context("not install game")?
            .peers
            .disconnect_for_proxy(proxy_id)
            .await;
        Ok(())
    }

    /// 注册代理
    #[inline]
    async fn register_proxy(&self, proxy_id: usize) -> Result<()> {
        ensure!(proxy_id != 0, "proxy not 0");
        log::info!("register proxy id:{proxy_id}");
        self.proxy_id.store(proxy_id, Ordering::Release);
        PROXY.add(proxy_id, self.token.get_session_id()).await;
        Ok(())
    }

    /// 新建peer token
    #[inline]
    async fn create_token(&self, account_id: i32) -> Result<u64> {
        GAME.get()
            .context("not install game")?
            .peers
            .create_peer(account_id)
            .await
    }

    /// 长连接携带token链接
    #[inline]
    async fn connect_token(&self, account_id: i32, token: u64) -> Result<()> {
        let proxy_id = self.proxy_id.load(Ordering::Acquire);
        GAME.get()
            .context("not install game")?
            .peers
            .connect_token(proxy_id, account_id, token)
            .await
    }

    /// peer 断线
    #[inline]
    async fn disconnect_token(&self, token: u64) {
        GAME.get()
            .context("not install game")
            .unwrap()
            .peers
            .disconnect_token(token)
            .await;
    }

    /// 功能调用
    #[inline]
    async fn func(&self, account_id: i32, token: u64, data: Vec<u8>) -> Result<Vec<u8>> {
        (GAME.get().context("not found game install")?.func)(self, account_id, token, data).await
    }

    /// 获取此用户所有token状态
    #[inline]
    async fn get_token_status(&self, account_id: i32) -> Result<Vec<GetTokenResult>> {
        let mut result = GAME
            .get()
            .context("not install game")
            .unwrap()
            .peers
            .get_token_state_by_account_id(account_id);

        result.sort_by_key(|x| x.last_elapsed_time);

        Ok(result)
    }
}
