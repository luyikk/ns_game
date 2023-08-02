use crate::controller::ProxyController;
use anyhow::{bail, Result};
use aqueue::RwModel;
use netxserver::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// 代理服务器查询器
#[derive(Default)]
pub struct ProxyService {
    proxy_map: HashMap<usize, i64>,
    manager: Option<Arc<dyn ITokenManager<ProxyController>>>,
}

impl ProxyService {
    /// 设置 netx token管理器
    #[inline]
    fn set_manager(&mut self, manager: Arc<dyn ITokenManager<ProxyController>>) {
        self.manager = Some(manager)
    }

    #[inline]
    fn add(&mut self, proxy_id: usize, session_id: i64) {
        self.proxy_map.insert(proxy_id, session_id);
    }

    #[inline]
    fn remove(&mut self, proxy_id: usize) {
        self.proxy_map.remove(&proxy_id);
    }

    #[inline]
    async fn get(&self, proxy_id: usize) -> Option<NetxToken<ProxyController>> {
        if let Some(session_id) = self.proxy_map.get(&proxy_id) {
            if let Some(ref manager) = self.manager {
                manager.get_token(*session_id).await
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    async fn get_all(&self) -> Result<Vec<NetxToken<ProxyController>>> {
        if let Some(ref manager) = self.manager {
            manager.get_all_tokens().await
        } else {
            bail!("manager is none")
        }
    }
}

#[async_trait::async_trait]
pub trait IProxyService {
    /// 设置 netx token管理器
    async fn set_manager(&self, manager: Arc<dyn ITokenManager<ProxyController>>);
    /// 添加代理服务器
    async fn add(&self, proxy_id: usize, session_id: i64);
    /// 删除代理服务器
    async fn remove(&self, proxy_id: usize);
    /// 查询代理服务器session id
    async fn get(&self, proxy_id: usize) -> Option<NetxToken<ProxyController>>;
    /// 获取所有代理
    async fn get_all_token(&self) -> Result<Vec<NetxToken<ProxyController>>>;
}
#[async_trait::async_trait]
impl IProxyService for RwModel<ProxyService> {
    #[inline]
    async fn set_manager(&self, manager: Arc<dyn ITokenManager<ProxyController>>) {
        self.call_mut(|mut inner| async move { inner.set_manager(manager) })
            .await
    }

    #[inline]
    async fn add(&self, proxy_id: usize, session_id: i64) {
        self.call_mut(|mut inner| async move { inner.add(proxy_id, session_id) })
            .await
    }
    #[inline]
    async fn remove(&self, proxy_id: usize) {
        self.call_mut(|mut inner| async move { inner.remove(proxy_id) })
            .await
    }
    #[inline]
    async fn get(&self, proxy_id: usize) -> Option<NetxToken<ProxyController>> {
        self.call(|inner| async move { inner.get(proxy_id).await })
            .await
    }

    #[inline]
    async fn get_all_token(&self) -> Result<Vec<NetxToken<ProxyController>>> {
        self.call(|inner| async move { inner.get_all().await })
            .await
    }
}
