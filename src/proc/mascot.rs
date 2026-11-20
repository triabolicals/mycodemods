use engage::{
    app::{HubMascotSequence, HubVariable_Mascot, ISingletonProcInst_1Methods, IFoodstuffDataMethods, IItemDataMethods, IMascotFoodDataMethods, IMascotFoodEatSequence, IMascotFriendlyContentMethods, IMascotMenuSequence, IMascotMenuSequenceMethods, IStructData_1Methods, ItemData, MascotFoodData, MascotFoodEatSequence, MascotMenuSequence, ProcDesc, BasicMenu_Result, MascotTopMenu_CustomMenuItem},
    ProcVoidMethodExt
};
use unity::{Array, Il2CppString, IlNull, OptionalMethod};
use crate::menus::hub::{HubQuickMenu, HubQuickMenuKind};
use crate::utils::to_opt;
use super::*;

pub fn mascot_create_bind(proc: ProcInst) -> BasicMenu_Result {
    let mascot = HubMascotSequence::new();
    let descs = create_mascot_descs(mascot);
    mascot.create_bind(proc, descs, Il2CppString::null());
    BasicMenu_Result::se_cursor()
}

pub fn create_mascot_descs(proc: HubMascotSequence) -> Array<ProcDesc> {
    let descs = [
        Proc::label(0),
        Proc::call_2(ProcVoidMethod::from_fn(proc.into(), check_mascot_found).unwrap()),
        Proc::label(3),
        Proc::call_2(ProcVoidMethod::new(proc.into(), HubMascotSequence::first_demo_name1_method_info().into())),
        Proc::call_2(ProcVoidMethod::new(proc.into(), HubMascotSequence::first_demo_name2_method_info().into())),
        Proc::label(4),
        Proc::call_2(ProcVoidMethod::from_fn(proc.into(), mascot_menu_bind).unwrap()),
        Proc::call_2(ProcVoidMethod::from_fn(proc, restore_menu).unwrap()),
        Proc::end(),
    ];
    Array::<ProcDesc>::from(descs)
}
extern "C" fn restore_menu(proc: HubMascotSequence, _: OptionalMethod){
    if HubVariable_Mascot::is_done_strok() && HubVariable_Mascot::is_done_eat_food() {
        if let Some(hub_menu) = proc.get_super().try_cast::<HubQuickMenu>() {
            hub_menu.update_menu_item(HubQuickMenuKind::Mascot, true);
        }
    }
    open_menu(proc.into(), None)

}
extern "C" fn create_top_menu(proc: MascotMenuSequence, _: OptionalMethod) {
    proc.create_top_menu();
    if let Some(menu) = proc.get_child().try_cast::<BasicMenu>(){
        remove_menu_item_from_menu::<MascotTopMenu_CustomMenuItem>(menu);
    }
}
extern "C" fn check_mascot_found(proc: HubMascotSequence, _optional_method: OptionalMethod) {
    if HubVariable_Mascot::is_found() { proc.jump(4); }
    else {
        HubVariable_Mascot::found();
        proc.jump(3);
    }
}

extern "C" fn mascot_menu_bind(proc: HubMascotSequence, _optional_method: OptionalMethod) {
    let mascot = MascotMenuSequence::new();
    let descs = mascot.create_desc();
    descs.set(6, Proc::call_2(ProcVoidMethod::from_fn(mascot, create_top_menu).unwrap()));
    descs.set(16, Proc::call_2(ProcVoidMethod::from_fn(mascot, mascot_create_food_eat_bind).unwrap()));
    descs.set(31, Proc::call_2(ProcVoidMethod::from_fn(mascot, add_strok_pts).unwrap()));
    for i in 19..31 { descs.set(i, Proc::call_2(ProcVoidMethod::from_fn(mascot.into(), proc_null).unwrap())); }
    mascot.create_bind(proc, descs, "MascotMenuSequence");
}
pub extern "C" fn add_strok_pts(this: MascotMenuSequence, _method_info: OptionalMethod) {
    if !HubVariable_Mascot::is_done_strok() {
        HubVariable_Mascot::add_point(5);
        this.m_friendly_gauge().try_popup();
    }
}
extern "C" fn mascot_done_eating_food(_this: MascotFoodEatSequence, _method_info: OptionalMethod) { HubVariable_Mascot::done_eat_food(); }
extern "C" fn mascot_eat_food(this: MascotFoodEatSequence, _method_info: OptionalMethod) {
    if let Some(food_data) = to_opt(this.m_food_stuff_data()){
        if let Some(item) = to_opt(ItemData::get(food_data.get_iid())){
            item.add_inventory(-1);
            if let Some(mascot_food) = to_opt(MascotFoodData::get(food_data.get_iid())){
                HubVariable_Mascot::add_point(mascot_food.get_value());
                MascotMenuSequence::get_instance().m_friendly_gauge().try_popup();
            }
        }
    }
}
pub extern "C" fn mascot_create_food_eat_bind(this: MascotMenuSequence, _method_info: OptionalMethod) {
    if let Some(food_data) = to_opt(this.get_foodstuff()) {
        let mascot_eat = MascotFoodEatSequence::new(food_data);
        let descs = create_mascot_eat_desc(mascot_eat);
        let descs_array = Array::<ProcDesc>::from(descs);
        mascot_eat.create_bind(this, descs_array, "MascotEatFoodSequence");
    }
}

fn create_mascot_eat_desc(this: MascotFoodEatSequence) -> Vec<ProcDesc> {
    vec![
        Proc::call_2(ProcVoidMethod::new(this.into(), MascotFoodEatSequence::get_bond_method_info().into())),
        Proc::call_2(ProcVoidMethod::from_fn(this.into(), mascot_eat_food).unwrap()),
        Proc::r#yield(),
        Proc::call_2(ProcVoidMethod::from_fn(this.into(), mascot_done_eating_food).unwrap()),
        Proc::end()
    ]
}