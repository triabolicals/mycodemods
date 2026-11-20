use engage::{
    List_1Ext,
    app::{
        goddata::*,
        basicmenuitem::*, basicmenu::*, procinst::*,
        skillinheritancemenu::*, skillinheritancesequence::*,
        godpool::*, godunit::*, singletonpool_2::*,
        sortieselectionunitmanager::*, sortiesequenceunitselect::*,
        ISingletonClass_1Methods,
        ISingletonProcInst_1Methods,
        Proc, ProcVoidMethod, UnitInfo
    },
    GameVariableManager,
    Mess,
    ProcVoidMethodExt,
    prelude::List_1,
    system::collections::generic::IList_1Methods,
    app::{IUnitMethods}
};
use unity::{Cast, ClassIdentity, Il2CppString, OptionalMethod};
use crate::utils::to_opt;

#[unity::inject(namespace="MyCodeMods", name="SortieSkillInheritItem", parent=BasicMenuItem)]
pub struct SortieSkillInheritanceMenuItem {}

#[unity::injected_methods]
impl SortieSkillInheritanceMenuItem {
    #[override_virtual(name="ACall")]
    pub fn a_call(self) -> BasicMenu_Result {
        let sortie = SortieSequenceUnitSelect::get_instance();
        sortie.close_all();
        sortie.post_closed();
        SkillInheritanceSequence::create_bind(sortie);
        if let Some(skill) = sortie.get_child().try_cast::<SkillInheritanceSequence>(){
            let descs = skill.m_descs();
            descs.set(4,Proc::call_2(ProcVoidMethod::from_fn(skill, skill_inheritance_start_sequence).unwrap()));
            descs.set(6, Proc::jump(2));
            descs.set(13, Proc::jump(3));
            descs.set(15, Proc::call_2(ProcVoidMethod::from_fn(skill, skill_inheritance_end_sequence).unwrap()));
        }
        BasicMenu_Result::close_decide()
    }
    #[override_virtual(name="GetName")]
    pub fn get_name(self) -> Il2CppString { Mess::get("MID_Hub_Skill_Inheritance") }

    #[override_virtual(name="BuildAttribute")]
    pub fn build_attribute(self) -> BasicMenuItem_Attribute {
        if GameVariableManager::get_bool("G_Cleared_M004") {
            let has_emblems = GodPool::get_instance().m_sort().iter()
                .filter(|g_unit| g_unit.m_data().get_force_type().value == 0 && !g_unit.get_is_escaping())
                .count() > 0;
            if has_emblems { BasicMenuItem_Attribute::enable() }
            else { BasicMenuItem_Attribute::hide() }
        }
        else { BasicMenuItem_Attribute::hide() }
    }
}
extern "C" fn skill_inheritance_start_sequence(this: SkillInheritanceSequence, _optional_method: OptionalMethod) {
    if let Some(unit_select) = to_opt(SortieSequenceUnitSelect::get_instance()) {
        unit_select.close_all();
        unit_select.post_closed();
        unit_select.jump(0);
    }
    this.start_sequence();
    this.set_select_unit(SortieSelectionUnitManager::get_instance().m_unit());
    let god_pool_list = GodPool::get_instance().m_sort();
    let list = List_1::<GodUnit>::new();
    god_pool_list.iter()
        .filter(|g_unit| g_unit.m_data().get_force_type().value == 0 && !g_unit.get_is_escaping())
        .for_each(|g_unit| { list.add(g_unit); });
    this.set_select_unit_god_list(list);
}

extern "C" fn skill_inheritance_end_sequence(this: SkillInheritanceSequence, _optional_method: OptionalMethod) {
    this.end_sequence();
    UnitInfo::set_visible(true);
    if let Some(unit_select) = to_opt(SortieSequenceUnitSelect::get_instance()) {
        unit_select.disp_all();
        unit_select.jump(0);
    }
}