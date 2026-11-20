use engage::{
    app::dlcmanager::*,
    GameVariableManager,
    app::hubsequence::*,
    app::IStructDataArray_1Methods,
    app::{GameUserData, IGameUserData, IGameVariableMethods, ISingletonClass_1Methods}
};
use engage::app::RewardData;
use unity::OptionalMethod;

const GIFT_KEY: [&str; 4] = [
    "G_拠点_DLC特典アイテム0受け取り済み", 
    "G_拠点_DLC特典アイテム1受け取り済み",
    "G_拠点_Patch0特典アイテム受け取り済み", 
    "G_拠点_Patch3特典アイテム受け取り済み"
];
pub fn obtained_gifts(dlc: bool) -> bool {
    if dlc { GameVariableManager::get_number(GIFT_KEY[0]) != 0 || GameVariableManager::get_number(GIFT_KEY[1]) != 0 }
    else { GameVariableManager::get_number(GIFT_KEY[2]) != 0 || GameVariableManager::get_number(GIFT_KEY[3]) != 0 }
}
pub fn has_hero_dlc() -> bool { DLCManager::has_content(DLCManager_Content::h0()) }
fn set_gift_key(idx: usize, skip: bool) {
    let key = GIFT_KEY[idx];
    if RewardData::get_count() == 0 {
        if !GameVariableManager::is_exist(key) { GameVariableManager::make_entry_norewind(key, 0); }
        return;
    }
    if skip { if !GameVariableManager::is_exist(key) { GameVariableManager::make_entry_norewind(key, 0); } }
    else {
        if GameVariableManager::is_exist(key){
            if GameVariableManager::get_number(key) == 0 { GameUserData::get_instance().m_variable().remove(key); }
        }
    }
}
pub fn add_hub_gift_key() {
    set_gift_key(0, crate::menus::config::menuitem::GiftDLC.get_flag_value());
    set_gift_key(1, crate::menus::config::menuitem::GiftDLC.get_flag_value());
    set_gift_key(2, crate::menus::config::menuitem::GiftPatch.get_flag_value());
    set_gift_key(3, crate::menus::config::menuitem::GiftPatch.get_flag_value());
}

pub extern "C" fn hub_sequence_content(this: HubSequence, _: OptionalMethod) {
    if !crate::menus::config::menuitem::GiftHero.get_flag_value() { this.contents_event(); }
}