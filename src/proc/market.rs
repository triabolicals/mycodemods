use engage::{
    app::{
        jukeboxsequence::*,
        hubitemshopsequence::*,
        hubweaponshopsequence::*,
        hubrefineshopsequence::*,
        hubfleamarketsequence::*,
        refreshsequence::*,
        BackgroundManager,
        BasicMenu_Result,
        GodRoomPedestalSequence,
        GodRoomPedestalTopMenu_RingCleaningItem,
        HubFacilityData,
        IGodRoomPedestalSequenceMethods,
        IHubFacilityDataMethods,
        ITitleBarMethods,
        SortieSequenceItemShop,
        SortieSequenceWeaponShop
    },
};
use engage::app::{IGodRoomPedestalSequence, IRingDataMethods, ISynthesisRingMenuTop, RingData, RingGachaSequence, RingSynthesisSequence, RingSynthesisSequence_Result1Proc};
use unity::Il2CppString;
use crate::menus::hub::{HubQuickMenu, HubQuickMenuKind};
use crate::utils::{try_get_data_by_hash, try_get_data_by_key};
use super::*;

pub fn create_bind_refresh(this: ProcInst) -> BasicMenu_Result {
    let aid = "AID_水遊び";
    if let Some(fac) = try_get_data_by_key::<HubFacilityData>(aid) {
        let refresh = RefreshSequence::new(aid.into());
        fac.set_first_access_flag();
        let descs = refresh.create_desc();
        descs.set(20, Proc::call_2(ProcVoidMethod::from_fn(refresh.into(), proc_null).unwrap()));
        descs.set(0x1a, Proc::call_2(ProcVoidMethod::from_fn(refresh, unload_refresh).unwrap()));
        refresh.create_bind(this, descs, Il2CppString::null());
        BasicMenu_Result::se_cursor()
    }
    else { BasicMenu_Result::pass() }
}
extern "C" fn unload_refresh(refresh: RefreshSequence, optional_method: OptionalMethod) {
    refresh.unload_resources();
    open_menu(refresh.into(), optional_method);
    if let Some(hub_fac) = refresh.get_super().try_cast::<HubQuickMenu>() {
        hub_fac.update_menu_item(HubQuickMenuKind::Refresh, HubQuickMenuKind::Refresh.disable());
    }
}

