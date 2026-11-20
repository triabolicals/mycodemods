use engage::{app::{
    unit::*, unitutil::*, persondata::*, unitpool::*, godpool::*,
    IStructData_1Methods,
}, GameVariableManager, List_1Ext};
use engage::app::{BasicMenuItem_Attribute, GodData, IItemDataMethods, IMyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem, ItemData, MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem, Transporter};
use unity::{Cast, OptionalMethod};
use crate::utils::override_vtable;

pub fn recruit_all_clear_chapters() {
    [
        "リュール", "ヴァンドレ", "クラン", "フラン", "アルフレッド", "エーティエ", "ブシュロン", "セリーヌ",
        "クロエ", "ルイ", "ユナカ", "スタルーク", "シトリニカ", "ラピス", "ディアマンド", "アンバー",
        "ジェーデ", "アイビー", "カゲツ", "ゼルコバ", "フォガート", "パンドロ", "ボネ", "ミスティラ",
        "パネトネ", "メリン", "オルテンシア", "セアダス", "ロサード", "ゴルドマリー", "リンデン",
        "ザフィーア", "ヴェイル", "モーヴ", "アンナ", "ジャン", "エル", "ラファール", "セレスティア",
        "グレゴリー", "マデリーン"
    ].into_iter().for_each(|x| {
        let pid = format!("PID_{}", x);
        let person = PersonData::get(pid.into());
        if !person.is_null() {
            let mut unit = UnitPool::get_from_person(person, false);
            if unit.is_null() {
                unit = UnitUtil::join_unit_2(person);
                unit.set_m_exp(99);
            }
            else { unit.set_m_exp(99); }
        }
    });
    for i in ["G_Cleared_M00", "G_Cleared_M01"]{
        GameVariableManager::find_starts_with(i).iter().for_each(|x| { GameVariableManager::set_bool(x, true); });
    }
    ["GID_マルス", "GID_シグルド", "GID_セリカ", "GID_ミカヤ", "GID_ロイ", "GID_リーフ", "GID_ルキナ", "GID_リン", "GID_アイク",
        "GID_ベレト", "GID_カムイ", "GID_エイリーク", "GID_エーデルガルト", "GID_チキ", "GID_ヘクトル", "GID_ヴェロニカ", "GID_セネリオ",
        "GID_カミラ", "GID_クロム", "GID_リュール"].into_iter().for_each(|x| {
        GodPool::create(GodData::get(x.into()));
    });
    // override_vtable("App", "MyRoomRelianceSubSelect.MyRoomRelianceSubSelectItem", "BuildAttribute", support_item_bypass as _);
    ItemData::get_list().iter().for_each(|x| {
        if x.get_gain_skill_point() >= 100 {
            let count = Transporter::get_item_count(x);
            for _ in 0..10-count { Transporter::add_2(x); }
        }
    })
}

pub fn support_item_bypass(this: MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem, _optional_method: OptionalMethod) -> BasicMenuItem_Attribute {
    if !this.m_unit_l().is_null() && !this.m_unit_r().is_null() { BasicMenuItem_Attribute::enable() }
    else { unsafe { MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem::build_attribute(this) } }
}