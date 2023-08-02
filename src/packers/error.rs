use super::IntoResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 返回 提供serial错误格式化
/// ``` ignore
/// ret_error!("error")
/// ret_error!("error:{}",err)
/// ret_error!("error:{err}")
/// ret_error!(serial,"error")
/// ret_error!(serial,"error:{}",err)
/// ret_error!(serial,"error:{err}")
/// ret_error!(serial,-10,"error")
/// ret_error!(serial,-10,"error:{}",err)
/// ret_error!(serial,-10,"error:{err}")
/// ```
#[macro_export]
macro_rules! ret_error {
    ($serial:expr,$err_id:expr;$($arg:tt)*) => {
        return $crate::packers::error::format_gen_error($serial,$err_id,std::format!($($arg)*).into())
    };
    ($serial:expr;$($arg:tt)*) => {
        return $crate::packers::error::format_gen_error($serial,0i32,std::format!($($arg)*).into())
    };
    (;$($arg:tt)*) => {
        return $crate::packers::error::format_gen_error(None,0i32,std::format!($($arg)*).into())
    };
    ($serial:expr,$msg:expr) => {
        return $crate::packers::error::format_gen_error($serial, 0i32, $msg.into())
    };
    ($serial:expr,$err_id:expr,$msg:expr) => {
        return $crate::packers::error::format_gen_error($serial, $err_id, $msg.into())
    };
    ($msg:expr) => {
        return $crate::packers::error::format_gen_error(None, 0i32, $msg.into())
    };

}

/// 返回 不提供serial 错误格式化
/// ``` ignore
/// ret_err!("error")
/// ret_err!("error:{}",err)
/// ret_err!("error:{err}")
/// ret_err!(-10,"error")
/// ret_err!(-10,"error:{}",err)
/// ret_err!(-10,"error:{err}")
/// ```
#[macro_export]
macro_rules! ret_err {
    ($msg:expr) => {
        return $crate::packers::error::format_gen_error(None, 0i32, $msg.into())
    };
    ($err_id:expr,$msg:expr) => {
        return $crate::packers::error::format_gen_error(None, $err_id, $msg.into())
    };
    (;$($arg:tt)*) => {
       return $crate::packers::error::format_gen_error(None,0i32,std::format!($($arg)*).into())
    };
    ($err_id:expr;$($arg:tt)*) => {
       return $crate::packers::error::format_gen_error(None,$err_id,std::format!($($arg)*).into())
    };
}

/// 返回通用错误
#[inline]
pub fn format_gen_error(
    serial: Option<i64>,
    error_id: i32,
    msg: Cow<'static, str>,
) -> Result<Vec<u8>> {
    GeneralError::new(error_id, msg).to(serial)
}

/// 通用错误
#[derive(Deserialize, Serialize)]
pub struct GeneralError {
    /// 错误id
    pub error_id: i32,
    /// 消息
    pub msg: Cow<'static, str>,
}

impl GeneralError {
    #[inline]
    pub fn new(error_id: i32, msg: Cow<'static, str>) -> Self {
        Self { error_id, msg }
    }
}
