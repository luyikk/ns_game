use aqueue::RwModel;
use once_cell::sync::Lazy;
use std::env::current_dir;
use std::path::Path;

use crate::config::Config;
use crate::services::{BroadcastService, MasterService, ProxyService};

/// 当前运行路径
pub static CURRENT_EXE_PATH: Lazy<String> = Lazy::new(|| match std::env::current_exe() {
    Ok(path) => {
        if let Some(current_exe_path) = path.parent() {
            return current_exe_path.to_string_lossy().to_string();
        }
        panic!("current_exe_path get error: is none");
    }
    Err(err) => panic!("current_exe_path get error:{err:?}"),
});

/// 配置
pub static BASE_CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::load_config(
        &load_content("base_config.toml").expect("read base_config.toml file error"),
    )
    .expect("toml base_config.toml error")
});

/// 代理管理器
pub static PROXY: Lazy<RwModel<ProxyService>> = Lazy::new(|| RwModel::new(ProxyService::default()));

/// 广播服务
pub static BROADCAST_SERVICE: Lazy<BroadcastService> = Lazy::new(|| BroadcastService);

/// MASTER 服务器
pub static MASTER_SERVICE: Lazy<MasterService> =
    Lazy::new(|| MasterService::new(BASE_CONFIG.master.clone()));

/// 加载文件内容
#[inline]
pub fn load_content(filename: &str) -> std::io::Result<String> {
    let exe_path = format!("{}/{}", CURRENT_EXE_PATH.as_str(), filename);
    let config_path = Path::new(&exe_path);
    let path = {
        if !config_path.exists() {
            let json_path = format!(
                "{}/{}",
                current_dir().expect("not found current dir").display(),
                filename
            );
            let path = Path::new(&json_path);
            if !path.exists() {
                let json_path = format!(
                    "{}/{}",
                    current_dir()
                        .expect("not found current dir")
                        .parent()
                        .expect("not found config file")
                        .display(),
                    filename
                );
                let path = Path::new(&json_path);
                if !path.exists() {
                    panic!("not found config file:{path:?}");
                } else {
                    path.to_path_buf()
                }
            } else {
                path.to_path_buf()
            }
        } else {
            config_path.to_path_buf()
        }
    };

    std::fs::read_to_string(path)
}
