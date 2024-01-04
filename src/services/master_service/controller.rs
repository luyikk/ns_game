use crate::GAME;
use anyhow::{Context, Result};
use netxclient::prelude::*;

use super::interface::*;

///服务器调度控制器
pub struct MasterController {
    server_id: u32,
    server: NetxClientArcDef,
}

impl MasterController {
    pub fn new(server_id: u32, server: NetxClientArcDef) -> Self {
        Self { server_id, server }
    }
}

///服务器调度控制器
#[build(MasterController)]
pub trait IMasterController {
    #[tag(connect)]
    async fn connected(&self) -> Result<()>;
    #[tag(disconnect)]
    async fn disconnect(&self) -> Result<()>;
    /// ping
    #[tag(1000)]
    async fn ping(&self, time: i64) -> Result<i64>;
}

#[build_impl]
impl IMasterController for MasterController {
    #[inline]
    async fn connected(&self) -> Result<()> {
        let server = impl_ref!(self.server=>IMaster);
        if server.register_slot_server(self.server_id).await? {
            log::info!("connect master server Ok");
        } else {
            panic!("register server id fail,check server id!!")
        }
        Ok(())
    }

    #[inline]
    async fn disconnect(&self) -> Result<()> {
        //和大厅断线处理
        //清除所有的token 和peer
        GAME.get()
            .context("not found GAME?")?
            .peers
            .clear_all()
            .await;
        Ok(())
    }

    /// ping
    #[inline]
    async fn ping(&self, time: i64) -> Result<i64> {
        log::debug!("master ping:{time}");
        Ok(time)
    }
}
