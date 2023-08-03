use crate::controller::{ImplCreateProxyController, ProxyController};
use crate::services::{ILinkPeerManager, IProxyService};
use crate::static_def::{BASE_CONFIG, MASTER_SERVICE, PROXY};
use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use netxserver::prelude::NetXServer;
use std::sync::Arc;

pub mod config;
pub mod controller;
pub mod packers;
pub mod peer;
pub mod services;
pub mod static_def;
pub mod time;
pub mod timer;

/// 静态安装配置
pub static GAME: tokio::sync::OnceCell<Game> = tokio::sync::OnceCell::const_new();

pub type Func =
    for<'a> fn(&'a ProxyController, i32, u64, Vec<u8>) -> BoxFuture<'a, Result<Vec<u8>>>;

/// 基本安装
pub struct Game {
    pub peers: Arc<dyn ILinkPeerManager>,
    pub func: Func,
}

impl Game {
    pub async fn start(peers: Arc<dyn ILinkPeerManager>, func: Func) -> Result<()> {
        GAME.set(Self { peers, func })
            .map_err(|_| anyhow!("not install game"))?;

        if let Err(err) = MASTER_SERVICE.init(BASE_CONFIG.base.server_id).await {
            log::error!("connect master server error:{}", err);
        }

        //新建服务器,需要设置和接口实现
        let server =
            NetXServer::new(BASE_CONFIG.proxy_listen.clone(), ImplCreateProxyController).await;
        PROXY
            .set_manager(server.get_token_manager().upgrade().unwrap())
            .await;
        // 开始服务器,堵塞模式
        log::info!("starting ns game service:{}",BASE_CONFIG.base.server_id);
        server.start_block().await?;
        Ok(())
    }
}
