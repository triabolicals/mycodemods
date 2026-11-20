use super::*;
use engage::app::{AchieveData, GameUserGmapData_EncountCounterType, HubUtil, IGameUserGmapDataMethods, IMuscleExercisePrizeDataMethods, ISingletonClass_1Methods, IUnit, IUnitEnhanceFactorsMethods, MuscleExercisePrizeData, UnitPool};
use unity::{Array, Cast};
use crate::menus::hub::{HubQuickMenu, HubQuickMenuItem, HubQuickMenuKind};

const PRIZE_LEVEL: [&'static str; 4] = ["Normal", "Hard", "Master", "Endless"];
const EXCERCISE: [&'static str; 3] = ["PushUp", "SitUp", "Squat"];
const STAT: [&'static str; 6] = ["HP", "力", "技", "HP", "Str", "Tec"];
pub fn do_exercise(this: ProcInst, kind: i32) {
    let new_proc = ProcInst::new();
    let level = get_current_unlocked_difficulty() as i8;
    GameUserData::get_instance().get_gmap_data().add_encount_counter(GameUserGmapData_EncountCounterType::minigame());
    if kind == 2 {
        match level {
            0 => AchieveData::play_report_add_count_squat_normal(),
            1 => AchieveData::play_report_add_count_squat_hard(),
            _ => AchieveData::play_report_add_count_squat_expert(),
        }
    }
    else if kind == 1 {
        match level {
            0 => AchieveData::play_report_add_count_ads_normal(),
            1 => AchieveData::play_report_add_count_ads_hard(),
            _ => AchieveData::play_report_add_count_ads_expert(),
        }
    }
    else {
        match level {
            0 => AchieveData::play_report_add_count_push_ups_normal(),
            1 => AchieveData::play_report_add_count_push_ups_hard(),
            _ => AchieveData::play_report_add_count_push_ups_expert(),
        }
    }
    collect_achievements();
    let kind = kind as i8;
    unity::field_set_value_at_offset::<i8>(new_proc, 0x1c, kind);
    unity::field_set_value_at_offset::<i8>(new_proc, 0x1d, level);
    GameVariableManager::set_number("G_MuscleExercise_GetPrize", 1);
    GameUserData::get_instance().get_gmap_data().add_encount_counter(GameUserGmapData_EncountCounterType::minigame());
    AchieveData::add_count_mini_game();

    let descs = [
        Proc::call_2(ProcVoidMethod::from_fn(new_proc, get_prize_bond).unwrap()),
        Proc::call_2(ProcVoidMethod::from_fn(new_proc, get_prize_bonus).unwrap()),
        Proc::call_2(ProcVoidMethod::from_fn(new_proc, restore_menu).unwrap()),
        Proc::end(),
    ];
    let array = Array::<ProcDesc>::from_slice(&descs).unwrap();
    new_proc.create_bind(this, array, "QuickExercise");
}
fn get_current_unlocked_difficulty() -> i32 {
    if HubUtil::get_is_condition_muscle_master_and_endless_opened() { 2 }
    else if HubUtil::get_is_condition_muscle_hard_opened() { 1 }
    else { 1 }
}
extern "C" fn get_prize_bond(this: ProcInst, _: OptionalMethod) {
    let kind = unity::field_get_value_at_offset::<i8>(this, 0x1c);
    let level = unity::field_get_value_at_offset::<i8>(this, 0x1d);
    let prize_id = format!("ID_Prize{}_{}", EXCERCISE[kind as usize], PRIZE_LEVEL[level as usize]);
    let prize = MuscleExercisePrizeData::get(prize_id.into());
    let bond = prize.get_bond_sss();
    add_bond_fragment_dialog(this, bond);
}
extern "C" fn get_prize_bonus(this: ProcInst, _: OptionalMethod) {
    let kind = unity::field_get_value_at_offset::<i8>(this, 0x1c);
    let level = unity::field_get_value_at_offset::<i8>(this, 0x1d);
    let prize_id = format!("ID_Prize{}_{}", EXCERCISE[kind as usize], PRIZE_LEVEL[level as usize]);
    let prize = MuscleExercisePrizeData::get(prize_id.into());
    let bonus = prize.get_bonus_sss().to_rust_string();
    if let Some((stat, amount)) = bonus.split_once("+"){
        if let Some((v, i)) = amount.parse::<i32>().ok().zip(STAT.iter().position(|&s| s == stat).filter(|&s| s < 3)){
            Mess::set_argument_2(0, Mess::get(format!("MID_SYS_{}", STAT[3+i])));
            Mess::set_argument(1, v);
            GameSound::post_event("Status_Up", Character::null());
            GameMessage::create_key_wait(this, Mess::get("MID_Hub_MuscleExercises_StatusUp"));
            let hero = UnitPool::get_hero(false);
            hero.m_enhance_factors().set_hub_value_2(i as i32, v);
        }
    }
}
extern "C" fn restore_menu(this: ProcInst, _: OptionalMethod) {
    if let Some(muscle_menu) = this.get_super().try_cast::<HubQuickMenu>(){
        if let Some(hub_fac_menu) = muscle_menu.get_super().try_cast::<HubQuickMenu>(){
            hub_fac_menu.update_menu_item(HubQuickMenuKind::Muscle, true);
        }
        IBasicMenuMethods::close_anime(muscle_menu);
        muscle_menu.close();
    }
}