pub mod error;
pub mod success;
pub mod update;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_type_name::type_name;
use std::borrow::Cow;

/// 用于快速将packer转换成发送结果
pub trait IntoResult {
    fn to(self, serial: Option<i64>) -> Result<Vec<u8>>;
}

#[derive(Serialize)]
pub struct SerializeSerialJson<'a, T: Serialize> {
    serial: i64,
    func: &'a str,
    context: T,
}

#[derive(Serialize)]
pub struct SerializeJson<'a, T: Serialize> {
    func: &'a str,
    context: T,
}

impl<T: Serialize> IntoResult for T {
    #[inline]
    fn to(self, serial: Option<i64>) -> Result<Vec<u8>> {
        let func = type_name(&self)?;
        if let Some(serial) = serial {
            let json_value = SerializeSerialJson {
                serial,
                func,
                context: self,
            };
            Ok(serde_json::to_vec(&json_value)?)
        } else {
            let json_value = SerializeJson {
                func,
                context: self,
            };
            Ok(serde_json::to_vec(&json_value)?)
        }
    }
}

#[macro_export]
macro_rules! ret {
    ($pack:expr) => {
        return $crate::packers::IntoResult::to($pack, None)
    };
    ($pack:expr,$serial:tt) => {
        return $crate::packers::IntoResult::to($pack, $serial)
    };
}

/// Ping
/// return->Pong
#[derive(Deserialize, Serialize)]
pub struct Ping {
    /// 时间戳
    pub tick: i64,
}

/// Pong
#[derive(Deserialize, Serialize)]
pub struct Pong {
    /// 时间戳
    pub tick: i64,
}

/// GetToken 回包
#[derive(Serialize)]
pub struct GetTokenResult {
    /// token
    pub token: u64,
    /// 超时时间
    pub timeout: i64,
}

/// 获取玩家信息数据包
#[derive(Serialize)]
pub struct GetUserInfoResult<'a> {
    /// 玩家账号id
    pub account_id: i32,
    /// 玩家昵称
    pub nickname: Cow<'a, str>,
    /// 玩家头像id
    pub avatar_id: i32,
    /// 玩家email
    pub email: Cow<'a, str>,
    /// 玩家渠道id
    pub channel_id: i32,
    /// 玩家累计充值金额
    pub total_recharge: f64,
    /// 玩家累计退款金额
    pub total_refund: f64,
    /// 玩家身上金币
    pub money: f64,
    /// 玩家保险箱金币
    pub money_safe: f64,
    /// 玩家绑定金币
    pub money_gift: f64,
    /// 玩家绑定金币保险箱
    pub money_gift_safe: f64,
    /// 玩家累计洗码值
    pub amount_of_wash_code: f64,
    /// 玩家vip等级
    pub vip_level: i32,
}
