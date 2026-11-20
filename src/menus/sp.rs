use engage::{
    app::{
        basicdialog::*,
        basicdialogitem::*,
        itemdata::*,
        pad::*,
        unit::*,
        transporter::*,
        BasicDialogContent,
        BasicMenuItem,
        BasicMenu_Result,
        IBasicMenu,
        IBasicMenuItemContentMethods,
        IBasicMenuItemMethods,
        IBasicMenuMethods,
        IProcInstMethods,
        IStructData_1Methods,
        IUnitGrowSequenceMethods,
        ProcInst,
        UnitGrowSequence
    },
    List_1Ext,
    Mess,
    nn::hid::NpadButton,
    prelude::List_1,
    system::collections::generic::IList_1Methods
};
use engage::app::{BasicMenu, IBasicMenuItem, ISkillInheritanceMenuContentMethods, ISkillInheritanceMenuMethods, SkillInheritanceMenu, SkillInheritanceMenuContent, SkillInheritanceMenuItem, SkillInheritanceSequence};
use unity::{Cast, FromIlInstance, Il2CppString, OptionalMethod};
use crate::utils::override_vtable;

#[unity::inject(namespace="MyCodeMods", name="SPDialogMenu", parent=BasicDialog)]
pub struct SPDialogMenu{ pub unit: Unit, }

