use engage::app::{IBattleCalculatorMethods, IMapSequenceBattleAction, IMapSequenceBattleActionMethods, IProcInst, IProcInstMethods, MapSequenceBattleAction, Proc, ProcVoidMethod};
use engage::ProcVoidMethodExt;
use skyline::patching::Patch;
use unity::OptionalMethod;
use crate::menus::config::menuitem::{BattleAction, BGM};
use crate::proc::proc_null;

pub fn patch_imm_bgm(){
    if BGM.get_flag_value() {
        Patch::in_text(0x0228d414).bytes(&[0x01, 0x00, 0x80, 0x52]).unwrap();
        Patch::in_text(0x02d568dc).bytes(&[0x14, 0x00, 0x80, 0x52]).unwrap(); // mov w20, 0x0 force 0 
        Patch::in_text(0x02d568e0).nop().unwrap(); // conditional check
    }
    else {
        Patch::in_text(0x0228d414).bytes(&[0xe1, 0x03, 0x13, 0x2a]).unwrap();
        Patch::in_text(0x02d568dc).bytes(&[0x3f, 0x01, 0x14, 0x6b]).unwrap(); // mov w20, 0x0 force 0   3f 01 14 6b
        Patch::in_text(0x02d568e0).bytes(&[0x29, 0x02, 0x00, 0x54]).unwrap(); // conditional check 29 02 00 54
    }
}

pub fn map_sequence_battle_action_edit(this: MapSequenceBattleAction){
    if BattleAction.get_flag_value() {
        let descs = this.m_descs();
        [14, 15, 16, 22, 31].into_iter().for_each(|i| {
            descs.set(i, Proc::call_2(ProcVoidMethod::from_fn(this.into(), proc_null).unwrap()));
        });
        descs.set(23, Proc::call_2(ProcVoidMethod::from_fn(this, next_wait).unwrap()));
        descs.set(34, Proc::call_2(ProcVoidMethod::from_fn(this, battle_action_wait).unwrap()));
        descs.set(52, Proc::call_2(ProcVoidMethod::from_fn(this, battle_action_wait).unwrap()));
    }
}
extern "C" fn battle_action_wait(this: MapSequenceBattleAction, _: OptionalMethod) {
    if this.m_calculator().is_multi_battle() {
        this.r#yield();
        return;
    }
    this.set_m_wait_time(0.0);
}
extern "C" fn next_wait(this: MapSequenceBattleAction, _: OptionalMethod) {
    this.next_wait();
    if !this.m_calculator().is_multi_battle() { this.set_m_wait_time(0.15); }
}