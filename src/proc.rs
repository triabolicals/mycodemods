pub use engage::{
    ProcVoidMethodExt, ProcBoolMethodExt,
    app::{
        ISingletonClass_1Methods,
        proc::*, procinst::*, hubsequence::*,
        procdesc::*, procvoidmethod::*,
        gameuserdata::*, mess::*,
    },
    GameVariableManager,
};
use engage::{
    app::{BasicMenu, EventDemoSequence, HubVariable, IBasicMenu, IBitFieldTemplate32_1Methods, ISingletonProcInst_1Methods, ITitleBarMethods, LevelUpSequnece, MovieSequence, MyRoomRelianceSubSelect_MyRoomRelianceCallSequence},
    BasicMenuExt,
    List_1Ext,
    system::collections::generic::IList_1Methods
};
use engage::app::{AchieveData, AchieveData_Status, ExpSequence, GameMessage, GameSound, HubMenu, IAchieveDataMethods, IBasicMenuItemMethods, IBasicMenuMethods, IEventDemoSequence, IStructData_1Methods, MapSequenceBattleAction, UnitGrowSequence, Ut};
use engage::combat::Character;
use engage::root::ConfigMenu;
use unity::{Cast, ClassIdentity, FromIlInstance, IlNull, OptionalMethod};
use crate::{gifts, support, test};
use crate::gifts::add_hub_gift_key;
use crate::menus::config::menuitem::{ConfigMenuItemKind, MyCodeModConfigItem};
use crate::menus::hub::HubQuickMenu;
use crate::menus::sortie::skillinheritance::SortieSkillInheritanceMenuItem;
use crate::utils::to_opt;

pub mod arena;
pub mod mascot;
pub mod well;
pub mod fishing;
pub mod muscle;
pub mod cook;
pub mod animal;
pub(crate) mod collector;
pub(crate) mod market;
pub(crate) mod noticeboard;
mod reliance;
pub(crate) mod talk;
mod levelup;
pub mod exp;
pub(crate) mod map_human;
pub(crate) mod engrave;
pub(crate) mod dragonride;
pub(crate) mod map;
pub(crate) mod jukebox;

const SORTIE_SEQUENCE: i32 = -1624221522;
const MAP_SEQUENCE: i32 = 1650205480;
const HUB_SEQUENCE: i32 = 570083351;
const GMAP_SEQUENCE: i32 = 971901799;

pub extern "C" fn proc_null(_this: ProcInst, _optional_method: OptionalMethod){}

