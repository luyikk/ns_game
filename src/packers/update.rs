use serde::Serialize;

/// 无 serial 序列化 返回 Vec<u8>
/// 一般用于消息广播
#[macro_export]
macro_rules! des {
    ($pack:expr) => {
        $crate::packers::IntoResult::to($pack, None)?
    };
}

/// 资产修改包
#[derive(Serialize)]
pub struct MoneyChanged {
    /// 账户余额
    pub money: f64,
    /// 账户保险箱余额
    pub money_safe: f64,
    /// 绑定金币
    pub money_gift: f64,
    /// 绑定金币保险箱
    pub money_gift_safe: f64,
    /// 累计退款金额
    pub total_refund: f64,
    /// 累计充值金额
    pub total_recharge: f64,
    /// vip 等级
    pub vip_level: i32,
}
