use engage::app::ScriptSystem;
use engage::moon_sharp::interpreter::DynValue;
use unity::{Array, Il2CppString};
use super::*;

#[unity::hook("App", "Talk", "StartBind")]
pub fn talk_start_bind(this: ProcInst, mid: Il2CppString, is_continuous: bool, optional_method: OptionalMethod) {
    if crate::menus::config::menuitem::Talk.get_value() != 2 { call_original!(this, mid, is_continuous, optional_method); }
}
pub extern "C" fn script_talk(this: Array<DynValue>, _optional_method: OptionalMethod) { 
    if crate::menus::config::menuitem::Talk.get_value() != 1 { ScriptSystem::talk(this); }
}
