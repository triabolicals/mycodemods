use engage::{
    unity_engine::*,
    app::{unit::*, unitstatussetter::*, unitinfoparamsetter::*},
    Mess,
    app::{IJobDataMethods, Ut},
    tm_pro::{ITMP_TextMethods, TextMeshProUGUI},
    unity_engine::{recttransform::*, component::*, transform::*}
};
use unity::{Cast, OptionalMethod};
use crate::utils::to_opt;

#[skyline::hook(offset = 0x1f9d320)]
pub fn unit_info_set_level_hook(this: UnitInfoParamSetter, unit: Unit, x: i32, z: i32, selected_god: bool, god: u64, method_info: OptionalMethod){
    call_original!(this, unit, x, z, selected_god, god, method_info);
    if !crate::menus::config::menuitem::InternalLevel.get_flag_value() { return; }
    // if GameVariableManager::get_number(INTERNAL) & 1 == 0 { return; }
    if let Some(unit) = to_opt(unit) {
        let displayed_level = unit.get_enhanced_level();
        if unit.get_internal_level() > 0 {
            this.m_level().set_text_2((displayed_level + unit.get_internal_level()).to_string(), true);
        }
    }
}
#[skyline::hook(offset = 0x1c66980)]
pub fn set_total_level(this: UnitStatusSetter, unit: Unit, unit_no_enhance: Unit, method_info: OptionalMethod){
    call_original!(this, unit, unit_no_enhance, method_info);
    let level_text_obj = this.m_lv().m_value();
    if !crate::menus::config::menuitem::InternalLevel.get_flag_value(){
        adjust_level_object(level_text_obj, false);
        return;
    }
    let enhance_level = unit.get_enhanced_level();
    let not_enhance_level = unit_no_enhance.get_enhanced_level();
    let unit_level = unit_no_enhance.get_level();
    let max_level = unit_no_enhance.get_job().get_max_level() as i32;
    let boost: i32 = (not_enhance_level < enhance_level) as i32;
    let at_limit: bool = max_level <= unit_level;
    let displayed_level = enhance_level;
    let internal_level = unit_no_enhance.get_internal_level();
    if internal_level > 0 {
        adjust_level_object(level_text_obj, true);
        let level_str = format!("{}/<size=-5>{}</size>", displayed_level, internal_level);
        this.m_lv().set_value_direct(level_str, boost, at_limit);
    }
    else { adjust_level_object(level_text_obj, false); }
}
pub fn try_get_parent(transform: Transform, parent_name: &str) -> Option<Transform> {
    let mut current = transform.get_parent();
    loop {
        if current.is_null() { return None; }
        let name = current.get_name().to_rust_string();
        if name == parent_name { return Some(current); }
        current = current.get_parent();
    }
}

pub fn adjust_level_object(value: TextMeshProUGUI, expand: bool) {
    let transform: Transform = IComponentMethods::get_transform(value);
    let parent = transform.get_parent();
    if let Some(title) = to_opt(Ut::find_child_component_2::<TextMeshProUGUI>(parent.get_game_object(), "Title")){
        let rect = title.get_rect_transform();
        let v_rect = value.get_rect_transform();
        if expand {
            title.set_text_2("Lv", true);
            rect.set_size_delta(Vector2{x: 30.0, y: 30.0});
            v_rect.set_size_delta(Vector2{x: 45.0, y: 36.0});
            v_rect.set_anchored_position(Vector2{x: 45.0, y: 0.0});
            title.set_font_size_max(20.0);
            title.set_font_size(20.0);
            title.set_font_size_min(18.0);
            value.set_font_size(26.0);
            value.set_font_size_max(26.0);
            value.set_font_size_min(22.0);
        }
        else {
            title.set_text_2(Mess::get("MID_SYS_LV"), true);
            rect.set_size_delta(Vector2{x: 40.0, y: 40.0});
            v_rect.set_size_delta(Vector2{x: 36.0, y: 36.0});
            v_rect.set_anchored_position(Vector2{x: 55.0, y: 0.0});
            title.set_font_size_max(24.0);
            title.set_font_size(24.0);
            title.set_font_size_min(22.0);
            value.set_font_size(28.0);
            value.set_font_size_max(28.0);
            value.set_font_size_min(24.0);
        }
    }
}