pub fn add_bond_fragment_dialog(this: ProcInst, add_bond_amount: i32) {
    Mess::set_argument_2(0, Mess::create_sprite_tag(Mess_IconCategory::system(), "Bonds"));
    Mess::set_argument_2(1, Ut::to_string_with_comma(add_bond_amount));
    let message = Mess::get("MID_MSG_GET_PIECE");
    GameSound::post_event("ItemGet", Character::null());
    GameMessage::create_key_wait(this, message);
    crate::utils::add_bond(add_bond_amount);
}
pub fn collect_achievements() {
    if !crate::menus::config::menuitem::Achievements.get_flag_value() { return; }
    let mut total = 0;
    AchieveData::get_list().iter()
        .filter(|v|{ let status = v.get_status().value; status == 1 || status == 2 })
        .for_each(|v|{
            total += v.get_kizuna_reward();
            v.set_status(AchieveData_Status::completed());
        });
    if total > 0 {
        let user_data = GameUserData::get_instance();
        let bond = user_data.get_piece_of_bond();
        user_data.set_piece_of_bond(bond + total);
    }
}
pub extern "C" fn proc_edit(this: ProcInst) {
    let hash = this.get_hash_code();
    // let parent = this.get_super();
    /*
    if !parent.is_null() {
        println!("Proc BIND: {} ({}), Parent: {} ({}) at {}", this.get_name(), this.get_hash_code(), parent.get_name(), parent.get_hash_code(), parent.get_desc_index());
    }
     */
    if hash == SORTIE_SEQUENCE || hash == MAP_SEQUENCE || hash == GMAP_SEQUENCE || hash == HUB_SEQUENCE {
        collect_achievements();
        support::update_reliance();
    }
    if this.try_cast::<MovieSequence>().is_some() && crate::menus::config::menuitem::Cutscene.get_flag_value() {
        skip_proc(this);
        return;
    }
    if let Some(event) = this.try_cast::<EventDemoSequence>() {
        let mess_name = event.m_mess_file_name().to_rust_string();
        if crate::menus::config::menuitem::Cutscene.get_flag_value() && !(mess_name.starts_with("R_") || mess_name.starts_with("GR_")) {
            skip_proc(this);
            return;
        }
    }
    if let Some(room) = this.try_cast::<MyRoomRelianceSubSelect_MyRoomRelianceCallSequence>(){
        reliance::edit_my_room_reliance_descs(room);
        return;
    }
    if let Some(hub) = this.try_cast::<HubSequence>() { // Gift
        add_hub_gift_key();
        hub.m_descs().set(36, Proc::call_2(ProcVoidMethod::from_fn(hub, gifts::hub_sequence_content).unwrap()));
    }
    if let Some(grow) = this.try_cast::<LevelUpSequnece>() {
        let grow_mode = crate::menus::config::menuitem::Growth.get_value();
        if grow_mode < 2 {
            GameUserData::get_instance().m_status().set_or_clear(grow_mode == 1, GameUserData_Statuses::fixed_grow());
            return;
        }
        else { grow.m_descs().set(1, Proc::call_2(ProcVoidMethod::from_fn(grow, levelup::level_up_sequence_effect).unwrap())); }
    }
    if let Some(hub_menu) = this.try_cast::<HubMenu>(){
        // test::recruit_all_clear_chapters();
        crate::menus::hub::hub_menu_edit(hub_menu);
    }
    if let Some(grow) = this.try_cast::<UnitGrowSequence>() { exp::unit_grow_sequence_edit(grow); }
    else if let Some(map_battle) = this.try_cast::<MapSequenceBattleAction>(){ map::map_sequence_battle_action_edit(map_battle); }
    else {
        let hash = this.get_hash_code();
        match hash {
            -1028322135 => {
                if let Some(menu) = this.try_cast::<ConfigMenu>(){
                    menu.m_full_menu_item_list().add(MyCodeModConfigItem::new(ConfigMenuItemKind::Menu).into());
                    menu.set_m_desc_index(0);
                    menu.set_m_show_row_num(9);
                    menu.rebuild();
                }
            }
            -845322556 => {
                if let Some(menu) = this.try_cast::<BasicMenu>(){
                    let skill = SortieSkillInheritanceMenuItem::instantiate().unwrap();
                    IBasicMenuItemMethods::ctor(skill);
                    menu.m_full_menu_item_list().add(skill.into());
                    menu.set_m_desc_index(0);
                    menu.set_m_show_row_num(menu.m_full_menu_item_list().count());
                    menu.rebuild();
                }
            }
            1525873615 => { map_human::map_sequence_human_edit(this); }
            -366846237 => {
                if !GameVariableManager::is_exist("G_Taste") { GameVariableManager::make_entry("G_Taste", -1); }
                else { GameVariableManager::set_number("G_Taste", -1); }
            }
            _ => {}
        }
    }

}

pub extern "C" fn open_menu(this: ProcInst, _optional_method: OptionalMethod) {
    if let Some(menu) = this.get_super().try_cast::<HubQuickMenu>() {
        IBasicMenuMethods::open_anime(menu);
        engage::app::TitleBar::get_instance().close_footer();
    }
    else if let Some(menu) = this.get_super().try_cast::<BasicMenu>() {
        BasicMenuExt::open_anime_all(menu);
        engage::app::TitleBar::get_instance().close_footer();
    }
}
pub fn skip_proc(this: ProcInst) {
    println!("Skipping: {}", this.get_name());
    this.set_m_desc_index(0);
    this.m_descs().set(0, Proc::end());
}

pub fn remove_menu_item_from_menu<C: ClassIdentity + FromIlInstance >(menu: BasicMenu) {
    if !menu.is_null() {
        let list = menu.m_full_menu_item_list();
        if !list.is_null() {
            if let Some(index) = list.iter().position(|item| item.try_cast::<C>().is_some() ) { list.remove_at(index as i32); }
        }
    }
}

pub fn is_hub_solanel() -> bool {
    to_opt(HubSequence::get_instance()).is_some() && HubVariable::get_current_scene_name().to_rust_string() == "Hub_Solanel"
}