#[unity::inject(namespace="MyCodeMods", name="SPDialogMenuItem", parent=BasicDialogItem)]
pub struct SPDialogMenuItem {
    pub sp_item: ItemData,
    pub count: i32,
    pub transporter_count: i32,
}
#[unity::injected_methods]
impl SPDialogMenuItem {
    #[override_virtual(name="GetName")]
    pub fn get_name(self) -> Il2CppString {
        format!("{} {} ({}/{})",
                engage::app::mess::Mess::create_item_sprite_tag(self.sp_item(), true),
                Mess::get(self.sp_item().get_name()),
                self.count(),
                self.transporter_count()
        ).into()
    }
}
#[unity::injected_methods]
impl SPDialogMenu {
    #[override_virtual(name="CustomCall")]
    pub fn custom_call(self) -> BasicMenu_Result {
        let left = Pad::is_trigger(NpadButton::left());
        let right = Pad::is_trigger(NpadButton::right());
        if Pad::is_trigger(NpadButton::left()) != Pad::is_trigger(NpadButton::right()) {
            if let Some(sp_item) = self.get_menu_item(self.get_select_index()).try_cast::<SPDialogMenuItem>(){
                let init_sp = self.unit().get_skill_point();
                let current_count = sp_item.count();
                let max = sp_item.transporter_count();
                let sp_added = self.get_total_added();
                let total_sp = init_sp + sp_added;
                if (total_sp >= 9999 && right) || (current_count == 0 && left) || ((current_count == max) && right){ return BasicMenu_Result::se_miss(); }
                if right && (current_count < max) { sp_item.set_count(current_count + 1); }
                else if left && (current_count > 0) { sp_item.set_count(current_count - 1); }
                else { return BasicMenu_Result::se_miss(); }
                sp_item.get_menu_item_content().build_text();
                self.update_amount_text();
                return BasicMenu_Result::se_cursor();
            }
        }
        BasicMenu_Result::pass()
    }
    #[override_virtual(name="ACall")]
    pub fn a_call(self) -> BasicMenu_Result {
        if self.get_menu_item(self.get_select_index()).try_cast::<SPDialogMenuItem>().is_none() {
            if self.get_total_added() > 0 {
                let grow = UnitGrowSequence::create_bind(self);
                self.consume_items();
                grow.set_unit_grow_data_3(self.unit(), 0, self.get_total_added(), false);
                BasicMenu_Result::close_decide()
            }
            else { BasicMenu_Result::close_cancel() }
        }
        else { BasicMenu_Result::pass() }
    }
    #[override_virtual(name="OnDispose")]
    pub fn on_dispose(self) {
        if let Some(skill_inheritance) = self.get_super().try_cast::<SkillInheritanceMenu>() {
            if let Some(content) = skill_inheritance.m_menu_content().try_cast::<SkillInheritanceMenuContent>() {
                content.set_active(true);
            }
            skill_inheritance.update_skill_point();
            skill_inheritance.m_full_menu_item_list().iter().for_each(|item| {
                if let Some(skill_inherit) = item.try_cast::<SkillInheritanceMenuItem>() {
                    skill_inherit.set_attribute(skill_inherit.build_attribute());
                    let content = skill_inherit.get_menu_item_content();
                    if !content.is_null() { content.build(skill_inherit); }
                }
            })
        }
        if let Some(menu) = self.get_super().try_cast::<BasicMenu>() { IBasicMenuMethods::open_anime(menu); }
        unsafe { BasicMenu::on_dispose(self); }
    }
}
impl SPDialogMenu {
    pub fn inject() {
        if cobapi::injection::register::<SPDialogMenuItem>().is_ok() { println!("SPDialogMenuItem is Injected"); }
        else { return; }
        if cobapi::injection::register::<SPDialogMenu>().is_ok() { println!("SPDialogMenuItem is Injected"); }
        else { return; }
        override_vtable("App", "SkillInheritanceMenuItem", "PlusCall", skill_inherit_plus_call as _);
    }
    pub fn consume_items(self) {
        self.m_full_menu_item_list().iter()
            .flat_map(|i| i.try_cast::<SPDialogMenuItem>())
            .for_each(|i|{ for _ in 0..i.count() { Transporter::delete_item(i.sp_item()); } });
    }
    pub fn create_dialog_bind(proc: impl Into<ProcInst>, unit: Unit) -> bool {
        let sp_items: Vec<_> =
            ItemData::get_list()
                .iter()
                .filter(|i| i.get_gain_skill_point() >= 100 && Transporter::get_item_count(*i) > 0)
                .collect();
        
        if sp_items.is_empty() { return false; }
        let menu_items = List_1::<BasicMenuItem>::new();
        sp_items.into_iter().for_each(|sp_item| {
            let count = Transporter::get_item_count(sp_item);
            let i = SPDialogMenuItem::instantiate().unwrap();
            IBasicDialogItemMethods::ctor(i, format!("{} {}", engage::app::mess::Mess::create_item_sprite_tag(sp_item, true), Mess::get(sp_item.get_name())));
            i.set_transporter_count(count);
            i.set_sp_item(sp_item);
            i.set_count(0);
            menu_items.add(i.into());
        });
        let confirm_item = BasicDialogItem::new(Mess::get("MID_SysDialog_Move_Cancel"));
        menu_items.add(confirm_item.into());
        let dialog = SPDialogMenu::instantiate().unwrap();
        IBasicDialogMethods::ctor(dialog, menu_items, BasicDialogContent::create());
        dialog.set_unit(unit);
        let title = format!("Add SP for {}. Current: {}", unit.get_name(), unit.get_skill_point());
        dialog.set_text(title);
        let descs = IBasicDialogMethods::create_default_desc(dialog);
        dialog.create_bind(proc, descs, "SPUseDialog");
        dialog.bind_parent_menu();
        true
    }
    pub fn get_total_added(self) -> i32 {
        self.m_full_menu_item_list().iter()
            .flat_map(|i| i.try_cast::<SPDialogMenuItem>())
            .map(|i| i.count() * ( i.sp_item().get_gain_skill_point() ))
            .sum::<i32>()
    }
    pub fn update_amount_text(self){
        let amount = self.get_total_added();
        if let Some(confirm) = self.m_full_menu_item_list().iter()
            .find(|i| i.try_cast::<SPDialogMenuItem>().is_none())
            .and_then(|i| i.try_cast::<BasicDialogItem>())
        {
            if amount == 0 { confirm.set_m_text(Mess::get("MID_SysDialog_Move_Cancel")); }
            else {
                let a = if amount >= 9999 { 9999 - self.unit().get_skill_point() } else { amount };
                confirm.set_m_text(format!("{} {} {}", Mess::get("MID_KEYHELP_MENU_DECIDE"), a,Mess::get("MID_SYS_SP")).into());
            }
            confirm.get_menu_item_content().build_text();
        }
    }
}
fn skill_inherit_plus_call(this: BasicMenuItem, optional_method: OptionalMethod) -> BasicMenu_Result {
    if SPDialogMenu::create_dialog_bind(this.m_menu(), SkillInheritanceMenu::get_m_unit()) {
        if let Some(content) = this.m_menu().m_menu_content().try_cast::<SkillInheritanceMenuContent>() {
            content.set_active(false);
        }
        BasicMenu_Result::se_decide()
    }
    else { BasicMenu_Result::se_miss() }
}