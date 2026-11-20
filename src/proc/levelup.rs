use super::*;
use engage::app::{unit::*, levelupsequnece::*, ICapabilityBase_1Methods};

pub extern "C" fn level_up_sequence_effect(this: LevelUpSequnece, _optional_method: OptionalMethod) {
    if this.m_is_class_change() { return; }
    let unit = this.m_unit();
    let grow = this.m_grow();
    if !grow.is_null() {
        let grow_mode = crate::menus::config::menuitem::Growth.get_value();
        let new_cap = grow.get_base_capability();
        let old_cap = unit.get_base_capability();
        if grow_mode == 2 {
            for i in 0..11 { new_cap.set(i, old_cap.get(i)); }
            grow.update_state();
        }
        else if grow_mode == 3 {    // Perfect
            let mut d_level = this.m_level() - unit.m_level() as i32;
            if d_level < 1 { d_level = 1; }
            for i in 0..9 {
                let grow = ( d_level * unit.get_capability_grow(i, false) + 99) / 100;
                let stat = old_cap.get(i) as i32 + grow;
                let stat = if stat >= 120 { 120 } else if stat <= 0 { 0 } else { stat } as i8;
                new_cap.set(i, stat);
            }
            grow.update_state();
        }
    }
    this.effect();
}