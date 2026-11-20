use super::*;
use engage::{
    app::{
        classchangejobmenu::*, classchange::*,
        basicmenuitem::*, basicmenu::*,
        classchangeroot::*,
        unitinfo::*, unitinfowindowcharaupdater::*, unitinfowindowcharamodel::*, unit::*,
        assettable::*,
        ISingletonProcInst_1Methods, IStructBase, ItemData, JobData,
        talk3_d::CharacterFactoryAsync_2
    },
    combat::{Character, ICharacterMethods},
    system::object::*,
    unity_engine::{Animator, IAnimatorMethods}
};
use unity::{Cast, FromIlInstance, IlNull, OptionalMethod};
use crate::{utils::override_vtable};

fn get_unit_cap_mode(this: ClassChangeJobMenu) -> u8 { unity::field_get_value_at_offset(this, 0xc4) }
fn set_is_grow_mode(this: ClassChangeJobMenu, mode: u8) { unity::field_set_value_at_offset(this, 0xc4, mode); }

pub fn class_change_vtable_edit() {
    override_vtable("App", "ClassChangeJobMenu.ClassChangeJobMenuItem", "OnSelect", class_change_job_menu_item_on_selected as _);
    override_vtable("App", "ClassChangeJobMenu.ClassChangeJobMenuItem", "XCall", class_change_job_menu_item_x_call as _);
    override_vtable("App", "ClassChangeJobMenu", "AfterBuild", class_change_menu_after_build as _);
    override_vtable("App", "ClassChangeJobMenu", "CustomCall", class_change_custom_call as _);
    if cobapi::injection::register::<UnitJobModelCreate>().is_ok() { println!("UnitJobModelCreate is Injected"); }
}
fn class_change_job_menu_item_x_call(this: ClassChangeJobMenu_ClassChangeJobMenuItem, _: OptionalMethod) -> BasicMenu_Result {
    if let Some(menu) = this.m_menu().try_cast::<ClassChangeJobMenu>() {
        let mode = get_unit_cap_mode(menu);
        if mode == 2 {
            let unit = ClassChangeJobMenu::get_unit();
            menu.m_root().set_unit_info_before(unit);
            menu.set_unit_info_after(unit, this.get_job_data());
        }
        set_unit_job_grow_status_setters(menu, this.get_job_data(), (mode + 1) % 3);
        set_is_grow_mode(menu, (mode + 1) % 3);
        BasicMenu_Result::se_cursor()
    }
    else { BasicMenu_Result::pass() }
}
fn class_change_menu_after_build(this: ClassChangeJobMenu, _: OptionalMethod) {
    unsafe { ClassChangeJobMenu::after_build(this) };
    let unit = ClassChangeJobMenu::get_unit();
    let idx = unit.get_person().index() as i16;
    if unity::field_get_value_at_offset::<i16>(this, 0xc6) != idx { unity::field_set_value_at_offset::<i16>(this, 0xc6, idx); }
    if let Some(item) = this.get_menu_item(this.m_select_index()).try_cast::<ClassChangeJobMenu_ClassChangeJobMenuItem>(){
        set_unit_job_grow_status_setters(this, item.get_job_data(), get_unit_cap_mode(this));
    }
}
fn class_change_custom_call(this: ClassChangeJobMenu, _: OptionalMethod) -> BasicMenu_Result {
    let result = unsafe { ClassChangeJobMenu::custom_call(this) };
    let idx = ClassChangeJobMenu::get_unit().get_person().index() as i16;
    if unity::field_get_value_at_offset::<i16>(this, 0xc6) != idx { unity::field_set_value_at_offset::<i16>(this, 0xc6, idx); }
    result
}

