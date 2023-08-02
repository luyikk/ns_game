use serde::{Deserialize, Serialize};

/// 通用成功
#[derive(Deserialize, Serialize)]
pub struct Success {}

/// 返回通用成功
/// ``` ignore
/// ret_success!();
/// ret_success!(serial);
/// ```
#[macro_export]
macro_rules! ret_success {
    () => {
        return $crate::packers::IntoResult::to($crate::packers::success::Success {}, None)
    };
    ($serial:tt) => {
        return $crate::packers::IntoResult::to($crate::packers::success::Success {}, $serial)
    };
}
