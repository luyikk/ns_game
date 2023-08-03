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

/// 游戏彩金信息
#[derive(Deserialize,Serialize)]
pub struct LotteryInfo {
    /// 彩金ID
    pub lottery_id: i32,
    /// 大厅展示 0不展示  1,是基本展示,101-福树 102-冒火 103-闪电
    pub hall_show: i32,
    /// 彩金类型 0为常规 其他为有奖 1普通奖励 2小奖分值 3中奖分值 4大奖分值 5巨奖分值
    pub lottery_type: i32,
    /// 彩金的属性:0倍数还是 1钱
    pub bet_or_money: i32,
    /// 当前的彩金值/彩金倍数
    pub lottery_real: i64,
    /// 彩金属性为倍数时,彩金最小倍数
    pub lottery_min_bet: i64,
    /// 彩金属性为倍数时,彩金最大倍数
    pub lottery_max_bet: i64,
    /// 该彩金最小的押注,超过该值才能影响彩金的变动
    pub coin_min: i64,
    /// 该彩金的增量 ,用于客服端模拟自增
    pub lottery_real_increment: i32,
    /// 该彩金关联的游戏
    pub game_ids: Vec<u32>,
}
