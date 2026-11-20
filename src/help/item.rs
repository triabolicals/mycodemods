use engage::app::{GodUnit, HelpParamSetter, IHelpParamSetter, IItemDataMethods, ISkillArray, ISkillDataMethods, IUnit, IUnitItemMethods, ItemData, SkillData_Categorys, Unit, UnitItem};
use engage::{List_1Ext, Mess};
use engage::tm_pro::ITMP_TextMethods;
use unity::{Cast, OptionalMethod};
use crate::utils::to_opt;

#[unity::hook("App","HelpParamSetter", "SetItemData")]
pub fn help_param_setter_set_item_data_hook(this: HelpParamSetter, frame: u64, data: ItemData, unit: Unit, god: GodUnit, ring: u64, endurance: i32, item: UnitItem, is_use_enchant: bool, method_info: OptionalMethod){
    call_original!(this, frame, data, unit, god, ring, endurance, item, is_use_enchant, method_info);
    if data.is_null() || !data.is_weapon() { return; }
    if let Some(unit_item) = to_opt(item){
        let power = unit_item.get_power();
        let original_power = data.get_power() as i32;
        let weight = item.get_weight();
        let mut power_string = Mess::get("MID_SYS_Mt").to_rust_string();
        power_string += ": ";
        power_string += power.to_string().as_str();
        if power != original_power { power_string += format!(" ({:+})", power - original_power).as_str(); }
        if weight != 0 { power_string += format!(" / {}: {}", Mess::get("MID_SYS_Weight"), weight).as_str(); }
        let eff = check_effectiveness_unit(unit, item);
        if eff > 1 {
            let atk_type = this.m_title_atk().get_text().to_string();
            let atk_value = this.m_value_atk().get_text().to_string();
            if let Ok(atk_value) = atk_value.parse::<i32>(){
                let mut count = 0;
                let mut eff_atk_str = "".to_string();
                for x in 2..6 {
                    if eff & (1 << x) != 0 {
                        let eff_atk = atk_value + (x - 1) * power;
                        if count == 0 { eff_atk_str = format!("{} ({}x)", eff_atk, x); }
                        else { eff_atk_str = format!("{} / {} ({}x)", eff_atk_str, eff_atk, x); }
                        count += 1;
                    }
                }
                power_string += format!("\n{} {}: {}\n", Mess::get("MID_SYS_Eff"), atk_type, eff_atk_str).as_str();
            }
        }
        else {
            power_string += "\n";
        }
        power_string += this.m_contents_text().get_text().to_rust_string().as_str();
        this.m_contents_text().set_text_2(power_string, true);
    }
}
pub fn check_effectiveness_unit(unit: Unit, item: UnitItem) -> i32 {
    let mut result = 0;
    if let Some(item_equip_skills) = to_opt(item.get_equip_skills()) {
        item_equip_skills.m_list().iter().map(|mut e| e.get_skill())
            .for_each(|skill| {
                let eff = skill.get_efficacy_value();
                if eff > 1 { result |= 1 << eff; }
            });
    }
    unit.m_mask_skill().m_list().iter()
        .filter(|mut e|{
            let cat = e.clone().get_category();
            cat == SkillData_Categorys::equip() || cat == SkillData_Categorys::item()
        })
        .filter_map(|mut e| to_opt(e.get_skill()).filter(|s| s.get_efficacy_value() > 1 && s.get_efficacy_value() < 128))
        .for_each(|s| {
            let eff = s.get_efficacy_value();
            if eff > 1 { result |= 1 << eff; }
        });
    result
}