use engage::{
    app::{
        HelpParamSetter,
        IBitField32Methods,
        IForceMethods,
        IHelpParamSetter,
        IMapSituation,
        IPersonDataMethods,
        ISingletonClass_1Methods,
        IStructDataArray_1Methods,
        MapSituation,
        unit::*,
        jobdata::*,
        aidata::*,
        unitai::*,
        aivalue::*
    },
    List_1Ext,
    Mess,
    tm_pro::ITMP_TextMethods
};
use engage::app::{GameUserData, IGameUserDataMethods};
use unity::{Cast, OptionalMethod};
use crate::utils::to_opt;

const AI_ORDER_TYPE: [&'static str; 4] = ["Activation", "Mind", "Attack", "Move"];
const AI_ARGS: [&'static str; 4] = ["V_0", "V_1", "V_2", "V_3"];
const AI_MOVE: [&'static str; 5] = ["Treasure", "Village", "Escape", "EscapeSlow", "Seize"];
#[skyline::hook(offset=0x0215a660)]
pub fn help_param_setter_set_person(this: HelpParamSetter, frame: u64, unit: Unit, method_info: OptionalMethod){
    call_original!(this, frame, unit, method_info);
    let sequence = GameUserData::get_instance().get_sequence().value;
    if sequence != 2 && sequence != 3 { return; }
    if crate::menus::config::menuitem::AI.get_flag_value() {
        if unit.is_null() || to_opt(unit.get_force()).is_none_or(|v| v.get_force_type().value != 1){
            return;
        }
        let ai = unit.m_ai();
        let mut ai_string = format!("{} Position ({}, {})", unit.get_pid(), unit.get_x(), unit.get_z());
        for x in 0..4 {
            let order = print_ai_order(ai, unit, x);
            if !order.is_empty() {
                ai_string += "\n";
                ai_string += AI_ORDER_TYPE[x as usize];
                ai_string += " - ";
                ai_string += order.as_str();
            }
        }
        ai_string += "\n";
        if let Some(flags) = print_flags(ai) { ai_string += format!("\nFlags: {}\n", flags).as_str(); }
        ai_string += format!("Band: {}\tActive: {}\tPriority: {}", ai.m_band(), ai.m_active(), ai.get_priority()).as_str();
        let move_limit = ai.get_move_limit();
        if move_limit.m_type() != 0 {
            let limit_str =
                match move_limit.m_type() {
                    1 => { "Move" },
                    2 => { "Distance" },
                    3 => { "Rectangle" },
                    4 => { "Interference Rod"},
                    _ => { "None" },
                };
            let x = move_limit.get_x();
            let x1 = x + move_limit.get_w();
            let z = move_limit.get_z();
            let z1 = move_limit.get_h();
            ai_string = format!("{}\nMove Limit: {} (x= {} to {}, z = {} to {})", ai_string, limit_str, x, x1, z, z1);
        }
        this.m_contents_text().set_text_2(ai_string, true);
    }
}
fn try_get_person(v: AIValue, exclude: bool) -> Option<String> {
    to_opt(v.get_person()).map(|p|format!("{}{} [{}]", if exclude { "Exclude: " } else { "" }, Mess::get(p.get_name()), p.get_pid()))
}
fn print_ai_order(this: UnitAI, unit: Unit, order: i32) -> String {
    let mut sequence = this.m_a_sequence().get(order as usize).to_rust_string();
    if let Some(a) = ["AI_AT_", "AI_AC_", "AI_MI_", "AI_MV_"].into_iter().find(|a| sequence.starts_with(a)){
        sequence = sequence.replace(a, "");
    }
    if sequence == "Null" { return String::new(); }
    let mut out = sequence.clone();
    if let Some(ai) = to_opt(AIData::get(this.m_a_sequence().get(order as usize))) {
        println!("AI Order: {} [{}]", order,ai.base().base().get(0).get_array_name());
        let mut ais: Vec<String> = vec![];
        ai.base().base().iter().for_each(|v|{
            let is_arg1 = v.get_value0().is_argument();
            let is_arg2 = v.get_value1().is_argument();
            let mind = v.get_mind();
            println!("MIND: {} / CODE: {}, ARG 1: {}, ARG 2: {}", mind, v.get_code(), is_arg1, is_arg2);
            let arg1 = if is_arg1 { v.get_value0().get_argument(AIValue_Order{value: order}, unit) } else { AIValue::new() };
            let arg2 = if is_arg2 { v.get_value1().get_argument(AIValue_Order{value: order}, unit) } else { AIValue::new() };
            if v.get_code() == 1 {
                match mind {
                    2 | 6 => { if let Some(a) = try_get_person(arg2, true) { ais.push(a); } }
                    8 => {}
                    9 => {
                        let (x, z) = (arg1.get_x(), arg1.get_z());
                        let (w, h) = (arg2.get_x(), arg2.get_z());
                        ais.push(format!("Area: x={}to{}, z={}to{}", x, x + w, z, z + h));
                    }
                    10 => { ais.push(format!("Turn: {} [Current: {}]", arg1.get_value(), MapSituation::get_instance().m_turn())); }
                    11 => { ais.push(format!("Flag {} is True ({})", arg1.get_string(), arg1.get_flag_value() != 0)); }
                    12 => { ais.push(format!("Flag {} is False ({})", arg1.get_string(), arg1.get_flag_value() != 0)); }
                    _ => {}
                }
            }
            else if v.get_code() == 3 {
                match mind {
                    4|5 => {
                        if let Some(a) = try_get_person(arg1, mind == 5) { ais.push(a); }
                    }
                    6 => {
                        if !arg1.is_default() { ais.push(format!("Exclude Band: {}", arg1.get_value())); }
                    }
                    7|8 => {
                        if !arg1.is_default() {
                            if let Some(job) = to_opt(arg1.get_job()) { ais.push(format!("Class: {}", Mess::get(job.get_name()))); }
                        }
                    }
                    9|92 => {
                        let v = arg1.get_value();
                        if v < 3 && v >= 0 { ais.push(format!("Force({})", if v == 0 { "Player"} else if v == 1 { "Enemy"} else { "Ally"})); }
                    }
                    50 => {
                        if this.m_prohibit_engage_attack() > 0 && !this.check_flag(UnitAI_Flag::engage_attack_once_done()){
                            ais.push("EngAtk Available".to_string());
                        }
                    }
                    61..66 => {
                        if !arg1.is_default() { ais.push(format!("{} at ({}, {})", AI_MOVE[mind as usize - 61], arg1.get_x(), arg1.get_z())); }
                    }
                    71..75 => {
                        if let Some(a) = try_get_person(arg1, false) { ais.push(a); }
                    }
                    93 => {
                        if let Some(a) = try_get_person(arg1, true) { ais.push(a); }
                    }
                    94..97 => { // Move Force
                        if !arg1.is_default() {
                            ais.push(format!("{} at ({}, {})", AI_MOVE[mind as usize - 94], arg1.get_x(), arg1.get_z()));
                        }
                    }
                    _ => {}
                }
            }
        });
        if !ais.is_empty() {
            let mut iter = ais.iter();
            if let Some(a) = iter.next() {
                out += ": ";
                out += a.as_str();
                while let Some(a) = iter.next() {
                    out += ", ";
                    out += a.as_str();
                }
            }
        }
    }
    out
}
fn print_flags(ai: UnitAI) -> Option<String> {
    let v = ai.m_flag().get_value();
    if v == 0 { None}
    else {
        let mut out = String::new();
        if v & 1073741824 != 0 { out += " EngAtkOnce (Done)"; }
        else if v & 536870912 != 0 { out += " EngAtkOnce"; }
        if v & 256 != 0 { out += " ThinkNoMove"; }
        if v & 917504 != 0 {
            out += " BandActivation";
            if v & 262144 != 0 {
                out += "(Move";
                if v & 524288 != 0 { out += ",Attacked"; }
                out += ")";
            }
            else if v & 524288 != 0 { out += "Attacked)"; }
        }
        if v & 7168 != 0 {
            let mut move_kinds = String::new();
            if v & 1024 != 0 { move_kinds  += "_Break"; }
            if v & 2048 != 0{ move_kinds += "_Through"; }
            if v & 4096 != 0{ move_kinds += "_Slow"; }
            if v & 8192 != 0 { move_kinds += "_Atk"; }
            move_kinds = move_kinds.replacen("_", "", 1);
            move_kinds = move_kinds.replace("_", ",");
            out += " Move(";
            out += move_kinds.as_str();
            out += ")";
        }
        if v & 251658240 != 0 {
            let mut target_kinds = String::new();
            if v & 16777216 != 0 { target_kinds += "_Atk"; }
            if v & 33554432 != 0 { target_kinds += "_Heal"; }
            if v & 67108864 != 0 { target_kinds += "_Inter";}
            if v & 134217728 != 0 { target_kinds += "_Cannon";}
            target_kinds = target_kinds.replacen("_", "", 1);
            target_kinds = target_kinds.replace("_", ",");
            out += format!(" Target({})", target_kinds).as_str();
        }
        if v & 16384 != 0 { out += " Break"; }
        if v & 32768 != 0 { out += " Chain"; }
        if v & 65536 != 0 { out += " AutoRange" }
        if v & 512 != 0 { out += " NoDmgAtk"; }
        if out.is_empty() { None } else { Some(out) }
    }
}
