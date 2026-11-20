use engage::app::IBasicMenuMethods;
use super::*;
use engage::app::myroomreliancesubselect::*;
use engage::app::pad::*;
use engage::nn::hid::NpadButton;

pub fn edit_my_room_reliance_descs(this: MyRoomRelianceSubSelect_MyRoomRelianceCallSequence) {
    let descs = this.m_descs();
    let skip = crate::menus::config::menuitem::Cutscene.get_flag_value();
    let pad = Pad::get_instance();
    let state = pad.m_npad_state().buttons.value;
    if skip == (state & 256 == 0) {
        descs.set(0, Proc::call_2(ProcVoidMethod::from_fn(this, update_talk_state).unwrap()));
        descs.set(1, Proc::call_2(ProcVoidMethod::new(this.into(), MyRoomRelianceSubSelect_MyRoomRelianceCallSequence::level_up_method_info().into())));
        descs.set(2, Proc::call_2(ProcVoidMethod::new(this.into(), MyRoomRelianceSubSelect_MyRoomRelianceCallSequence::level_up_after_method_info().into())));
        descs.set(3, Proc::end());
    }
}
extern fn update_talk_state(this: MyRoomRelianceSubSelect_MyRoomRelianceCallSequence, _optional_method: OptionalMethod) {
    if let Some(sub_select) = this.get_super().try_cast::<MyRoomRelianceSubSelect>() {
        sub_select.update_talk_state();
        sub_select.rebuild_instant(false);
    }
}