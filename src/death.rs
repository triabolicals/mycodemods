use engage::{
    app::{
        IChapterDataMethods,
        loadinglogo::*, unit::*, unitrecord::*, force::*,
        tutorialdata::*, IStructDataArray_1Methods,
    },
    unity_engine::{gameobject::*, component::*, },
    tm_pro::{ITMP_Text, ITMP_TextMethods},
    prelude::*,
};
use crate::utils::{enable_component, to_opt};

#[unity::hook("App", "LoadingLogo", "SetTipsData")]
pub fn loading_set_tip_text_hook(this: LoadingLogo, tips: u64, method_info: OptionalMethod){
    if crate::menus::config::menuitem::DeathList.get_flag_value(){
        let dead = Force::get(Force_Type::dead());
        let count = dead.iter().count();
        if count == 0 { return call_original!(this, tips, method_info); }
        enable_component(this.m_tips_unit_icon(), false);
        enable_component(this.m_skill_icon_image(), false);
        this.m_title_text().set_text_2(format!("{} Dead", count), true);
        let mut lines = vec![];
        dead.iter().for_each(|unit| {
            let mut name = unit.get_name().to_rust_string();
            let record = unit.m_record();
            let flags = record.get_dead_flag().value;
            if flags & 2 != 0{ name += " (Wounded)"; }
            if let Some(dead) = to_opt(unit.m_record().get_dead_chapter()) {
                let mut prefix = Mess::get(format!("{}_PREFIX", dead.get_name())).to_string();
                if flags & 1 != 0 { prefix.push_str(format!(" ({})", Mess::get("MID_SYS_EncounterBattleMap")).as_str()); }
                let dead_chapter_name = Mess::get(dead.get_name());
                lines.push(format!("{} {}: {}", name, prefix, dead_chapter_name));
            }
            else { lines.push(name); }
        });
        let text = lines.join("\n");
        this.m_tips_text().set_text_2(text, true);
    }
    else { call_original!(this, tips, method_info); }
}

pub fn set_tutorials_to_normal() {
    TutorialData::get_list().base().iter().for_each(|s| {
        s.base().base().iter().for_each(|s| {
            if s.get_notice().value == 2 { s.set_notice(TutorialData_Notices::normal()); }
        });
    })
}