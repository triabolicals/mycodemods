use engage::app::{itemdata::*, BackgroundManager, BasicMenu_Result, CommonRewardSequence, GameIcon, HubUtil, IBasicDialogMethods, IItemListYesNoDialog_ItemParam, IStructBase, IUnitItemMethods, ItemData, ItemListYesNoDialog, ItemListYesNoDialog_ItemParam, ItemListYesNoDialog_MoneyParam, Random_2, Transporter, WellTopMenu_EvilMapStartMenuItem};
use super::*;
use engage::app::wellsequence::*;
use engage::prelude::List_1;
use engage::TitleBar;
use unity::Array;
use crate::menus::hub::{HubQuickMenu, HubQuickMenuKind};

pub fn well_create_bind(this: ProcInst) -> BasicMenu_Result {
    let new_proc = ProcInst::new();
    let descs = [
        Proc::call_2(ProcVoidMethod::from_fn(new_proc, well_item_branch).unwrap()),
        Proc::label(1),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::get_evil_weapon1_method_info().into())),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::get_evil_weapon2_method_info().into())),
        Proc::label(2),
        Proc::call_2(ProcVoidMethod::from_fn(new_proc, close_hub_menu).unwrap()),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), BackgroundManager::bind_2_method_info().into())),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::load_prefab_async_method_info().into())),
        Proc::tick_2(ProcVoidMethod::new(new_proc.into(), WellSequence::wait_load_prefab_method_info().into())),
        Proc::label(3),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::get_item_method_info().into())),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::open_header_method_info().into())),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::create_item_select_menu_method_info().into())),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::check_item_select_result_method_info().into())),
        Proc::label(4),
        Proc::call_2(ProcVoidMethod::from_fn(new_proc, try_create_fade_out_effect).unwrap()),
        Proc::call_2(ProcVoidMethod::new(new_proc.into(), WellSequence::unload_prefab_method_info().into())),
        Proc::call_2(ProcVoidMethod::from_fn(new_proc, open_hub_menu).unwrap()),
        Proc::label(5),
        Proc::end(),
    ];
    let array = Array::<ProcDesc>::from_slice(&descs).unwrap();
    new_proc.create_bind(this, array, "WellItemExchange");
    BasicMenu_Result::se_cursor()
}
extern "C" fn try_create_fade_out_effect(_this: ProcInst, _optional_method: OptionalMethod) { if is_hub_solanel() { WellSequence::try_fadeout_effect(); } }
extern "C" fn open_hub_menu(this: ProcInst, _optional_method: OptionalMethod) {
    if BackgroundManager::is_bind() { BackgroundManager::unbind(); }
    TitleBar::close_header();
    if let Some(menu) = this.get_super().try_cast::<HubQuickMenu>() {
        if WellSequence::get_use_flag() == WellSequence_UseFlags::used() {
            menu.update_menu_item(HubQuickMenuKind::Well, true);
        }
        IBasicMenuMethods::open_anime(menu);
    }
    else if let Some(menu) = this.get_super().try_cast::<BasicMenu>() {
        IBasicMenuMethods::open_anime(menu);
    }
}
extern "C" fn close_hub_menu(this: ProcInst, _optional_method: OptionalMethod) {
    if let Some(menu) = this.get_super().try_cast::<BasicMenu>() {
        IBasicMenuMethods::close_anime(menu);
    }
}

pub fn bind_well_preview(this: ProcInst) {
    let mut count: Vec<(i32, i32)> = Vec::new();
    let well = WellSequence::new();
    let seed = WellSequence::get_seed();
    let random = Random_2::new_2(seed as u32);
    let items = well.calc_item_exchange(WellSequence::get_exchange_level(), random);
    if items.is_null() {
        GameMessage::create_key_wait(this,"Cannot calculate Well items.");
        return;
    }
    items.iter().for_each(|item|{
        if let Some(found) = count.iter_mut().find(|x| x.0 == item.hash()) { found.1 += 1; }
        else { count.push( (item.hash(), 1));}
    });
    if count.is_empty() {
        GameMessage::create_key_wait(this,"No available items.");
        return;
    }
    let param_list = List_1::<ItemListYesNoDialog_ItemParam>::new();
    count.into_iter().for_each(|(hash, count)|{
        let item = ItemData::try_get_from_hash(hash);
        if !item.is_null() {
            let dialog_item = ItemListYesNoDialog_ItemParam::new();
            dialog_item.set_sprite(GameIcon::try_get_item_2(item));
            dialog_item.set_num(count);
            dialog_item.set_name(Mess::get(item.get_name()));
            param_list.add(dialog_item);
        }
    });
    let money_param = ItemListYesNoDialog_MoneyParam::null();
    let title = format!("{} (Preview)", Mess::get("MID_Hub_Well"));
    let proc = CommonRewardSequence::try_create_bind_discard_message(this, false);
    let dialog = ItemListYesNoDialog::create_bind(proc, title, param_list, money_param);
    dialog.set_background_blur_off();
}

extern "C" fn well_item_branch(this: ProcInst, _optional_method: OptionalMethod) {
    let state = WellSequence::get_evil_weapon_event_state();
    if state == WellSequence_EvilWeaponEventStates::none() {
        if HubUtil::is_complete("M006") {
            this.jump(1);
            WellSequence::set_evil_weapon_event_state(WellSequence_EvilWeaponEventStates::already_talk());
        }
        else {
            GameMessage::create_key_wait(this, Mess::get("MID_Hub_Well_Nothing1"));
            this.jump(5);
        }
    }
    else if state == WellSequence_EvilWeaponEventStates::can_get() {
        this.jump(1);
        WellSequence::set_evil_weapon_event_state(WellSequence_EvilWeaponEventStates::already_talk());
    }
    else {
        if WellSequence::get_use_flag() == WellSequence_UseFlags::used() {
            GameMessage::create_key_wait(this, Mess::get("MID_Hub_Well_ItemExchange_Message"));
            this.jump(5);
        }
        else { this.jump(2); }
    }
}