fn class_change_job_menu_item_on_selected(this: ClassChangeJobMenu_ClassChangeJobMenuItem, _: OptionalMethod){
    unsafe { ClassChangeJobMenu_ClassChangeJobMenuItem::on_select(this); }
    let unit = ClassChangeJobMenu::get_unit();
    if let Some(menu) = this.m_menu().try_cast::<ClassChangeJobMenu>() {
        let idx = unit.get_person().index() as i16;
        if unity::field_get_value_at_offset::<i16>(menu, 0xc6) != idx { unity::field_set_value_at_offset::<i16>(this, 0xc6, idx); }
        let grow = get_unit_cap_mode(menu);
        if grow != 0 { set_unit_job_grow_status_setters(menu, this.m_job_data(), grow); }
    }
    update_class_change_person(unit, this.m_job_data().get_job());
}
fn set_unit_job_grow_status_setters(menu: ClassChangeJobMenu, data: ClassChange_ChangeJobData, grow: u8) {
    let root = menu.m_root();
    if !root.is_null() {
        let new_unit = ClassChangeJobMenu::get_unit();
        let before = root.m_unit_info_before();
        let after = root.m_unit_info_after();
        if !before.is_null() {change_parameter_mode(before, new_unit, grow, true); }
        if !after.is_null() {
            if grow != 0 {
                new_unit.class_change(data.get_job(), ItemData::null());
                new_unit.set_selected_weapon(data.get_job_weapon_mask());
                new_unit.item_equip();
                new_unit.update_state();
            }
            change_parameter_mode(after, new_unit, grow, true);
        }
    }
}
fn update_class_change_person(unit: Unit, job: JobData) {
    if unit.is_null() || job.is_null() { return; }
    let info = UnitInfo::get_instance();
    let char_model_window = info.m_windows().get(0).m_unit_info_window_chara_model();
    unit.set_job(job);
    let result = AssetTable_Result::get_for_unit_info(unit);
    let character = CharacterFactoryAsync_2::create_common(result, "PID_不明", char_model_window.m_game_object(), false, false, false);
    let create_character_object = UnitJobModelCreate::instantiate().unwrap();
    create_character_object.set_character(character);
    create_character_object.set_unit_info_window(char_model_window);
    create_character_object.set_is_job(true);
    let action = engage::system::Action::new(create_character_object.into(), create_char_model_method_info().into());
    character.call_on_setup_done(action);
}
#[unity::inject(namespace = "MyCodeMods", name = "UnitJobModelCreate", parent=Object)]
pub struct UnitJobModelCreate {
    pub unit_info_window: UnitInfoWindowCharaModel,
    pub character: Character,
    pub is_job: bool,
    pub mount: bool,
    pub reset_animation: bool,
}

#[unity::callback]
pub fn create_char_model(this: UnitJobModelCreate, _: OptionalMethod) {
    let character = this.character();
    let unit_info_window = this.unit_info_window();
    if !character.is_null() && !unit_info_window.is_null() {
        let old_char = unit_info_window.m_chara();
        let body_states = AnimatorStates::new(old_char.get_body_animator());
        let face_states = AnimatorStates::new(old_char.get_face_animator());
        unit_info_window.delete_chara_model_2(old_char);
        unit_info_window.set_m_chara(character);
        let char = unit_info_window.create_chara_model_2(character);
        let update = unit_info_window.m_chara_updater();
        update.set_m_is_request_to_offset(true);
        update.try_update_offset(char);
        if !this.reset_animation() {
            if let Some(body) = body_states{ body.set_animator(char.get_body_animator()); }
            if let Some(face) = face_states { face.set_animator(char.get_face_animator()); }
        }
    }
}
pub struct AnimatorStates { pub states: Vec<(i32, i32, f32)>}
impl AnimatorStates {
    pub fn new(animator: Animator) -> Option<AnimatorStates> {
        if animator.is_null() { return None;}
        let n_layers = animator.get_layer_count();
        let mut states = vec![];
        for i in 0..n_layers {
            let state = animator.get_current_animator_state_info(i);
            states.push((i, state.m_full_path, engage::combat::Kaneko::fixed_time(state)));
        }
        Some(Self { states })
    }
    pub fn set_animator(&self, animator: Animator) {
        if animator.is_null() { return; }
        self.states.iter().for_each(|&(index, hash, fixed_time)|{ animator.play_in_fixed_time_4(hash, index, fixed_time) });
    }
}