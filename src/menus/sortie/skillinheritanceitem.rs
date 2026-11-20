use engage::{List_1Ext, app::{
    godleveldata::*,
    ISingletonScriptableObject_1Methods,
    shopbuyyesnodialog::*,
    AchieveData, BasicMenuItem_Attribute, BasicMenu_Result, GameUserData, GodUnit, IBasicMenu, IBasicMenuItem, IGameUserDataMethods, IGodUnitMethods, ISingletonClass_1Methods, ISkillInheritanceMenuContent, ISkillInheritanceMenuItemMethods, IStructData_1Methods, ITitleBarMethods, IUnitMethods, Mess_IconCategory, ShopBuyYesNoDialog_YesEventHandler, ShopUnitSelectMenu_DecideEventHandler, SkillInheritanceMenu, SkillInheritanceMenuContent, SkillInheritanceMenuItem, TitleBar_FooterType, Unit
}, Mess};
use engage::app::{GameColor, ISkillInheritanceMenuItemContent, SkillInheritanceMenuItemContent, Ut};
use engage::tm_pro::{ITMP_Text, ITMP_TextMethods};
use unity::{Cast, OptionalMethod};
use crate::utils::override_vtable;

pub fn get_user_data() -> GameUserData { GameUserData::get_instance() }

pub fn skill_inheritance_vtable_edit() {
    override_vtable("App", "SkillInheritanceMenuItem", "OnSelect", skill_inheritance_on_select as _);
    override_vtable("App", "SkillInheritanceMenuItem", "BuildAttribute", skill_inheritance_build_attribute as _);
    override_vtable("App", "SkillInheritanceMenuItem", "ACall", skill_inheritance_menu_item_a_call as _);
}
fn skill_inheritance_menu_item_a_call(this: SkillInheritanceMenuItem, _optional_method: OptionalMethod) -> BasicMenu_Result {
    if skill_inheritance_build_attribute(this, None) == BasicMenuItem_Attribute::enable() {
        if !this.is_enough_level() {
            let god_unit = this.get_god();
            let unit = this.get_unit();
            let level = this.get_skill_level();
            if level < god_unit.get_max_level(unit) {
                let bond_frag_needed = get_bond_frag_required(god_unit, unit, level );
                if bond_frag_needed < get_user_data().get_piece_of_bond() && bond_frag_needed > 0 {
                    let yes = ShopBuyYesNoDialog_YesEventHandler::new(this.into(), increase_bond_and_inherit_method_info().into());
                    ShopBuyYesNoDialog::create_bind_6(this.m_menu(), this.get_skill(), this.get_skill_cost(), yes);
                    return BasicMenu_Result::se_decide();
                }
            }
        }
    }
    unsafe { SkillInheritanceMenuItem::a_call(this) }
}
#[unity::callback]
extern "C" fn increase_bond_and_inherit(this: SkillInheritanceMenuItem, _optional_method: OptionalMethod) {
    let god_unit = this.get_god();
    let unit = this.get_unit();
    let skill_level = this.get_skill_level();
    let bond_frag_needed = get_bond_frag_required(god_unit, unit, skill_level);
    let level_diff = skill_level - god_unit.get_level(unit);
    let count = if level_diff < 5 { 1 } else { level_diff / 5 };
    let piece_of_bond =  get_user_data().get_piece_of_bond();
    god_unit.set_level(unit, skill_level);
    unit.inherit_aptitude(god_unit);
    for _ in 0..count {
        AchieveData::add_count_god_battle();
        AchieveData::add_count_god_battle_win();
    }
    engage::app::TitleBar::get_instance().update_footer_values();
    get_user_data().set_piece_of_bond(piece_of_bond - bond_frag_needed);
    this.on_inherite();
}
pub fn skill_inheritance_on_select(this: SkillInheritanceMenuItem, _optional_method: OptionalMethod) {
    unsafe { SkillInheritanceMenuItem::on_select(this) };
    if !this.is_inherited() && !this.is_enough_level() {
        if let Some(content) = this.m_menu().m_menu_content().try_cast::<SkillInheritanceMenuContent>() {
            let god_unit = this.get_god();
            let unit = this.get_unit();
            let level = this.get_skill_level();
            if this.get_skill_level() < god_unit.get_max_level(unit) {
                let bond_frag_needed = get_bond_frag_required(god_unit, unit, level);
                let bond = GameUserData::get_instance().get_piece_of_bond();
                if bond_frag_needed > 0 {
                    let text1 = content.m_text_help_ng().get_text().to_rust_string();
                    let mut level_required = format!("{} / {} {}{}",
                         engage::app::Mess::get_2("MID_Hub_Inheritance_LevelHelp", Ut::to_string_with_comma(level)),
                         engage::app::Mess::get("MID_MSG_COST"), engage::app::Mess::create_sprite_tag(Mess_IconCategory::system(), "Bonds"),
                         Ut::to_string_with_comma(bond_frag_needed)
                    );
                    if bond_frag_needed <= bond {
                        level_required = format!("<color=\"green\">{}</color>", level_required);
                    }
                    if !this.is_enough_sp() {
                        level_required += format!("\n{} {} {}", Mess::get("MID_MSG_COST"), Mess::get("MID_SYS_SP"), this.get_skill_cost()).as_str();
                    }
                    else {
                        let inheritance_invalid = engage::app::Mess::get_2("MID_Hub_Inheritance_Invalid", god_unit.get_name()).to_rust_string();
                        if text1.contains(&inheritance_invalid) {
                            level_required += "\n";
                            level_required += inheritance_invalid.as_str();
                        }
                    }

                    content.m_text_help_ng().set_text_2(level_required, true);
                    return;
                }
            }
        }
    }
}
pub fn skill_inheritance_build_attribute(this: SkillInheritanceMenuItem, _optional_method: OptionalMethod) -> BasicMenuItem_Attribute {
    let unit = SkillInheritanceMenu::get_m_unit();
    if this.is_inherited() || unit.get_skill_point() < this.get_skill_cost() { BasicMenuItem_Attribute::disable() }
    else {
        if !this.is_enough_level() {
            let bond_frag_needed = get_bond_frag_required(this.get_god(), unit, this.get_skill_level());
            if bond_frag_needed < GameUserData::get_instance().get_piece_of_bond() && bond_frag_needed > 0 { BasicMenuItem_Attribute::enable() }
            else {BasicMenuItem_Attribute::disable() }
        }
        else { BasicMenuItem_Attribute::enable() }
    }
}
fn get_bond_frag_required(god_unit: GodUnit, unit: Unit, level: i32) -> i32 {
    let mut bond_frag_needed = 0;
    let god_level_data = GodLevelData::get_list();
    let bond_level = god_unit.get_level(unit);
    let diff_exp = god_unit.get_diff_exp(unit);
    if bond_level >= god_level_data.count() || level <= bond_level { return -1; }
    let next_level_exp = god_level_data.get(bond_level+1).get_exp();
    let next_cost_scale = ( next_level_exp as f32 - diff_exp as f32) / (next_level_exp as f32);
    bond_frag_needed += ((god_level_data.get(bond_level+1).get_cost() as f32) * next_cost_scale ) as i32;
    for x in bond_level+2..level+1 {
        if x >= god_level_data.count() { break; }
        let cost = god_level_data.get(x).get_cost();
        bond_frag_needed += cost;
    }
    bond_frag_needed
}