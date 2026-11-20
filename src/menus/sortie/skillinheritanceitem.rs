use engage::app::{BasicMenuItem_Attribute, GameUserData, IBasicMenu, IBasicMenuItem, IGameUserDataMethods, IGodUnitMethods, ISingletonClass_1Methods, ISkillInheritanceMenuContent, ISkillInheritanceMenuItemMethods, ITitleBarMethods, IUnitMethods, Mess_IconCategory, SkillInheritanceMenu, SkillInheritanceMenuContent, SkillInheritanceMenuItem, TitleBar_FooterType};
use engage::tm_pro::ITMP_TextMethods;
use unity::{Cast, OptionalMethod};
use crate::menus::sortie::get_bond_frag_required;

pub fn skill_inheritance_on_select(this: SkillInheritanceMenuItem, _optional_method: OptionalMethod) {
    unsafe { SkillInheritanceMenuItem::on_select(this) };
    let title_bar = engage::app::TitleBar::get_instance();
    title_bar.close_footer();
    if !this.is_inherited() && !this.is_enough_level() && this.is_enough_sp() {
        if let Some(content) = this.m_menu().m_menu_content().try_cast::<SkillInheritanceMenuContent>() {
            let god_unit = this.get_god();
            let unit = this.get_unit();
            let level = this.get_skill_level();
            if this.get_skill_level() < god_unit.get_max_level(unit) {
                let bond_frag_needed = get_bond_frag_required(god_unit, unit, level);
                if bond_frag_needed < GameUserData::get_instance().get_piece_of_bond() && bond_frag_needed > 0 {
                    title_bar.open_footer(TitleBar_FooterType::piece_of_bond());
                    let text1 = content.m_text_help_ng().get_text().to_rust_string();
                    let text =
                        if let Some(v) = text1.split_once('\n'){ format!("{} ({}{})\n{}", v.0, engage::app::Mess::create_sprite_tag(Mess_IconCategory::system(), "Bonds"), bond_frag_needed, v.1) }
                        else { format!("{} ({}{})", text1,  engage::app::Mess::create_sprite_tag(Mess_IconCategory::system(), "Bonds"), bond_frag_needed) };
                    content.m_text_help_ng().set_text_2(text, true);
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