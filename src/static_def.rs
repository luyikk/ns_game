use aqueue::RwModel;
use std::env::current_dir;
use std::path::Path;

use crate::config::Config;
use crate::services::{BroadcastService, MasterService, ProxyService};

lazy_static::lazy_static! {
      /// 当前运行路径
    pub static ref CURRENT_EXE_PATH:String={
         match std::env::current_exe(){
            Ok(path)=>{
                if let Some(current_exe_path)= path.parent(){
                    return current_exe_path.to_string_lossy().to_string()
                }
                panic!("current_exe_path get error: is none");
            },
            Err(err)=> panic!("current_exe_path get error:{err:?}")
        }
    };

    /// 配置
    pub static ref BASE_CONFIG:Config={
        Config::load_config(&load_content("base_config.toml")
            .expect("read base_config.toml file error"))
            .expect("toml base_config.toml error")
    };

    /// 代理管理器
    pub static ref PROXY:RwModel<ProxyService>={
        RwModel::new(ProxyService::default())
    };

    /// 广播服务
    pub static ref BROADCAST_SERVICE:BroadcastService={
        BroadcastService
    };

    /// MASTER 服务器
    pub static ref MASTER_SERVICE:MasterService={
        MasterService::new(BASE_CONFIG.master.clone())
    };
}

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
                panic!("not found config file:{path:?}");
            } else {
                path.to_path_buf()
            }
        } else {
            config_path.to_path_buf()
        }
    };

    std::fs::read_to_string(path)
}
