use super::*;
use engage::{
    app::{
        unit::*,
        expsequence::*
    },
};
use engage::app::{IJobDataMethods, IUnitGrowSequence, IUnitGrowSequenceMethods, UnitGrowSequence};
pub fn unit_grow_sequence_edit(this: UnitGrowSequence) {
    this.m_descs().set(1, Proc::call_2(ProcVoidMethod::from_fn(this, unit_grow_sequence_gain_exp).unwrap()));
}
extern "C" fn unit_grow_sequence_gain_exp(this: UnitGrowSequence, _: OptionalMethod) {
    let battle_action = (ConfigMenuItemKind::BattleAction.get_flag_value() || ConfigMenuItemKind::ExpSequence.get_flag_value()) && GameUserData::get_instance().get_sequence().value == 3;
    if ConfigMenuItemKind::Talk.get_value() & 2 != 0 { this.set_m_is_talk(false); }
    match ConfigMenuItemKind::Exp.get_value() {
        1 => { return; }
        2 => {
            let unit = this.m_unit();
            if unit.get_level() < unit.get_job().get_max_level() as i32 {
                let exp = 100 - unit.m_exp() as i32;
                let sp = Unit::nomralize_skill_point(unit, this.m_skill_point());
                let sp = if sp != 0 { exp } else { 0 };
                if battle_action {
                    unit.add_exp(exp);
                    unit.add_skill_point(sp);
                }
                else { ExpSequence::create_bind(this, unit, exp, sp); }
            }
        }
        _ => {
            let unit = this.m_unit();
            let exp = unit.normalize_exp(this.m_exp());
            let sp = Unit::nomralize_skill_point(unit, this.m_skill_point());
            if battle_action {
                unit.add_exp(exp);
                unit.add_skill_point(sp);
                return;
            }
            else if exp != 0 || sp != 0 { this.gain_exp(); }
        }
    }
}