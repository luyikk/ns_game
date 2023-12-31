use serde::{Deserialize, Serialize};

/// 玩家账号表
/// table:account
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AccountContext {
    /// account id
    #[serde(default)]
    pub id: i32,
    /// 昵称 默认就是账号id
    #[serde(default)]
    pub nickname: String,
    /// 头像
    #[serde(default)]
    pub avatar_id: i32,
    /// 邮箱
    #[serde(default)]
    pub email: String,
    /// 最后登陆时间
    #[serde(default)]
    pub last_login_time: i64,
    /// geoip
    #[serde(default)]
    pub last_login_ip_area: String,
    /// 每次登陆时的IP
    #[serde(default)]
    pub last_login_ip: String,
    /// 渠道编号
    #[serde(default)]
    pub channel_id: i32,
    /// 累计充值金额
    #[serde(default)]
    pub total_recharge: f64,
    /// 累计退款金额
    #[serde(default)]
    pub total_refund: f64,
}

/// 玩家金币总表
/// table:account_money
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MoneyContext {
    /// 用户Id ( 随机 8 位整数 )
    #[serde(default)]
    pub id: i32,
    /// 账户余额( 保留4位小数位, 进部分游戏时会被清0, 结束时会兑换返还 )
    #[serde(default)]
    pub money: f64,
    /// 隐藏金币
    #[serde(default)]
    pub money_cache: f64,
    /// 保险箱( 玩家可在账户余额间搬运数据 )
    #[serde(default)]
    pub money_safe: f64,
    /// 锁定金额
    #[serde(default)]
    pub money_gift: f64,
    /// 锁定金币保险箱
    #[serde(default)]
    pub money_gift_safe: f64,
    /// 锁定金币生产总值(明码)
    #[serde(default)]
    pub amount_of_money_gift: f64,
    /// 累积洗码总值(游戏中获得)
    #[serde(default)]
    pub amount_of_wash_code: f64,
    /// 玩家消耗的总金币(游戏中消费)
    #[serde(default)]
    pub consume_money: f64,
    /// 玩家消耗的总绑定金币(游戏中消耗)
    #[serde(default)]
    pub consume_money_gift: f64,
    /// 玩家在游戏中获得的总金币(游戏中收益)
    #[serde(default)]
    pub income_money: f64,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct AccountInfoRet {
    /// 账号信息
    #[serde(default)]
    pub account_content: AccountContext,
    /// 金币信息
    #[serde(default)]
    pub money_content: MoneyContext,
    /// vip 等级
    #[serde(default)]
    pub vip_level: i32,
}
