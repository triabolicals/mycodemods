use engage::{
    app::{
        unititem::*, goddata::*, unitpool::*,
        refineitemdetailwindow::*, itemmenudetailsetter::*,
    },
    ForceExt,
    Mess,
    app::{Force, Force_Type, IItemDataMethods, IRefineShopEngraveGodRoot, IRefineShopEngraveGodRootMethods, IStructBase, IUnit, IUnitItemList, RefineShopEngraveGodRoot, ShopCore},
    tm_pro::ITMP_TextMethods
};
use unity::{Cast, ClassIdentity, OptionalMethod};
use crate::menus::config::menuitem::Engrave;

pub fn engrave_menu_root_edit() {
    if let Some(method) = RefineShopEngraveGodRoot::class().raw_mut().get_method_from_name("OnSelectMenuItem", 2) {
        method.method_ptr = refine_shop_god_update_item_detail as _;
    }
}

fn check_unit_item_engrave(unit_item: UnitItem, god_data: GodData) -> bool {
    if unit_item.is_null() || god_data.is_null() ||  unit_item.m_item().index() == 0 || unit_item.m_engrave().is_null() { false }
    else { unit_item.m_engrave().hash() == god_data.hash() }
}
fn check_for_engrave(god_data: GodData) -> Option<UnitItem> {
    Force::get(Force_Type::player()).iter().chain(Force::get(Force_Type::absent()).iter())
        .find_map(|u|{
            u.m_item_list().m_unit_items().iter().find(|i| check_unit_item_engrave(*i, god_data))
        })
}

fn refine_shop_god_update_item_detail(this: RefineShopEngraveGodRoot, god_data: GodData, unit_item: UnitItem, _method_info: OptionalMethod){
    let mut name = Mess::get(god_data.get_engrave_word()).to_rust_string();
    if !Engrave.get_flag_value() {
        unsafe { on_select_menu_item(this, god_data, unit_item, None) };
        return;
    }
    if let Some(item) = check_for_engrave(god_data) {
        this.update_item_detail(this.m_item_detail_window_base(), item);
        name += " ";
        name += Mess::get("MID_MENU_REFINE_SHOP_ENGRAVE_CONTENTS").to_rust_string().as_str();
        name += ": ";
        name += item.get_name(true).to_rust_string().as_str();
        let item = ShopCore::get_unit_item(this.m_unit(), this.m_owner_item_index());
        this.update_item_detail_2(this.m_item_detail_window_target(), item, unit_item);
        let data = item.m_item();
        let hit = if data.get_hit() < 0 { "--".to_string() } else { data.get_hit().to_string() };
        let message = format!("{}:\n{}: {} / {}: {} / {}: {}\n{}: {} / {}: {} / {}: {}",
            item.get_name(true),
            Mess::get("MID_SYS_Mt"), item.get_power(), Mess::get("MID_SYS_Hit"), hit, Mess::get("MID_SYS_Crit"), item.get_critical(),
            Mess::get("MID_SYS_Weight"), item.get_weight(), Mess::get("MID_SYS_Avo"), item.get_avoid(), Mess::get("MID_SYS_Secure"), item.get_secure()
        );
        this.m_item_detail_window_target().m_item_menu_detail_setter().m_contents_text().set_text_2(message, true);
    }
    else { unsafe { on_select_menu_item(this, god_data, unit_item, None) }; }
    let message = format!("{}\n{}: {:+} / {}: {:+} / {}: {:+}\n{}: {} / {}: {:+} / {}: {:+}",
          name,
          Mess::get("MID_SYS_Mt"), god_data.get_engrave_power(), Mess::get("MID_SYS_Hit"), god_data.get_engrave_hit(), Mess::get("MID_SYS_Crit"), god_data.get_engrave_critical(),
          Mess::get("MID_SYS_Weight"), god_data.get_engrave_weight(), Mess::get("MID_SYS_Avo"), god_data.get_engrave_avoid(), Mess::get("MID_SYS_Secure"), god_data.get_engrave_secure()
    );
    this.m_item_detail_window_base().m_item_menu_detail_setter().m_contents_text().set_text_2(message, true);
}

#[skyline::from_offset(0x295cf80)]
fn on_select_menu_item(this: RefineShopEngraveGodRoot, god_data: GodData, unit_item: UnitItem, method_info: OptionalMethod);