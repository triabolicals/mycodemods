use engage::{
    app::{unit::*, unitreliance::*, achievedata::*, unitreliancedata::*, force::* },
    prelude::*,
};
use crate::proc::collect_achievements;
use crate::utils::to_opt;

#[skyline::hook(offset=0x01c5a930)]
pub fn reliance_can_level_up_hook(unit_a: Unit, unit_b: Unit, method_info: OptionalMethod) -> bool {
    let result = call_original!(unit_a, unit_b, method_info);
    if result && crate::menus::config::menuitem::Support.get_flag_value(){
        UnitReliance::level_up_2(unit_a, unit_b);
        if let Some(data) = to_opt(UnitReliance::try_get_2(unit_a, unit_b)){
            let level = data.get_level().value;
            if level == 3 { AchieveData::add_count_reliance_a(); }
            else if level == 2 { AchieveData::add_count_reliance_b(); }
            AchieveData::set_clear_reliance(unit_a.get_person());
            AchieveData::set_clear_reliance(unit_b.get_person());
            collect_achievements();
        }
    }
    result
}

pub fn update_reliance() {
    if !crate::menus::config::menuitem::Support.get_flag_value() { return; }
    let player = Force::get(Force_Type::player());
    let absent = Force::get(Force_Type::absent());
    let units = player.iter().chain(absent.iter()).collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..units.len() {
        for j in i+1..units.len() {
            count += reliance_can_level_up_hook(units[i], units[j], None) as i32;
        }
    }
    if count != 0 { println!("[MyCodeMods] {} supports are leveled up", count); }
    else { println!("[MyCodeMods] No Supports are leveled up"); }
}

