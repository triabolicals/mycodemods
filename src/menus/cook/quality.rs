use engage::{
    app::{
        basicdialogitem::*,
        BasicDialog, BasicMenu_Result,
        CookData, HubUtil,
        IBasicDialogMethods, IBasicMenuItem, ICookDataMethods,
        IStructBase, IStructData_1Methods, ITasteDataMethods,
        TasteData,
        cooking_menu::UnitSelectMenu_UnitSelectMenuItem,
    },
    GameVariableManager,
    Mess,
    prelude::List_1,
    system::collections::generic::IList_1Methods
};
use engage::app::{BasicMenuItem, Unit};
use unity::{Cast, FromIlInstance, Il2CppString, OptionalMethod};
use crate::utils::{override_vtable, to_opt};

pub fn taste_selection_install() {
    if cobapi::injection::register::<TasteSelectionDialogMenuItem>().is_ok()  { println!("TasteSelectionDialogMenuItem is Injected"); }
    override_vtable("App.CookingMenu", "UnitSelectMenu.UnitSelectMenuItem", "PlusCall", cooking_menu_unit_select_menu_unit_select_menu_item_plus_call as _);
}
#[unity::hook("App", "Cooking", "DetermineTaste")]
pub fn determine_taste(unit: Unit, quality: i32, method_info: OptionalMethod) -> TasteData {
    let taste = GameVariableManager::get_number("G_Taste");
    if taste < 0 || taste >= TasteData::get_count() { call_original!(unit, quality, method_info) }
    else if let Some(taste) = to_opt(TasteData::get_2(taste)) { taste }
    else { call_original!(unit, quality, method_info) }
}

#[unity::inject(namespace="MyCodeMods", name="TasteSelectionDialogMenuItem", parent=BasicDialogItem)]
pub struct TasteSelectionDialogMenuItem { pub taste_index: i32, }

#[unity::injected_methods]
impl TasteSelectionDialogMenuItem {
    #[override_virtual(name="ACall")]
    pub fn a_call(self) -> BasicMenu_Result {
        if to_opt(TasteData::get_2(self.taste_index())).is_some() { GameVariableManager::set_number("G_Taste", self.taste_index()); }
        else { GameVariableManager::set_number("G_Taste", -1); }
        BasicMenu_Result::close_decide()
    }
}

fn cooking_menu_unit_select_menu_unit_select_menu_item_plus_call(this: UnitSelectMenu_UnitSelectMenuItem, _: OptionalMethod) -> BasicMenu_Result {
    let cook_data = CookData::get(HubUtil::get_current_cooking_pid());
    if cook_data.is_null() { return BasicMenu_Result::se_miss(); }
    Mess::load("Cook");
    let mut menu_list = vec![];
    for i in 0..3 {
        let tid = unity::field_get_value_at_offset::<Il2CppString>(cook_data, 0x28+0x8*i);
        if !tid.is_null() {
            if let Some(taste) = to_opt(TasteData::get(tid)){
                menu_list.push( (taste.get_name().to_rust_string(), taste.index()));
                if !taste.get_derived_tid().is_null() {
                    if let Some(derive_taste) = to_opt(TasteData::get(taste.get_derived_tid())){
                        if menu_list.iter().find(|x| x.1 == derive_taste.index()).is_none() {
                            menu_list.push( (derive_taste.get_name().to_rust_string(), derive_taste.index()));
                        }
                    }
                }
            }
        }
    }
    if menu_list.len() > 0 {
        let list = List_1::<TasteSelectionDialogMenuItem>::new();
        menu_list.into_iter().for_each(|(i, idx)|{
            let item = TasteSelectionDialogMenuItem::instantiate().unwrap();
            IBasicDialogItemMethods::ctor(item, Mess::get(i));
            item.set_taste_index(idx);
            list.add(item);
        });
        let item = TasteSelectionDialogMenuItem::instantiate().unwrap();
        IBasicDialogItemMethods::ctor(item, Mess::get("MID_SYS_None"));
        item.set_taste_index(-1);
        list.add(item);
        let mut title = format!("Select Taste ({})", engage::app::mess::Mess::get_game_data_name(cook_data.get_pid()));
        if let Some(current_taste) = to_opt(TasteData::get_2(GameVariableManager::get_number("G_Taste"))).map(|v| Mess::get(v.get_name())) {
            title.push_str("\nCurrent: ");
            title.push_str(current_taste.to_string().as_str());
        }
        let dialog = unsafe { BasicDialog::create_basic_dialog_bind(this.m_menu(), list.cast::<List_1::<BasicMenuItem>>()) };
        dialog.set_text(title);
        BasicMenu_Result::se_cursor()
    }
    else { BasicMenu_Result::pass() }
}

