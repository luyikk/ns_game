pub mod error;
pub mod success;
pub mod update;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_type_name::type_name;

/// 用于快速将packer转换成发送结果
pub trait IntoResult {
    fn to(self, serial: Option<i64>) -> Result<Vec<u8>>;
}

#[derive(Serialize)]
struct SerializeSerialJson<'a, T: Serialize> {
    serial: i64,
    func: &'a str,
    context: T,
}

#[derive(Serialize)]
struct SerializeJson<'a, T: Serialize> {
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

/// 返回 序列化 Vec<u8> 结果
/// ``` ignore
///  ret!(Foo{..})
///  ret!(Foo{..},serial)
/// ```
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
