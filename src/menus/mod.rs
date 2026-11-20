pub use engage::app::{
    basicmenu::*,
    basicmenuitem::*,
    procinst::*,
};
use engage::system::collections::generic::IList_1Methods;
use unity::{ClassIdentity, OptionalMethod};
use crate::menus::sp::SPDialogMenu;
use crate::utils::override_vtable;

pub(crate) mod config;
pub(crate) mod hub;
pub(crate) mod sortie;
pub(crate) mod cook;
pub(crate) mod sp;

pub use config::menuitem::ConfigMenuItemKind::*;
pub fn install_menu_items() {
    if cobapi::injection::register::<config::SettingMenu>().is_ok()  { println!("SettingMenu is Injected"); }
    if cobapi::injection::register::<config::menuitem::MyCodeModConfigItem>().is_ok() { println!("MyCodeModConfigItem is Injected"); }
    if cobapi::injection::register::<hub::HubQuickMenuItem>().is_ok() { println!("HubQuickMenuItem is Injected"); }
    if cobapi::injection::register::<hub::HubQuickMenu>().is_ok() { println!("HubQuickMenu is Injected"); }
    sortie::inject_sortie_menu_items();
    cook::install_cook_menus();
    hub::mascot::mascot_vtable_edit();
    SPDialogMenu::inject();
    override_vtable("App", "GmapMenuSequence.GmapMenu.ShopItem", "ACall", gmap_market_a_call as _);
}

fn gmap_market_a_call(this: BasicMenuItem, _: OptionalMethod) -> BasicMenu_Result {
    let menu = this.m_menu();
    menu.m_full_menu_item_list().clear();
    hub::HubQuickMenu::create_bind(menu.into(), false);
    BasicMenu_Result::se_cursor()
}