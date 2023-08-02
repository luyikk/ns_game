use super::account::*;
use serde::{Deserialize, Serialize};

/// 老虎机请求旋转
#[derive(Deserialize, Serialize, Debug)]
pub struct ReqSlotSpin {
    /// 游戏级别
    pub game_level_id: i32,
    /// 游戏服ID
    pub game_id: u32,
    /// 押注金额
    pub in_money: i64,
    /// 期望获得的金额
    pub hope_get: i64,
    /// 押注金币类型 0 是普通金币 1是绑定金币
    pub money_type: u32,
    /// 是否影响该游戏服关联的彩金
    pub need_change_lottery: bool,
    /// 是否需要退钱
    pub need_refund: bool,
    /// (need_refund=false才判定)当0代表没有押注彩金/其他表示该次想打下的彩金
    pub lottery_id: i32,
    /// 旋转类型 0不打包 1免费打包 2落地牌打包 3..
    pub spin_type: u32,
}

/// 老虎机请求旋转结果
#[derive(Deserialize, Serialize, Default)]
pub struct SlotSpinRet {
    /// code
    pub code: i32,
    /// 信息
    pub info: Option<SlotsSpinInfo>,
}

/// 老虎机请求旋转结果内容
#[derive(Deserialize, Serialize)]
pub struct SlotsSpinInfo {
    /// 账号id
    pub account_id: i32,
    /// 是否赢钱
    pub is_win: bool,
    /// 押注金币类型
    pub money_type: u32,
    /// 押注金额
    pub in_money: i64,
    /// 赢得金币
    pub win_money: i64,
    /// 账号信息
    pub account_info: AccountInfo,
    /// vip 等级升级奖励
    pub level_ups: Vec<i64>,
}

/// 老虎机请求退钱
#[derive(Deserialize, Serialize, Debug)]
pub struct ReqSlotRefund {
    /// 游戏级别
    pub game_level_id: i32,
    /// 游戏服ID
    pub game_id: u32,
    /// 押注金额
    pub in_money: i64,
    /// 需要退还的金额
    pub refund_get: i64,
    /// 0代表没有押注彩金/其他表示该次退款被打下的彩金
    pub lottery_id: i32,
}

#[derive(Deserialize, Serialize, Default)]
pub struct SlotRefundRet {
    /// code
    pub code: i32,
    /// 信息
    pub info: Option<SlotRefundInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct SlotRefundInfo {
    /// 账号id
    pub account_id: i32,
    /// 游戏级别
    pub game_level_id: i32,
    /// 游戏服ID
    pub game_id: u32,
    /// 押注金额
    pub in_money: i64,
    /// 账号信息
    pub money_info: TableAccountMoney,
}
