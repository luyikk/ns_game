mod proxy;
mod proxy_interface;

use anyhow::Result;
use netxserver::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub use proxy::*;
pub use proxy_interface::*;

/// 代理控制器
pub struct ProxyController {
    pub(crate) token: NetxToken<Self>,
    pub(crate) proxy_id: AtomicUsize,
}

impl Drop for ProxyController {
    fn drop(&mut self) {
        let session_id = self.token.get_session_id();
        log::info!(
            "Proxy:{} Controller:{} drop",
            self.proxy_id.load(Ordering::Acquire),
            session_id
        );
    }
}

pub struct ImplCreateProxyController;

impl ICreateController for ImplCreateProxyController {
    type Controller = ProxyController;

    #[inline]
    fn create_controller(
        &self,
        token: NetxToken<Self::Controller>,
    ) -> Result<Arc<Self::Controller>> {
        Ok(Arc::new(ProxyController {
            token,
            proxy_id: Default::default(),
        }))
    }
}
