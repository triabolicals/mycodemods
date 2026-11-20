use super::*;
use engage::{
    app::hubcookingsequence::*,
};
use engage::app::BasicMenu_Result;
use crate::menus::hub::{HubQuickMenu, HubQuickMenuKind};

pub fn create_cook_bind(this: ProcInst) -> BasicMenu_Result {
    HubCookingSequence::create_bind(this);
    if let Some(child) = to_opt(this.get_child()).and_then(|c| c.try_cast::<HubCookingSequence>()){
        let descs = child.m_descs();
        for x in 14..27 { descs.set(x, Proc::call_2(ProcVoidMethod::from_fn(child.into(), proc_null).unwrap())); }
        for x in 28..42 { descs.set(x, Proc::call_2(ProcVoidMethod::from_fn(child.into(), proc_null).unwrap())); }
        [43, 44, 48].into_iter().for_each(|i|{ descs.set(i, Proc::call_2(ProcVoidMethod::from_fn(child.into(), proc_null).unwrap())); });
        descs.set(58, Proc::call_2(ProcVoidMethod::from_fn(child, hub_cooking_exit).unwrap()));
        BasicMenu_Result::se_cursor()
    }
    else { BasicMenu_Result::pass() }
}
extern "C" fn hub_cooking_exit(this: HubCookingSequence, optional_method: OptionalMethod) {
    this.exit();
    open_menu(this.into(), optional_method);
    if HubQuickMenuKind::Cafe.disable() {
        if let Some(hub_menu) = this.get_super().try_cast::<HubQuickMenu>() {
            hub_menu.update_menu_item(HubQuickMenuKind::Cafe, true);
        }
    }
}