pub fn create_bind_market(this: ProcInst, market_type: i32) -> BasicMenu_Result {
    engage::app::TitleBar::get_instance().show_footer();
    match market_type {
        0 => {
            let item = SortieSequenceItemShop::new();
            let descs = item.create_desc();
            descs.set(0x22, Proc::call_2(ProcVoidMethod::from_fn(item.into(), shop_end_sequence).unwrap()));
            item.create_bind(this, descs, Il2CppString::null());
        }
        1 => {
            let weapon = SortieSequenceWeaponShop::new();
            let descs = weapon.create_desc();
            weapon.create_bind(this, descs, Il2CppString::null());
        }
        2 => {
            let item = HubRefineShopSequence::new();
            let descs = item.create_desc();
            descs.set(5, Proc::call_2(ProcVoidMethod::from_fn(item, refine_create_top_menu).unwrap()));
            descs.set(0x2b, Proc::call_2(ProcVoidMethod::from_fn(item.into(), shop_end_sequence).unwrap()));
            item.create_bind(this, descs, Il2CppString::null());
        }
        3 => {
            let item = HubFleaMarketSequence::new(false);
            let descs = item.create_desc();
            descs.set(5, Proc::call_2(ProcVoidMethod::from_fn(item, flea_market_create_buy_menu).unwrap()));
            descs.set(8, Proc::call_2(ProcVoidMethod::from_fn(item.into(), shop_end_sequence).unwrap()));
            item.create_bind(this, descs, Il2CppString::null());
        }
        4 => {
            let juke = JukeboxSequence::new();
            let descs = juke.create_desc();
            descs.set(9, Proc::call_2(ProcVoidMethod::from_fn(juke, juke_box_exit).unwrap()));
            juke.create_bind(this, descs, Il2CppString::null());
        }
        5 => {
            let god = GodRoomPedestalSequence::new();
            let descs = god.create_desc();
            descs.set(5, Proc::call_2(ProcVoidMethod::from_fn(god, god_room_create_top_menu).unwrap()));
            descs.set(0x21, Proc::call_2(ProcVoidMethod::from_fn(god, create_ring_gacha_demo_result).unwrap()));
            descs.set(0x22, Proc::jump(4));
            descs.set(0x35, Proc::call_2(ProcVoidMethod::from_fn(god, create_ring_merge_result).unwrap()));
            descs.set(0x36,  Proc::jump(9));
            god.create_bind(this, descs, Il2CppString::null());
        }
        _ => { return BasicMenu_Result::pass() }
    }
    if let Some(dispose) = to_opt(this.get_child()).and_then(|v|v.get_class().raw_mut().get_virtual_method_mut("OnDispose")){
        dispose.method_ptr = open_menu as _;
    }
    BasicMenu_Result::se_cursor()
}
extern "C" fn shop_end_sequence(this: ProcInst, optional_method: OptionalMethod) {
    if let Some(end_sequence) = this.get_class().raw().get_method_from_name("EndSequence", 0){
        let end_sequence_call = unsafe { std::mem::transmute::<_, fn(ProcInst, OptionalMethod)>(end_sequence.method_ptr) };
        end_sequence_call(this, optional_method);
        if BackgroundManager::is_bind() { BackgroundManager::unbind(); }
    }
}
extern "C" fn create_ring_gacha_demo_result(pedestal: GodRoomPedestalSequence, _optional_method: OptionalMethod) {
    pedestal.execute_ring_gacha();
    pedestal.create_ring_gacha_demo_sequence();
    if let Some(ring_gacha) = pedestal.get_child().try_cast::<RingGachaSequence>() {
        let descs = ring_gacha.m_descs();
        descs.set(3, Proc::call_2(ProcVoidMethod::new(ring_gacha.into(), RingGachaSequence::create_result_window_method_info().into())));
        descs.set(4, Proc::call_2(ProcVoidMethod::new(ring_gacha.into(), RingGachaSequence::unload_resources_method_info().into())));
        descs.set(5, Proc::call_2(ProcVoidMethod::new(ring_gacha.into(), RingGachaSequence::end_method_info().into())));
        descs.set(6, Proc::end());
    }
    pedestal.add_achieve_of_refine_ring();
    collect_achievements();
}
extern "C" fn create_ring_merge_result(ped: GodRoomPedestalSequence, _optional_method: OptionalMethod) {
    ped.execute_ring_merge();
    ped.create_ring_merge_demo_sequence();
    if let Some(merge) = ped.get_child().try_cast::<RingSynthesisSequence>() {
        let descs = merge.m_descs();
        descs.set(3, Proc::call_2(ProcVoidMethod::new(merge.into(), RingSynthesisSequence::open_dialog_method_info().into())));
        descs.set(4, Proc::call_2(ProcVoidMethod::new(merge.into(), RingSynthesisSequence::unload_resources_method_info().into())));
        descs.set(5, Proc::call_2(ProcVoidMethod::new(merge.into(), RingSynthesisSequence::end_method_info().into())));
        descs.set(6, Proc::end());
    }
}
extern "C" fn refine_create_top_menu(refine: HubRefineShopSequence, _optional_method: OptionalMethod) {
    refine.create_refine_shop_top_menu();
    if !BackgroundManager::is_bind() { BackgroundManager::bind_2(); }
}
extern "C" fn flea_market_create_buy_menu(flea: HubFleaMarketSequence, _optional_method: OptionalMethod) {
    flea.create_flea_market_buy_menu();
    if !BackgroundManager::is_bind() { BackgroundManager::bind_2(); }
}
extern "C" fn god_room_create_top_menu(godroom: GodRoomPedestalSequence, _optional_method: OptionalMethod) {
    godroom.create_top_menu();
    if let Some(menu) = to_opt(godroom.m_child()).and_then(|m| m.try_cast::<BasicMenu>()){
        remove_menu_item_from_menu::<GodRoomPedestalTopMenu_RingCleaningItem>(menu);
    }
    if !BackgroundManager::is_bind() { BackgroundManager::bind_2(); }
}
extern "C" fn juke_box_exit(juke: JukeboxSequence, optional_method: OptionalMethod) {
    juke.exit();
    open_menu(juke.into(), optional_method);
}