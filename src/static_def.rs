use aqueue::RwModel;
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
    pub static ref CONFIG:Config={
        let path=format!("{}/base_config.toml", CURRENT_EXE_PATH.as_str());
        Config::load_config(Path::new(&path)).expect("read base_config.toml error")
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
        MasterService::new(CONFIG.master.clone())
    };
}
