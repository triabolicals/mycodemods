use engage::app::{BasicMenu_Result, noticeboardtopmenu::*, noticeboardsequence::*};
use super::*;
use unity::Il2CppString;

pub fn create_notice_board_bind(this: ProcInst) -> BasicMenu_Result {
    collect_achievements();
    let notice = NoticeBoardSequence::new();
    let descs = notice.create_desc();
    descs.set(5, Proc::call_2(ProcVoidMethod::from_fn(notice, create_top_notice_board).unwrap()));
    descs.set(28, Proc::call_2(ProcVoidMethod::from_fn(notice, notice_board_end_sequence).unwrap()));
    notice.create_bind(this, descs, Il2CppString::null());
    BasicMenu_Result::se_cursor()
}

extern "C" fn create_top_notice_board(this: NoticeBoardSequence, _optional_method: OptionalMethod) {
    this.create_top_menu();
    if let Some(menu) = to_opt(this.m_child()).and_then(|child| child.try_cast::<BasicMenu>()) {
        remove_menu_item_from_menu::<NoticeBoardTopMenu_SolanelInfoItem>(menu);
    }
}
extern "C" fn notice_board_end_sequence(this: NoticeBoardSequence, optional_method: OptionalMethod) {
    this.end_sequence();
    open_menu(this.into(), optional_method);
}