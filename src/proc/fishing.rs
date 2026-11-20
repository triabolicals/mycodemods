use std::collections::HashMap;
use super::*;
use engage::{
    app::{
        fishinggamesequence::*,
        fishingfishdata::*,
        BasicMenu_Result,
        FishingDisposPatternData,
        GameIcon,
        IBasicDialogMethods,
        IFishingDisposPatternDataMethods,
        IItemDataMethods,
        IItemListYesNoDialog_ItemParam,
        IRandom_2Methods,
        IStructBase,
        IStructData_1Methods,
        ItemData,
        ItemListYesNoDialog,
        ItemListYesNoDialog_ItemParam,
        ItemListYesNoDialog_MoneyParam,
        Random_2,
        Transporter,
        fishing::StickType
    },
    List_1Ext,
    prelude::List_1,
    system::collections::generic::IList_1Methods
};
use engage::app::GameUserGmapData_EncountCounterType;
use unity::{Cast, IlNull, OptionalMethod};
use crate::utils::to_opt;
use engage::app::IGameUserGmapDataMethods;
pub fn fishing_sequence_create_bind(this: ProcInst) -> BasicMenu_Result {
    let nfish = GameVariableManager::get_number("G_Fishing_PlayCount");
    if nfish == 0 { return BasicMenu_Result::se_miss() }
    GameUserData::get_instance().get_gmap_data().add_encount_counter(GameUserGmapData_EncountCounterType::minigame());
    unsafe { set_forecast_fish(0, None); }
    let mut count = 0;
    let is_auto = crate::menus::config::menuitem::AutoFisher.get_flag_value();
    let target = GameVariableManager::get_string("G_Fishing_TargetFish").to_rust_string();
    let mut items = HashMap::<i32, i32>::new();
    let param_list = List_1::<ItemListYesNoDialog_ItemParam>::new();
    for _ in 0..nfish {
        let fish = get_random_fish();
        if is_auto || fish.get_fish_name().to_rust_string() == target{ count += 2 * fish.get_piece_count(); }
        else { count += fish.get_piece_count(); }
        unsafe { increase_play_counter(0, None) };
        if let Some(item) = to_opt(ItemData::get(fish.get_food_type())){
            if let Some(v) = items.get_mut(&item.hash()) { *v += 1; }
            else { items.insert(item.hash(), 1); }
        }
    }
    items.into_iter().for_each(|(hash, count)| {
        let item = ItemData::try_get_from_hash(hash);
        if !item.is_null() {
            let dialog_item = ItemListYesNoDialog_ItemParam::new();
            dialog_item.set_sprite(GameIcon::try_get_item_2(item));
            dialog_item.set_num(count);
            dialog_item.set_name(Mess::get(item.get_name()));
            if item.get_use_type().value == 0x20 { item.add_inventory(1); }
            else { Transporter::add_2(item); }
            param_list.add(dialog_item);
        }
    });
    let dialog_item = ItemListYesNoDialog_ItemParam::new();
    dialog_item.set_name(Mess::get("MID_PieceOfBonds"));
    dialog_item.set_sprite(GameIcon::try_get_system("Bonds"));
    crate::utils::add_bond(count);
    dialog_item.set_num(count);
    param_list.add(dialog_item);
    let rod = get_rod();
    let fish_rod_miid =
        if rod == 1 { "MIID_FishingRod_Normal" }
        else if rod == 2 { "MIID_FishingRod_AllPurpose" }
        else { "MIID_FishingRod_Smoll" };

    let message = format!("{}: {}", Mess::get("MID_Hub_Talk_Fishing"), Mess::get(fish_rod_miid));

    let dialog = ItemListYesNoDialog::create_bind(this, message, param_list, ItemListYesNoDialog_MoneyParam::null());
    dialog.set_background_blur_off();
    GameVariableManager::set_number("G_Fishing_PlayCount", 0);
    BasicMenu_Result::se_cursor()
}
fn get_rod() -> i32 {
    let mut rod = 0;
    ItemData::get_list().iter().filter(|item| item.get_use_type().value == 35 && item.get_inventory() != 0 ).for_each(|item|{
        let name = item.get_name().to_string();
        if name == "MIID_FishingRod_Normal" && rod == 0 { rod = 1; }
        if name == "MIID_FishingRod_AllPurpose" && rod < 2 { rod = 2; }
    });
    rod
}
fn get_random_fish() -> FishingFishData {
    let rod = get_rod();
    let filter = match rod {
        1 => StickType::stick_b(),
        2 => StickType::stick_c(),
        _ => StickType::stick_a(),
    };
    let stick_type = if rod == 0 { "a" } else if rod == 1 { "b" } else { "c" };
    let time = HubVariable::get_timezone_type().value;
    let time = if time == 3 { 2 } else { time };
    let fish =
    FishingDisposPatternData::get_list().iter()
        .filter(|item| item.get_time() == time && item.get_stick_type().to_rust_string().contains(stick_type))
        .flat_map(|v| to_opt(FishingFishData::get(format!("ID_{}", v.get_fish_id()).into())))
        .collect::<Vec<FishingFishData>>();
    FishingGameSequence::set_m_rod_menu_result(filter);

    if crate::menus::config::menuitem::AutoFisher.get_flag_value() {
        fish.into_iter()
            .map(|i| (i, i.get_piece_count()))
            .max_by(|i, j| i.1.cmp(&j.1))
            .map(|v| v.0)
            .unwrap()
    }
    else {
        let rng = Random_2::get_system().get_value_2(fish.len() as i32);
        fish[rng as usize]
    }
}
#[skyline::from_offset(0x2ae4030)] fn set_forecast_fish(this: u64, method_info: OptionalMethod);
#[skyline::from_offset(0x27001d0)] fn increase_play_counter(this: u64, method_info: OptionalMethod);