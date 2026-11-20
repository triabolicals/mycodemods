use engage::app::{ISingletonProcInst_1Methods, BasicMenuItem, BasicMenu_Result, IBitField32Methods, MapSequenceHuman, BasicMenuItem_Attribute};
use unity::Il2CppString;
use crate::utils::override_vtable;
use super::*;

pub fn temp_save_vtable_edit() {
    let class = "MapSystemMenu.TemporarySaveItem";
    override_vtable("App", class, "ACall", temp_save_a_call as _);
    override_vtable("App", class, "HelpText", map_system_temp_save_get_help_text as _);
    override_vtable("App", class, "GetName", map_system_temp_save_menu_name as _);
    override_vtable("App", class, "MapAttribute", map_system_temp_save_build_attr as _);
}

pub fn map_sequence_human_edit(this: ProcInst) {
    let descs = this.m_descs();
    descs.set(0xd0, Proc::jump(47));
    descs.set(0xcb, Proc::call_2(ProcVoidMethod::from_fn(this, remove_temporary_game_status).unwrap()));
    descs.set(0xd0, Proc::call_2(ProcVoidMethod::from_fn(this, remove_temporary_game_status).unwrap()));
}
extern "C" fn remove_temporary_game_status(_proc: ProcInst, _method_info: OptionalMethod) {
    let status = GameUserData::get_instance().get_status();
    IBitField32Methods::set_or_clear(status, false, 0x200);
}
pub fn temp_save_a_call(this: BasicMenuItem, method_info: OptionalMethod) -> BasicMenu_Result {
    if GameUserData::get_instance().get_sequence().value == 3 {
        if let Some(map_sequence_human) = to_opt(MapSequenceHuman::get_instance()){
            map_sequence_human.jump(47);
            map_sequence_human_edit(map_sequence_human.into());
            return BasicMenu_Result::close_parent_decide()
        }
    }
    unsafe { temp_save_original_a_call(this, method_info) }
}
#[skyline::from_offset(0x01e40f80)] fn temp_save_original_a_call(_this: BasicMenuItem, optional_method: OptionalMethod) -> BasicMenu_Result;
fn map_system_temp_save_menu_name(_temp_save_menu_item: BasicMenuItem, _method_info: OptionalMethod) -> Il2CppString { Mess::get("MID_SORTIE_SAVE") }
fn map_system_temp_save_build_attr(_temp_save_menu_item: BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItem_Attribute {
    if GameVariableManager::get_bool("G_Ironman") { BasicMenuItem_Attribute::hide() }
    else { BasicMenuItem_Attribute::enable() }
}
fn map_system_temp_save_get_help_text(_temp_save_menu_item: BasicMenuItem, _method_info: OptionalMethod) -> Il2CppString { Mess::get("MID_SORTIE_SAVE_HELP") }
