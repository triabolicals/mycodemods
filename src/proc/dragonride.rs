use std::collections::HashMap;
use engage::{
    app::dragonridesequence::*,
};
use engage::app::{BasicMenu_Result, DragonRidePrizeData, GameIcon, GameUserGmapData_EncountCounterType, HubUtil, IBasicDialogMethods, IDragonRidePrizeDataMethods, IGameUserGmapDataMethods, IItemDataMethods, IItemListYesNoDialog_ItemParam, IRandom_2Methods, IStructBase, ItemData, ItemListYesNoDialog, ItemListYesNoDialog_ItemParam, ItemListYesNoDialog_MoneyParam, Random_2, Transporter};
use engage::prelude::List_1;
use super::*;

pub fn dragon_ride_bind(proc_inst: ProcInst) -> BasicMenu_Result {
    if GameVariableManager::get_bool("G_DragonRide_GetPrize") { return BasicMenu_Result::se_miss() }
    AchieveData::add_count_mini_game();
    GameUserData::get_instance().get_gmap_data().add_encount_counter(GameUserGmapData_EncountCounterType::minigame());
    let (difficulty, prize_id) =
        if HubUtil::get_is_condition_dragon_ride_expert_opened() {
            AchieveData::play_report_add_count_dragon_ride_expert();
            (3, "ID_Expert_SSS")
        }
        else if HubUtil::get_is_condition_muscle_hard_opened() {
            AchieveData::play_report_add_count_dragon_ride_hard();
            (2, "ID_Hard_SSS")
        }
        else {
            AchieveData::play_report_add_count_dragon_ride_normal();
            (1, "ID_Normal_SSS")
        };
    println!("Difficulty: {}", difficulty);
    if let Some(prize) = to_opt(DragonRidePrizeData::get(prize_id.into())) {
        let mut items = HashMap::<i32, i32>::new();
        let bond = prize.get_piece_of_bond();
        let param_list = List_1::<ItemListYesNoDialog_ItemParam>::new();
        let dialog_item = ItemListYesNoDialog_ItemParam::new();
        crate::utils::add_bond(bond);
        dialog_item.set_name(Mess::get("MID_PieceOfBonds"));
        dialog_item.set_sprite(GameIcon::try_get_system("Bonds"));
        dialog_item.set_num(bond);
        param_list.add(dialog_item);

        let item_count = prize.get_item_count();
        let mut items_list = prize.get_item_list().iter().flat_map(|iid| to_opt(ItemData::get(format!("IID_{}", iid).into())))
            .map(|i| i.hash())
            .collect::<Vec<_>>();
        let rng = Random_2::get_system();
        for _ in 0..item_count {
            let i = items_list.len() as i32;
            if i <= 0 { break; }
            let index = if i == 1 { 0 }
            else { rng.get_value_2(i) as usize };
            println!("Index: {}", index);
            let item = items_list.remove(index);
            if let Some(v) = items.get_mut(&item) { *v += 1; }
            else { items.insert(item, 1); }
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
        let diff_mess = if difficulty == 3 { Mess::get("MID_Hub_MiniGame_Expert") }
        else if difficulty == 2 { Mess::get("MID_SYS_Difficulty_Hard") }
        else { Mess::get("MID_SYS_Difficulty_Normal") };
        let messs = format!("{}: {}", Mess::get("MID_Hub_Talk_DragonRide"), diff_mess);
        println!("Title: {}", messs);
        let dialog = ItemListYesNoDialog::create_bind(
            proc_inst, messs, param_list,
            ItemListYesNoDialog_MoneyParam::null()
        );
        dialog.set_background_blur_off();
    }
    GameVariableManager::set_bool("G_DragonRide_GetPrize", true);
    BasicMenu_Result::se_cursor()
}