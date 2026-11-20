use engage::app::{BasicMenuItem, BasicMenuItem_Attribute, BasicMenu_Result, HubVariable_Mascot, IBasicMenuItemMethods};
use unity::OptionalMethod;
use crate::utils::override_vtable;

pub fn mascot_vtable_edit() {
    override_vtable("App", "MascotTopMenu.StrokMenuItem", "ACall", mascot_strok_a_call as _);
    override_vtable("App", "MascotTopMenu.StrokMenuItem", "BuildAttribute", build_attribute_mascot_strok as _);
    override_vtable("App", "MascotTopMenu.MealMenuItem", "ACall",  mascot_food_a_call as _);
    override_vtable("App", "MascotTopMenu.MealMenuItem", "BuildAttribute", build_attribute_mascot_food as _);
}

fn build_attribute_mascot_food(_: BasicMenuItem, _: OptionalMethod) -> BasicMenuItem_Attribute {
    if HubVariable_Mascot::is_done_eat_food() { BasicMenuItem_Attribute::disable() }
    else { BasicMenuItem_Attribute::enable() }
}

fn build_attribute_mascot_strok(_: BasicMenuItem, _: OptionalMethod) -> BasicMenuItem_Attribute {
    if HubVariable_Mascot::is_done_strok() { BasicMenuItem_Attribute::disable() }
    else { BasicMenuItem_Attribute::enable() }
}

fn mascot_food_a_call(item: BasicMenuItem, _: OptionalMethod) -> BasicMenu_Result {
    if !item.is_attribute_disable() { unsafe { original_food_eat_acall(item, None) } }
    else { BasicMenu_Result::se_miss() }
}

fn mascot_strok_a_call(item: BasicMenuItem, _: OptionalMethod) -> BasicMenu_Result {
    if !item.is_attribute_disable() { unsafe { original_strok_a_call(item, None) } }
    else { BasicMenu_Result::se_miss() }
}

#[skyline::from_offset(0x0208d2a0)] fn original_food_eat_acall(item: BasicMenuItem, _method_info: OptionalMethod) -> BasicMenu_Result;
#[skyline::from_offset(0x0208d3b0)] fn original_strok_a_call(_item: BasicMenuItem, _method_info: OptionalMethod) -> BasicMenu_Result;