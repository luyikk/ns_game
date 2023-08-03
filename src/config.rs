use anyhow::Result;
use serde::Deserialize;

/// 大厅配置
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// 基本设置
    pub base: BaseConfig,
    /// 服务器监听配置
    pub proxy_listen: netxserver::prelude::ServerOption,
    /// 主服务连接配置
    pub master: netxclient::prelude::ServerOption,
}

impl Config {
    ///加载
    pub fn load_config(content: &str) -> Result<Self> {
        Ok(toml::from_str(content)?)
    }
}

/// 基本设置
#[derive(Deserialize, Debug, Clone)]
pub struct BaseConfig {
    /// 服务器id
    pub server_id: u32,
    /// 服务器 PEER 清理时间
    pub peer_clean_timeout_sec: i64,
    /// 缓存的account信息 多久没访问清理(秒)
    pub account_cache_cleans_timeout_sec: i64,
}
