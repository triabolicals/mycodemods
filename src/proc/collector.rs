use std::collections::HashMap;
use super::*;
use engage::{app::{
    hubaccess::*,
    hubaccessdata::*,
    hublocatorgroup::*,
    animaldata::*,
    gameuserdata::*,
    itemlistyesnodialog::*,
    hubsequence::*,
    ISingletonClass_1Methods,
    CommonRewardSequence,
    GameIcon,
    HubFacilityData,
    HubUtil,
    IBasicDialogMethods,
    IHubFacilityDataMethods,
    IItemDataMethods,
    IStructBase,
    IStructData_1Methods,
    ItemData,
    ItemData_Kinds,
    Transporter
}, List_1Ext, prelude::List_1, system::collections::generic::IList_1Methods, unity_engine::IGameObjectMethods, TitleBar};
use engage::app::BasicMenu_Result;
use unity::IlNull;

pub fn can_collects_item() -> bool {
    to_opt(HubSequence::get_instance()).map(|hub|
        hub.get_locator_group()
            .get_access_list()
            .iter()
            .any(|access| !access.get_is_done() && access.get_item_count() > 0 && to_opt(access.aid()).is_some_and(|v| !ItemData::get(v).is_null()) )
    ).unwrap_or(false)
}

fn try_capture_animal(access: HubAccessData) -> Option<AnimalData>  {
    if !can_adopt(access){ None }
    else if let Some(animal) = to_opt(access.try_get_pid()).and_then(|pid| to_opt(AnimalData::get_by_pid(pid))){
        HubUtil::inc_animal_capture_num(animal, 1);
        Some(animal)
    }
    else { None }
}
pub fn collect_items(this: ProcInst) -> BasicMenu_Result {
    let mut monies = 0;
    let mut bond = 0;
    let mut items: HashMap<i32, i32> = HashMap::new();
    let mut adopt: HashMap<i32, i32> = HashMap::new();
    if let Some(hub) = to_opt(HubSequence::get_instance()) {
        hub.get_locator_group()
            .get_access_list()
            .iter()
            .filter(|access| !access.m_access_data().get_is_done())
            .for_each(|point| {
                let access = point.m_access_data();
                if can_adopt(access) {
                    if let Some(animal) = try_capture_animal(access) {
                        if let Some(i) = adopt.get_mut(&animal.index()) { *i += 1; }
                        else { adopt.insert(animal.index(), 1); }
                    }
                    access.done_access();
                    point.done_access();
                }
                else if let Some(item) = to_opt(access.aid()).and_then(|iid| to_opt(ItemData::get(iid))) {
                    if item.get_kind() == ItemData_Kinds::piece_of_bond() { bond += item.get_price(); }
                    else if item.get_kind() == ItemData_Kinds::gold() { monies += item.get_price(); }
                    else {
                        let amount = HubUtil::get_item_count_with_bonus(item, access.get_item_count());
                        if let Some(i) = items.get_mut(&item.hash()) { *i += amount; }
                        else { items.insert(item.hash(), amount); }
                    }
                    access.done_access();
                    point.done_access();
                    if !point.get_item_effect().is_null() { point.get_item_effect().set_active(false); }
                }
            });
    }
    let user_data = GameUserData::get_instance();
    user_data.set_gold(user_data.get_gold() + monies);
    user_data.set_piece_of_bond(user_data.m_piece_of_bond() + bond);
    let param_list = List_1::<ItemListYesNoDialog_ItemParam>::new();
    let mut discard = false;
    items.into_iter().for_each(|(hash, count)| {
        let item = ItemData::try_get_from_hash(hash);
        if !item.is_null() {
            let dialog_item = ItemListYesNoDialog_ItemParam::new();
            dialog_item.set_sprite(GameIcon::try_get_item_2(item));
            dialog_item.set_num(count);
            dialog_item.set_name(Mess::get(item.get_name()));
            if item.is_inventory() { item.add_inventory(count); }
            else {
                for _ in 0..count {
                    if Transporter::get_empty_index() < 0 {
                        discard = true;
                        break;
                    }
                    else { Transporter::add_2(item); }
                }
            }
            param_list.add(dialog_item);
        }
    });
    adopt.into_iter().for_each(|(hash, count)| {
        let item = AnimalData::get_2(hash);
        if !item.is_null() {
            let dialog_item = ItemListYesNoDialog_ItemParam::new();
            dialog_item.set_sprite(GameIcon::try_get_system(item.get_icon_name()));
            dialog_item.set_num(count);
            dialog_item.set_name(Mess::get(item.get_name()));
            param_list.add(dialog_item);
        }
    });
    if bond > 0 {
        let dialog_item = ItemListYesNoDialog_ItemParam::new();
        dialog_item.set_name(Mess::get("MID_PieceOfBonds"));
        dialog_item.set_sprite(GameIcon::try_get_system("Bonds"));
        dialog_item.set_num(bond);
        param_list.add(dialog_item);
        engage::app::TitleBar::get_instance().update_footer_values();
    }
    let money_param =
        if monies < 1 { ItemListYesNoDialog_MoneyParam::null() }
        else {
            let item = ItemListYesNoDialog_MoneyParam::new();
            item.set_num(monies);
            item
        };
    let title = Mess::get(if user_data.get_sequence().value == 4 { "MID_MENU_TITLE_HUB" } else { "MID_MENU_TITLE_KIZUNA" });
    let proc = CommonRewardSequence::try_create_bind_discard_message(this, discard);
    let dialog = ItemListYesNoDialog::create_bind(proc, title, param_list, money_param);
    dialog.set_background_blur_off();
    BasicMenu_Result::close_decide()
}
fn can_adopt(access: HubAccessData) -> bool {
    GameUserData::get_instance().get_sequence().value == 5 &&
        to_opt(HubFacilityData::get("AID_動物小屋".into())).is_some_and(|v| v.is_complete()) &&
        !access.get_is_done() &&
        access.get_is_animal() &&
        to_opt(access.try_get_pid()).and_then(|pid| to_opt(AnimalData::get_by_pid(pid))).is_some_and(|a| HubUtil::is_capture_animal(a))
}