use crate::controller::ProxyController;
use crate::services::{ILinkPeerManager, IProxyService};
use crate::static_def::{PEERS, PROXY};
use crate::{ret_err, ret_error};
use anyhow::{ensure, Result};
use netxserver::prelude::tcpserver::IPeer;
use netxserver::prelude::*;
use serde_json::Value;
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

    /// 匿名调用 不需要登录
    #[tag(2002)]
    async fn anonymous_call_on(&self, data: Vec<u8>) -> Result<Vec<u8>>;
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
        PEERS.disconnect_for_proxy(proxy_id).await;
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
        PEERS.create_peer(account_id).await
    }

    /// 长连接携带token链接
    #[inline]
    async fn connect_token(&self, account_id: i32, token: u64) -> Result<()> {
        let proxy_id = self.proxy_id.load(Ordering::Acquire);
        PEERS.connect_token(proxy_id, account_id, token).await
    }

    /// peer 断线
    #[inline]
    async fn disconnect_token(&self, token: u64) {
        PEERS.disconnect_token(token).await;
    }

    /// 功能调用
    #[inline]
    async fn func(&self, account_id: i32, token: u64, data: Vec<u8>) -> Result<Vec<u8>> {
        let jv: Value = serde_json::from_str(std::str::from_utf8(&data)?)?;
        let serial = jv["serial"].as_i64();
        if let Some(peer) = PEERS.get_peer(token) {
            if peer.get_account_id() != account_id {
                ret_error!(serial, -1000002, "token account error")
            }

            peer.update();
            match self.func(peer, serial, jv).await {
                Ok(result) => Ok(result),
                Err(err) => {
                    ret_error!(serial,-1000001;"error:{}", err);
                }
            }
        } else {
            ret_error!(serial, -1000000, "token does not exist")
        }
    }

    /// 调用加载
    #[inline]
    async fn anonymous_call_on(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        let proxy_id = self.proxy_id.load(Ordering::Acquire);
        let jv: Value = serde_json::from_str(std::str::from_utf8(&data)?)?;
        match self.anonymous_call_on(proxy_id, jv).await {
            Ok(result) => Ok(result),
            Err(err) => {
                ret_err!(-1000001;"error:{}", err)
            }
        }
    }
}
