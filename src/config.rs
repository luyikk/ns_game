use anyhow::Result;
use serde::Deserialize;
use std::env::current_dir;
use std::path::Path;

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
    pub fn load_config(config_path: &Path) -> Result<Self> {
        let path = {
            if !config_path.exists() {
                let json_path = format!(
                    "{}/base_config.toml",
                    current_dir().expect("not found current dir").display()
                );
                let path = Path::new(&json_path);
                if !path.exists() {
                    panic!("not found config file:{path:?}");
                } else {
                    path.to_path_buf()
                }
            } else {
                config_path.to_path_buf()
            }
        };

        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

/// 基本设置
#[derive(Deserialize, Debug, Clone)]
pub struct BaseConfig {
    /// 服务器id
    pub server_id: u32,
    /// 游戏id
    pub game_id: u32,
    /// 服务器 PEER 清理时间
    pub peer_clean_timeout_sec: i64,
    /// 缓存的account信息 多久没访问清理(秒)
    pub account_cache_cleans_timeout_sec: i64,
}
